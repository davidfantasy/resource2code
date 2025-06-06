# <img src="./public/logo.png" alt="示例图片" width="32" height="32"> resource2code：基于 AI 大模型的代码生成工具

**resource2code**是一款利用 AI 大模型和本地资源生成业务代码的生产力工具。它的原理很简单：

**资源 + 规则 + LLM = 代码**

- **资源**: 可以是任何生成代码依赖的东西：数据库表，API 文档，代码文件，配置文档等，resource2code 帮你将各种资源转换成 LLM 易于理解的格式
- **规则**: 是你对生成代码的要求，比如：代码规范，代码风格，代码示例等，使用 markdown 格式编写，你可以自由的管理维护各种规则，同时在各个项目中复用你的规则
- **LLM**: 支持接入自定义的 AI 大模型，目前支持 OpenAI 兼容格式以及 Ollama 两种格式的 API

和其它类似的代码助手相比，它的核心优势在于不依赖 IDE 独立工作，可以自由管理和复用规则，不必每次重复编写提示词，灵活的整合各种外部资源，不用手动复制和编辑

![示例图片](./public/readme/main.png)

# 快速开始

1. 下载预编译的可执行文件(暂时只提供 windows 版本)或者下载源码自行编译；
2. 在 LLM 配置中配置你的 AI 大模型；
3. 在右侧资源栏配置数据库连接信息和项目工程目录（项目工程目录请选择实际的源码目录 src，不要包含过多的无关目录，避免影响 LLM 判断代码的生成目录）
4. 管理你的代码规则（系统提供了一些默认规则作为参考）
5. 选择任务需要用到的额外的资源，开始生成代码

# 本地编译

整个项目是基于 tauri 框架的，请首先确保安装 rust 和 pnpm 环境。
然后在源码根目录执行以下命令：

```bash
pnpm install
pnpm tauri build
```

欢迎大家关注我的公众号（飞空之羽的技术手札），我会在上面定期分享一些关于技术的经验和感悟~

![二维码](https://github.com/davidfantasy/mybatis-plus-generator-ui/blob/master/imgs/wechat.jpg)
