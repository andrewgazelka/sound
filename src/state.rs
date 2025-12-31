use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    #[serde(default = "default_master")]
    pub master: f32,
    pub volumes: [f32; 4],
}

fn default_master() -> f32 {
    1.0
}

impl Default for State {
    fn default() -> Self {
        Self {
            master: 1.0,
            volumes: [0.0; 4],
        }
    }
}

fn state_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".sound.json"))
}

impl State {
    pub fn load() -> Self {
        state_path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = state_path() {
            let _ = std::fs::write(path, serde_json::to_string(self).unwrap_or_default());
        }
    }
}
