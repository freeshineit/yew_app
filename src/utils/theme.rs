use serde::{Deserialize, Serialize};

/// 主题颜色常量
pub struct Theme;

impl Theme {
    // 主色调
    pub const PRIMARY: &'static str = "#1890ff";
    pub const PRIMARY_HOVER: &'static str = "#40a9ff";
    pub const PRIMARY_ACTIVE: &'static str = "#096dd9";

    // 成功色
    pub const SUCCESS: &'static str = "#52c41a";
    pub const SUCCESS_HOVER: &'static str = "#73d13d";

    // 警告色
    pub const WARNING: &'static str = "#faad14";
    pub const WARNING_HOVER: &'static str = "#ffc53d";

    // 错误色
    pub const ERROR: &'static str = "#ff4d4f";
    pub const ERROR_HOVER: &'static str = "#ff7875";

    // 中性色
    pub const TEXT_PRIMARY: &'static str = "#000000d9";
    pub const TEXT_SECONDARY: &'static str = "#00000073";
    pub const TEXT_DISABLED: &'static str = "#00000040";

    pub const BORDER: &'static str = "#d9d9d9";
    pub const BORDER_LIGHT: &'static str = "#f0f0f0";

    pub const BG_PRIMARY: &'static str = "#ffffff";
    pub const BG_SECONDARY: &'static str = "#fafafa";
    pub const BG_HOVER: &'static str = "#f5f5f5";

    // 阴影
    pub const SHADOW_SM: &'static str = "0 2px 8px rgba(0, 0, 0, 0.08)";
    pub const SHADOW_MD: &'static str = "0 4px 12px rgba(0, 0, 0, 0.12)";
    pub const SHADOW_LG: &'static str = "0 8px 24px rgba(0, 0, 0, 0.16)";

    // 圆角
    pub const RADIUS_SM: &'static str = "2px";
    pub const RADIUS_MD: &'static str = "4px";
    pub const RADIUS_LG: &'static str = "8px";

    // 间距
    pub const SPACING_XS: &'static str = "4px";
    pub const SPACING_SM: &'static str = "8px";
    pub const SPACING_MD: &'static str = "16px";
    pub const SPACING_LG: &'static str = "24px";
    pub const SPACING_XL: &'static str = "32px";
}

/// 自定义主题配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomTheme {
    pub name: String,
    pub primary: String,
    pub primary_hover: String,
    pub primary_active: String,
    pub success: String,
    pub success_hover: String,
    pub warning: String,
    pub warning_hover: String,
    pub error: String,
    pub error_hover: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_disabled: String,
    pub border: String,
    pub border_light: String,
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_hover: String,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            primary: Theme::PRIMARY.to_string(),
            primary_hover: Theme::PRIMARY_HOVER.to_string(),
            primary_active: Theme::PRIMARY_ACTIVE.to_string(),
            success: Theme::SUCCESS.to_string(),
            success_hover: Theme::SUCCESS_HOVER.to_string(),
            warning: Theme::WARNING.to_string(),
            warning_hover: Theme::WARNING_HOVER.to_string(),
            error: Theme::ERROR.to_string(),
            error_hover: Theme::ERROR_HOVER.to_string(),
            text_primary: Theme::TEXT_PRIMARY.to_string(),
            text_secondary: Theme::TEXT_SECONDARY.to_string(),
            text_disabled: Theme::TEXT_DISABLED.to_string(),
            border: Theme::BORDER.to_string(),
            border_light: Theme::BORDER_LIGHT.to_string(),
            bg_primary: Theme::BG_PRIMARY.to_string(),
            bg_secondary: Theme::BG_SECONDARY.to_string(),
            bg_hover: Theme::BG_HOVER.to_string(),
        }
    }
}

impl CustomTheme {
    /// 创建深色主题
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            primary: "#177ddc".to_string(),
            primary_hover: "#3c9ae8".to_string(),
            primary_active: "#095db3".to_string(),
            success: "#49aa19".to_string(),
            success_hover: "#6abe39".to_string(),
            warning: "#d89614".to_string(),
            warning_hover: "#e8b339".to_string(),
            error: "#d32029".to_string(),
            error_hover: "#e84749".to_string(),
            text_primary: "#ffffffd9".to_string(),
            text_secondary: "#ffffff73".to_string(),
            text_disabled: "#ffffff40".to_string(),
            border: "#434343".to_string(),
            border_light: "#303030".to_string(),
            bg_primary: "#141414".to_string(),
            bg_secondary: "#1f1f1f".to_string(),
            bg_hover: "#262626".to_string(),
        }
    }

    /// 创建紫色主题
    pub fn purple() -> Self {
        Self {
            name: "Purple".to_string(),
            primary: "#722ed1".to_string(),
            primary_hover: "#9254de".to_string(),
            primary_active: "#531dab".to_string(),
            success: "#52c41a".to_string(),
            success_hover: "#73d13d".to_string(),
            warning: "#faad14".to_string(),
            warning_hover: "#ffc53d".to_string(),
            error: "#ff4d4f".to_string(),
            error_hover: "#ff7875".to_string(),
            text_primary: "#000000d9".to_string(),
            text_secondary: "#00000073".to_string(),
            text_disabled: "#00000040".to_string(),
            border: "#d9d9d9".to_string(),
            border_light: "#f0f0f0".to_string(),
            bg_primary: "#ffffff".to_string(),
            bg_secondary: "#fafafa".to_string(),
            bg_hover: "#f5f5f5".to_string(),
        }
    }

    /// 创建绿色主题
    pub fn green() -> Self {
        Self {
            name: "Green".to_string(),
            primary: "#52c41a".to_string(),
            primary_hover: "#73d13d".to_string(),
            primary_active: "#389e0d".to_string(),
            success: "#52c41a".to_string(),
            success_hover: "#73d13d".to_string(),
            warning: "#faad14".to_string(),
            warning_hover: "#ffc53d".to_string(),
            error: "#ff4d4f".to_string(),
            error_hover: "#ff7875".to_string(),
            text_primary: "#000000d9".to_string(),
            text_secondary: "#00000073".to_string(),
            text_disabled: "#00000040".to_string(),
            border: "#d9d9d9".to_string(),
            border_light: "#f0f0f0".to_string(),
            bg_primary: "#ffffff".to_string(),
            bg_secondary: "#fafafa".to_string(),
            bg_hover: "#f5f5f5".to_string(),
        }
    }

    /// 创建橙色主题
    pub fn orange() -> Self {
        Self {
            name: "Orange".to_string(),
            primary: "#fa8c16".to_string(),
            primary_hover: "#ffa940".to_string(),
            primary_active: "#d46b08".to_string(),
            success: "#52c41a".to_string(),
            success_hover: "#73d13d".to_string(),
            warning: "#faad14".to_string(),
            warning_hover: "#ffc53d".to_string(),
            error: "#ff4d4f".to_string(),
            error_hover: "#ff7875".to_string(),
            text_primary: "#000000d9".to_string(),
            text_secondary: "#00000073".to_string(),
            text_disabled: "#00000040".to_string(),
            border: "#d9d9d9".to_string(),
            border_light: "#f0f0f0".to_string(),
            bg_primary: "#ffffff".to_string(),
            bg_secondary: "#fafafa".to_string(),
            bg_hover: "#f5f5f5".to_string(),
        }
    }

    /// 获取所有预设主题
    pub fn presets() -> Vec<Self> {
        vec![
            Self::default(),
            Self::dark(),
            Self::purple(),
            Self::green(),
            Self::orange(),
        ]
    }

    /// 应用主题到 CSS 变量
    pub fn apply_to_css(&self) {
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    if let Ok(html_element) = root.dyn_into::<web_sys::HtmlElement>() {
                        let style = html_element.style();
                        let _ = style.set_property("--color-primary", &self.primary);
                        let _ = style.set_property("--color-primary-hover", &self.primary_hover);
                        let _ = style.set_property("--color-primary-active", &self.primary_active);
                        let _ = style.set_property("--color-success", &self.success);
                        let _ = style.set_property("--color-success-hover", &self.success_hover);
                        let _ = style.set_property("--color-warning", &self.warning);
                        let _ = style.set_property("--color-warning-hover", &self.warning_hover);
                        let _ = style.set_property("--color-error", &self.error);
                        let _ = style.set_property("--color-error-hover", &self.error_hover);
                        let _ = style.set_property("--color-text-primary", &self.text_primary);
                        let _ = style.set_property("--color-text-secondary", &self.text_secondary);
                        let _ = style.set_property("--color-text-disabled", &self.text_disabled);
                        let _ = style.set_property("--color-border", &self.border);
                        let _ = style.set_property("--color-border-light", &self.border_light);
                        let _ = style.set_property("--color-bg-primary", &self.bg_primary);
                        let _ = style.set_property("--color-bg-secondary", &self.bg_secondary);
                        let _ = style.set_property("--color-bg-hover", &self.bg_hover);
                    }
                }
            }
        }
    }
}

/// 主题类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeType {
    Default,
    Dark,
    Purple,
    Green,
    Orange,
}

impl ThemeType {
    pub fn to_theme(&self) -> CustomTheme {
        match self {
            ThemeType::Default => CustomTheme::default(),
            ThemeType::Dark => CustomTheme::dark(),
            ThemeType::Purple => CustomTheme::purple(),
            ThemeType::Green => CustomTheme::green(),
            ThemeType::Orange => CustomTheme::orange(),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            ThemeType::Default,
            ThemeType::Dark,
            ThemeType::Purple,
            ThemeType::Green,
            ThemeType::Orange,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            ThemeType::Default => "Default",
            ThemeType::Dark => "Dark",
            ThemeType::Purple => "Purple",
            ThemeType::Green => "Green",
            ThemeType::Orange => "Orange",
        }
    }
}
