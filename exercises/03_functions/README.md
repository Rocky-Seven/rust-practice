# 第3回：関数とスコープ - 練習問題

> **ブログ記事:** [Rust入門（3）「関数とスコープを学ぼう！」](https://my-studies.org/introduction-to-rust-3-lets-learn-about-functions-and-scope/)
> **学習ガイド:** [docs/03_functions.md](../../docs/03_functions.md)

---

## 🏋️ 練習問題の一覧

| ファイル | 内容 | 種類 |
| -------- | ---- | ---- |
| `ex01_basic.rs` | 関数の定義とパラメータ | 基本 |
| `ex02_return.rs` | 戻り値、タプルで複数の値を返す | 基本 |
| `ex03_scope.rs` | スコープ（出力を予想する） | 基本 |
| `challenge01_bmi.rs` | BMI計算関数 | チャレンジ |
| `challenge02_temperature.rs` | 温度変換関数 | チャレンジ |
| `challenge03_circle.rs` | 円の面積と円周 | チャレンジ |

---

## 📝 進め方

### 1. 練習問題の一覧を表示

```bash
cd /workspaces/rust-practice/exercises/03_functions
cargo run
```

### 2. ファイルを開いて、`todo!()` を書き換える

```bash
code src/bin/ex01_basic.rs
```

`todo!()` は「まだ実装していない」という目印です。このまま実行すると、プログラムは途中で止まります。自分のコードに書き換えてください。

### 3. 実行して確認

```bash
cargo run --bin ex01_basic
```

### 4. テストで答え合わせ

```bash
cargo test --bin ex01_basic
```

`test ... ok` と表示されれば正解です！🎉

> ※ `ex03_scope.rs` は `todo!()` がなく、出力を予想するタイプの問題です。テストはありません。

---

## 💡 ヒント

- 解答例は [docs/03_functions.md](../../docs/03_functions.md) にあります（チャレンジ1〜3）
- 詰まったら、エラーメッセージをよく読んでみましょう
- 戻り値の行に**セミコロンを付けない**ように注意！

---

**Happy Coding!** 🦀
