use bitcoin::{Network, bip32::Xpriv};
use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2, Params, Algorithm, Version};
use sha3::{Digest, Sha3_256};
use bip39::{Mnemonic, Language};
use tauri::Builder;
use bs58;

// エラーメッセージの定数化
const ERR_INPUT_LENGTH: &str = "Provided code must be at least 16 characters long and password must be at least 8 characters long";
const ERR_INPUT_FORMAT: &str = "Provided code or password is not in Base58 format";
const ERR_HASH_GENERATION: &str = "Failed to generate hash";
const ERR_MNEMONIC_GENERATION: &str = "Mnemonic generation error";

// 共通処理の抽出
fn hash_and_concatenate(provided_code: &str, password_string: &str) -> Result<Vec<u8>, String> {
    // 入力値の長さとBase58形式のチェック
    if provided_code.len() < 16 || password_string.len() < 8 || bs58::decode(provided_code).into_vec().is_err() || bs58::decode(password_string).into_vec().is_err() {
        return Err(format!("{}\n{}", ERR_INPUT_LENGTH, ERR_INPUT_FORMAT));
    }

    // provided_codeとpassword_stringのハッシュ計算と連結
    let mut provided_code_hasher = Sha3_256::new();
    provided_code_hasher.update(provided_code.as_bytes());
    let provided_code_hash = provided_code_hasher.finalize();

    let mut password_string_hasher = Sha3_256::new();
    password_string_hasher.update(password_string.as_bytes());
    let password_string_hash = password_string_hasher.finalize();

    Ok([&provided_code_hash[..], &password_string_hash[..]].concat())
}

// 新しい関数: ハッシュのストレッチ処理
fn stretch_hash(concatenated_hash: &[u8]) -> Result<Vec<u8>, String> {
    let mut final_hasher = Sha3_256::new();
    final_hasher.update(concatenated_hash);
    let final_hash = final_hasher.finalize();

    // ソルトの生成
    let salt = SaltString::encode_b64(&final_hash).expect("Failed to create salt");

    // Argon2idのカスタム設定
    let params = Params::new(65536, 3, 4, None).map_err(|_| ERR_HASH_GENERATION.to_string())?; // メモリコスト: 64MB, タイムコスト: 3, 並列度: 4
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let argon2_hash = argon2
        .hash_password(concatenated_hash, &salt)
        .map_err(|_| ERR_HASH_GENERATION.to_string())?
        .to_string();

    // SHA-3で追加ハッシュ化
    let mut sha3_hasher = Sha3_256::new();
    sha3_hasher.update(argon2_hash.as_bytes());
    let sha3_hash = sha3_hasher.finalize();

    Ok(sha3_hash.to_vec())
}

#[tauri::command]
fn generate_hd_wallet_xprv(provided_code: &str, password_string: &str) -> Result<(String, String), String> {
    let concatenated_hash = hash_and_concatenate(provided_code, password_string)?;
    let stretched_hash = stretch_hash(&concatenated_hash)?;

    let seed = &stretched_hash[..16];
    let mnemonic = Mnemonic::from_entropy_in(Language::English, seed).map_err(|e| format!("{}: {}", ERR_MNEMONIC_GENERATION, e))?;
     let mnemonic_phrase = mnemonic.word_iter().collect::<Vec<_>>().join(" ");
    let seed_bytes = mnemonic.to_seed("");

    let master_key = Xpriv::new_master(Network::Bitcoin, &seed_bytes).map_err(|e| e.to_string())?;
    let xprv = master_key.to_string();

    Ok((xprv, mnemonic_phrase))
}

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_hd_wallet_xprv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
