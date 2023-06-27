# LLM 应用示例：Co-mate 难点解析

## LLM 应用开发模式：Stream 封装

Kotlin 实现

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

TypeScript 实现

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
