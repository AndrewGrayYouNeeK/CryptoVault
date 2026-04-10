use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct VaultFile {
    pub name: String,
    pub cid: String,        // IPFS content id
    pub metadata_hash: String,
}

#[wasm_bindgen]
pub fn init_vault() -> Result<(), JsValue> {
    console_log!("CryptVault locked and loaded. Your secrets are safe... probably.");
    Ok(())
}

#[wasm_bindgen]
pub fn encrypt_file(data: Vec<u8>, filename: String) -> Result<JsValue, JsValue> {
    let vault_file = VaultFile {
        name: filename,
        cid: String::new(),
        metadata_hash: String::new(),
    };

    let json = serde_json::to_string(&vault_file)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(JsValue::from_str(&json))
}
