# Yarukoto

macOS (Apple Silicon) 向け Google Tasks クライアントの開発用リポジトリです。Tauri v2 + React (TypeScript) をベースに、TanStack Router/Query・Tailwind・shadcn/ui コンポーネントで構成しています。詳細な要件や設計は `docs/` 配下を参照してください。

## 1. 前提条件

- Bun 1.1+
- Rust 1.80+（`cargo`, `rustup`, `tauri-cli` が使えること）
- Xcode Command Line Tools / CocoaPods

初回は下記で依存解決を行います。

```bash
bun install
```

## 2. 開発コマンド

| コマンド | 説明 |
| --- | --- |
| `bun run dev` | Vite のみでフロントをホットリロード |
| `bun run tauri dev` | Tauri シェル込みで macOS アプリを起動 |
| `bun run build` | Vite ビルド (Tauri `beforeBuildCommand` 用) |
| `bun run lint` / `bun run lint --apply` | ultra-site (Biome) による Lint / フォーマット |
| `bun run test` | Vitest + Testing Library |
| `cargo fmt` / `cargo clippy -- -D warnings` | Rust 側の整形 / 静的解析 |

## 3. ディレクトリ概要

- `src/app` TanStack Router のルートと Provider
- `src/features/tasks` Google Tasks 連携用の hooks/api/components
- `src/components/ui` shadcn/ui ベースの共通 UI
- `src-tauri` Rust 層 (`oauth`, `google_tasks`, `storage`, `models`)
- `docs/requirements.md` など要件・進捗ドキュメント

## 4. 環境設定メモ

- Tailwind は `tailwind.config.ts` + `src/styles/tailwind.css`
- Query/Routing は `src/app/providers.tsx` で一元管理
- `tauri.conf.json` の `bundle.targets` は DMG のみ（Apple Silicon 想定）
- シークレットは `.env` ではなく macOS Keychain or Tauri Store で扱う想定
- Git hooks は Lefthook で管理し、`bunx lefthook install` でセットアップ。pre-commit で `bunx ultracite fix` が走り、ステージ済みファイルをフォーマットします。

## 5. OAuth 用環境変数

- `.env.local` に `CLIENT_ID` / `CLIENT_SECRET`（ともに大文字）を設定し、Tauri 側で `std::env::var` から参照します。
- 値は Google Cloud Console で作成した Desktop OAuth クライアントのものを使用し、Git にはコミットしないでください。
- リダイレクトは `http://127.0.0.1:<port>/callback`（コマンド実行時にポート決定）なので、Console 側で必要なポートを許可しておきます。

より詳細なルールは `docs/coding-guidelines.md` と `AGENTS.md` を参照してください。
