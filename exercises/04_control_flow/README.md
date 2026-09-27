# 第4回：制御構文 - 練習問題

> **ブログ記事:** [Rust入門（4）「制御構文を学ぼう！」](https://my-studies.org/introduction-to-rust-4-lets-learn-about-control-flow/)
> **学習ガイド:** [docs/04_control_flow.md](../../docs/04_control_flow.md)

---

## 🏋️ 練習問題の一覧

| ファイル | 内容 | 種類 |
| -------- | ---- | ---- |
| `ex01_if.rs` | if式（出力を予想する） | 基本 |
| `ex02_loop.rs` | loop・while・for | 基本 |
| `ex03_match.rs` | match式 | 基本 |
| `challenge01_fizzbuzz.rs` | FizzBuzz関数 | チャレンジ |
| `challenge02_prime.rs` | 素数判定関数 | チャレンジ |
| `challenge03_grade.rs` | 成績判定関数（match版） | チャレンジ |

---

## 📝 進め方

1. `cargo run` で一覧を表示
2. `code src/bin/ファイル名.rs` で開いて `todo!()` を書き換える
3. `cargo run --bin ファイル名` で実行して確認
4. `cargo test --bin ファイル名` でテストして答え合わせ

> ※ `ex01_if.rs` は `todo!()` がなく、出力を予想するタイプの問題です。テストはありません。

---

## 💡 ヒント

- 解答例は [docs/04_control_flow.md](../../docs/04_control_flow.md) にあります（チャレンジ1〜3）
- `match` はすべてのパターンを網羅する必要があります。`_` を忘れずに
- `if`式は、両方のブロックが同じ型を返す必要があります

---

**Happy Coding!** 🦀
