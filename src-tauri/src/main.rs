use bitcoin::{network::Network, PrivateKey};
use secp256k1::SecretKey;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use sha3::{Digest, Sha3_256};
use tauri::Builder;
use bs58;

// エラーメッセージの定数化
const ERR_INPUT_LENGTH: &str = "Provided code must be at least 16 characters long and password must be at least 8 characters long";
const ERR_INPUT_FORMAT: &str = "Provided code or password is not in Base58 format";


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
fn stretch_hash(concatenated_hash: &[u8]) -> String {
    let mut final_hasher = Sha3_256::new();
    final_hasher.update(concatenated_hash);
    let final_hash = final_hasher.finalize();

    // ソルトの生成
    let salt = SaltString::encode_b64(&final_hash).expect("Failed to create salt");

    // Argon2idでキーストレッチング
    let argon2 = Argon2::default();
    let argon2_hash = argon2
        .hash_password(concatenated_hash, &salt)
        .expect("Failed to hash with Argon2")
        .to_string();

    // SHA-3で追加ハッシュ化
    let mut sha3_hasher = Sha3_256::new();
    sha3_hasher.update(argon2_hash.as_bytes());
    let sha3_hash = sha3_hasher.finalize();

    format!("{:x}", sha3_hash)
}

#[tauri::command]
fn generate_bitcoin_private_key(provided_code: &str, password_string: &str) -> Result<String, String> {
    let concatenated_hash = hash_and_concatenate(provided_code, password_string)?;
    let stretched_hash = stretch_hash(&concatenated_hash);

    // 秘密鍵の生成
    let secret_key = SecretKey::from_slice(&hex::decode(stretched_hash).map_err(|_| "Failed to decode hex")?.as_slice()[..32])
        .map_err(|e| e.to_string())?;

    // 秘密鍵をWIF形式に変換
    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    Ok(private_key.to_wif())
}

#[tauri::command]
fn generate_private_key(provided_code: &str, password_string: &str) -> Result<String, String> {
    let concatenated_hash = hash_and_concatenate(provided_code, password_string)?;
    Ok(stretch_hash(&concatenated_hash))
}

fn main() {
   Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_private_key,
            generate_bitcoin_private_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}