# DSL 驱动开发

说了，这么多我们来看一个结合我之前设计的 DSL 示例，即 [Fklang](https://github.com/feakin/fklang)。

### 阶段一：让 ChatGPT 与你学 DDD

在 Fklang 中，我们采用下面的形式来表示 ContextMap：

```yaml
ContextMap TicketBooking {
    Reservation -> Cinema;
    Reservation -> Movie;
    Reservation -> User;
}
```

但是，自己设计这样的 DSL 太累了，可以让 ChatGPT 来设计。在你开始之前，需要确认一下  ChatGPT 中的 DDD、DSL、事件风暴等的概念是否与你理解的一致。

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

```yaml
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

### 阶段三：代码生成与低代码

只要 ChatGPT 上下文能力足够强壮，或者支持 LoRA 式的插件模式，我们就能实现从需要到现有的任何系统中。

第一个问题：为电影订票系统设计一个 DDD 风格 Java 工程的代码目录结构。

第二个问题：结合 Spring 设计一下购票流程的 RESTful API

……

有点懒，就先这样吧。后面的部分，就可以结合 GitHub Copilot 去实现了。

### 小结

结合 Prompt 编程，低代码到了一定的成熟度，我们就可以发现更好玩的东西：实时的软件生成
