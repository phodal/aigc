# LLM 微调示例：辅助代码生成

### 步骤 1. 准备数据

1. 下载 GitHub 上的项目（需要包含测试用例）
2. 遍历 `src/main` 下的 Java 文件。
3. 生成每个类的基本信息：

```
PostService(PostRepository, UserRepository, ImageService)
- fields: postRepository:PostRepository, userRepository:UserRepository, userPosts:Set<Post>, imageService:ImageService
- methods: findAll(): List<Post>, addNewPost(Post): Post, saveImageToPost(String, MultipartFile, Post): int
```

### 步骤 2. 生成指令

预期 JSON 格式：

- instruction: Implement the method ${methodName}
- input: ${classInformation}
- output: ${code}

示例：

```
{"instruction":"Implement the method action","input":"com.thoughtworks.go.config.rules.AbstractDirective(DirectiveType, DirectiveType, String, String, String)\n- fields: action:String, type:String, resource:String, configErrors:ConfigErrors, directiveType:DirectiveType\n- methods: validate(ValidationContext): void, isInvalid(String, List<String>): boolean, matchesAction(String): boolean, matchesType(Class<? extends Validatable>): boolean, matchesResource(String): boolean, errors(): ConfigErrors, addError(String, String): void, equals(Object): boolean, action(): String, type(): String, resource(): String, hashCode(): int, hasErrors(): boolean, getDirectiveType(): DirectiveType","output":"public abstract class AbstractDirective implements Directive {\n\n    @ConfigAttribute(value = \"action\", optional = false)\n    protected String action;\n\n    @ConfigAttribute(value = \"type\", optional = false)\n    protected String type;\n\n    @ConfigValue\n    private String resource;\n\n    private final ConfigErrors configErrors = new ConfigErrors();\n\n    private DirectiveType directiveType;\n\n    public AbstractDirective(DirectiveType allow) {\n        this.directiveType = allow;\n    }\n\n    public AbstractDirective(DirectiveType allow, String action, String type, String resource) {\n        this.directiveType = allow;\n        this.action = action;\n        this.type = type;\n        this.resource = resource;\n    }\n\n    @Override\n    public String action() {\n        return this.action;\n    }\n}\n"}
```

### 类信息格式

格式规范：

```
包名.类名(类的构造类型)
- fields：成员变量集合（变量名:类型）
- methods：方法抽象集合（方法名(参数类型): 输出类型）
```

结果：

```
com.thoughtworks.go.config.rules.AbstractDirective(DirectiveType, DirectiveType, String, String, String)
- fields: action:String, type:String, resource:String, configErrors:ConfigErrors, directiveType:DirectiveType
- methods: validate(ValidationContext): void, isInvalid(String, List<String>): boolean, matchesAction(String): boolean, matchesType(Class<? extends Validatable>): boolean, matchesResource(String): boolean, errors(): ConfigErrors, addError(String, String): void, equals(Object): boolean, action(): String, type(): String, resource(): String, hashCode(): int, hasErrors(): boolean, getDirectiveType(): DirectiveType
```

### 其它：核心代码逻辑

```kotlin
val javaProcessor = JavaProcessor(file.readText())
val shotClass = javaProcessor.toShortClass() ?: return@forEach

javaProcessor
   .removePackage()
   .removeAllImport()
   .removeLicenseInfoBeforeImport()

javaProcessor.splitMethods().forEach { (key, value) ->
   CodegenPrompt(
       instruction = "Implement the method $key",
       input = shotClass.toString(),
       output = value
   ).let { prompt ->
       val output = Json.encodeToString(prompt)
       File("$targetPath${key}.json").writeText(output)
   }
}
```
