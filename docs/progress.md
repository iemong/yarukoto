# Yarukoto 開発進捗

Yarukoto プロジェクトの進捗を管理するためのメモ。チェックボックス形式で進める。

## 0. 全体フェーズ

- [ ] フェーズ 1: 基盤セットアップ
- [ ] フェーズ 2: 認証フロー実装
- [ ] フェーズ 3: タスクリスト・タスク一覧実装
- [ ] フェーズ 4: タスク操作（追加 / 完了 / 編集）
- [ ] フェーズ 5: UI/UX 調整
- [ ] フェーズ 6: DMG ビルド & 配布テスト

---

## 1. フェーズ 1: 基盤セットアップ

- [ ] Tauri プロジェクト作成（bun ベース）
- [ ] React + TypeScript テンプレ確認
- [ ] Tailwind 導入
- [ ] shadcn/ui 導入 & 初期コンポーネント import
- [ ] TanStack Router セットアップ
- [ ] TanStack Query セットアップ
- [ ] ultra-site 導入 & Lint スクリプト追加

---

## 2. フェーズ 2: 認証フロー

- [ ] Google Cloud Console でプロジェクト作成
- [ ] Google Tasks API 有効化
- [ ] OAuth クライアント（Desktop）発行
- [ ] リダイレクト URI 設定（`http://127.0.0.1:<port>/callback`）
- [ ] Rust 側に `start_oauth` コマンド作成
- [ ] ローカル HTTP サーバで `code` を受け取る処理実装
- [ ] トークンエンドポイント呼び出し（`oauth2` クレート）
- [ ] トークン保存実装（Tauri Store 等）
- [ ] アプリ起動時にトークンを読み込んで自動ログイン

---

## 3. フェーズ 3: タスクリスト / タスク一覧

- [ ] Google Tasks API への疎通確認
- [ ] `list_tasklists` Tauri コマンド実装
- [ ] `list_tasks` Tauri コマンド実装
- [ ] TanStack Query でタスクリストを取得する hook 実装
- [ ] TanStack Query でタスク一覧を取得する hook 実装
- [ ] UI: 左ペインにタスクリスト表示
- [ ] UI: 右ペインにタスク一覧表示

---

## 4. フェーズ 4: タスク操作

- [ ] `insert_task` コマンド（新規タスク作成）
- [ ] `update_task` / `patch_task` コマンド（完了状態 / タイトル更新）
- [ ] TanStack Query の `useMutation` でラップ
- [ ] キャッシュ更新（楽観的更新 or 再フェッチ）
- [ ] UI: 新規タスク入力ボックス + Enter で追加
- [ ] UI: チェックボックスで完了状態切り替え
- [ ] UI: インライン編集でタイトル変更

---

## 5. フェーズ 5: UI/UX 調整

- [ ] ローディング状態（Skeleton / Spinner）追加
- [ ] エラー時トースト表示
- [ ] テーマ（ライト / ダーク）対応
- [ ] ショートカット整理（`Cmd+N`, `Cmd+R` など）
- [ ] アイコン・ロゴ（Yarukoto 用）作成

---

## 6. フェーズ 6: ビルド / リリース

- [ ] tauri.conf で Apple Silicon / DMG 設定
- [ ] `tauri build` でローカルビルド
- [ ] 実機 Mac でインストールテスト
- [ ] 最初の内部リリース（自分用配布）

---

## 7. メモ / 決定事項

- アプリ名: **Yarukoto**
- バンドルID（案）: `com.iemong.yarukoto`
- Linter: ultra-site（デフォルト厳しめ）
- 状態管理: TanStack Query（サーバー状態）
- ルーティング: TanStack Router
- 認証: Google OAuth2（Authorization Code + PKCE, system browser + loopback）
