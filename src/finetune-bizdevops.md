# BizDevOps + 微调

训练步骤和数据集见：[https://github.com/unit-mesh/unit-minions](https://github.com/unit-mesh/unit-minions) 。

AI 研发提效依赖于对研发效能的标准化，并尽可能细地拆分每一个步骤。

## 研发效能

为了训练的结果更加准确，我们详细拆分了软件开发的步骤，以确保每一步生成的是准确，进而驱动出准确的结果。如下是我们早期拆分的一小部分细流程的示例：

* split_user_story_tasks
* create_agile_user_story
* design_restful_api
* design_plantuml_java_datastructure
* implementation_mock_mvc_test
* implementation_spring_controller
* implementation_controller_test
* implementation_spring_service
* ….

我们需要拆分到每一个尽可能小的步骤，在每一个细化的步骤里，喂入数据，才会让 AI 产生最大的复读机效果。

## Unit Mesh

Todos

# 数据准备

我们使用非常简单的 instruct，并尽可能提供，以便于集成到工具中使用。如下：

* 领域知识。instruction：领域知识。
* 拆分任务。instruction：split user story tasks，input：折分用户故事任务
* 需求细化。instruction：create Agile user story for following topic，input：功能的基本信息
* 代码生成。instruction：Implement the method xxx，input：类的基本信息
* 测试生成。instruction：Write test for follow code，input：类的基本信息
* SQL 生成。instruction：text to sql，input：问题
* 文本转 Java 代码。instruction：text to java code，input：问题

对应的功能介绍：

* 需求细化。AI 辅助将模糊的需求转变为的需求设计，比如 “注册” 功能，生成为：”作为一个用户 xxx，填入用户名、密码信息等等，接着由人类去检查和完善。
* 代码生成。AI 辅助将详细的需求设计翻译为目标的代码，再接着由人类去检查和完善。
* 测试生成。AI 辅助根据生成的代码生成对应的测试代码，再接着由人类去检查和完善。

从测试结果来看，随着数据量的增多，比如 20000 个代码用例比 10000 个代码用例更加的 “聪明”。
