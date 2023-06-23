# Workflow + AI 2.0

## 高质量流程



## ChatFlow 的诞生动机：人类设计高质量流程 + AI 完成细节

在我使用了 ChatGPT （GPT 3.5）一个月多月之后，大抵算是掌握了它的脾气。简单来说，ChatGPT 即是一个硅基生物，也是一个非常好的人类助手。作为一个工具，你使用 prompt 的能力决定了它的上限和下限。

简单来说，ChatGPT 在经验丰富的开发人员手中，有一定 prompt 经历的人手中，会发挥非常强大的作用。而对于经验不那么丰富的开发人员，可能会因为缺乏任务分解能力，无法写出合理地 prompt，让 AI 有创意地瞎写代码。

诸如于，我们可以通过如下的注释，让 ChatGPT 或者 GitHub Copilot 直接生成可用的代码：

```jsx
// 1. convert resources in src/assets/chatgpt/category/*.yml to json
// 2. generate src/assets/chatgpt/category.json
// the yaml file is like this:
// ```yml
// ···
```

这也就是为什么我们做了 [ClickPrompt]([https://github.com/prompt-engineering/click-prompt](https://github.com/prompt-engineering/click-prompt)) ， 用于一键轻松查看、分享和执行 Prompt。而在完善 ClickPrompt 的过程中，我们发现将 AI 绑定到自己的工作流中，才能更好地提升效率。因此，我们在 ClickPrompt 中提取了两个功能，构建了 ChatFlow：

- ChatGPT 聊天室：一个集成了 ChatGPT API 的简易 ChatGPT聊天室。
- ClickFlow：一个基于 Yaml 构建的工作流。

结合 ClickPrompt 不丰富的组件，它可以勉强 work 了。

## ChatFlow 是什么？

让 AI 来读标题，回答一下这个问题：

- Write about ChatFlow：开源 ChatFlow：让人类设计高质量流程，让 ChatGPT 生成与复读

ChatFlow 是一个基于自然语言处理（NLP）的流程编排工具，具有以下特点：

- 易于理解的 YAML：ChatFlow 使用简单易懂的 YAML 格式来描述流程的各个元素，包括条件、循环和变量等。无需编程技能，让流程设计变得简单易懂。
- 丰富的可视化组件：ChatFlow 提供了丰富的可视化组件，例如表格、图表和交互式界面等，让用户可以更加方便地与流程进行交互和管理。
- 自动化执行流程：ChatFlow 使用 NLP 技术自动翻译自然语言描述的流程为可执行的代码，并支持自定义函数和自动生成文档功能，让用户更加灵活和高效地管理流程。

总之，ChatFlow 提供了一种灵活、易用、自动化的流程编排工具，让用户可以更加高效地管理复杂的流程，提高工作效率和准确性，同时降低工作的复杂性和学习成本。

## ChatFlow 示例

在过去的一段时间内，我们不断尝试开发一些工作流：

1. 需求与代码生成：从一个模糊的需求开始，生成标准的用户 Story（包含多个 AC），然后根据 AC 生成流程图、测试用例和测试代码。
2. 软件系统设计：从一个简单的系统开始，分析系统对应的用户旅程，生成对应的处理过程 DSL 等等，以帮助我们思考如何基于 AI 进行系统设计。
3. 写作的发散与探索：从一个主题开始，进行对应的发散和收敛，直至辅助我们完成一篇文章的草稿、大纲、内容编写。
4. ClickPrompt 工作流：围绕 ClickPrompt 项目的开发，结合创建 issue、issue 分析、Code Review 等构建的工作流。

在线示例：[https://www.clickprompt.org/zh-CN/click-flow/](https://www.clickprompt.org/zh-CN/click-flow/)

### ChatFlow 示例：需求与代码生成

用于帮助开发人员快速生成代码并进行测试，从而加快开发进度和提高代码质量。

### ChatFlow 示例：**软件系统设计**

用于帮助系统设计人员快速理解用户需求并生成对应的系统设计方案。

### ChatFlow 示例：写作的发散与探索

用于帮助写作人员快速生成文章并进行修改和编辑，从而提高写作效率和文章质量。

### ChatFlow 示例：ClickPrompt 工作流

用于帮助开发团队快速解决问题并进行代码审查，从而加快项目进度和提高代码质量。

## 未来：ChatFlow 的下一步

作为一个刚挖的新坑，我们缺乏关于这一领域的相关知识，所以如果你也有兴趣，欢迎来加入我们。

与一个简单的工具相比，我们更想把 ChatFlow 做成一个框架，类似于 Hexo 这一类的博客软件。所以，我们暂时计划：

1. 扩展更多的可视化组件：除了表格和时间轴，还可以考虑增加图表、地图、树形结构等更多的组件。这样，用户可以更加方便地构建自己的工作流，实现更加复杂的业务需求。
2. 插件开发机制：建立一个开放的插件开发机制，让开发者可以开发和共享自己的插件，从而增加 ChatFlow 的可扩展性。这样，用户可以根据自己的需求选择合适的插件，也可以为其他用户贡献自己的插件。
3. 易于编写的工作流：将编写工作流的难度降到最低，尽可能让用户只需拖拽和连接组件就能完成工作流的构建。同时，还可以为高级用户提供更多的编程接口，让他们能够更加自由地控制工作流的执行。
