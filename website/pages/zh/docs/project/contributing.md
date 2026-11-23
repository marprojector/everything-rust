---
title: 参与贡献
description: 开发环境搭建、测试放在哪里、如何添加 crate 映射或新语言，以及如何分享真实的迁移案例。
---

# 参与贡献

感谢你的帮助。欢迎提交 bug 报告、修复、新语言支持以及真实迁移案例的总结，
修复 bug 的 Pull Request 无需事先询问即可直接提交。

## 开始之前

- **bug：** 提交一个 [issue](https://github.com/marprojector/everything-rust/issues/new/choose)，附上
  你运行的命令、它的输出、你的平台以及 everything-rust 版本
  （`everything-rust --version`）。一个很小的复现项目最有帮助。
- **较大的改动**（新语言、新命令、改变单元或录制结果的定义）：
  请先提一个 issue，以便在你动手之前就整体方案达成一致。过往决策背后的原因
  见 [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md)。
- **安全问题：** 不要提交公开 issue；见 [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md)。

## 环境搭建

你需要 Rust（stable，1.97 或更新版本）。测试还会运行各部分所处理的语言，
缺少对应运行时的部分会被跳过，所以请安装你要改动的部分所需的运行时：

| 用途 | 安装 |
| --- | --- |
| TypeScript/JavaScript | Node 24（除 `profile` 外，22.6+ 也可以） |
| Python | Python 3.9+ |
| C | C 编译器（`cc`） |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | JDK（17+） |
| PHP | PHP 8 |
| C# | .NET SDK |
| wasm 桥接层 | `rustup target add wasm32-unknown-unknown` 以及对应版本的 `wasm-bindgen-cli` |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` 会列出你已安装的运行时、它们的版本，以及安装或更新每一项的
命令。如果希望缺少运行时的测试直接失败而不是跳过（CI 就是这样做的），请在 `EVERYTHING_RUST_REQUIRE` 中列出它，例如
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`。

该钩子会运行 `scripts/check-secrets.sh`，拒绝你提交的内容中出现密钥、令牌和个人主目录
路径。CI 会对每个文件运行它。

## 进行修改

- **每个修复都要附带一个没有该修复就会失败的测试。** 先在旧代码上运行一次该测试，
  确认它确实会失败。
- **测试放在哪里：**
  - `tests/analyze.rs`：语言检测、单元、风险、依赖、`--whole-app`。
  - `tests/loop.rs`：各语言的 bridge、record、diff 和 shadow，用替身代替真实构建。
  - `tests/native.rs`：真实构建的桥接层，然后加载并回放。
  - `tests/app.rs` 和 `tests/http.rs`：整个应用的录制、回放、前门和收口，覆盖 CLI 和
    服务端。
  - `tests/library.rs`：整个程序库的迁移。
  - 单元测试放在代码旁边。
- **测试夹具**是 `tests/fixtures/` 下的小型应用。CLI 和 HTTP 测试夹具在每种语言中实现相同的
  命令和路由（`tcli upper|len|div|words|copy|now`、`/hello`、`/upper`、
  `/div/{a}/{b}`、`/now`），所以新增的测试夹具在行为上要与其他的保持一致。
- **生成的代码**（everything-rust 写出的桥接层、驱动和前门）也必须通过
  `clippy -D warnings`，因为 `gate` 会在用户的 crate 上运行它。
- **风格：** 与周围的代码保持一致。注释只写代码本身表达不出来的内容，
  修改的理由请写在 Pull Request 中。

## 添加 crate 映射

`analyze` 根据 `src/crates.toml` 为代码导入的每个库给出对应的 Rust crate。要添加
一个映射，请在对应语言下追加一项：`names`（精确的包名）或 `prefixes`（名称的开头，
用于 Java 和 C#）、`crates`、`fit`（`good`、`partial` 或 `none`），以及在匹配度
需要解释时加一个 `note`。第一个匹配的条目生效，所以请把具体的条目放在宽泛的前缀之前。不在
映射表中的库会报告为 `unknown`；永远不要猜测 crate。

## 添加一种语言

PHP 和 C# 是以预览形式在两个提交中加入的，值得先读一读：
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243)（PHP）和
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a)（C#）。新语言
也以同样的方式开始：

1. `src/lang/<name>.rs` 中的插件：发现、扫描（导入和风险）、解析和导出项，
   以及它的库对应的 crate，作为 `[[<name>]]` 条目写入 `src/crates.toml`。
2. `src/lang/mod.rs` 中的检测和分发。
3. `src/whole_app.rs` 中的整应用支持：入口点、服务端、声明的依赖。
4. CLI 和 HTTP 测试夹具、一个程序库测试夹具，以及 `tests/analyze.rs`、`tests/app.rs`
   和 `tests/http.rs` 中的测试。
5. 在桥接层实现之前，`bridge` 会拒绝该语言的单元，并给出明确的提示。

## 翻译 README

欢迎翻译，译文由社区维护。新的译文：

1. 放在 `docs/README_<CODE>.md` 中（例如 `docs/README_PT.md`）；根目录的 `README.md` 保持
   英文，永远不会被重命名或替换。
2. 使用从 `docs/` 出发有效的路径：`assets/logo.svg`、`../LICENSE`、`../README.md`。
3. 以与其他译文相同的语言行开头，其中自己的语言用纯文本表示，后面跟一句说明：译文由社区维护，可能落后于英文
   README（从 `docs/README_DE.md` 复制并翻译）。
4. 放在语言行的**末尾**，`README.md` 和每份译文中都是如此。已列出语言的
   顺序不变。

## 文档站点

文档网站位于 `website/`（Void、`@void/md`、React）。它的许多页面会在构建时引入
`README.md`、`CONTRIBUTING.md` 和 `docs/*.md` 中的章节，所以只需编辑这些文件，
站点就会随之更新。用 `cd website && npm ci --ignore-scripts && npm run dev` 运行它，用
`npm run build` 构建它。CLI 参考来自 `website/content/cli-help.json` 中的 `--help` 快照；
修改 CLI 后，请在 `website/` 中运行 `cargo build --release` 和
`npm run cli-help` 刷新它。

## 提交 Pull Request

推送之前：

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- 说明哪里出了问题、如何复现，以及这个改动如何修复它。
- 关联 issue（`Fixes #123`）。
- 注明你没能测试的内容（其他操作系统、你没有的运行时）。
- 只包含一个改动：两个修复分成两个 Pull Request 更容易评审。

CI 在 Ubuntu 和 macOS 上运行，并安装了所有运行时。对于首次贡献者，GitHub
会等待维护者批准 CI 运行，所以检查开始前会有短暂的延迟。

Pull Request 会保留其中的提交进行合并，所以你的提交历史仍然属于你。如果改动需要
小幅调整，维护者可能会在上面追加一个提交，而不是要求你再改一轮。

## 分享真实的迁移案例

真实项目的迁移是证明什么可行、什么会出问题的最好证据。如果你在真实项目上使用了
everything-rust，非常欢迎提交 issue 或 Pull Request，把它加到
[docs/real-world.md](/docs/real-world/overview) 中。请写明项目、你运行的
命令、用例数量，以及任何让你感到意外的地方。

## 行为准则

每一位参与者都应遵守[行为准则](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md)。
