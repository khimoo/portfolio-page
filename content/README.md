# Content Directory

このディレクトリには、ポートフォリオサイトのコンテンツ（記事と画像）が含まれています。

## 構造

```
content/
├── articles/           # Markdown記事ファイル
│   ├── *.md           # 記事ファイル
│   └── Templates/     # 記事テンプレート
└── assets/            # 静的アセット
    └── img/           # 画像ファイル
```

## 設計原則

### DRY (Don't Repeat Yourself)
- 記事とアセットを一箇所に集約
- 重複する設定や処理を排除
- **設定の一元管理**: `project.toml`で全パスを管理

### ETC (Easy to Change)
- コンテンツとアプリケーションコードを分離
- 将来の別リポジトリ移行に対応
- **設定ベースの構成**: パス変更は設定ファイルのみで対応

### KISS (Keep It Simple, Stupid)
- シンプルなディレクトリ構造
- 明確な責任分離
- **統一された設定システム**: 一つの設定ファイルで全体を管理

## 設定システム

プロジェクトの全パス設定は `project.toml` で一元管理されています：

```toml
[paths]
articles_dir = "content/articles"
assets_dir = "content/assets"
images_dir = "content/assets/img"
data_dir = "khimoo-portfolio/data"
# ... その他の設定
```

### 設定の取得方法

```bash
# Python経由で設定値を取得
python3 scripts/config.py articles_dir          # 絶対パス
python3 scripts/config.py articles_dir --relative  # 相対パス
```

### 利点

- **DRY**: パスのハードコーディングを排除
- **ETC**: 設定変更が容易
- **KISS**: 一箇所での設定管理

## 将来の別リポジトリ管理

このディレクトリは将来的に独立したリポジトリとして管理することを想定しています：

### 移行手順（将来）
1. `content/` ディレクトリを独立したリポジトリに移動
2. `project.toml` の `articles_dir` を更新
3. Git submoduleまたはGit subtreeとして統合
4. CI/CDパイプラインでコンテンツの自動同期

### 利点
- コンテンツとコードの独立した管理
- 記事執筆者とエンジニアの作業分離
- バージョン管理の柔軟性
- **設定システムによる透明な移行**

## 記事の作成

新しい記事を作成する際は：

1. `content/articles/` に `.md` ファイルを作成
2. Front matterで必要なメタデータを設定
3. 画像は `content/assets/img/` に配置
4. `just process-articles` でデータを更新

## 画像の最適化

画像を追加・更新した際は：

```bash
# 記事処理と同時に画像最適化
just process-articles-with-images

# または CI/CD パイプラインで
just ci-optimize-images
```

これにより、WebP形式への変換や複数サイズの生成が自動実行されます。

### 画像最適化の仕組み

- **統合処理**: 記事処理の一部として画像最適化を実行
- **効率性**: 記事で参照されている画像のみを最適化
- **設定ベース**: `project.toml`で最適化パラメータを管理
- **複数サイズ**: 小サイズ(64x64)、中サイズ(128x128)を自動生成
- **WebP対応**: 軽量なWebP形式での保存