use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

/// Amounts are stored in minor units (hundredths) to avoid floating-point errors.
#[derive(Serialize, Deserialize)]
pub struct Bank {
    pub balance: i64,
    pub goal: Option<i64>,
}

pub type Banks = BTreeMap<String, Bank>;

fn data_file() -> Result<PathBuf, String> {
    let dir = dirs::data_dir().ok_or("could not find a data directory")?;
    Ok(dir.join("piggy").join("banks.json"))
}

pub fn load() -> Result<Banks, String> {
    let path = data_file()?;
    if !path.exists() {
        return Ok(Banks::new());
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("parsing {}: {e}", path.display()))
}

pub fn save(banks: &Banks) -> Result<(), String> {
    let path = data_file()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("creating {}: {e}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(banks).map_err(|e| e.to_string())?;
    fs::write(&path, text).map_err(|e| format!("writing {}: {e}", path.display()))
}