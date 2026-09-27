# 第4回：制御構文 - 体験用プロジェクト

> **ブログ記事:** [Rust入門（4）「制御構文を学ぼう！」](https://my-studies.org/introduction-to-rust-4-lets-learn-about-control-flow/)

このディレクトリは、ブログ記事を読みながら自由にコードを試すための場所です。

---

## 📝 使い方

```bash
cd /workspaces/rust-practice/trials/04_control_flow
code src/main.rs
cargo run
```

---

## 💡 このディレクトリの目的

- ブログ記事の内容を**実際に試す**場所
- エラーが出ても問題なし
- 自由に実験してください
- 失敗は学習の一部です！

---

## 🏋️ 練習問題について

体験が終わったら、構造化された練習問題に挑戦しましょう：

```bash
cd /workspaces/rust-practice/exercises/04_control_flow

# 練習問題一覧を表示
cargo run

# 個別の練習問題を実行
cargo run --bin ex01_if
cargo run --bin ex02_loop
cargo run --bin ex03_match
cargo run --bin challenge01_fizzbuzz
cargo run --bin challenge02_prime
cargo run --bin challenge03_grade
```

---

## 🔄 リセット方法

```bash
cd /workspaces/rust-practice/trials/04_control_flow

cat > src/main.rs << 'RESET'
// 第4回：制御構文 - 体験用
// このファイルは自由に編集してください

fn main() {
    println!("制御構文を学ぼう！");

    // ここに自由にコードを書いてください
}
RESET

cargo run
```

---

**Happy Coding!** 🦀
