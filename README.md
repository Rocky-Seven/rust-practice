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
**ブログ記事：**  [Rust入門（2）「変数と型を学ぼう！」](https://my-studies.org/introduction-to-rust-2-lets-learn-about-variables-and-types/)
- 変数の宣言（`let`、`mut`）
- シャドーイング
- 基本的なデータ型
- タプルと配列

**リポジトリ資料：**
- [学習ガイド](./docs/02_variables.md)
- [練習問題](./exercises/02_variables/)

---

### 第3回：関数とスコープ
**ブログ記事：** [Rust入門（3）「関数とスコープを学ぼう！」](https://my-studies.org/introduction-to-rust-3-lets-learn-about-functions-and-scope/)

- 関数の定義と呼び出し
- パラメータと戻り値
- 式と文の違い
- スコープ

**リポジトリ資料：**

- [学習ガイド](https://github.com/Rocky-Seven/rust-practice/blob/main/docs/03_functions.md)
- [練習問題](https://github.com/Rocky-Seven/rust-practice/blob/main/exercises/03_functions)


---

### 第4回：制御構文
**ブログ記事：** [Rust入門（4）「制御構文を学ぼう！」](https://my-studies.org/introduction-to-rust-4-control-flow-statement/)

- if式
- loop、while、for
- match式

**リポジトリ資料：**

- [学習ガイド](https://github.com/Rocky-Seven/rust-practice/blob/main/docs/04_control_flow.md)
- [練習問題](https://github.com/Rocky-Seven/rust-practice/blob/main/exercises/04_control_flow)

---

### 第5回：所有権のルールとムーブ

**ブログ記事：** [Rust入門（5）「所有権のルールとムーブ」](https://my-studies.org/introduction-to-rust-5-ownership01/)

- 所有権の3つのルール
- ムーブ（所有権の移動）

**リポジトリ資料：**

- [学習ガイド](https://github.com/Rocky-Seven/rust-practice/blob/main/docs/05_ownership.md)
- [練習問題](https://github.com/Rocky-Seven/rust-practice/blob/main/exercises/05_ownership)

### 第6回：クローンと借用

**ブログ記事：** [Rust入門（6）「クローンと借用」](https://my-studies.org/introduction-to-rust-6-ownership02/)

- クローン（`.clone()`）
- 借用と参照（`&`、`&mut`）
- 借用のルール

**リポジトリ資料：**

- [学習ガイド](https://github.com/Rocky-Seven/rust-practice/blob/main/docs/06_borrowing.md)
- [練習問題](https://github.com/Rocky-Seven/rust-practice/blob/main/exercises/06_borrowing)

### 第7回：スライス

**ブログ記事：** [Rust入門（7）「スライス」](https://my-studies.org/introduction-to-rust-7-slices/)

- 文字列スライス（`&str`）
- 配列スライス（`&[T]`）
- 所有権シリーズのまとめ

**リポジトリ資料：**

- [学習ガイド](https://github.com/Rocky-Seven/rust-practice/blob/main/docs/)
- [練習問題](https://github.com/Rocky-Seven/rust-practice/blob/main/exercises/)
```

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

### 体験する

ブログ記事を読みながら、`trials/` で自由に試してみましょう：

```bash
# 第2回の体験用プロジェクト
cd trials/02_variables

# コードを編集
code src/main.rs

# 実行
cargo run
```

### 練習問題に取り組む

各回の練習問題は `exercises/` フォルダにあります：

```bash
# 第2回の練習問題ディレクトリに移動
cd exercises/02_variables

# 練習問題一覧を表示
cargo run

# 個別の練習問題を実行
cargo run --bin ex01_basic
cargo run --bin ex02_shadowing
cargo run --bin challenge01_bmi
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
├── docs/                     # 📚 学習ドキュメント（ブログ連動）
│   ├── 01_setup.md           # 第1回：環境構築
│   ├── 02_variables.md       # 第2回：変数と型
│   ├── 03_functions.md       # 第3回：関数とスコープ
│   ├── 04_control_flow.md    # 第4回：制御構文
│   ├── 05_ownership.md       # 第5回：所有権のルールとムーブ
│   ├── 06_borrowing.md       # 第6回：クローンと借用
│   └── 07_slices.md          # 第7回：スライス
│
├── t# 🧪 体験・お試し用
│   ├── 02_variables/        # 第2回で自由に試す場所
│   ├── 03_functions/        # 第3回で自由に試す場所
│   ├── 04_control_flow/     # 第4回で自由に試す場所
│   ├── 05_ownership/        # 第5回で自由に試す場所
│   ├── 06_borrowing/        # 第6回で自由に試す場所
│   └── 07_slices/           # 第7回で自由に試す場所
│
├── exercises/               # 🏋️ 練習問題
│   ├── 02_variables/        # 第2回の練習問題
│   ├── 03_functions/        # 第3回の練習問題
│   ├── 04_control_flow/     # 第4回の練習問題
│   ├── 05_ownership/        # 第5回の練習問題
│   ├── 06_borrowing/        # 第6回の練習問題
│   └── 07_slices/           # 第7回の練習問題
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
- [ ] 第3回：関数とスコープの学習
- [ ] 第3回：練習問題を全て完了
- [ ] 第4回：制御構文の学習
- [ ] 第4回：練習問題を全て完了
- [ ] 第5回：所有権のルールとムーブの学習
- [ ] 第5回：練習問題を全て完了
- [ ] 第6回：クローンと借用の学習
- [ ] 第6回：練習問題を全て完了
- [ ] 第7回：スライスの学習
- [ ] 第7回：練習問題を全て完了

---
## 補足：trials/*/.gitignore について

`trials/05_ownership`、`06_borrowing`、`07_slices` の `src/main.rs` は、第4回までと同じく `.gitignore` で追跡対象外にしてあります。`git add -f` で強制追加しないかぎり、実験コードはpushされません。

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
