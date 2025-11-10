# Repository Guidelines

## Project Structure & Module Organization
`src/app` で TanStack Router を初期化し、機能別モジュールは `src/features/<domain>/` にまとめます。UI コンポーネントは `src/components/ui`、共有ユーティリティは `src/lib`、型は `src/types` に置きます。Google Tasks 連携ロジックは feature 配下の hooks と api ディレクトリに分離し、テストダブルは `__mocks__` 直下へ。Tauri 側は `src-tauri/` に `oauth`, `google_tasks`, `storage`, `models` を切り、設定値は `tauri.conf.json` と `.env.example` の 2 箇所で同期してください。仕様・進捗はリポジトリ直下の `requirements.md`, `tech-stack.md`, `progress.md` を常に更新します。

## Build, Test, and Development Commands
依存関係は `bun install`、統合開発は `bun tauri dev` を使用します。フロントのみを確認する場合は `bun run dev`、Lint/Format は `bun run lint`（ultra-site）で実行し、自動修正は `bun run lint --apply` を許可します。Rust 側は `cargo fmt` と `cargo clippy -- -D warnings` を事前フック化し、最終的な DMG は `bun tauri build --target universal-apple-darwin` で生成して arm64 実機で確認してください。

## Coding Style & Naming Conventions
TypeScript は `strict` 前提で `any` を禁止、2 スペースインデントを統一します。ファイルは `kebab-case.tsx`、コンポーネントは `PascalCase`、hooks は `useTaskLists` のような `use` 接頭辞、Tauri コマンドは Rust 側 `snake_case` + TS ラッパ `camelCase` を徹底します。インポートは「標準 → 外部 → 内部」の順に並べ、Tailwind を基本とし、重複クラスは `cn()` や `class-variance-authority` で整理してください。

## Testing Guidelines
現状はビジネスロジック中心のユニットテストを優先し、TanStack Query のデータ整形や日付計算を Vitest で検証します。テストファイルは対象と同じ階層に `*.test.ts` を置き、ケース名は「対象_期待動作_条件」を日本語で簡潔に記述します。Rust 側は `cargo test --package src-tauri` を最低限実行し、OAuth と Google Tasks API の統合は将来的に Playwright か Tauri E2E で自動化します。主要フローのカバレッジ 80% を目安に、不足箇所は issue に記録してください。

## Commit & Pull Request Guidelines
コミットは Conventional Commits 互換の `feat:`, `fix:`, `chore:` などで始め、1 つの論理変更に絞ります。ブランチ名は `feature/<topic>` や `fix/<topic>` を推奨し、PR では概要・スクリーンショット・動作確認コマンド・関連 issue・リスクとロールバック手順をテンプレートに沿って記載してください。レビュー前に `bun run lint`, `bun run test`, `cargo clippy` の結果を貼り付け、仕様差分があれば `progress.md` にも反映します。

## Security & Configuration Tips
`GOOGLE_CLIENT_ID` と `GOOGLE_CLIENT_SECRET` は `.env` ではなく macOS Keychain か Tauri Store で管理し、TS 側へ渡さないでください。`tauri.conf.json` の `bundle.identifier` は `com.iemong.yarukoto` を使用し、共有ログにはユーザー情報やトークンを含めないようマスクします。開発用 `.env.local` はサンプルのみ追跡し、実値は 1Password などのセキュアストレージ経由で配布してください。
