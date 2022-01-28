use crate::APP_NAME;
use confy;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlickConfig {
    pub key: String,
}

/// `SlickConfig` implements `Default`
impl ::std::default::Default for SlickConfig {
    fn default() -> Self {
        Self {
            key: String::new(),
        }
    }
}

impl SlickConfig {
    pub fn save_key(&mut self, key: String) {
        let cfg = SlickConfig {
            key: key,
        };

        confy::store(APP_NAME, cfg).expect("保存 config 错误");
    }
}
