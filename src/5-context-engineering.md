# LLM 架构设计原则：上下文工程

上个月在计划为 AutoDev 添加多语言支持时候，发现 GitHub Copilot 的插件功能是语言无关的（通过 plugin.xml 分析），便想研究一下它是如何使用
TreeSitter 的。可惜的是，直到最近才有空，研究一下它是如何实现的。

在探索的过程中，发现：Copilot 围绕上下文做了非常之多的工作，便想着写一篇文章总结一下。

## GitHub Copilot 的上下文构建

与 ChatGPT 相比，GitHub Copilot 的强大之处在于，它构建了足够多的上下文，结合其对 LLM 的训练（或微），可以写出非常精准的**生产级代码
**。

### Copilot 的可见上下文

在肉眼可见的级别里，即我们自身的使用感受，我们可以发现 Copilot 不仅是读取当前文件的源码，而是一系列相关文件的源码，以构建更详细的上下文。

简单可以先划分三个场景：

- 当前文件。可以感知某个类的属性和方法，并做出自动填充。
- 相近文件。如测试文件，可以知道被测类的信息，并自动编写用例。
- 编辑历史（疑似）。即当我们以某种方式修改多个代码时，它也能识别出这个变化。

而在未来，相信它会获取诸如项目上下文等信息，如 Gradle 依赖、NPM 依赖等信息，避免在打开的 tab 不够用的情况下，引用不存在的依赖。

而针对于企业自身的 AI 编程工具而言，还可以结合服务上下文、业务上下文进行优化。

### Copilot 的不可见过程

结合网上的逆向工程资料，以及自己对代码的 debug 尝试，最后梳理了一个大致的 “四不像” （实在是懒得继续画）架构图。

其作用如下：

- 监听用户操作（IDE API ）。监听用户的 Run Action、快捷键、UI 操作、输入等，以及最近的文档操作历史。
- IDE 胶水层（Plugin）。作为 IDE 与底层 Agent 的胶水层，处理输入和输出。
- 上下文构建（Agent）。JSON RPC Server，处理 IDE 的各种变化，对源码进行分析，封装为 “prompt” （疑似） 并发送给服务器。
- 服务端（Server）。处理 prompt 请求，并交给 LLM 服务端处理。

而在整个过程中，最复杂的是在 Agent 部分，从上下文中构建出 prompt。

### Copilot 的 Prompt 与上下文

在 “公开” 的 Copilot-Explorer 项目的研究资料里，可以看到 Prompt 是如何构建出来的。如下是发送到的 prompt 请求：

```kotlin
{
  "prefix": "# Path: codeviz\\app.py\n#....",
  "suffix": "if __name__ == '__main__':\r\n    app.run(debug=True)",
  "isFimEnabled": true,
  "promptElementRanges": [
    { "kind": "PathMarker", "start": 0, "end": 23 },
    { "kind": "SimilarFile", "start": 23, "end": 2219 },
    { "kind": "BeforeCursor", "start": 2219, "end": 3142 }
  ]
}
```

其中：

- 用于构建 prompt 的 `prefix` 部分，是由 promptElements
  构建了，其中包含了：`BeforeCursor`, `AfterCursor`, `SimilarFile`, `ImportedFile`, `LanguageMarker`, `PathMarker`, `RetrievalSnippet`
  等类型。从几种 `PromptElementKind` 的名称，我们也可以看出其真正的含义。
- 用于构建 prompt 的 `suffix` 部分，则是由光标所在的部分决定的，根据 tokens 的上限（2048 ）去计算还有多少位置放下。而这里的
  Token 计算则是真正的 LLM 的 token 计算，在 Copilot 里是通过 Cushman002 计算的，诸如于中文的字符的 token
  长度是不一样的，如： `{ context: "console.log('你好，世界')", lineCount: 1, tokenLength: 30 }` ，其中 context 中的内容的
  length 为 20，但是 tokenLength 是 30，中文字符共 5 个（包含 `，` ）的长度，单个字符占的 token 就是 3。

到这里，我算是解决我感兴趣的部分，Agent 包里的 TreeSitter 则用于分析源码，生成 `RetrievalSnippet` ，其中支持语言是 Agent
自带的 `.wasm` 相关的包，诸如：Go、JavaScript、Python、Ruby、TypeScript 语言。

## LLM 的上下文工程

上下文工程是一种让 LLM 更好地解决特定问题的方法。它的核心思想是，通过给 LLM
提供一些有关问题的背景信息，比如指令、示例等，来激发它生成我们需要的答案或内容。上下文工程是一种与 LLM 有效沟通的技巧，它可以让
LLM 更准确地把握我们的目的，并且提升它的输出水平。

简而言之，上下文工程是如何在有限的 token 空间内，传递**最相关的上下文信息**。

所以，我们就需要定义什么是该场景下的，**最相关的上下文信息**。

### 基于场景与旅程的上下文设计

它的基本思想是，通过分析用户在不同场景下的操作和行为，来获取与当前任务相关的上下文信息，从而指导 LLM 生成最佳的代码提示。

Copilot 分析了用户在不同场景下的操作和行为，如何使用 IDE 的旅程，以及与当前任务相关的指令和例子等信息，从而获取最相关的上下文信息。这些上下文信息可以帮助
LLM 更好地理解用户的意图，并生成更准确、更有用的代码提示。

例如，在用户编写 JavaScript
代码时，Copilot会分析用户在编辑器中的光标位置、当前文件的内容、变量、函数等信息，以及用户的输入历史和使用习惯等上下文信息，来生成最相关的代码提示。这些代码提示不仅能够提高用户的编码效率，还能够帮助用户避免常见的编程错误。

### 就地矢量化（Vector）与相似度匹配

“众知周知”，在 LLM 领域非常火的一个工具是 LangChain，它的处理过程类似于 langchain-ChatGLM 总结的：


> 加载文件 -> 读取文本 -> 文本分割 -> 文本向量化 -> 问句向量化 -> 在文本向量中匹配出与问句向量最相似的`top k`个 ->
> 匹配出的文本作为上下文和问题一起添加到`prompt`中 -> 提交给`LLM`生成回答。
>

为了处理大规模的自然语言处理任务，Copilot 在客户端使用了 Cushman + ONNX 模型处理。具体来说，Copilot 将 Cushman
模型的输出转换为向量表示，然后使用向量相似度计算来匹配最相关的本地文件。

除了就地矢量化（Vector）与相似度匹配，Copilot 还使用了本地的相似计算与 token 处理来管理 token，以便更好地处理大规模自然语言处理任务。

### 有限上下文信息的 Token 分配

而由于 LLM 的处理能力受到 token 数的限制，如何在有限的 token 范围内提供最相关的上下文信息，便是另外一个重要的问题。

诸如于如上所述的 Copilot 本地 prompt 分为了 prefix 和 suffix 两部分，在 suffix 部分需要配置
suffixPercent，其用于指定在生成代码提示时要用多少 prompt tokens 来构建后缀，默认值似乎是 15%。

通过增加 suffixPercent，可以让 Copilot 更关注当前正在编写的代码片段的上下文信息，从而生成更相关的代码提示。而通过调整
fimSuffixLengthThreshold，可以控制 Fill-in-middle 的使用频率，从而更好地控制生成的代码提示的准确性。

## 结论

GitHub Copilot 可以在有限的 token 范围内提供最相关的上下文信息，从而生成更准确、更有用的代码提示。这些策略提供了一定的灵活性，用户可以根据自己的需要来调整
Copilot 的行为，从而获得更好的代码自动补全体验。

我们跟进未来的路，依旧很长。

Copilot 逆向工程相关资料：

- [https://github.com/thakkarparth007/copilot-explorer](https://github.com/thakkarparth007/copilot-explorer)
- [https://github.com/saschaschramm/github-copilot](https://github.com/saschaschramm/github-copilot)

其它相关资料：

- [https://github.com/imClumsyPanda/langchain-ChatGLM](https://github.com/imClumsyPanda/langchain-ChatGLM)
