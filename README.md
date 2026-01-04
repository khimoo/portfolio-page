# Khimoo.io Portfolio

インタラクティブなマインドマップ形式のポートフォリオサイトです。記事間のリンク関係を可視化し、物理シミュレーションによるノード配置を実現しています。

## 記事データ処理フロー

### 1. 記事の読み込みと処理 (`ci-process-articles`)

**処理コマンド**: `just ci-process-articles`  
**実装ファイル**: `khimoo-portfolio/src/bin/process_articles.rs`

#### 処理ステップ

1. **記事ファイルのスキャン**
   - `khimoo-portfolio/articles/` ディレクトリからMarkdownファイルを再帰的に検索
   - `Templates/` ディレクトリは除外される
   - `.md` 拡張子のファイルのみを処理

2. **Front Matterのパース**
   - 各Markdownファイルの先頭にあるYAML形式のメタデータを抽出
   - 実装: `khimoo-portfolio/src/article_processing.rs` の `FrontMatterParser`
   - 抽出されるメタデータ:
     - `title`: 記事タイトル（必須）
     - `home_display`: ホーム画面に表示するかどうか（デフォルト: false）
     - `category`: カテゴリ（オプション）
     - `importance`: 重要度（1-5、デフォルト: 3）
     - `related_articles`: 関連記事のスラッグリスト
     - `tags`: タグリスト
     - `created_at`, `updated_at`: 日時情報
     - `author_image`: 作者画像のパス（オプション）

3. **リンクの抽出**
   - 実装: `khimoo-portfolio/src/article_processing.rs` の `LinkExtractor`
   - サポートされるリンク形式:
     - **WikiLink**: `[[article-name]]` または `[[target|display]]`
     - **MarkdownLink**: `[text](slug)` 形式（外部リンクは除外）
   - 各リンクから以下を抽出:
     - `target_slug`: リンク先のスラッグ
     - `link_type`: リンクの種類
     - `context`: リンク周辺の文脈（100文字）
     - `position`: リンクの位置（バイト位置）

4. **スラッグの生成**
   - ファイル名から自動生成（例: `about-khimoo.md` → `about-khimoo`）
   - 小文字に変換、スペースをハイフンに置換

5. **インバウンドリンク数の計算**
   - 全記事のアウトバウンドリンクを走査
   - 各記事が参照されている回数をカウント
   - `inbound_count` フィールドに設定

6. **JSONデータの生成**
   - 出力先: `khimoo-portfolio/data/articles.json`
   - データ構造:
     ```json
     {
       "articles": [
         {
           "slug": "article-slug",
           "title": "Article Title",
           "content": "Markdown content...",
           "metadata": { ... },
           "file_path": "articles/article.md",
           "outbound_links": [ ... ],
           "inbound_count": 3,
           "processed_at": "2024-01-01T00:00:00Z"
         }
       ],
       "generated_at": "2024-01-01T00:00:00Z",
       "total_count": 10,
       "home_articles": ["slug1", "slug2", ...]
     }
     ```
   - `home_articles` は `home_display: true` の記事のスラッグリスト

### 2. リンクグラフの生成 (`generate-link-graph`)

**処理コマンド**: `just generate-link-graph`  
**実装ファイル**: `khimoo-portfolio/src/bin/generate_link_graph.rs`

#### 処理ステップ

1. **記事データの読み込み**
   - `articles.json` から記事データを読み込む（`--graph-only` オプション時）
   - または記事を直接処理（通常モード）

2. **グラフ構造の構築**
   - 各記事をノードとして表現
   - アウトバウンドリンクをエッジとして追加
   - 内部リンクのみを対象（存在する記事へのリンクのみ）

3. **双方向リンクの検出**
   - 記事Aが記事Bにリンクし、記事Bも記事Aにリンクしている場合を検出
   - `bidirectional: true` フラグを設定
   - `ConnectionType::Bidirectional` として分類

4. **リンク数の集計**
   - 同じ記事への複数のリンクをカウント
   - `link_count` フィールドに記録

5. **JSONデータの生成**
   - 出力先: `khimoo-portfolio/data/link-graph.json`
   - データ構造:
     ```json
     {
       "graph": {
         "article-slug": {
           "connections": [
             {
               "target": "target-slug",
               "connection_type": "DirectLink" | "Bidirectional",
               "bidirectional": true | false,
               "link_count": 1
             }
           ],
           "inbound_count": 3,
           "outbound_count": 2
         }
       },
       "generated_at": "2024-01-01T00:00:00Z",
       "total_connections": 15,
       "bidirectional_pairs": 2,
       "direct_links": 11
     }
     ```

### 3. 画像の最適化 (`ci-optimize-images`)

**処理コマンド**: `just ci-optimize-images`  
**実装ファイル**: `scripts/optimize_images.py`

#### 処理ステップ

1. **元画像の読み込み**
   - `khimoo-portfolio/articles/img/author_img.png` を読み込み

2. **最適化されたバージョンの生成**
   - **小サイズ (64x64)**: `author_img_small.png` と `author_img_small.webp`
     - ノード表示用の軽量版
   - **中サイズ (128x128)**: `author_img_medium.png`
     - 将来の拡張用

3. **画像の配置**
   - 最適化された画像は `khimoo-portfolio/articles/img/` に保存
   - ビルド時に `dist/articles/img/` にコピーされる

### 4. WebAssemblyアプリケーションのビルド (`ci-build-wasm`)

**処理コマンド**: `just ci-build-wasm`  
**ビルドツール**: Trunk

#### 処理ステップ

1. **Trunkビルド**
   - `khimoo-portfolio/` ディレクトリで `trunk build --release` を実行
   - RustコードをWebAssemblyにコンパイル
   - `public-url` オプションで `/portfolio-page/` を指定（GitHub Pages用）

2. **アセットのコピー**
   - 画像ファイル: `articles/img/` → `dist/articles/img/`
   - データファイル: `data/*.json` → `dist/data/*.json`

3. **ビルド成果物**
   - `dist/index.html`: メインHTMLファイル
   - `dist/*.wasm`: WebAssemblyバイナリ
   - `dist/*.js`: JavaScriptバインディング

### 5. ホーム画面でのノード表示

**実装ファイル**: `khimoo-portfolio/src/home/components.rs`

#### データ読み込み

1. **DataLoaderによるデータ取得**
   - 実装: `khimoo-portfolio/src/home/data_loader.rs`
   - `use_articles_data()` フックを使用
   - `/data/articles.json` と `/data/link-graph.json` を非同期で読み込み
   - GitHub Pages環境では `/portfolio-page/data/` を自動検出

2. **ArticlesDataの構造**
   - `articles`: 全記事の配列
   - `home_articles`: ホーム表示対象のスラッグリスト

#### ノードレジストリの生成

1. **create_node_registry_from_articles() 関数**
   - `home_display: true` の記事のみをフィルタリング
   - 各記事からノードを生成:
     - **作者記事** (`author_image` が設定されている場合):
       - 中央に配置
       - 半径60px
       - `NodeContent::Author` タイプ
       - 最適化された中サイズ画像を使用
     - **通常記事**:
       - 円形に配置（コンテナ中心から半径30%の円周上）
       - 半径30px（重要度とリンク数で動的調整）
       - `NodeContent::Text` タイプ

2. **ノードの配置計算**
   - コンテナの中心座標を計算
   - 作者記事は中央に配置
   - その他の記事は円形に等間隔配置
   - 角度: `2π / 記事数` のステップ

3. **エッジ（リンク）の追加**
   - 各記事の `outbound_links` を走査
   - リンク先がホーム表示記事の場合、エッジを追加
   - `NodeRegistry.add_edge()` で登録

4. **ノードの属性設定**
   - `importance`: メタデータの重要度（1-5）
   - `inbound_count`: インバウンドリンク数
   - これらはノードのサイズ計算に使用される

#### 物理シミュレーション

1. **PhysicsWorldの初期化**
   - 実装: `khimoo-portfolio/src/home/physics_sim.rs`
   - ノードレジストリを物理世界に登録
   - 力の設定:
     - **反発力**: ノード間の距離に基づく反発
     - **リンク力**: エッジで接続されたノード間の引力
     - **中心力**: コンテナ中心への引力
     - **減衰**: 動きの減衰

2. **レンダリングループ**
   - `use_interval` フックで約120fps（8ms間隔）で更新
   - 各フレームで:
     - `PhysicsWorld.step()` で物理演算を1ステップ実行
     - ノード位置を更新
     - 再レンダリングをトリガー

3. **ノードの描画**
   - SVGでエッジを描画（背景レイヤー）
   - HTML要素でノードを描画（前景レイヤー）
   - ノードのサイズは重要度とリンク数で動的計算:
     ```rust
     base_radius + importance_bonus + popularity_bonus
     ```
   - 作者ノードは固定サイズ（半径60px）

#### インタラクション

1. **ノードのドラッグ**
   - マウスダウンでドラッグ開始
   - 5px以上移動したらドラッグモードに移行
   - ドラッグ中はノードをキネマティック（固定位置）に設定
   - マウスアップで動的モードに戻す

2. **ノードのクリック**
   - ドラッグしていない場合、クリックイベントを発火
   - `Route::ArticleShow { slug }` に遷移
   - 記事詳細ページを表示

## データフロー図

```
Markdown記事 (articles/*.md)
    ↓
[ci-process-articles]
    ├─ Front Matterパース
    ├─ リンク抽出
    ├─ スラッグ生成
    └─ インバウンドリンク数計算
    ↓
articles.json (data/articles.json)
    ↓
[generate-link-graph]
    ├─ グラフ構造構築
    ├─ 双方向リンク検出
    └─ リンク数集計
    ↓
link-graph.json (data/link-graph.json)
    ↓
[ci-optimize-images]
    └─ 画像最適化
    ↓
最適化画像 (articles/img/*)
    ↓
[ci-build-wasm]
    ├─ WebAssemblyコンパイル
    └─ アセットコピー
    ↓
dist/ ディレクトリ
    ├─ index.html
    ├─ *.wasm
    ├─ *.js
    ├─ data/
    │   ├─ articles.json
    │   └─ link-graph.json
    └─ articles/img/
        └─ *.png, *.webp
    ↓
[ブラウザでの実行]
    ├─ DataLoader.load_articles()
    ├─ DataLoader.load_link_graph()
    ├─ create_node_registry_from_articles()
    ├─ PhysicsWorld初期化
    └─ レンダリングループ
    ↓
ホーム画面のノード表示
```

## 使用されていないファイル

以下のファイルは現在のコードベースで使用されていないか、開発・テスト用途のみで本番環境では使用されていません:

### 1. `test-data-loader.html`
- **場所**: プロジェクトルート
- **用途**: DataLoaderのURL検出ロジックをテストするためのHTMLファイル
- **状態**: デプロイ時に `public/` にコピーされるが、実際のアプリケーションでは使用されていない
- **推奨**: 開発・デバッグ用途のため、`.gitignore` に追加するか、別のテストディレクトリに移動することを推奨

### 2. `khimoo-portfolio/src/home/article_manager.rs`
- **状態**: モジュールとして定義されているが、実際のコードでは使用されていない
- **理由**: `components.rs` で直接 `ArticlesData` を使用してノードレジストリを生成しているため
- **推奨**: 
  - 将来的に記事管理機能を追加する予定がある場合は保持
  - 使用予定がない場合は削除を検討

### 3. `.obsidian/` ディレクトリ
- **場所**: `khimoo-portfolio/articles/.obsidian/`
- **用途**: Obsidianエディタの設定ファイル
- **状態**: アプリケーションの実行には不要
- **推奨**: `.gitignore` に追加するか、開発環境のみで管理

### 4. 記事ファイル内の一時ファイル
- `about-khimoo.md~` (バックアップファイル)
- `無題のファイル.base` (一時ファイル)
- その他のテスト用Markdownファイル（`test.md`, `testa.md`, `asdfa.md` など）
- **推奨**: 不要なファイルは削除

## CI/CDパイプライン

`.github/workflows/ci-cd.yml` で定義されている処理フロー:

1. **環境セットアップ** (`ci-verify-setup`)
2. **画像最適化** (`ci-optimize-images`)
3. **記事処理** (`ci-process-articles`)
4. **リンクグラフ生成** (`generate-link-graph`)
5. **WebAssemblyビルド** (`ci-build-wasm`)
6. **デプロイ準備** (`ci-prepare-deploy`)
7. **検証** (`ci-verify`)
8. **GitHub Pagesへのデプロイ**

## 技術スタック

- **言語**: Rust (WebAssembly)
- **フレームワーク**: Yew (Rust製Webフレームワーク)
- **ビルドツール**: Trunk
- **物理エンジン**: カスタム実装 (`physics_sim.rs`)
- **データ形式**: JSON
- **デプロイ**: GitHub Pages

## 開発コマンド

- `just dev`: 開発サーバー起動（ホットリロード付き）
- `just dev-rebuild`: 開発用リビルド（データ処理 + WASMビルド）
- `just build`: 本番用ビルド
- `just process-articles`: 記事処理のみ
- `just generate-link-graph`: リンクグラフ生成のみ
- `just validate-links`: リンクの検証
