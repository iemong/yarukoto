## フェーズ2-1: Google Cloud 側の準備手順

1. **GCP プロジェクト作成**
   - https://console.cloud.google.com/projectcreate で新規プロジェクトを作成（例: `yarukoto-prod`）。課金アカウントを紐付け、プロジェクト ID を控える。

2. **Google Tasks API 有効化**
   - 対象プロジェクトで API ライブラリを開き、「Google Tasks API」を検索→「有効にする」をクリック。
   - 今後の監視用に API ダッシュボードへショートカットを残しておく。

3. **OAuth 同意画面設定**
   - 「API とサービス」→「OAuth 同意画面」でユーザータイプを External に設定。
   - アプリ情報（アプリ名: `Yarukoto`, サポートメール, 開発者メール）を入力し、スコープに `https://www.googleapis.com/auth/tasks` を追加。テストユーザーとして自分の Google アカウントを登録。

4. **OAuth クライアント作成（Desktop）**
   - 「認証情報」→「認証情報を作成」→「OAuth クライアント ID」。
   - アプリケーションの種類に「デスクトップアプリ」を選択し、名前を `yarukoto-desktop` などに設定。
   - 発行された `client_id` と `client_secret` を控![img.png](img.png)え、実装側の `.env.local` や macOS Keychain に保存（Repository にはコミットしない）。

5. **リダイレクト URI 登録**
   - Desktop アプリでは loopback アドレスが固定なので、`http://127.0.0.1:60123/callback` のように任意ポートを登録。
   - 実装側でポートを可変にする場合、`http://127.0.0.1` + 任意ポートの許可が必要なので、使用ポート候補をまとめて登録しておく。

6. **トークンアクセス検証**
   - OAuth Playground などでクライアントを使い、`tasks.readonly` ではなく `tasks` スコープでトークンを発行できることを確認。
   - 発行結果（アクセストークン/リフレッシュトークン）を `curl https://tasks.googleapis.com/tasks/v1/users/@me/lists` などで叩き、API の疎通済みか検証する。

> 以上を完了したら `docs/progress.md` のフェーズ2、最初の4チェック（プロジェクト作成～リダイレクト URI）を埋められる状態です。
