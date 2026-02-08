use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
    pub credits: u64,
    pub reputation: u64,
    pub level: u32,
    pub unlocked_packs: Vec<String>,
    pub completed_missions: Vec<String>,
    pub hardware: HardwareUpgrades,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HardwareUpgrades {
    pub cpu_tier: u32,
    pub ram_gb: u32,
    pub storage_tier: u32,
    pub gpu_tier: u32,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            credits: 500,
            reputation: 0,
            level: 1,
            unlocked_packs: vec!["base_pack".to_string()],
            completed_missions: Vec::new(),
            hardware: HardwareUpgrades {
                cpu_tier: 1,
                ram_gb: 8,
                storage_tier: 1,
                gpu_tier: 1,
            },
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        if !Path::new(path).exists() {
            return Ok(Self::new());
        }
        let data = fs::read_to_string(path)?;
        let state = serde_json::from_str(&data)?;
        Ok(state)
    }
}
