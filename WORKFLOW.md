# 開発ワークフロー

## プロジェクト構成

```
portfolio-page/          # フロントエンド (Rust/Yew → WASM)
  content → ../portfolio_content/  (symlink)
portfolio_content/       # 記事コンテンツ (Obsidian vault)
```

`content/` はシンボリックリンクで、`portfolio_content` リポジトリを参照します。
2つのリポジトリは独立して管理されます。

## 初期セットアップ

```bash
# 1. 両方のリポジトリをクローン
git clone git@github.com:khimoo/portfolio-page.git
git clone git@github.com:khimoo/portfolio_content.git

# 2. symlink を作成
cd portfolio-page
just setup
```

## ワークフロー別ガイド

### 記事の作成・編集

Obsidian で `portfolio_content/` を vault として開いて編集します。

```bash
# 1. Obsidian で記事を書く (portfolio_content/articles/my-article.md)
# 2. 書き終わったら content リポジトリに push
cd portfolio_content
git add articles/my-article.md
git commit -m "add my-article"
git push

# 3. 自動で portfolio-page の CI が起動し、数分後にデプロイされる
#    (repository_dispatch による自動連携)
```

手動で CI を起動したい場合は GitHub Actions の `workflow_dispatch` を使用してください。

### 記事の見た目確認（ローカルプレビュー）

```bash
cd portfolio-page
just dev
# → http://127.0.0.1:8080 でプレビュー
# → content/articles/*.md の変更は watchexec が自動検知して反映
```

Obsidian で記事を編集しながら、ブラウザでリアルタイムに確認できます。

### portfolio-page のコード修正

```bash
cd portfolio-page

# 1. コードを修正
# 2. ローカルで確認
just dev

# 3. commit & push → CI が自動でデプロイ
git add -p
git commit -m "fix: ..."
git push
```

## デプロイの仕組み

CI/CD (GitHub Actions) は以下のいずれかで起動します:

| トリガー | 条件 |
|----------|------|
| `repository_dispatch` | content リポジトリへの push (自動連携) |
| `push` | portfolio-page の main ブランチへの push |
| `schedule` | 毎日 0:00 UTC |
| `workflow_dispatch` | 手動実行 |

CI では `portfolio_content` リポジトリを `git clone` して `content/` に配置し、ビルド・デプロイを行います。

## よく使う just コマンド

| コマンド | 用途 |
|----------|------|
| `just setup` | 初期セットアップ (symlink 作成) |
| `just dev` | 開発サーバー起動 |
| `just build` | プロダクションビルド |
| `just process-data` | articles.json 再生成 |
| `just clean` | ビルド成果物を削除 |
