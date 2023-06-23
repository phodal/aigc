# DSL 驱动开发

设计 DSL 的目的，在于让开发者可以用更简单的方式来表达自己的想法。DSL 通常是一种领域特定语言，它的语法和语义都是针对某个特定领域而设计的。
除了具备很好的易读性，作为一个 LLM 与机器的语言，它还应该具备易写性。

如下是一个由 LLM 生成的 DSL 示例：

```yml
EventStorming:
    Domain { Name: "电影订票系统" }
        Event { Name: "用户提交订单" }
            Triggered By: "用户选择电影、场次、座位，确认订单"
            Description: "用户提交订单，包括所选电影、场次、座位等信息"
            Actors: ["用户"]
            Action: "将用户提交的订单信息保存到订单数据库中"
            Outcome: "订单状态被标记为已提交"
```

## 常规 DSL

常规 DSL，诸如于 JSON、YAML、XML 等，是一种用于描述数据的语言。

于是，我们让 ChatGPT 帮我们设计了一个 DSL 来描述：帮我设计一个 DSL 来表示一个系统的处理流程。然后，得到了一个 DSL：

```kotlin
System("BlogSystem") {
  Entities {
    Blog { title: string, ..., comments: [Comment]? },
    Comment { ...}
  }
  Operation {
    Ops("CreateBlog", {
        in: { title: string, description: string },
        out: { id: number }
        pre: title is unique and (title.length > 5 && title.length < 120)
        post: id is not null
    })
  }
  API {
    Route(path: String, method: HttpMethod operation: Operation)
  }
}
```

它可以分析某一个场景的业务，基于这个业务做分析。在这个 DSL，反复让 ChatGPT 设计之后，勉强可以详细拆开任务：

- Operation。通过 Ops 的输入、输出、先验条件、后验条件，我们可以构建出更准确的函数。
- Entitiies。是可独立从 DSL 拆解出来的，并且与数据库结构是绑定的，所以可以用来做数据库设计（ChatGPT 设计了一个诡异的 []? 语法 ）。
- API。API 其实对于编码的帮助是有限的，不过其最大的用处是用于自动化测试，用于确保 ChatGPT 的代码是正确的。

所以，我们只需要拆解任务，并发送到各个管道里，再 merge 一下，就可能能得到一份可工作的代码。至于，前端部分，我们可以用类似的方式来设计。

## 流式 DSL

由于 LLM 的 stream response 特性，我们可以设计 stream DSL 来处理它们。流式响应 DSL 是一种特殊的 DSL，它的特点是：

1. 支持流式数据处理：与传统的批处理方式不同，流式响应 DSL 能够处理实时产生的数据流，无需等待所有数据都到齐才开始处理。
2. 高效的数据处理：流式响应 DSL 可以对数据进行实时处理和转换，而不需要将所有数据都加载到内存中，这使得它可以处理非常大的数据集。
3. 灵活的数据处理：流式响应 DSL 具有高度的灵活性，可以根据具体的需求进行定制和扩展。例如，可以通过添加不同的操作符来实现数据的过滤、聚合、转换等操作。

示例：

```markdown
HasMatchFunction: true
Thought: I need to introduce the system to the team and ensure that it aligns with our overall architecture and governance policies.
Action: introduce_system
Action Input: https://github.com/archguard/ddd-monolithic-code-sample
```

我们预期它可以实时解析用户的输入，并渲染到 UI 上。这样，用户就可以实时看到自己的输入。

对应的解析代码示例：

```javascript
const hasMatchFunctionRegex = /\s*HasMatchFunction:\s*(.*(?:\n(?!\s*\/\/).*)*)/i;
const thoughtRegex = /\s*Thought:\s*(.*(?:\n(?!\s*\/\/).*)*)/i;
const actionRegex = /\s*Action:\s*(.*(?:\n(?!\s*\/\/).*)*)/i;
const actionInputRegex = /\s*Action\s*Input:\s*(.*(?:\n(?!\s*\/\/).*)*)/i;

function messageToThought(splitContent: string[]) {
  let thought = thoughtRegex.exec(splitContent[0])?.[1] ?? "";
  let action = ""
  if (splitContent.length >= 2) {
    action = actionRegex.exec(splitContent[1])?.[1] ?? "";
  }

  let actionInput = ""
  if (splitContent.length >= 3) {
    actionInput = actionInputRegex.exec(splitContent[2])?.[1] ?? "";
  }

  let tooling: ToolingThought = {
    thought: thought,
    action: action,
    actionInput: actionInput
  }

  return tooling;
}
```

## 端到端 DSL 示例

### 阶段一：让 ChatGPT 与你学 DDD

说了，这么多我们来看一个结合我之前设计的 DSL 示例，即 [Fklang](https://github.com/feakin/fklang)。

在 Fklang 中，我们采用下面的形式来表示 ContextMap：

```yaml
ContextMap TicketBooking {
    Reservation -> Cinema;
    Reservation -> Movie;
    Reservation -> User;
}
```

但是，自己设计这样的 DSL 太累了，可以让 ChatGPT 来设计。在你开始之前，需要确认一下 ChatGPT 中的 DDD、DSL、事件风暴等的概念是否与你理解的一致。

第一个问题：`设计一个 DSL 来表示 DDD 里的 Context Map 。`

```yaml
ContextMap:
    Subdomain { Name: [subdomain_name] }
        [CollaboratesWith: [other_subdomain_name], ...]
        [ConformistWith: [other_subdomain_name], ...]
        [Antagonizes: [other_subdomain_name], ...]
    ...
```

第二个问题：用这个 ContextMap DSL 对在线电影订票系统建模。对于其它问题也是相似的。

第三个问题：设计一个 DSL 来描述事件风暴的 事件建模 吗？

第四个问题：能用 事件建模的 DSL 来表示用户购票的流程吗？

一旦，我们觉得设计跑歪了，就可以重新设计。如果下是生成的复杂的 DSL：

```yaml
EventStorming:
    Domain { Name: "电影订票系统" }
        Event { Name: "用户提交订单" }
            Triggered By: "用户选择电影、场次、座位，确认订单"
            Description: "用户提交订单，包括所选电影、场次、座位等信息"
            Actors: ["用户"]
            Action: "将用户提交的订单信息保存到订单数据库中"
            Outcome: "订单状态被标记为已提交"
```

第五个问题……

剩下的你都会了，不会的，可以问 ChatGPT。

### 阶段二：完整设计 + DSL 生成

第一个问题：能总结一下，我们这里用 DDD 设计 购票系统到落地代码的过程吗？

第二个问题：能为每个过程，设计一个合理的 DSL 吗，并展示他们？

展示一部分神奇的 DSL：

```markdown
generate java-code from-domain-model
    target-package: com.example.movieticket.order
    source-model: order-domain-model

    service-mapper 订单服务映射
        map-method: 查询电影
        to-class: MovieRepository
        to-method: findBySchedule

        map-method: 查询座位
        to-class: SeatRepository
        to-method: findByRowAndColumnAndStatus
```

现在，有意思的地方来，有了上面的一系列 DSL 之后，我们就可以接入到代码系统中。

