# nanokvm-discord-capture

NanoKVM から定期的に JPEG を取得し、Discord Webhook に画像を投稿するための Rust コンポーネントです。  
`wasm32-wasip2` をターゲットにし、`imago` 経由での実行を想定しています。

## 仕組み

- `imago:nanokvm/capture` 経由で NanoKVM にログイン
- 60 秒ごとに 1 枚 JPEG を取得
- `DISCORD_WEBHOOK_URL` に `multipart/form-data` で `capture.jpg` を送信
- 送信失敗時は終了せず、エラーログを出して次ループへ継続

## 必要な環境変数

`.env.example` を参考に設定してください。

```env
NANOKVM_USERNAME="admin"
NANOKVM_PASSWORD="admin"
DISCORD_WEBHOOK_URL=""
```

- `NANOKVM_USERNAME`: NanoKVM ログインユーザー名
- `NANOKVM_PASSWORD`: NanoKVM ログインパスワード
- `DISCORD_WEBHOOK_URL`: Discord Incoming Webhook URL（必須）

## ビルド

このリポジトリは `.cargo/config.toml` で `wasm32-wasip2` を既定ターゲットにしています。

```bash
cargo check --offline
cargo build --release
```

明示的にターゲットを指定する場合:

```bash
cargo build --target wasm32-wasip2 --release
```

## imago 設定

certのために`imago certs generate`を実行してください。

`imago.toml` の主な設定:

- `main`: `target/wasm32-wasip2/release/nanokvm-discord-capture.wasm`
- `build.command`: `cargo build --target wasm32-wasip2 --release`
- `target.default.remote`: 接続先 imagod (`192.168.1.143:4443`)

環境に応じて `target.default` の接続情報や証明書パス（`certs/client.key`）を調整してください。

## ライセンス

このプロジェクトは MIT License の下で提供されます。詳細は `LICENSE` を参照してください。
