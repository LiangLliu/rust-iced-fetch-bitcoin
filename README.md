# Bitcoin Price Monitor

基于 [Iced 0.14](https://github.com/iced-rs/iced) 的实时比特币价格监控桌面应用。

![screenshot](./asserts/iced-fetch-bitcoin.png)

## 功能

- 实时获取 BTC 对 45 种法币价格（CoinGecko API）
- 可配置自动刷新间隔
- 11 种内置主题切换（Nord / Tokyo Night / Dracula …）
- 多页面导航，页面状态保持
- SVG 国旗并发下载
- `tracing` 结构化日志

## 快速开始

```bash
cargo run                                   # 默认 debug 日志
RUST_LOG=iced_fetch_bitcoin=trace cargo run # trace 级别
cargo build --release                       # 发布构建
```

## 项目结构

```
src/
├── main.rs          # 入口：窗口配置、日志初始化
├── app.rs           # App 状态、路由分发、subscription
├── message.rs       # Message / BitcoinMessage / SettingsMessage / AboutMessage
├── route.rs         # Route 枚举
├── api.rs           # CoinGecko API + 错误处理 + debug 日志
├── country.rs       # 45 国货币静态数据
├── http_utils.rs    # 并发下载 SVG 国旗
├── pages/           # 页面层（业务逻辑 + 状态）
│   ├── bitcoin_page.rs
│   ├── settings_page.rs
│   └── about_page.rs
└── views/           # 视图层（纯 UI 渲染）
    ├── bitcoin_view.rs
    ├── settings_view.rs
    ├── about_view.rs
    └── navigation.rs
```

## 架构

采用 **App → Page → View** 三层分离 + Elm 风格消息驱动：

```
App (路由 + 消息分发 + theme/subscription)
 ├── BitcoinPage  →  BitcoinView    # 价格展示
 ├── SettingsPage →  SettingsView   # 主题/刷新/通知
 └── AboutPage    →  AboutView      # 应用信息
```

**消息流**：用户操作 → `Message` → `App::update()` 分发到 Page → Page 返回 `Task` → 异步完成后回调。

**关键设计**：
- Settings 直接驱动 `App::theme()` 和 `App::subscription()`，修改立即生效
- 页面切换只改 `current_route`，各页面状态保持不丢失
- `iced::time::every` 实现可配置的自动刷新

### 新增页面只需 3 步

1. `route.rs` 加枚举值
2. `message.rs` 加消息类型
3. `pages/` + `views/` 各加一个文件，在 `app.rs` 中接入

## 依赖

| Crate | 用途 |
|-------|------|
| `iced 0.14` | GUI 框架 |
| `reqwest` | HTTP 请求 |
| `serde` / `serde_json` | JSON 序列化 |
| `tokio` | 异步运行时 |
| `futures` | 并发下载 |
| `tracing` + `tracing-subscriber` | 结构化日志 |

## 参考

- [iced-rs/iced](https://github.com/iced-rs/iced)
- [YouTube 教程](https://www.youtube.com/watch?v=Kmkz3_WwILk)