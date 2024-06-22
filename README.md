# Bitcoin Mnemonic Phrase Generator

## Overview
This Tauri application, built with Rust, generates Bitcoin mnemonic phrase using a combination of user-provided codes and passwords. It employs Argon2 for key stretching and SHA-3 for additional hashing to ensure security and uniqueness of the keys.

## Features
- **Bitcoin Mnemonic Phrase Generation**: Generates Bitcoin mnemonic phrase in Wallet Import Format (WIF).
- **Secure Key Derivation**: Uses Argon2 and SHA-3 for secure key derivation.
- **Base58 Validation**: Validates the provided code and password for Base58 format compliance.

## Usage
- `generate_mnemonic_phrase`: Accepts a provided code and password to generate a Bitcoin mnemonic phrase.


## How to Run
Sure, here is the translation:

1. Ensure that Rust and Tauri dependencies are installed on your local machine.
2. Ensure that the node.js and node modules runtime environment is installed on your local machine.
3. Clone the repository from the terminal (command prompt).
4. Open the terminal and run `npm run build` to build the application for macOS and Windows.

## Dependencies
- `bitcoin` for Bitcoin-related functionalities.
- `secp256k1` for elliptic curve cryptography.
- `argon2` for password hashing.
- `sha3` for SHA-3 hashing.
- `bs58` for Base58 encoding and decoding.

## License
Copyright © 2024 Solidity Materials Co., Ltd. All Rights Reserved.

## Disclaimer
This software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

---

# ビットコイン秘密鍵ジェネレーター

## 概要
このTauriアプリケーションはRustで構築され、ユーザーが提供したコードとパスワードの組み合わせを使用してビットコインの秘密鍵を生成します。セキュリティと鍵のユニークさを確保するためにArgon2とSHA-3を使用しています。

## 特徴
- **ビットコイン秘密鍵生成**: Wallet Import Format（WIF）でビットコインの秘密鍵を生成します。
- **安全な鍵導出**: 鍵導出のためにArgon2とSHA-3を使用します。
- **Base58検証**: 提供されたコードとパスワードがBase58形式に準拠しているか検証します。

## 使い方
- `generate_mnemonic_phrase`: 提供されたコードとパスワードを受け取り、ビットコインのニーモニックフレーズを生成します。

## 実行方法
1. RustおよびTauriの依存関係がローカル端末にインストールされていることを確認します。
2. node.jsおよびnode modulesの実行環境がローカル端末にインストールされている事を確認します。
3. ターミナル（コマンドプロンプト）からリポジトリをクローンします。
4. ターミナルを開いて`npm run build`を実行してmacosとwindowsのアプリケーションをビルドします。

## 依存関係
- `bitcoin`：ビットコイン関連の機能のため。
- `secp256k1`：楕円曲線暗号のため。
- `argon2`：パスワードハッシュのため。
- `sha3`：SHA-3ハッシュのため。
- `bs58`：Base58エンコーディングとデコーディングのため。

## ライセンス
Copyright © 2024 Solidity Materials Co., Ltd. All Rights Reserved.

## 免責事項
このソフトウェアは「現状のまま」提供されており、商品性、特定目的への適合性、および権利侵害を含め、明示的または暗黙的ないかなる保証も伴いません。作者または著作権所有者は、契約、不法行為、またはその他の形態にかかわらず、このソフトウェアの使用またはその他の取引から生じるいかなるクレーム、損害、その他の責任についても責任を負いません。
