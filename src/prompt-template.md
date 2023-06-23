# Prompt Patterns 模板

回忆一下我们的高考作文题：

```markdown
阅读下面的材料，根据要求写作。（60分）

　　人们因技术发展得以更好地掌控时间，但也有人因此成了时间的仆人。

　　这句话引发了你怎样的联想与思考？请写一篇文章。

要求：选准角度，确定立意，明确文体，自拟标题；不要套作，不得抄袭；不得泄露个人信息；不少于800字。
```

这部分的文字非常易于我们理解，它由以下几部分构成：

- 指令（Instruction）：阅读下面的材料，根据要求写作。
- 要求（Requirements）：选准角度，确定立意，明确文体，自拟标题；不要套作，不得抄袭；不得泄露个人信息；不少于800字。
- 素材：人们因技术发展得以更好地掌控时间，但也有人因此成了时间的仆人。

## Prompt 编写模板

通常来说，一个易于 LLM 理解的模板可以由以为几部分构成：

- 角色（Role）：解释这个模板的使用场景和目的。
- 指令（Instruction）：告诉 GPT-3.5 执行的具体任务或要求。
- 要求（Requirements）：对 GPT-3.5 生成的结果的期望和要求，例如长度、主题、语言风格等。
- 示例（Sample）：提供一个或多个示例，以便 GPT-3.5 理解任务的上下文和生成的结果的期望。
- 扩展数据（Extended Data）(可选，编程时需要）：提供一些额外的数据，例如词汇、语法结构、语言模型等，以帮助 GPT-3.5 生成更加丰富、准确的结果。

一个简单的示例如下：

> 角色（Role）：你是一位旅游公司的营销人员，需要撰写一篇旅游目的地介绍的文本。

> 指令（Instruction）：请使用 GPT-3.5 生成一篇介绍文本，其中包括目的地的名称、位置、历史和文化背景、景点、美食、住宿和交通等相关信息。

> 要求（Requirements）：生成的文本长度应为 500-1000 个单词，语言风格应为正式、客观、流畅，包含准确的地理、历史和文化知识。

> 示例（Sample）：请以以下地点为例，生成一篇介绍文本：福建漳州。

> 扩展数据（Extended Data）：可以提供一些关于福建、漳州的历史、文化、地理等方面的数据，以帮助 GPT-3.5 生成更加准确、详细的文本。

输出示例：

> 漳州，位于福建省东南沿海，是一个具有悠久历史和丰富文化的城市。作为福建省的一个重要城市，漳州拥有独特的地理位置和文化底蕴，是一个备受游客青睐的旅游目的地。

> 漳州的历史可以追溯到公元前三千年左右的新石器时代，是中国历史上一个非常重要的文化和商业中心。在宋、元、明、清等历史时期，漳州都是一个繁荣的商贸城市，曾经是丝绸之路的海上起点之一。在漳州，游客可以看到许多保留完好的历史遗迹，如南靖土楼、东山古城等。

> 漳州的文化底蕴非常深厚，它是闽南文化的代表之一。在漳州，游客可以领略到许多独特的文化风情，如闽南建筑、福建茶文化、福建南音等。此外，漳州还是福建省的一个重要的宗教中心，拥有众多的寺庙和教堂，如南山寺、开元寺等。

> 漳州拥有许多著名的景点，如南靖土楼、天柱山、东山古城、南山寺等。其中，南靖土楼是漳州最著名的景点之一，它是闽南土楼的代表之一，被誉为“中国传统建筑艺术的瑰宝”。天柱山是漳州的一个自然保护区，拥有丰富的自然景观和独特的生态环境，是一个极具观赏价值的景点。

> 漳州的美食也非常有名，如海鲜、福建菜等。漳州的海鲜非常新鲜，口感鲜美，深受游客的喜爱。福建菜是中国八大菜系之一，以其独特的口味和烹饪方法而著名。

> 对于住宿和交通，漳州也有很好的配套设施。漳州有许多高档酒店和旅游胜地，如漳州嘉禾智选假日酒店、漳浦金海岸度假酒店等。此外，漳州还有许多交通工具，如高速公路、铁路、机场等，方便游客前往。

> 总之，漳州是一个迷人的旅游目的地，拥有丰富的历史和文化底蕴、独特的景点、美味的美食、高品质的住宿和交通，是一个值得游客来此一游的好地方。

当然了，GPT 是生成式的，所以在这时往往没有根据 "事实" 来生成内容，所以需要你添加一些额外的数据，以帮助 GPT 生成更加丰富、准确的结果。

除此，我们并不一定要按照这个模板来定义 Prompt，只要能够清晰地表达出上述几个要素即可。

## Kotlin 代码实现示例

在 ArchGuard Co-mate 中，我们通过如下的 Kotlin 代码来实现 Prompt 模板的定义：

```kotlin
interface BaseTemplate {
    fun getRole(): String = ""
    fun getInstruction(): String = ""
    fun getRequirements(): String = ""
    fun getSample(): String = ""
    fun getExtendData(): String = ""
}
```

对应的一个实现示例：

```kotlin
class LayeredStylePrompt(
    val context: ComateContext,
    override val strategy: Strategy,
) : CodePromptStrategy {
    override fun getRole(): String = "Software Architecture"
    override fun getInstruction(): String =
        "根据下面的信息，分析项目的分层是否符合业内的通用规范？并绘制 Graphviz 图来表示。"

    override fun getRequirements(): String = """
1. 如果存在相互引用，请用红线展示出来。
2. 只展示重要的分层，不要展示过多的细节。
4. 结合分层、subgraph 的方式来表示分层。
4. 示例如下：

###
digraph G {
    rankdir=TB;
    node [shape=record, fontname=Helvetica];
    edge [color=black, penwidth=1.0];
    subgraph cluster_{} {
        label="{} Layer"
    }
###
"""

    override fun getExtendData(): String {
        val introduction = context.fetchReadmeIntroduction()
        return """$introduction

package fan in: ${context.fetchPackageDependencies()}
"""
.trimIndent()

    }

}
```

输出结果示例：

```markdown
You're an Software Architecture,根据下面的信息，分析项目的分层是否符合业内的通用规范？并绘制 Graphviz 图来表示。Here is requirements: 
1. 如果存在相互引用，请用红线展示出来。
2. 只展示重要的分层，不要展示过多的细节。
4. 结合分层、subgraph 的方式来表示分层。
4. 示例如下：

```dot
digraph G {
    rankdir=TB;
    node [shape=record, fontname=Helvetica];
    edge [color=black, penwidth=1.0];
    subgraph cluster_{} {
        label="{} Layer"
    }
```

Project introduction: Co-mate is an AI-powered software architecture copilot, design and governance tools.

package fan in: {org.archguard.architecture=[org.archguard.architecture.layered], org.archguard.comate.cli...
```
