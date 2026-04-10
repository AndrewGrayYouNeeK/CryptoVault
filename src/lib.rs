use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VaultFile {
    pub name: String,
    pub cid: String,           // IPFS content id
    pub metadata_hash: String,
}

#[wasm_bindgen]
pub fn init_vault() -> Result<(), JsValue> {
    console_log("CryptVault cock locked and loaded. Your secrets are safe... probably.");
    Ok(())
}

#[wasm_bindgen]
pub fn encrypt_file(data: Vec<u8>, filename: String) -> Result<String, JsValue> {
    // TODO: replace with actual Kyber + libsodium hybrid
    let fake_cid = format!("Qm{}", hex::encode(&data[0..16]));

    console_log(&format!(
        "Encrypted {} ({} bytes). Not really, this is fake as fuck right now.",
        filename,
        data.len()
    ));
    Ok(fake_cid)
}

fn console_log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}
