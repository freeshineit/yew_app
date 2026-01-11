# 项目结构说明

## 目录结构

```
yew_app/
├── src/
│   ├── components/           # 组件目录
│   │   ├── atoms/           # 原子组件（最小可复用单元）
│   │   │   ├── button.rs    # 按钮组件
│   │   │   ├── input.rs     # 输入框组件
│   │   │   ├── loading.rs   # 加载组件
│   │   │   ├── error.rs     # 错误提示组件
│   │   │   ├── checkbox.rs  # 复选框组件
│   │   │   ├── layout.rs    # 布局组件（Header, Footer）
│   │   │   └── mod.rs       # 模块导出
│   │   ├── icons/           # 图标组件
│   │   │   ├── check.rs     # 勾选图标
│   │   │   ├── delete.rs    # 删除图标
│   │   │   ├── icon_wrapper.rs
│   │   │   └── mod.rs
│   │   └── mod.rs           # 组件模块导出
│   ├── pages/               # 页面组件
│   │   ├── home.rs          # 首页
│   │   ├── login.rs         # 登录页
│   │   ├── todo_list.rs     # Todo列表页
│   │   ├── videos.rs        # 视频列表页
│   │   ├── not_found.rs     # 404页面
│   │   └── mod.rs           # 页面模块导出
│   ├── utils/               # 工具模块
│   │   ├── theme.rs         # 主题颜色系统
│   │   ├── storage.rs       # LocalStorage封装
│   │   └── mod.rs           # 工具模块导出
│   ├── state.rs             # 全局状态管理
│   ├── lib.rs               # 库入口（路由配置）
│   └── main.rs              # 应用入口
├── assets/                  # 静态资源
│   ├── favicon.ico
│   ├── logo.png
│   ├── filter.png
│   └── videos.json
├── dist/                    # 构建输出目录
├── target/                  # Cargo 构建缓存
├── index.html               # HTML 模板
├── index.scss               # 全局样式
├── Cargo.toml               # Rust 项目配置
├── Trunk.toml               # Trunk 构建配置
├── docker-compose.yml       # Docker 部署配置
├── nginx.conf               # Nginx 配置
├── README.md                # 项目说明
├── OPTIMIZATION.md          # 优化说明文档
├── CHANGELOG.md             # 更新日志
└── PROJECT_STRUCTURE.md     # 本文件
```

## 核心模块说明

### 1. Components（组件）

#### Atoms（原子组件）
最小的可复用 UI 单元，遵循原子设计原则：

- **Button**: 可配置的按钮组件
  - 支持 danger 模式
  - 支持 disabled 状态
  - 使用主题颜色系统
  
- **Input**: 输入框组件
  - 支持多种输入类型
  - 支持 disabled 状态
  - 统一的样式和交互

- **Loading**: 加载状态组件
  - 旋转动画效果
  - 可自定义提示文本

- **ErrorMessage**: 错误提示组件
  - 统一的错误展示样式
  - 友好的用户提示

- **Layout**: 布局组件
  - LayoutHeader: 顶部导航栏
  - LayoutFooter: 底部信息栏
  - Layout: 页面容器

#### Icons（图标）
SVG 图标组件，用于 UI 装饰和交互提示

### 2. Pages（页面）

- **Home**: 应用首页，展示导航和欢迎信息
- **Login**: 登录页面，表单验证和提交
- **TodoList**: Todo 应用，支持增删改查和持久化
- **Videos**: 视频列表，异步数据加载和展示
- **NotFound**: 404 错误页面

### 3. Utils（工具）

- **theme.rs**: 主题颜色常量
  - 主色调、成功色、警告色、错误色
  - 中性色、边框色、背景色
  - 阴影、圆角、间距常量

- **storage.rs**: LocalStorage 封装
  - get/set/remove/clear 方法
  - 类型安全的存储操作

### 4. State（状态管理）

全局状态管理系统：
- AppState: 应用状态结构
- AppStateProvider: 状态提供者组件
- use_app_state: 状态访问 hook

## 数据流

```
用户交互
    ↓
页面组件
    ↓
状态更新（use_state / use_app_state）
    ↓
组件重渲染
    ↓
LocalStorage 持久化（可选）
```

## 路由系统

使用 yew-router 实现客户端路由：

```rust
Route::Home       → "/"
Route::Login      → "/login"
Route::TodoList   → "/todo_list"
Route::Videos     → "/videos"
Route::NotFound   → "/404"
```

## 样式系统

使用 Stylist 实现 CSS-in-Rust：
- 组件级样式隔离
- 支持 CSS 变量和动画
- 响应式设计支持

## 构建流程

1. **开发模式**: `trunk serve`
   - 热重载
   - 开发服务器
   - Source maps

2. **生产构建**: `trunk build --release`
   - 代码优化
   - 资源压缩
   - WASM 优化

3. **部署**: Docker + Nginx
   - 静态文件服务
   - Gzip 压缩
   - 缓存策略

## 技术栈

- **Yew**: Rust Web 框架
- **Yew Router**: 路由管理
- **Stylist**: CSS-in-Rust
- **Serde**: 序列化
- **Reqwasm**: HTTP 请求
- **Web-sys**: Web API 绑定
- **Wasm-bindgen**: JS 互操作

## 最佳实践

1. **组件设计**: 遵循原子设计原则
2. **状态管理**: 使用 hooks 和 context
3. **样式管理**: 使用主题常量
4. **错误处理**: 统一的错误展示
5. **类型安全**: 充分利用 Rust 类型系统
6. **代码组织**: 清晰的模块划分