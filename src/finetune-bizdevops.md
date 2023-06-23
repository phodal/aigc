# BizDevOps + 微调

在尝试了不同场景和业务模式的代码生成，我们探索了四种不同方式，后续可能还会有一些，但是差异可能不大。

四种方法的太长不读版：

- 代码示例生成法。通过输入示例文本和目标代码，让 AI 模型根据代码规律生成新的代码。
- 测试驱动生成法。通过输入测试代码，让 AI 模型根据测试代码生成对应的业务代码。
- 元数据辅助生成法。通过提供更多的元数据信息，如变量类型、函数参数等，让AI模型更精确地生成通用代码。
- 信息匹配生成法。如通过结合数据源中的表信息，让 AI 模型根据需求生成 SQL 语句。

PS：训练步骤和数据集见：[https://github.com/unit-mesh/unit-minions](https://github.com/unit-mesh/unit-minions) 。

## AI 研发提效的探索：四种方式

在学习炼丹（训练模型）的同时，我们也在探索更有效的方式。

### 代码示例生成法：文本生成 SQL

这种方式通过输入一些示例文本和目标代码，让 AI 模型学习如何生成符合目标的代码。模型会根据示例代码和目标代码的特征和规律，学习生成类似的代码。由于的是输入文本，所以生成的代码也存在一定的随机性。这种方式适用于生成通用代码，例如生成一些常见的数据处理、图形化界面等代码。

我们找了一个 HuggingFace 上的数据集，训练了一个 LLaMA 的 LoRA。作为一个失败的版本，由于数据集太小、质量不太高，所以生成的也比较奇怪。

示例输入：

```java
evaluate("text to sql", "谁是最美丽的人", 0.1, 0.75, 40, 4, 512)
```

示例输出：

```java
SELECT MOST BEAUTIFUL FROM table WHERE BEAUTIFUL = 最美丽的人
```

比较明显的问题是，由于不知道对应的表等信息，所以生成出来的永远不尽人意。理想的方式是，训练时候加入表信息，并进行预处理融入表信息等等。

### 测试驱动生成法：测试用例生成

这种方式是将测试代码作为输入，生成适合测试代码的业务代码。这种方式能够更准确地生成适合特定业务场景的代码，因为测试代码通常会包含业务场景的特征和规律。同时，由于输入的测试代码比较固定，所以生成的代码也比较稳定。这种方式适用于生成适合测试代码的业务代码。

我们自己写了测试方法的解析，以及对应的类信息，来更好地生成测试代码，数据集大概在 8000 个。

输入示例：

```java
evaluate("write test", """
public Set<String> listFilesUsingJavaIO(String dir) {
    return Stream.of(new File(dir).listFiles())
      .filter(file -> !file.isDirectory())
      .map(File::getName)
      .collect(Collectors.toSet());
}
""", 0.1, 0.75, 40, 4, 512)
```

输出示例：

```java
@Test
public void shouldListFilesUsingJavaIO() {
    Set<String> files = ListFilesUsingJavaIO.listFilesUsingJavaIO(new File("dir"));
    assertThat(files, containsInAnyOrder("file1", "file2", "file3"));
}
```

从结果来说，既然没有给对应的类信息，生成的测试用例还是可以的。不过，还存在一定的数据集太小，以及 LLaMA 权重的问题，导致生成的代码有一定的问题。

### 元数据辅助生成法：代码辅助生成

结合元信息的代码生成方式会提供更多的元数据信息，如变量类型、函数参数等，来帮助模型更准确地生成代码。这种方式能够更准确地生成符合特定需求的代码，因为提供的元数据信息能够帮助模型更好地理解代码的语义和结构。这种方式适用于生成通用代码。

我们采用的方式是让 LLM 学会填空，算是比较经典的方式。为了避免大小导致的问题，我们探索了一下给 API 的行数，只提供类的信息。

输入示例：

```java
evaluate("Implement the method updatePostCommentsSize", "PostRepository()\n- methods: findById(Long): Optional<Post>, updatePostCommentsSize(int, Long): void, findAllByPostTopics(String): List<Post>, findDistinctByPostTopics(String): List<Post>", 0.1, 0.75, 40, 4, 512)
```

输出示例：

```java
public class PostRepository {
    public void updatePostCommentsSize(int size, Long postId) {
        Post post = findById(postId);
        if (post == null) {
            return;
        }
        post.setPostCommentsSize(size);
        save(post);
    }
}
```

从结果来说，写 CRUD 的代码质量还行，不过考虑到数据集来说，还是有进一步的改进空间。由于的缺点是，为了实现这样的功能，还需要进一步和各类工具整合到一起。

### 信息匹配生成法：文本转 Repository

结合表信息的 SQL + Repository 生成方式会通过多步骤结合数据源中的表信息来生成符合需求的 SQL + Repository 语句。这种方式能够更准确地生成符合特定需求的 SQL 语句，因为结合了数据表信息，能够更好地理解数据表的结构和关系。这种方式适用于生成符合需求的 SQL 语句。

我们采用的方式是会对输入完的结果进行预处理，以生成一个带表等信息的文本，并添加表的信息。如此一来，生成的数据会更加准确。

原数据格式示例：

```markdown
instruction:
我想查找特定月份（monthly_id）下在某个日期（date）之前的所有费用（expense），以方便了解特定月份内的开销情况。
input:
data class ExpenseEntity(....)

output:
@Query("SELECT * FROM expense WHERE monthly_id = :recurringExpenseId AND date < :beforeDate")
suspend fun getAllExpensesForRecurringExpenseBeforeDate(recurringExpenseId: Long, beforeDate: LocalDate): List<ExpenseEntity>
```

输入示例：

```java
evaluate("text to kotlin repository with class", "我想查询指定年龄的用户（User）的博客数量。\n ###data class User(var age: Int, val blogId: Int) data class Post(val title: String)###", 0.1, 0.75, 40, 4, 512)
```

对应的输出示例：

```kotlin
@Query("SELECT COUNT(*) FROM User WHERE age = :age")
abstract fun getBlogCount(age: Int): Long
```

在这个试验里，虽然生成的 Repository 代码都比较让人满意。但是呢，由于人类普遍喜欢用 `SELECT *` 就导致生成出来的代码各种 `SELECT *`，所以也就没有那么满意了。

## 差异对比

这四种方式各有优缺点，适用于不同的场景和需求。

- 方式 1：代码示例生成法。这种方式的优点是简单直接，不需要额外的信息或步骤。缺点是生成的代码可能不够准确或完整，需要人工检查和修改。适用于一些简单的代码生成任务，或者作为初步的代码草稿。
- 方式 2：测试驱动生成法。这种方式的优点是能够根据测试代码的要求，生成满足条件的业务代码。缺点是需要提供高质量的测试代码，否则可能导致错误或低效的业务代码。适用于一些有明确测试标准和规范的代码生成任务，或者作为代码优化和重构的辅助工具。
- 方式 3：元数据辅助生成法。这种方式的优点是能够利用更多的上下文信息，提高代码生成的准确性和可读性。缺点是需要收集和提供更多的元数据信息，增加了数据准备和处理的工作量。适用于一些有复杂逻辑和结构的代码生成任务，或者作为代码质量和规范性的保障手段。
- 方式 4：信息匹配生成法。这种方式的优点是能够根据数据源中的表结构和关系，生成符合需求和规范的 SQL 语句。缺点是需要多步骤的交互和反馈，增加了用户和模型之间的沟通成本。适用于一些有特定数据源和查询需求的 SQL 生成任务，或者作为 SQL 学习和教育的辅助工具。

结果如下表所示：

| 方式 | 输入 | 随机性 | 附加信息 | 应用场景 |
| --- | --- | --- | --- | --- |
| 直接代码生成 | 示例文本和目标代码 | 较高 | 无 | 生成通用代码 |
| 代码反向生成 | 测试代码 | 低 | 类信息 | 生成适合测试代码的业务代码 |
| 结合元信息的代码生成 | 元数据信息 | 中等 | 变量类型、函数参数等元数据信息等 | 生成通用代码 |
| 结合表信息的 SQL 生成 | 数据库中的表信息 | 低 | 变量类型、函数参数等元数据信息 | 生成符合需求的SQL语句 |

只有提供更丰富的信息，AI 才能生成更准确的代码。

## 小结

本文介绍了四种 AI 代码生成微调方式，包括代码示例生成法、测试驱动生成法、元数据辅助生成法和信息匹配生成法。每种方式都有其优缺点和适用场景，但都需要提供更多的信息才能生成更准确的代码。

欢迎自己动手试验：[https://github.com/unit-mesh/unit-minions](https://github.com/unit-mesh/unit-minions)
