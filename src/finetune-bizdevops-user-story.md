# LLM 微调示例：用户故事编写

基本思路：

1. 结合常见的领域（如在线网站），借用 OpenAI 生成用户任务（如登录、浏览列表等等）
2. 根据用户故事生成用户故事。
3. 训练 Alpaca LoRA。

如下所示：

![](images/alpaca-user-story-lora.jpeg)

### 步骤 1. 生成用户任务

1. 调用 OpenAI 按分类创建用户任务。prompt 如下：

```markdown
Design a User Story Mapping for ${domain} application based on your understanding. Here are the requirements:

1. Your user story map should include only user tasks to demonstrate how users will interact with the application.
2. Our user story map should be based on your understanding of the ${domain} application and its users, and should be
   designed to address their needs and pain points.
3. You may use any tools or formats you choose to create your user story map, but it should be easily shareable and
   understandable by stakeholders.
4. Your expression should be more concise and clear.
5. Your return should be like as follows:

###

User Tasks:

1. ...

###
```

示例输出：

```
User Tasks:
1. Browse and search for animations and comics
2. View details of animations and comics
3. Create an account
4. Log in to the account
5. Add animations and comics to favorites
6. Download animations and comics
7. Share animations and comics with friends
8. Rate and review animations and comics
9. Create and upload animations and comics
10. Participate in online forums and discussions
```

### 步骤 2. 分解用户任务为用户故事

1. 调用 OpenAI 根据用户任务创建用户故事。prompt 如下：

```markdown
为下面的需求编写用户故事：${domain} 应用的 ${story_name} 功能。 要求如下：

1. 必须要考虑尽可能考虑各种异常场景，添加更多的 AC。
2. 你的返回模板如下所示：

###

用户故事：可以选择宝贝出行服务
作为 莉莉妈
我想 在滴滴打车的手机客户端里选择宝贝出行服务
以便于 我能够带宝宝打车出行的时候打到有儿童座椅的车

AC 1:  莉莉妈可以选择宝贝出行服务
假设 xxx
当 xxx
于是 xxx

###
```

示例输出：

```
用户故事：可以创建和上传动画和漫画
作为一个 Animation and Comics 应用的用户
我想要创建和上传动画和漫画
以便于我可以分享我的作品给其他用户

AC 1: 用户可以创建和上传动画和漫画
假设 用户已经登录到 Animation and Comics 应用
当 用户点击创建和上传动画和漫画按钮
于是 用户可以创建和上传动画和漫画
```
