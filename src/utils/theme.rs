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
