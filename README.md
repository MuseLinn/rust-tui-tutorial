# 🦀 rust-tui-tutorial

一个基于终端的交互式 Rust 教学应用，通过引导式课程和实时代码练习，帮助你掌握 Rust 的核心概念。

**GitHub**: https://github.com/MuseLinn/rust-tui-tutorial

---

## ✨ 特性

- **沉浸式 TUI 体验**：基于 `ratatui` + `crossterm` + `tokio` 构建的现代终端界面
- **6 阶段完整课程**：从基础语法到 Async/Await 的渐进式学习路径
- **实时代码编辑器**：内置 `tui-textarea` 代码编辑组件
- **混合验证模式**：
  - 结构检查（无需本地 Rust 工具链）
  - 本地 `rustc` 编译验证（最真实的反馈）
- **进度持久化**：使用 SQLite 自动保存学习进度
- **中英双语**：界面中文，代码和术语保留英文

---

## 🚀 运行方式

### 前置要求

1. **安装 Rust 工具链**（推荐 Rust 1.80+）
   ```bash
   # 访问 https://rustup.rs/ 安装
   rustc --version
   cargo --version
   ```

2. **确保已安装 `rustc`（用于编译验证）**
   混合验证模式中的复杂练习会调用本地 `rustc` 进行编译。如果未安装，结构检查类练习仍可正常运行。

### 克隆并运行

```bash
git clone https://github.com/MuseLinn/rust-tui-tutorial.git
cd rust-tui-tutorial
cargo run
```

### 快捷键

| 按键 | 功能 |
|------|------|
| `Enter` | 继续 / 选择课程 |
| `↑` / `↓` | 菜单选择 / 内容滚动 |
| `Ctrl + R` | 运行编译验证 |
| `?` | 显示 / 关闭提示 |
| `h` | 显示帮助 |
| `q` | 退出 |
| `Esc` | 在练习页面返回课程目录 |

---

## 📚 课程内容（6 个阶段）

| 阶段 | 主题 | 核心内容 |
|------|------|---------|
| **Phase 1** | 基础与语法 | Variables, Mutability, Functions, Control Flow |
| **Phase 2** | Ownership 高墙 | Ownership, References & Borrowing, Slices |
| **Phase 3** | 自定义类型 | Structs, Methods, Enums, Pattern Matching |
| **Phase 4** | 标准库与错误处理 | Collections, Option/Result/? |
| **Phase 5** | 抽象与惯用法 | Generics, Traits, Iterators, Modules, Testing |
| **Phase 6** | 进阶专题 | Smart Pointers, Concurrency, Async/Await |

---

## 🧪 测试与验证

### 运行测试

```bash
cargo test
```

**当前测试结果**：
```
running 11 tests
test action::tests::test_action_enum ... ok
test models::tests::test_models_construct ... ok
test validator::pattern::tests::test_fallback_substring ... ok
test validator::pattern::tests::test_missing_pattern ... ok
test validator::pattern::tests::test_contains_mut ... ok
test content::loader::tests::test_load_manifest ... ok
test db::tests::test_db_crud ... ok
test event::tests::test_event_handler_creation ... ok
test validator::compiler::tests::test_compile_error ... ok
test validator::compiler::tests::test_compile_success ... ok
test validator::compiler::tests::test_runtime_mismatch ... ok

test result: ok. 11 passed; 0 failed
```

### 代码检查

```bash
cargo clippy -- -D warnings  # 零警告通过
```

---

## 🏗️ 技术栈

- **TUI 框架**：ratatui 0.29
- **终端 I/O**：crossterm 0.28
- **异步运行时**：tokio
- **数据库**：rusqlite（bundled）
- **代码编辑器**：tui-textarea
- **配置解析**：toml + serde

---

## 📁 项目结构

```
src/
├── main.rs           # 入口，初始化 Tokio 运行时
├── app.rs            # App 核心，事件循环，panic 恢复
├── action.rs         # 用户交互 Action 枚举
├── update.rs         # TEA update 函数
├── view.rs           # 顶级渲染与覆盖层管理
├── event.rs          # 异步事件处理器（tokio::select!）
├── tui.rs            # 终端初始化与恢复
├── models.rs         # 数据模型（课程、练习、状态）
├── db.rs             # SQLite 进度持久化
├── screens/          # 各页面渲染逻辑
│   ├── welcome.rs
│   ├── lesson_menu.rs
│   ├── lesson.rs
│   ├── exercise.rs
│   └── summary.rs
├── components/       # 可复用 UI 组件
│   ├── code_block.rs
│   ├── hint_popup.rs
│   ├── navigation.rs
│   └── progress_bar.rs
├── validator/        # 代码验证
│   ├── compiler.rs   # rustc 编译验证
│   ├── pattern.rs    # 正则模式检查
│   └── mod.rs
└── content/          # 课程内容（6 个 TOML 文件）
    ├── phase01_basics.toml
    ├── phase02_ownership.toml
    ├── phase03_types.toml
    ├── phase04_collections.toml
    ├── phase05_abstractions.toml
    └── phase06_advanced.toml
```

---

## 📝 架构设计

详细的架构设计文档请参阅：
- 项目内：`ARCHITECTURE.md`
- Obsidian Vault：`Documents/Obsidian/Note/项目/rust-tui-tutorial/架构设计文档.md`

---

## 📄 License

MIT / Apache-2.0（任选其一）
