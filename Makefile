# Makefile for Yew App

.PHONY: help fmt fmt-check fmt-nightly lint build dev clean test

# 默认目标
help:
	@echo "可用命令:"
	@echo "  make fmt          - 格式化所有 Rust 代码（稳定版）"
	@echo "  make fmt-nightly  - 格式化所有 Rust 代码（nightly 版本，更多选项）"
	@echo "  make fmt-check    - 检查代码格式（不修改文件）"
	@echo "  make lint         - 运行 clippy 检查代码质量"
	@echo "  make build        - 构建项目"
	@echo "  make dev          - 启动开发服务器"
	@echo "  make clean        - 清理构建产物"
	@echo "  make test         - 运行测试"

# 格式化代码（稳定版）
fmt:
	@echo "格式化 Rust 代码（稳定版）..."
	cargo fmt --all

# 格式化代码（nightly 版本）
fmt-nightly:
	@echo "格式化 Rust 代码（nightly 版本）..."
	@if command -v rustup >/dev/null 2>&1; then \
		cargo +nightly fmt --all -- --config-path rustfmt-nightly.toml; \
	else \
		echo "错误: 需要安装 rustup 和 nightly 工具链"; \
		echo "运行: rustup install nightly"; \
		exit 1; \
	fi

# 检查代码格式
fmt-check:
	@echo "检查代码格式..."
	cargo fmt --all -- --check

# Clippy 代码检查
lint:
	@echo "运行 Clippy 检查..."
	cargo clippy --all-targets --all-features -- -D warnings

# 构建项目
build:
	@echo "构建项目..."
	trunk build --release

# 开发模式
dev:
	@echo "启动开发服务器..."
	trunk serve --open

# 清理构建产物
clean:
	@echo "清理构建产物..."
	cargo clean
	rm -rf dist

# 运行测试
test:
	@echo "运行测试..."
	cargo test

# 完整检查（格式化 + lint + 测试）
check: fmt lint test
	@echo "所有检查通过！"
