use std::error::Error;

use tauri_hotkey::parse_hotkey;
use tauri_hotkey::Hotkey;
use tauri_hotkey::HotkeyManager;
use tauri_hotkey::Key;

fn get_hotkey_map(hotkey: &str) -> Option<Hotkey> {
    let hotkey = hotkey.to_uppercase();
    match parse_hotkey(&hotkey) {
        Ok(key) => Some(key),
        Err(_) => None,
    }
}

pub struct BindHotKey {
    hotkey_manager: HotkeyManager,
}

impl BindHotKey {
    pub fn new() -> Self {
        Self {
            hotkey_manager: HotkeyManager::new(),
        }
    }

    pub fn bind<F>(&mut self, hotkey: String, callback: F) -> bool
    where
        F: 'static + FnMut() + Send,
    {
        match get_hotkey_map(&hotkey) {
            Some(hotkey) => {
                self.hotkey_manager.register(hotkey, callback).expect("绑定事件错误");
                true
            }
            None => false,
        }
    }

    pub fn un_bind(&mut self) {
        self.hotkey_manager
            .unregister_all()
            .expect("unregister_error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotkey_map() {
        assert_eq!(true, get_hotkey_map("P").is_some());
    }
}
