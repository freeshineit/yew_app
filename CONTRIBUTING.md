# 贡献指南

感谢你对本项目的关注！以下是参与贡献的指南。

## 代码规范

### Rust 代码格式化

本项目使用 `rustfmt` 来保持代码风格的一致性。

#### 格式化代码

在提交代码之前，请确保运行格式化命令：

```bash
# 使用稳定版 rustfmt（推荐）
cargo fmt --all

# 或使用 make
make fmt

# 如果你安装了 nightly 工具链，可以使用更多格式化选项
make fmt-nightly
```

#### 安装 nightly 工具链（可选）

如果你想使用更多高级格式化选项：

```bash
# 安装 nightly 工具链
rustup install nightly

# 使用 nightly 格式化
cargo +nightly fmt --all -- --config-path rustfmt-nightly.toml
# 或
make fmt-nightly
```

#### 检查代码格式

检查代码是否符合格式规范（不修改文件）：

```bash
# 使用 cargo
cargo fmt --all -- --check

# 或使用 make
make fmt-check
```

### 代码质量检查

使用 Clippy 进行代码质量检查：

```bash
# 使用 cargo
cargo clippy --all-targets --all-features -- -D warnings

# 或使用 make
make lint
```

### 完整检查

运行完整的检查流程（格式化 + lint + 测试）：

```bash
make check
```

## 格式化配置

项目的格式化规则定义在 `rustfmt.toml` 文件中，主要规则包括：

- **最大行宽**: 100 字符
- **缩进**: 4 个空格
- **换行风格**: Unix (LF)
- **导入排序**: 按 Std -> External -> Crate 分组
- **注释宽度**: 100 字符

## 编辑器配置

项目包含 `.editorconfig` 文件，支持的编辑器会自动应用这些设置：

- 字符编码: UTF-8
- 行尾符: LF
- 缩进: 4 个空格（Rust）
- 自动删除行尾空格
- 文件末尾添加空行

### VS Code 推荐设置

在 `.vscode/settings.json` 中添加：

```json
{
  "editor.formatOnSave": true,
  "rust-analyzer.rustfmt.extraArgs": ["--config-path", "rustfmt.toml"],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

### 推荐的 VS Code 插件

- `rust-analyzer` - Rust 语言支持
- `EditorConfig` - 编辑器配置支持
- `Better TOML` - TOML 文件支持

## Git Hooks

建议设置 pre-commit hook 来自动检查代码格式：

```bash
# 创建 .git/hooks/pre-commit 文件
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh

echo "运行代码格式检查..."
cargo fmt --all -- --check

if [ $? -ne 0 ]; then
    echo "❌ 代码格式检查失败！"
    echo "请运行 'cargo fmt' 或 'make fmt' 来格式化代码"
    exit 1
fi

echo "✅ 代码格式检查通过"
exit 0
EOF

# 添加执行权限
chmod +x .git/hooks/pre-commit
```

## 提交规范

### Commit Message 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Type 类型

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整（不影响功能）
- `refactor`: 重构代码
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具链相关

#### 示例

```
feat(button): 添加主题颜色支持

- 使用 CSS 变量替代硬编码颜色
- 支持 danger 属性显示错误色
- 添加悬停和禁用状态样式

Closes #123
```

## Pull Request 流程

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 确保代码通过所有检查 (`make check`)
5. 推送到分支 (`git push origin feature/amazing-feature`)
6. 创建 Pull Request

## 测试

运行测试：

```bash
# 运行所有测试
cargo test

# 或使用 make
make test
```

## 构建

```bash
# 开发模式
trunk serve --open
# 或
make dev

# 生产构建
trunk build --release
# 或
make build
```

## 问题反馈

如果遇到问题，请在 [Issues](https://github.com/freeshineit/yew_app/issues) 中提交。

## 许可证

通过贡献代码，你同意你的贡献将在与本项目相同的许可证下发布。
