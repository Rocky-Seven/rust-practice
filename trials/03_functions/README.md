# 第3回：関数とスコープ - 体験用プロジェクト

> **ブログ記事:** [Rust入門（3）「関数とスコープを学ぼう！」](https://my-studies.org/introduction-to-rust-3-lets-learn-about-functions-and-scope/)

このディレクトリは、ブログ記事を読みながら自由にコードを試すための場所です。

---

## 📝 使い方

### 1. このディレクトリに移動

```bash
cd /workspaces/rust-practice/trials/03_functions
```

### 2. コードを編集

```bash
# VSCodeで開く
code src/main.rs
```

### 3. 実行

```bash
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
cd /workspaces/rust-practice/exercises/03_functions

# 練習問題一覧を表示
cargo run

# 個別の練習問題を実行
cargo run --bin ex01_basic
cargo run --bin ex02_return
cargo run --bin ex03_scope
cargo run --bin challenge01_bmi
cargo run --bin challenge02_temperature
cargo run --bin challenge03_circle
```

---

## 🔄 リセット方法

もし最初からやり直したい場合：

```bash
cd /workspaces/rust-practice/trials/03_functions

# src/main.rsを初期状態に戻す
cat > src/main.rs << 'RESET'
// 第3回：関数とスコープ - 体験用
// このファイルは自由に編集してください

fn main() {
    println!("関数とスコープを学ぼう！");

    // ここに自由にコードを書いてください
}
RESET

cargo run
```

---

**Happy Coding!** 🦀
