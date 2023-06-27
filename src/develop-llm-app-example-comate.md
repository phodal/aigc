# LLM 应用示例：Co-mate 难点解析

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

## 客户端 API 调用：TypeScript 实现

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
