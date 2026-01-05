# ディレクトリ構造リファクタリング計画

## 現在の問題
- `src/articles/` と `src/bin/` の責任重複
- CLI処理ロジックが分散
- UI関連とデータ処理が混在

## 新しい構造

```
src/
├── lib.rs                    # ライブラリエントリポイント
├── main.rs                   # WebAssemblyアプリケーション
├── config.rs                 # 設定管理
├── config_loader.rs          # 設定ローダー
│
├── cli/                      # CLI専用モジュール
│   ├── mod.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── process_articles.rs
│   │   └── validate_links.rs
│   └── utils/
│       ├── mod.rs
│       └── output_formatter.rs
│
├── core/                     # コアビジネスロジック
│   ├── mod.rs
│   ├── articles/
│   │   ├── mod.rs
│   │   ├── metadata.rs
│   │   ├── processor.rs
│   │   └── links/
│   │       ├── mod.rs
│   │       ├── extractor.rs
│   │       └── validator.rs
│   └── media/
│       ├── mod.rs
│       └── image_optimizer.rs
│
└── web/                      # Webアプリケーション全体
    ├── mod.rs
    ├── app.rs                # ルートアプリケーション
    ├── routes.rs             # ルーティング定義
    ├── header.rs             # 共通ヘッダー
    ├── data_loader.rs        # データローダー
    ├── article_manager.rs    # 記事管理
    ├── physics_sim.rs        # 物理シミュレーション
    ├── styles/               # スタイル定義
    │   ├── mod.rs
    │   └── layout_styles.rs
    ├── pages/                # ページコンポーネント
    │   ├── mod.rs
    │   ├── home.rs           # ホーム画面（ノードグラフ）
    │   ├── article_index.rs  # 記事一覧
    │   └── article_view.rs   # 記事詳細
    ├── components/           # 共通コンポーネント
    │   ├── mod.rs
    │   ├── node_graph_container.rs
    │   ├── physics_renderer.rs
    │   ├── node_renderer.rs
    │   ├── debug_panel.rs
    │   ├── node_data_manager.rs
    │   ├── article_header.rs
    │   ├── article_content.rs
    │   └── article_state_renderer.rs
    ├── config/               # UI設定
    │   ├── mod.rs
    │   ├── theme_config.rs
    │   ├── style_config.rs
    │   └── physics_config.rs
    └── types/                # UI型定義
        ├── mod.rs
        ├── ui_types.rs
        ├── data_types.rs
        ├── node_types.rs
        └── physics_types.rs
```

## 責任分離

### `cli/` - CLI専用
- コマンドライン引数処理
- 出力フォーマット
- プログレス表示
- エラーハンドリング

### `core/` - ビジネスロジック
- 記事処理の核となるロジック
- メタデータ解析
- リンク抽出・検証
- 画像最適化
- データ変換

### `web/` - Webアプリケーション
- ルーティング（`/`, `/article`, `/article/:slug`）
- ページコンポーネント（ホーム、記事一覧、記事詳細）
- 共通コンポーネント（ヘッダー、ノードグラフ、物理シミュレーション）
- UI状態管理
- スタイル定義

## 主要な変更点

1. **`src/home/` → `src/web/`**: より適切な命名
2. **`src/home/app.rs` → `src/web/pages/home.rs`**: ホーム画面を明確に分離
3. **記事関連コンポーネント**: `src/web/components/` に統合
4. **`src/articles/` → `src/core/articles/`**: ビジネスロジックとして明確化
5. **`src/bin/` → `src/cli/commands/`**: CLI専用として整理

## 移行手順

1. 新しいディレクトリ構造作成
2. `core/` モジュールの移行（`src/articles/` → `src/core/articles/`）
3. `cli/` モジュールの移行（`src/bin/` → `src/cli/commands/`）
4. `web/` モジュールの移行（`src/home/` → `src/web/`）
5. `main.rs` の import パス更新
6. `bin/` ディレクトリの削除
7. 依存関係の更新
8. テスト実行

## 期待される効果

- **DRY原則**: 記事処理ロジックの重複排除
- **ETC原則**: 変更しやすい構造（CLI、Web、コアロジックが独立）
- **KISS原則**: シンプルで理解しやすい責任分離