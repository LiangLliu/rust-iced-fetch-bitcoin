# 多页面架构实现指南

本文档展示了如何在 Iced 框架中实现优雅的多页面应用程序架构。

## 🎯 核心概念

### 1. 路由系统 (Route)
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Bitcoin,   // 比特币价格页面
    Settings,  // 设置页面  
    About,     // 关于页面
}
```

### 2. 应用状态 (App)
```rust
pub struct App {
    current_route: Route,              // 🧭 当前页面
    bitcoin_page: BitcoinPage,         // 📊 比特币页面状态
    settings_page: SettingsPage,       // ⚙️ 设置页面状态
    about_page: AboutPage,             // ℹ️ 关于页面状态
}
```

### 3. 分层消息系统
```rust
pub enum Message {
    Navigate(Route),                   // 🔀 页面导航
    Bitcoin(BitcoinMessage),           // 📊 比特币页面消息
    Settings(SettingsMessage),         // ⚙️ 设置页面消息
    About(AboutMessage),               // ℹ️ 关于页面消息
}
```

## 🏗️ 架构优势

### ✅ 清晰的关注点分离
- **路由管理**: `Route` 枚举统一管理页面导航
- **状态隔离**: 每个页面维护独立的状态
- **消息分发**: 应用层负责消息路由到对应页面

### ✅ 高度可扩展
```rust
// 🆕 添加新页面只需3步：

// 1. 扩展路由
pub enum Route {
    Bitcoin,
    Settings, 
    About,
    History,  // ← 新增历史页面
}

// 2. 添加消息类型
pub enum Message {
    Navigate(Route),
    Bitcoin(BitcoinMessage),
    Settings(SettingsMessage),
    About(AboutMessage),
    History(HistoryMessage),  // ← 新增消息类型
}

// 3. 在 App 中集成
pub struct App {
    current_route: Route,
    bitcoin_page: BitcoinPage,
    settings_page: SettingsPage, 
    about_page: AboutPage,
    history_page: HistoryPage,  // ← 新增页面状态
}
```

### ✅ 页面状态保持
每个页面的状态在切换时会被保留：
```rust
impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => {
                self.current_route = route;  // 🔄 仅切换路由，状态保持
                Task::none()
            }
            // 页面状态独立更新...
        }
    }
}
```

## 🎨 UI 组件架构

### 导航栏组件
```rust
pub struct Navigation {
    current_route: Route,
}

impl Navigation {
    pub fn view(self) -> iced::Element<'static, Message> {
        // 🧭 渲染导航按钮，高亮当前页面
        // 📤 发送 Message::Navigate(route) 消息
    }
}
```

### 页面与视图分离
```rust
// 📄 页面组件 - 业务逻辑
impl SettingsPage {
    pub fn update(&mut self, message: SettingsMessage) -> Task<SettingsMessage> {
        // 🔧 处理设置变更逻辑
    }
    
    pub fn view(&self) -> iced::Element<'_, SettingsMessage> {
        SettingsView::new(/* 传递数据 */).view()  // 📤 委托给视图渲染
    }
}

// 🎨 视图组件 - UI 渲染
impl SettingsView {
    pub fn view(self) -> iced::Element<'static, SettingsMessage> {
        // 🎯 专注于 UI 布局和样式
    }
}
```

## 🔄 消息流程

1. **用户点击导航** → `Message::Navigate(Route::Settings)`
2. **App 路由切换** → `self.current_route = Route::Settings`
3. **渲染对应页面** → `settings_page.view()`
4. **用户操作设置** → `Message::Settings(SettingsMessage::ThemeChanged)`
5. **页面状态更新** → `settings_page.update(message)`

## 🚀 实际效果

运行应用程序后，您会看到：

- **🧭 顶部导航栏**: 三个可点击的页面按钮
- **📊 Bitcoin Prices**: 实时比特币价格展示
- **⚙️ Settings**: 主题选择、刷新间隔、通知设置
- **ℹ️ About**: 应用信息和版本详情

### 特色功能

1. **🎯 无缝切换**: 页面间切换流畅，状态保持
2. **🎨 响应式设计**: 现代化的 UI 界面
3. **⚙️ 实时设置**: 设置变更立即生效
4. **🔄 状态管理**: 每个页面独立管理自己的状态

## 📈 扩展建议

### 1. 添加历史图表页面
```rust
// pages/history_page.rs
pub struct HistoryPage {
    price_history: Vec<PricePoint>,
    selected_timeframe: Timeframe,
}
```

### 2. 实现主题动态切换
```rust
impl App {
    pub fn theme(&self) -> Theme {
        match self.settings_page.selected_theme {
            ThemeType::Light => Theme::Light,
            ThemeType::Dark => Theme::Dark,
            ThemeType::Nord => Theme::Nord,
        }
    }
}
```

### 3. 添加页面间数据共享
```rust
pub struct SharedState {
    bitcoin_price: f64,
    last_update: DateTime<Utc>,
}
```

这种架构模式为您的应用提供了坚实的基础，可以轻松扩展到包含任意数量的页面和功能！
