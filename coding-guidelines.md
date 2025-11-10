# Yarukoto コーディングガイドライン

Yarukoto プロジェクトのコード規約と設計方針。

## 1. 全体方針

- 読みやすさ・一貫性・低認知負荷を優先
- 迷ったら TypeScript / TanStack / shadcn の公式推奨に寄せる
- 再発明がいらないところはライブラリに任せる

## 2. 言語・型の方針

### 2.1 TypeScript

- `strict: true` を基本とする
- `any` は原則禁止（やむを得ず使う場合はコメントで理由を書く）
- 共有したい構造は `type` / `interface` で定義

### 2.2 命名規則

- ファイル名: `kebab-case.tsx` / `kebab-case.ts`
- React コンポーネント: `PascalCase`
- hooks: `useXxx`（例: `useTaskLists`, `useCreateTask`）
- Tauri コマンド（Rust 側）: `snake_case`
- Tauri ラッパ関数（TS 側）: `camelCase`

例:

- Rust: `#[tauri::command] async fn list_tasklists(...)`  
- TS: `listTasklists()` → 内部で `invoke("list_tasklists")`

## 3. フロントエンド設計

### 3.1 ディレクトリ構成（例）

```text
src/
  app/              # TanStack Router エントリ
  features/
    tasks/
      components/
      hooks/
      api/
  components/ui/    # shadcn ベースの共通 UI
  lib/              # 汎用ユーティリティ
```

- 機能単位の feature ベース構成を基本とする
- 汎用 UI は `components/ui` にまとめる

### 3.2 TanStack Query

- Google Tasks / Tauri I/O はすべて TanStack Query 経由にする
- 生の `invoke` をコンポーネント内で直接呼ばない
- `useQuery` / `useMutation` は feature 配下の hook に閉じ込める

### 3.3 UI / shadcn

- まず shadcn/ui で実現できないかを検討してから自作する
- 色・タイポグラフィは基本 shadcn / Tailwind のプリセットを使う

### 3.4 スタイリング

- CSS-in-JS は使わず Tailwind を利用
- 繰り返し出るクラスパターンは `cn()` ヘルパーや `class-variance-authority` 等で整理

## 4. Tauri / Rust 設計

### 4.1 モジュール構成（案）

- `oauth`: Google OAuth フロー
- `google_tasks`: Google Tasks API 呼び出し
- `storage`: トークン / 設定の保存
- `models`: 共通の型定義

### 4.2 エラーハンドリング

- `Result<T, E>` を基本にする
- フロントに返すエラーは、なるべく意味の分かるメッセージ or 構造体に変換

### 4.3 シークレット

- `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` は `.env` / 環境変数から読み込む
- `.env` は Git にコミットしない

## 5. Lint / Format

### 5.1 ultra-site（Biome）

- ルールは基本 **デフォルトのまま** 使用する
- 合わないルールが出た場合のみ `biome.json` で局所的にオフ / 緩和
- フォーマットも ultra-site に統一し、Prettier は併用しない

### 5.2 Rust

- `cargo fmt` で整形
- `cargo clippy -- -D warnings` を lint の基準とし、警告は潰す方針

## 6. テスト方針（暫定）

- まずはビジネスロジック（タスク変換・日付処理など）のユニットテストを優先
- E2E / UI テストは将来的に Playwright / Vitest + jsdom 等で検討

## 7. コミット / ブランチ運用

- コミットメッセージ:
  - 例: `feat: add task list sidebar`, `fix: handle token refresh error`
- ブランチ命名:
  - `feature/xxx`, `fix/xxx` など用途が分かるもの
