# workflows

個人リポジトリ共通の GitHub Actions 再利用可能ワークフロー置き場。

## rust-gates.yml

Rust リポジトリ向けの標準検証ゲート。ローカルの標準検証ループと同じ順序で
実行する。

1. `cargo fmt --all -- --check`
2. `cargo check --all-targets`（既定で `--locked`）
3. `cargo test --all-targets`（既定で `--locked`）
4. `cargo clippy --all-targets -- -D warnings`（既定で `--locked`、`clippy_args` を追記）
5. `cargo audit`（任意。`audit: true` のとき）
6. `extra_gates`（呼び出し側が指定する追加ゲート）

呼び出し例（Tauri アプリの想定）:

```yaml
name: CI

on:
  push:
    branches:
      - master
  pull_request:
  workflow_dispatch:

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

permissions:
  contents: read

jobs:
  gates:
    uses: onoderakeita/workflows/.github/workflows/rust-gates.yml@v1
    with:
      apt_packages: |
        libwebkit2gtk-4.1-dev
        libayatana-appindicator3-dev
      wasm_targets: wasm32-unknown-unknown
      extra_gates: |
        cargo check -p my-core --target wasm32-unknown-unknown --locked
        cargo check -p my-web --target wasm32-unknown-unknown --locked
```

### 入力

| 名前 | 型 | 既定 | 説明 |
| --- | --- | --- | --- |
| `apt_packages` | string | `""` | apt で追加導入するパッケージ（空白・改行区切り） |
| `rust_toolchain` | string | `stable` | rustup のツールチェーン名 |
| `wasm_targets` | string | `""` | `rustup target add` する対象（カンマ区切り） |
| `extra_gates` | string | `""` | 固定ゲートの後に実行する追加コマンド |
| `clippy_args` | string | `""` | clippy の `-D warnings` に追記する引数 |
| `env_vars` | string | `""` | ゲート前に `$GITHUB_ENV` へ追加する環境変数 |
| `working_directory` | string | `"."` | `Cargo.toml` のあるディレクトリ |
| `locked` | boolean | `true` | 各 cargo ゲートに `--locked` を付ける |
| `audit` | boolean | `false` | `cargo audit` による依存関係監査を実行する |
| `runner` | string | `ubuntu-24.04` | 実行するランナー |
| `timeout_minutes` | number | `60` | ジョブのタイムアウト（分） |

`clippy_args` は `-- -D warnings` の後ろに追記される
（例: `-A clippy::module-inception`）。

### 運用メモ

- 呼び出し側で `permissions: contents: read` と `concurrency` を設定する。
- `Cargo.lock` をコミットしていないリポジトリは `locked: false` を指定する。
- テスト用の環境変数は `env_vars` で渡す（呼び出し側の `env:` は再利用側へ
  伝播しない）。
- 参照はタグ `v1` にピン留めする。互換を保つ更新はタグを付け替え
  （`git tag -f v1 && git push -f origin v1`）、破壊的変更は `v2` を切る。
- このリポジトリは public（2026-10-17 公開）。public・private を問わず
  どのリポジトリからも参照できる（private 提供側で必要だった Access
  設定は不要）。
- 監査ゲートは `audit: true` で有効化する。脆弱性検出時に失敗し、
  unmaintained 等の warning は既定では失敗しない。ignore は各
  リポジトリの `.cargo/audit.toml` で管理する。`--deny warnings` に
  したい場合は `extra_gates` に `cargo audit --deny warnings` を
  追加する。
- `self-test.yml` が `fixtures/` のクレートに対して回帰テストを実行する
  （push ごとに検証される）。
- 秘密情報は渡さない（`secrets` は使わず、入力は `with` のみ）。
