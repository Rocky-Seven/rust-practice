# 🦀 Rust Practice - GitHub Codespacesで学ぶRust

> **ブラウザだけで始められる、初心者向けRust学習リポジトリ**  
> ブログ「[学びを始めよう](https://my-studies.org/)」と連動した実践的な教材です。

---

## 🎯 このリポジトリについて

このリポジトリは、プログラミング初心者がGitHub CodespacesでRustを学ぶためのものです。

**特徴：**
- ✅ **パソコンに何もインストール不要** - ブラウザだけで完結
- ✅ **ブログ記事と連動** - 理論と実践を同時に学べる
- ✅ **段階的な学習** - 基礎から応用まで順を追って習得
- ✅ **実践的な練習問題** - 手を動かして学べる

---

## 📚 学習コンテンツ（ブログ連動）

### 第1回：環境構築
**ブログ記事：** [GitHub CodespacesでRustを始めよう！](https://my-studies.org/get-started-with-rust-on-github-codespaces/)
- GitHub Codespacesの使い方
- Rustのインストール
- 最初の"Hello, World!"

**リポジトリ資料：**
- [補足ドキュメント](./docs/01_setup.md)

---

### 第2回：変数と型
**ブログ記事：** （リンクは公開後に追加）
- 変数の宣言（`let`、`mut`）
- シャドーイング
- 基本的なデータ型
- タプルと配列

**リポジトリ資料：**
- [学習ガイド](./docs/02_variables.md)
- [練習問題プロジェクト](./exercises/02_variables/)

---

### 第3回：関数とスコープ（準備中）
**ブログ記事：** （準備中）
- 関数の定義と呼び出し
- パラメータと戻り値
- 式と文の違い

---

### 第4回：制御構文（準備中）
**ブログ記事：** （準備中）
- if式
- loop、while、for
- match式

---

### 第5回：所有権（準備中）
**ブログ記事：** （準備中）
- 所有権とは
- 借用（Borrowing）
- スライス（Slice）

---

## 🚀 クイックスタート

### 1. Codespacesを起動

1. 画面上部の緑色の **「<> Code」** ボタンをクリック
2. **「Codespaces」** タブをクリック
3. **「Create codespace on main」** をクリック

→ 1〜2分待つと、ブラウザ内でコードエディタが開きます！

### 2. 最初のプログラムを実行

ターミナル（画面下部）で以下を実行：

```bash
cargo new hello_world
cd hello_world
cargo run
```

**「Hello, world!」と表示されれば成功です！** 🎉

詳しくは[第1回のブログ記事](https://my-studies.org/get-started-with-rust-on-github-codespaces/)をご覧ください。

---

## 📖 使い方

### ドキュメントを読む

各回の学習内容は `docs/` フォルダにあります：

```bash
# 第1回の補足を読む
cat docs/01_setup.md

# 第2回の学習ガイドを読む
cat docs/02_variables.md
```

### 練習問題に取り組む

各回の練習問題は `exercises/` フォルダにあります：

```bash
# 第2回の練習問題に取り組む
cd exercises/02_variables
cargo run --bin variables_exercises  # ← 明示的に指定
# または単に
cargo run  # ← Cargo.tomlでdefault-run設定済みならこれでOK

# 個別の練習問題を実行
cargo run --bin ex01_basic
cargo run --bin ex02_shadowing
cargo run --bin challenges
```

### コードを編集

1. 左側のファイル一覧から編集したいファイルを開く
2. コードを編集
3. 保存（Ctrl+S または Cmd+S）
4. `cargo run` で実行

### 変更をGitHubに保存

```bash
cd /workspaces/rust-practice
git add .
git commit -m "Complete exercises 02"
git push
```

---

## 🗂️ ディレクトリ構造

```
rust-practice/
├── docs/                     # 📚 学習ドキュメント（ブログ連動）
│   ├── 01_setup.md          # 第1回：環境構築
│   ├── 02_variables.md      # 第2回：変数と型
│   └── ...
│
├── exercises/                # 🏋️ 練習問題
│   ├── 02_variables/        # 第2回の練習
│   ├── 03_functions/        # 第3回の練習
│   └── ...
│
└── projects/                 # 🚀 実践プロジェクト
    ├── hello_rust/          # Hello, World!
    └── ...
```

---

## 💡 学習のヒント

### 効率的な学習方法

1. **ブログ記事を読む** - まず理論を理解する
2. **ドキュメントを参照** - `docs/` で詳細を確認
3. **コードを書く** - `exercises/` で実際に手を動かす
4. **エラーから学ぶ** - エラーメッセージは親切な先生

### ショートカットキー

| キー | 動作 |
|------|------|
| **Ctrl+S** | ファイル保存 |
| **Ctrl+`** | ターミナル表示/非表示 |
| **Ctrl+/** | コメント切り替え |
| **Ctrl+C** | 実行中の処理を中断 |

---

## 🤝 よくある質問

### Q. どの順番で学べばいい？

ブログ記事の順番通りに進めてください：

1. 第1回：環境構築 → Hello, World!
2. 第2回：変数と型 → 練習問題に挑戦
3. 第3回以降：順次公開

### Q. Codespacesの無料枠は？
### Q. Codespacesの無料枠は？

GitHubの無料アカウントには、以下の無料枠があります：

**無料枠（2025年12月時点）：**
- **コンピューティング時間：** 月120時間（2コアマシンの場合）
- **ストレージ容量：** 15 GB

**重要な注意点：**
- Codespacesを**停止しても、ストレージは消費し続けます**
- 長期間使わないCodespacesは**削除**することを推奨

**節約のコツ：**
1. **使わないときは停止する**
   - 左下の「Codespaces」ボタン → 「Stop Current Codespace」
   - または30分操作しないと自動停止
   
2. **不要なCodespacesは削除する**
   - GitHubの設定 → Codespaces → 削除
   - 削除してもGitHubにpushしたコードは残る
   
3. **定期的に確認する**
   - [Codespaces使用状況ページ](https://github.com/settings/billing)で確認

**ストレージ管理のベストプラクティス：**
- 作業が終わったら必ず `git push` してからCodespacesを削除
- 1つのリポジトリに複数のCodespacesを作らない
- 長期間（1週間以上）使わないCodespacesは削除する

### Q. エラーが出た！

**よくあるエラー：**

#### ターミナルが見えない
- 上部メニュー「View」→「Terminal」
- または **Ctrl+`**

#### git index.lockエラー
```bash
cd /workspaces/rust-practice
rm -f .git/index.lock
```

#### コマンドを間違えた
- **Ctrl+C** で中断

---

## 🌐 外部リソース

### 公式ドキュメント
- [The Rust Programming Language（日本語版）](https://doc.rust-jp.rs/book-ja/)
- [Rust by Example（日本語版）](https://doc.rust-jp.rs/rust-by-example-ja/)
- [Rust公式サイト](https://www.rust-lang.org/ja)

### ブログ
- [学びを始めよう - Rustカテゴリ](https://my-studies.org/category/manabi/it-basic/rust/)

---

## 📊 学習の進捗チェックリスト

- [ ] 第1回：Codespacesの起動
- [ ] 第1回：Hello, World! の実行
- [ ] 第1回：GitHubへの保存（commit & push）
- [ ] 第2回：変数と型の学習
- [ ] 第2回：練習問題を全て完了
- [ ] 第3回：関数の学習
- [ ] 第4回：制御構文の学習
- [ ] 第5回：所有権の理解

---

## 🎓 最後に

プログラミングは、**楽しみながら続けること**が一番大切です。

- わからないことがあっても大丈夫
- エラーが出ても大丈夫
- ゆっくり進めて大丈夫

**一歩ずつ、楽しく学んでいきましょう！** 🦀

---

## 📝 ライセンス

このリポジトリは学習目的で自由に使用できます。

---

**Happy Coding!** 🚀

**ブログ記事：** [学びを始めよう](https://my-studies.org/)