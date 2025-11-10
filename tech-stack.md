# Yarukoto Tech Stack

mac で動く Google Tasks クライアント **Yarukoto** の技術スタック定義。

## アプリ概要

- 名前: **Yarukoto**
- 種別: デスクトップアプリ（Tauri）
- プラットフォーム: **macOS (Apple Silicon のみ)**
- 用途: Google Tasks のクライアント / 「Google Tasks 再発明」系アプリ

## フロントエンド

- 言語: **TypeScript**
- フレームワーク: **React**
- ルーティング: **TanStack Router**
- サーバー状態管理: **TanStack Query**
- UI ライブラリ: **shadcn/ui**
- スタイリング: **Tailwind CSS**
- アイコン: **lucide-react**

### フロントの原則

- 画面遷移・ルーティングは TanStack Router で統一
- API 呼び出しや Tauri コマンド呼び出しは TanStack Query の `useQuery` / `useMutation` でラップ
- UI コンポーネントは可能な限り shadcn/ui ベースで作る
- ビジネスロジックはカスタムフック（`useXxx`）に寄せる

## デスクトップ / ネイティブ層

- ランタイム: **Tauri v2**
- 言語: **Rust**
- 主な責務:
  - Google OAuth 2.0 (Authorization Code + PKCE)
  - Google Tasks API への HTTP 通信
  - アクセス / リフレッシュトークンの管理
  - ローカルストレージ（設定 / キャッシュ）

### Rust 側で想定している主なクレート

- HTTP クライアント: `reqwest`
- OAuth2 クライアント: `oauth2`
- ローカル HTTP サーバ: `axum` または `hyper`
- JSON: `serde`, `serde_json`
- 設定 / ストレージ: `tauri-plugin-store` など

## 認証基盤（Google OAuth 2.0）

- フロー: Authorization Code + PKCE
- ブラウザ: **システムデフォルトブラウザ**（埋め込み WebView は使わない）
- リダイレクト URI: `http://127.0.0.1:<dynamic-port>/callback`
- スコープ: `https://www.googleapis.com/auth/tasks`
- クライアント種別: Desktop Application

### シークレットの扱い

- `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` は Rust 側のみで保持
- `.env` や OS の環境変数経由で読み込む
- フロント（React）側には一切流さない

## パッケージ / ビルド / 配布

- パッケージマネージャ: **bun**
- ビルド: Tauri 標準（内部的には Vite ベース）
- 配布形式: **DMG のみ**
- 対応アーキテクチャ: **Apple Silicon (arm64)**

## 開発ツール

- Linter / Formatter: **ultra-site**（Biome ラップ。できるだけデフォルト厳しめ）
- Rust 側: `cargo fmt` + `cargo clippy` を実行
- エディタ想定: IntelliJ IDEA / VSCode など

## 今後の拡張候補

- macOS Keychain を使ったトークン保存
- メニューバー常駐のミニウィンドウ
- グローバルショートカットによるクイックタスク追加
- Web 版（TanStack Start など）の検討
