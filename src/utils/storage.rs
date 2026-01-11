use web_sys::window;

/// LocalStorage 工具函数
pub struct LocalStorage;

impl LocalStorage {
    /// 获取 localStorage 项
    pub fn get(key: &str) -> Option<String> {
        window()?
            .local_storage()
            .ok()?
            .and_then(|storage| storage.get_item(key).ok()?)
    }

    /// 设置 localStorage 项
    pub fn set(key: &str, value: &str) -> Result<(), String> {
        window()
            .ok_or("Window not available")?
            .local_storage()
            .map_err(|_| "LocalStorage not available")?
            .ok_or("LocalStorage not available")?
            .set_item(key, value)
            .map_err(|_| "Failed to set item".to_string())
    }

    /// 删除 localStorage 项
    pub fn remove(key: &str) -> Result<(), String> {
        window()
            .ok_or("Window not available")?
            .local_storage()
            .map_err(|_| "LocalStorage not available")?
            .ok_or("LocalStorage not available")?
            .remove_item(key)
            .map_err(|_| "Failed to remove item".to_string())
    }

    /// 清空 localStorage
    pub fn clear() -> Result<(), String> {
        window()
            .ok_or("Window not available")?
            .local_storage()
            .map_err(|_| "LocalStorage not available")?
            .ok_or("LocalStorage not available")?
            .clear()
            .map_err(|_| "Failed to clear storage".to_string())
    }
}
