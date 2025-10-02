# rust-samlocalapi-localstack

Rust Lambda関数とLocalStack DynamoDBを使用したユーザー管理APIのサンプルプロジェクト

## 概要

このプロジェクトは、AWS SAMを使用してRust製のLambda関数を構築し、LocalStackでDynamoDBと連携するサンプル実装です。
ユーザー情報のCRUD操作を行うREST APIを提供します。

## 技術スタック

- **Runtime**: Rust (provided.al2023)
- **Framework**: AWS SAM (Serverless Application Model)
- **Database**: DynamoDB (LocalStack)
- **Dependencies**:
  - aws-sdk-dynamodb
  - lambda_runtime
  - serde / serde_json
  - aws_lambda_events

## 前提条件

以下のツールがインストールされている必要があります:

- [Docker](https://www.docker.com/)
- [AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html)
- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-lambda](https://www.cargo-lambda.info/guide/installation.html)

```bash
# cargo-lambdaのインストール
cargo install cargo-lambda
```

## セットアップ

### 1. LocalStackの起動

DynamoDBをローカル環境で動かすためにLocalStackを起動します。

```bash
docker compose up -d
```

起動すると、`scripts/init.sh`が自動実行され、`users`テーブルが作成されます。

### 2. ビルド

```bash
sam build
```

### 3. ローカルでAPIを起動

```bash
sam local start-api --docker-network rust-samlocalapi-localstack_default
```

APIは `http://127.0.0.1:3000` で利用可能になります。

## API仕様

### GET / - ユーザー一覧取得

usersテーブルから全ユーザーを取得します。

**レスポンス例:**
```json
{
  "users": [
    {
      "user_id": "001",
      "name": "田中太郎",
      "email": "tanaka@example.com"
    }
  ],
  "count": 1
}
```

**ステータスコード:**
- `200`: 成功
- `500`: サーバーエラー

### POST / - ユーザー作成

新しいユーザーをusersテーブルに登録します。

**リクエスト例:**
```json
{
  "user_id": "001",
  "name": "田中太郎",
  "email": "tanaka@example.com"
}
```

**レスポンス例:**
```json
{
  "message": "User created successfully",
  "user": {
    "user_id": "001",
    "name": "田中太郎",
    "email": "tanaka@example.com"
  }
}
```

**ステータスコード:**
- `201`: 作成成功
- `400`: 不正なリクエスト
- `500`: サーバーエラー

## 使用例

### ユーザーの作成

```bash
curl -X POST http://127.0.0.1:3000/ \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "001",
    "name": "田中太郎",
    "email": "tanaka@example.com"
  }'
```

### ユーザー一覧の取得

```bash
curl http://127.0.0.1:3000/
```

## プロジェクト構成

```
.
├── Cargo.toml              # Rustプロジェクト設定
├── template.yml            # SAMテンプレート
├── swagger.yml             # OpenAPI仕様書
├── docker-compose.yml      # LocalStack設定
├── scripts/
│   └── init.sh            # DynamoDBテーブル初期化スクリプト
└── src/
    ├── get_function.rs    # GET / ハンドラー
    └── post_function.rs   # POST / ハンドラー
```

## DynamoDBテーブル構造

### usersテーブル

| 属性名 | 型 | 説明 |
|--------|-----|------|
| user_id | String | ユーザーID (パーティションキー) |
| name | String | ユーザー名 |
| email | String | メールアドレス |

## トラブルシューティング

### LocalStackに接続できない

Docker Networkが正しく設定されているか確認してください:

```bash
docker network ls
docker compose ps
```

### ビルドエラーが発生する

cargo-lambdaが正しくインストールされているか確認してください:

```bash
cargo lambda --version
```

## ライセンス

MIT License

## 関連リンク

- [AWS SAM Documentation](https://docs.aws.amazon.com/serverless-application-model/)
- [LocalStack Documentation](https://docs.localstack.cloud/)
- [cargo-lambda Documentation](https://www.cargo-lambda.info/)
