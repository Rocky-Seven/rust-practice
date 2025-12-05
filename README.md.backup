# Rust Practice - 初心者向けRust学習リポジトリ

このリポジトリは、プログラミング初心者がGitHub CodespacesでRustを学ぶためのものです。

**パソコンに何もインストール不要！** ブラウザだけで始められます。

---

## 🚀 クイックスタート：5分で始める

### 1. このリポジトリでCodespacesを起動

1. 画面上部の緑色の **「<> Code」** ボタンをクリック
2. **「Codespaces」** タブをクリック
3. **「Create codespace on main」** をクリック

→ 1〜2分待つと、ブラウザ内でコードエディタが開きます！

### 2. 自動セットアップを待つ

初回起動時、Rust環境が自動的にインストールされます（数分かかります）。

**画面下部のターミナルで以下のメッセージが出たら完了です：**
```
Done. Press any key to close the terminal.
```

### 3. Hello, World! を実行

ターミナル（画面下部の黒い部分）で、以下をコピー＆ペーストしてEnterキーを押してください：

```bash
cargo new hello_world
cd hello_world
cargo run
```

**「Hello, world!」と表示されれば成功です！** 🎉

---

## 📖 このリポジトリの使い方

### ファイル構成

```
Rust-practice/
├── README.md                    # このファイル
├── SETUP-FOR-beginner.md        # 詳しいセットアップガイド
├── STUDY.md                     # Rustの学習ガイド
├── 2_Variables.md               # 変数と型の練習
├── .devcontainer/               # 自動セットアップ設定
│   └── devcontainer.json
└── hello_world/                 # 最初のプロジェクト（作成後）
    └── src/
        └── main.rs
```

### 学習の流れ

1. **環境構築** - このREADMEで完了！
2. **Hello, World!** - 最初のプログラム実行
3. **変数と型** - `2_Variables.md`を読んで練習
4. **関数** - 次のステップ
5. **制御構文** - if、loop、forなど
6. **所有権** - Rustの最重要概念

詳しくは `STUDY.md` を参照してください。

---

## 🛠️ 基本的な使い方

### プロジェクトの作成

```bash
cargo new プロジェクト名
cd プロジェクト名
```

### プログラムの実行

```bash
cargo run
```

### コードの編集

1. 左側のファイル一覧から `src/main.rs` を開く
2. コードを編集
3. 保存（Ctrl+S または Cmd+S）
4. `cargo run` で実行

### 変更をGitHubに保存

```bash
# リポジトリのルートに移動
cd /workspaces/Rust-practice

# 変更を追加
git add .

# コミット（保存）
git commit -m "説明メッセージ"

# GitHubにアップロード
git push
```

---

## 📚 学習リソース

### このリポジトリ内

- **SETUP-FOR-beginner.md** - 詳しい環境構築ガイド
- **STUDY.md** - Rustとは？学習の意義
- **2_Variables.md** - 変数と型の練習メニュー

### 外部リソース

- [The Rust Programming Language (日本語版)](https://doc.rust-jp.rs/book-ja/) - 公式入門書
- [Rust by Example (日本語版)](https://doc.rust-jp.rs/rust-by-example-ja/) - 実例で学ぶ
- [公式サイト](https://www.rust-lang.org/ja)

---

## 💡 よくある質問

### Q. Codespacesって何？

GitHub Codespacesは、ブラウザ上で動く開発環境です。自分のパソコンに何もインストールせずにプログラミングができます。

### Q. 無料で使える？

はい！GitHubの無料アカウントで月に一定時間まで無料で使えます。使わないときはCodespacesを停止すると節約できます。

### Q. データは保存される？

はい。`git commit`と`git push`をすれば、GitHubに永久保存されます。Codespacesを削除してもデータは残ります。

### Q. エラーが出た！

**よくあるエラーと解決方法：**

#### 「git index.lock」エラー
```bash
cd /workspaces/Rust-practice
rm -f .git/index.lock
```

#### ターミナルが見えない
- 上部メニュー「View」→「Terminal」
- または Ctrl+` (バッククォート)

#### コマンドを間違えた
- Ctrl+C で中断できます

### Q. Codespacesを停止するには？

1. 左下の緑色の「Codespaces」ボタンをクリック
2. 「Stop Current Codespace」を選択

または、ブラウザのタブを閉じて、30分放置すると自動停止します。

---

## 🎯 最初の目標

### Step 1: Hello, World! を実行 ✅

```bash
cargo new hello_world
cd hello_world
cargo run
```

### Step 2: コードを変更してみる

`src/main.rs`を以下のように変更：

```rust
fn main() {
    println!("こんにちは、Rustの世界へ！");
    println!("私の最初のプログラムです！");
}
```

保存して再度実行：
```bash
cargo run
```

### Step 3: GitHubに保存

```bash
cd /workspaces/Rust-practice
git add .
git commit -m "My first Rust program"
git push
```

---

## 📝 次のステップ

Hello, World! が動いたら、次は変数と型を学びましょう！

**`2_Variables.md`を開いて、順番に練習してください。**

```bash
cargo new variables
cd variables
# 2_Variables.mdの手順に従って練習
```

---

## 🦀 Rustについて

Rustは、**安全で高速な**プログラミング言語です。

### なぜRustを学ぶ？

- ✅ メモリ安全性 - バグが少ない
- ✅ 高速性 - C/C++並みのパフォーマンス
- ✅ 親切なエラーメッセージ - 学びやすい
- ✅ 需要が高い - 企業での採用が増加中
- ✅ 楽しい！ - Stack Overflowで最も愛される言語

詳しくは `STUDY.md` を読んでください。

---

## 🤝 困ったときは

### エラーメッセージをよく読む

Rustのエラーメッセージはとても親切です。解決方法も提案してくれます。

### ドキュメントを見る

このリポジトリの `.md` ファイルに詳しい説明があります。

### 公式ドキュメントを確認

- https://doc.rust-jp.rs/book-ja/

---

## ✨ Tips

### ショートカットキー

- **Ctrl+S** (Cmd+S) - ファイル保存
- **Ctrl+`** - ターミナル表示/非表示
- **Ctrl+/** - コメント切り替え
- **Ctrl+C** - 実行中の処理を中断

### 効率的な学習方法

1. **コードを書く** - 読むだけでなく手を動かす
2. **エラーから学ぶ** - エラーは敵ではなく先生
3. **小さく始める** - 完璧を求めず、まず動かす
4. **繰り返す** - 何度も書いて慣れる

---

## 📊 学習の進捗

学習が進んだらチェックしていきましょう！

- [ ] Codespacesの起動
- [ ] Hello, World! の実行
- [ ] コードの編集
- [ ] GitHubへの保存（commit & push）
- [ ] 変数と型の学習
- [ ] 関数の学習
- [ ] 制御構文の学習
- [ ] 所有権の理解
- [ ] 構造体の学習
- [ ] エラー処理の学習

---

## 🎓 最後に

プログラミングは、**楽しみながら続けること**が一番大切です。

- わからないことがあっても大丈夫
- エラーが出ても大丈夫
- ゆっくり進めて大丈夫

**一歩ずつ、楽しく学んでいきましょう！** 🦀

---

**Happy Coding!** 🚀