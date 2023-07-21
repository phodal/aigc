# LLM 应用示例：最佳实践示例

## LLM 应用开发模式：轻量级 API 编排

在 LangChain 中使用了思维链的方式来选择合适的智能体（Agent），在 Co-mate 中，我们也是采取了类似的设计，在本地构建好函数，然后交由
LLM 来分析用户的输入适合调用哪个函数。

如下是我们的 prompt 示例：

```
Answer the following questions as best you can. You have access to the following tools:

introduce_system: introduce_system is a function to introduce a system.

Use the following format:

Question: the input question you must answer
Thought: you should always think about what to do
Action: the action to take, should be one of [introduce_system]
Action Input: the input to the action
Observation: the result of the action
... (this Thought/Action/Action Input/Observation can repeat N times)
Thought: I now know the final answer
Final Answer: the final answer to the original input question

Begin!

Question: Introduce the following system: https://github.com/archguard/ddd-monolithic-code-sample
```

这里的 `Question` 便是用户的输入，然后再调用对应的 `introduce_system` 函数进行分析。

## LLM 应用开发模式：DSL 动态运行时 

与事实能力相比，我们更信任 LLM 的编排能力，因此我们在 Co-mate 中采用了 DSL 的方式来编排函数，这样可以更加灵活的编排函数。

为了支撑这样的能力，我们在 Co-mate 中引入了 Kotlin 作为 DSL 的运行时：

```kotlin
// 初始化运行时
val repl = KotlinInterpreter()
val mvcDslSpec = repl.evalCast<FoundationSpec>(InterpreterRequest(code = mvcFoundation))

// 从用户的输入中获取 action
val action = ComateToolingAction.from(action.lowercase())

// 添加默认的 DSL spec
if (action == ComateToolingAction.FOUNDATION_SPEC_GOVERNANCE) {
    comateContext.spec = mvcDslSpec
}
```

对应的 DSL 示例（由 ChatGPT 根据 DDD 版本 spec 生成）：

```kotlin
foundation {
    project_name {
        pattern("^([a-z0-9-]+)-([a-z0-9-]+)(-common)?\${'$'}")
        example("system1-webapp1")
    }

    layered {
        layer("controller") {
            pattern(".*\\.controller") { name shouldBe endsWith("Controller") }
        }
        layer("service") {
            pattern(".*\\.service") {
                name shouldBe endsWith("DTO", "Request", "Response", "Factory", "Service")
            }
        }
        layer("repository") {
            pattern(".*\\.repository") { name shouldBe endsWith("Entity", "Repository", "Mapper") }
        }

        dependency {
            "controller" dependedOn "service"
            "controller" dependedOn "repository"
            "service" dependedOn "repository"
        }
    }

    naming {
        class_level {
            style("CamelCase")
            pattern(".*") { name shouldNotBe contains("${'$'}") }
        }
        function_level {
            style("CamelCase")
            pattern(".*") { name shouldNotBe contains("${'$'}") }
        }
    }
}
```

## LLM 应用开发模式：本地小模型

在 Co-mate 中，我们在本地引入了 SentenceTransformer 来处理用户的输入，优在本地分析、匹配用户的输入，并处理。当匹配到结果后直接调用本地的函数，当匹配不到结果时调用远端的处理函数来处理。

HuggingFace: [https://huggingface.co/sentence-transformers](https://huggingface.co/sentence-transformers)

在原理上主要是参考了 GitHub Copilot、 Bloop 的实现，通过本地的小模型来处理用户的输入，然后再通过远端的大模型来处理用户的输入。

### Rust 实现示例

Rust 相关示例：[https://github.com/unit-mesh/unit-agent](https://github.com/unit-mesh/unit-agent)

```rust
pub fn embed(&self, sequence: &str) -> anyhow::Result<Embedding> {
    let tokenizer_output = self.tokenizer.encode(sequence, true).unwrap();

    let input_ids = tokenizer_output.get_ids();
    let attention_mask = tokenizer_output.get_attention_mask();
    let token_type_ids = tokenizer_output.get_type_ids();
    let length = input_ids.len();
    trace!("embedding {} tokens {:?}", length, sequence);

    let inputs_ids_array = ndarray::Array::from_shape_vec(
        (1, length),
        input_ids.iter().map(|&x| x as i64).collect(),
    )?;

    let attention_mask_array = ndarray::Array::from_shape_vec(
        (1, length),
        attention_mask.iter().map(|&x| x as i64).collect(),
    )?;

    let token_type_ids_array = ndarray::Array::from_shape_vec(
        (1, length),
        token_type_ids.iter().map(|&x| x as i64).collect(),
    )?;

    let outputs = self.session.run([
        InputTensor::from_array(inputs_ids_array.into_dyn()),
        InputTensor::from_array(attention_mask_array.into_dyn()),
        InputTensor::from_array(token_type_ids_array.into_dyn()),
    ])?;

    let output_tensor: OrtOwnedTensor<f32, _> = outputs[0].try_extract().unwrap();
    let sequence_embedding = &*output_tensor.view();
    let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();

    Ok(pooled.to_owned().as_slice().unwrap().to_vec())
}
```

### Kotlin 实现示例

```kotlin
class Semantic(val tokenizer: HuggingFaceTokenizer, val session: OrtSession, val env: OrtEnvironment) {
    fun embed(
        sequence: String,
    ): FloatArray {
        val tokenized = tokenizer.encode(sequence, true)

        val inputIds = tokenized.ids
        val attentionMask = tokenized.attentionMask
        val typeIds = tokenized.typeIds

        val tensorInput = OrtUtil.reshape(inputIds, longArrayOf(1, inputIds.size.toLong()))
        val tensorAttentionMask = OrtUtil.reshape(attentionMask, longArrayOf(1, attentionMask.size.toLong()))
        val tensorTypeIds = OrtUtil.reshape(typeIds, longArrayOf(1, typeIds.size.toLong()))

        val result = session.run(
            mapOf(
                "input_ids" to OnnxTensor.createTensor(env, tensorInput),
                "attention_mask" to OnnxTensor.createTensor(env, tensorAttentionMask),
                "token_type_ids" to OnnxTensor.createTensor(env, tensorTypeIds),
            ),
        )

        val outputTensor = result.get(0) as OnnxTensor
        val output = outputTensor.floatBuffer.array()

        return output
    }


    companion object {
        fun create(): Semantic {
            val classLoader = Thread.currentThread().getContextClassLoader()

            val tokenizerPath = classLoader.getResource("model/tokenizer.json")!!.toURI()
            val onnxPath =  classLoader.getResource("model/model.onnx")!!.toURI()

            try {
                val env: Map<String, String> = HashMap()
                val array: List<String> = tokenizerPath.toString().split("!")
                FileSystems.newFileSystem(URI.create(array[0]), env)
            } catch (e: Exception) {
//                e.printStackTrace()
            }

            val tokenizer = HuggingFaceTokenizer.newInstance(Paths.get(tokenizerPath))
            val ortEnv = OrtEnvironment.getEnvironment()
            val sessionOptions = OrtSession.SessionOptions()

            // load onnxPath as byte[]
            val onnxPathAsByteArray = Files.readAllBytes(Paths.get(onnxPath))

            val session = ortEnv.createSession(onnxPathAsByteArray, sessionOptions)

            return Semantic(tokenizer, session, ortEnv)
        }
    }
}
```

## LLM 应用开发模式：Stream 封装

### 服务端 API 调用：Kotlin 实现

机制：结合 callbackFlow 来实现

```kotlin
fun stream(text: String): Flow<String> {
    val systemMessage = ChatMessage(ChatMessageRole.USER.value(), text)

    messages.add(systemMessage)

    val completionRequest = ChatCompletionRequest.builder()
        .model(openAiVersion)
        .temperature(0.0)
        .messages(messages)
        .build()

    return callbackFlow {
        withContext(Dispatchers.IO) {
            service.streamChatCompletion(completionRequest)
                .doOnError(Throwable::printStackTrace)
                .blockingForEach { response ->
                    val completion = response.choices[0].message
                    if (completion != null && completion.content != null) {
                        trySend(completion.content)
                    }
                }

            close()
        }
    }
}
```

### 客户端 API 调用：TypeScript 实现

机制：依赖于 Vercel 的 AI 库，提供对于 Stream 的封装

```typescript
import { Message, OpenAIStream, StreamingTextResponse } from 'ai'
import { Configuration, OpenAIApi } from 'openai-edge'

export async function stream(apiKey: string, messages: Message[], isStream: boolean = true) {
  let basePath = process.env.OPENAI_PROXY_URL
  if (basePath == null) {
    basePath = 'https://api.openai.com'
  }

  const configuration = new Configuration({
    apiKey: apiKey || process.env.OPENAI_API_KEY,
    basePath
  })

  const openai = new OpenAIApi(configuration)

  const res = await openai.createChatCompletion({
    model: 'gpt-3.5-turbo',
    messages,
    temperature: 0.7,
    stream: isStream
  })

  if (!isStream) {
    return res
  }

  const stream = OpenAIStream(res, {})

  return new StreamingTextResponse(stream)
}
```

### 客户端 UI 实现：Fetch

```typescript
const decoder = new TextDecoder()

export function decodeAIStreamChunk(chunk: Uint8Array): string {
  return decoder.decode(chunk)
}

await fetch("/api/action/tooling", {
  method: "POST",
  body: JSON.stringify(tooling),
}).then(async response => {
  onResult(await response.json())
  let result = ""
  const reader = response.body.getReader()
  while (true) {
    const { done, value } = await reader.read()
    if (done) {
      break
    }

    result += decodeAIStreamChunk(value)
    onResult(result)
  }

  isPending = false
});
```

### 服务端实现转发： Java + Spring

```java
@RestController
public class ChatController {

    private WebClient webClient = WebClient.create();

    @PostMapping(value = "/api/chat")
    public Flux<StreamingResponseBody> chat(@RequestBody ChatInput input) {
        return webClient.post()
                .uri("http://127.0.0.1:8000/api/chat")
                .bodyValue(input)
                .retrieve()
                .bodyToFlux(String.class)
                .map(response -> outputStream -> {
                    outputStream.write(response.getBytes());
                    outputStream.flush();
                });
    }
}
```

### 服务端转发：Python

```python

app = FastAPI()

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

# Initialize OpenAI client
openai.api_key = OPENAI_API_KEY


class ChatInput(BaseModel):
    message: str


error503 = "OpenAI server is busy, try again later"
openai_model = "gpt-3.5-turbo"
max_responses = 1
temperature = 0.7
max_tokens = 8192


def generate_reply_stream(input_data: ChatInput):
    prompt = input_data.message
    try:
        prompt = prompt
        response = openai.ChatCompletion.create(
            model=openai_model,
            temperature=temperature,
            max_tokens=max_tokens,
            n=max_responses,
            top_p=1,
            frequency_penalty=0,
            presence_penalty=0,
            messages=[
                {"role": "system",
                 "content": "You are an expert creative marketer. Create a campaign for the brand the user enters."},
                {"role": "user", "content": prompt},
            ],
            stream=True,
        )
    except Exception as e:
        print("Error in creating campaigns from openAI:", str(e))
        raise HTTPException(503, error503)
    try:
        for chunk in response:
            current_content = chunk["choices"][0]["delta"].get("content", "")
            yield current_content

    except Exception as e:
        print("OpenAI Response (Streaming) Error: " + str(e))
        raise HTTPException(503, error503)


@app.post("/api/chat", response_class=Response)
async def chat(input_data: ChatInput):
    return StreamingResponse(generate_reply_stream(input_data), media_type="text/event-stream")
```

