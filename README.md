# 构筑大语言模型应用：应用开发与架构设计

> aka. Unlocking the Potential of Large Language Models: Real-World Use Cases

2023 年的上半年里，我（@phodal）和 Thoughtworks
的同事们（如：@[tianweiliu](https://github.com/tianweiliu)、@[teobler](https://github.com/teobler)、@[mutoe](https://github.com/mutoe) 等）、
开源社区的同伴们（如：
卷王@[CGQAQ](https://github.com/CGQAQ)、@[genffy](https://github.com/genffy)、 @[liruifengv](https://github.com/liruifengv)
等)
一起，创建了一系列的流行的或者不流行的开源项目。它们涉及了：

- Prompt 的编写、开发和管理
- 最好的大语言模型能带来什么？
    - 探索未来的软件开发架构：Unit Mesh
    - 基于 AI 2.0 （ChatGPT）的应用开发探索
    - 基于 AI 2.0 （ChatGPT + Copilot）如何去设计软件开发流程
- 如何基于开源模型构建自己的模型
    - 基于 LLaMA、ChatGLM 的微调
    - 开源大语言模型 + 软件开发生命周期的结合
- LLM 应用架构的设计与落地
    - 如何设计一个基于 LLM 的应用架构
    - 如何将 LLM 应用到软件开发中

围绕于上述的一系列内容，我们也在思考软件开发能给我们带来了什么。所以，我重新整理了过去半年的一些思考、文章，重新编写了这本开源电子书，希望能够帮助到大家。

关注我的微信公众号（搜索 phodal-weixin），获得更多及时的更新：

![微信公众号](src/images/qrcode.jpg)

我们发起的相关开源项目如下：

| 名称                                                                   | 描述                                                                                       | 类型          | Stars |
|----------------------------------------------------------------------|------------------------------------------------------------------------------------------|-------------|-------|
| [理解 Prompt](https://github.com/prompt-engineering/understand-prompt) | 基于编程、绘画、写作的 AI 探索与总结。                                                                    | 文档          | 3k    |
| [Prompt 编写模式](https://github.com/prompt-engineering/prompt-patterns) | 如何将思维框架赋予机器，以设计模式的形式来思考 prompt。                                                          | 文档          | 2.1 k |
| [ClickPrompt](https://github.com/prompt-engineering/click-prompt)    | 用于一键轻松查看、分享和执行您的 Prompt。                                                                 | 应用          | 1.6k  |
| [ChatFlow](https://github.com/prompt-engineering/chat-flow)          | 打造个性化 ChatGPT 流程，构建自动化之路。                                                                | 框架          | 570   |
| [Unit Mesh](https://github.com/unit-mesh/unit-mesh)                  | 基于 AI 为核心的软件 2.0 思想的软件架构。                                                                | 架构          | 121   | 
| [Unit Minions](https://github.com/unit-mesh/unit-minions)            | AI 研发提效研究：自己动手训练 LoRA                                                                    | 微调教程、指南、数据集 | 712   |
| [DevTi](https://github.com/unit-mesh/devti)                          | 基于 LLM 的微调来提供全面智能化解决方案，助力开发人员高效完成开发任务，以实现自动化用户任务拆解、用户故事生成、自动化代码生成、自动化测试生成等等。             | 微调代码        | 102   |
| [AutoDev](https://github.com/unit-mesh/auto-dev)                     | 一款 Intellij IDEA 的 LLM/AI 辅助编程插件。AutoDev 能够与您的需求管理系统（例如 Jira、Trello、Github Issue 等）直接对接。 | IDEA 插件     | 207   |
| [ArchGuard Co-mate](https://github.com/archguard/co-mate)            | 基于人工智能技术的架构副驾驶、设计和治理工具                                                                   | 架构协同应用      | 25    |

我们在 QCon
上的演讲：[演讲：探索软件开发新工序：LLM 赋能研发效能提升](https://qcon.infoq.cn/2023/guangzhou/presentation/5319)

> LLM（如 ChatGPT + GitHub
> Copilot）作为一种创新的工具组合，为我们带来了全新的机遇。它能够帮助业务人员和开发者在需求、架构、编码、测试等环节提高效率和质量，实现从设计到验证的端到端流程。在本次分享中，我将向大家介绍
> LLM 在研发效能方面的应用场景和实践案例，展示它是如何在各个环节中发挥作用的。同时，我们还将分享如何构建私有化的 LLM
> 工程化方式，使其更好地适应组织的需求。欢迎对 LLM + 研发效能感兴趣的朋友们参加本次分享，与我们一起探讨研发效能的未来。

我们在 Bilibili 上的大语言模型微调相关的视频：

- LLaMA
  系列在线视频： 《[代码辅助生成](https://www.bilibili.com/video/BV1Rh411u74H/)》 、《[测试代码生成](https://www.bilibili.com/video/BV1jg4y1G7Xc/)》 、《[详细需求生成](https://www.bilibili.com/video/BV1Us4y1N7rd/)》 、《[文本转 SQL](https://www.bilibili.com/video/BV1uv4y1H7bg/)》
- ChatGLM 系列在线视频： 《[LoRA 大比拼：ChatGLM vs LLaMA，谁更会写需求文档？](https://www.bilibili.com/video/BV1fv4y1n7Y3/)》

欢迎大家一起来参与我们的开源项目，一起来探索 LLM + 软件开发的未来。
