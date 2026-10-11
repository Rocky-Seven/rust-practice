# Rust入門（6）「クローンと借用」

前回は、所有権のルールと**ムーブ**について学んだ。今回は、同じ値を何度も使うための**クローン**と、所有権を移動せずに値を貸し借りする**借用（参照）**について学ぶ。

## 学習の進め方

前回と同じく、2段階で進める。

### 1. 体験する（trials/）

- ブログ記事を読みながら、`trials/06_borrowing/`で自由にコードを試す
- 自分で書き換えたり、わざとエラーを起こしたりして実験する

### 2. 練習する（exercises/）

- 体験が終わったら、`exercises/06_borrowing/`で構造化された練習問題に挑戦する
- `todo!()`の部分を自分のコードに書き換えて完成させる
- 解答例も用意されている

## 準備

```bash
cd /workspaces/rust-practice
git pull
cd trials/06_borrowing
code src/main.rs
```

## クローン：中身を複製する

前回、`String`は代入のたびに所有権が移動すると学んだ。もし中身を複製して、両方使えるようにしたいなら、`.clone()`を使う。

```rust
fn main() {
    let s1 = String::from("こんにちは");
    let s2 = s1.clone(); // 中身を複製する

    println!("s1 = {}", s1); // OK
    println!("s2 = {}", s2); // OK
}
```

```
s1 = こんにちは
s2 = こんにちは
```

`.clone()`は、ヒープ上のデータまで丸ごとコピーする。確実だが、データが大きいほど処理に時間がかかる。何でも`.clone()`で解決するのではなく、次に説明する**借用**を使うほうが適切な場面も多い。

## 借用：値を貸すだけで渡す

所有権を移動させず、値への「参照」を渡す方法がある。変数の前に`&`を付ける。

```rust
fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s); // 参照を渡す（借用）

    println!("'{}' の長さは {}", s, len); // sはまだ使える！
}

fn calculate_length(text: &String) -> usize {
    text.len()
}
```

```
'hello' の長さは 5
```

`calculate_length`は`text: &String`、つまり`String`への参照を受け取っている。所有権は移動しないため、関数を抜けても呼び出し元の`s`はそのまま使える。これを**借用**と呼ぶ。「値を貸しているだけで、所有権は渡していない」というイメージである。

## 可変な借用

借用した値を変更したい場合は、`&mut`を使う。

```rust
fn main() {
    let mut s = String::from("hello");

    add_world(&mut s);

    println!("{}", s);
}

fn add_world(text: &mut String) {
    text.push_str(", world");
}
```

```
hello, world
```

元の変数`s`も`mut`で宣言し、渡すときも`&mut s`と書く必要がある。

## 借用のルール

借用には、安全のための重要なルールがある。

- **同時に持てる可変参照（`&mut`）は1つだけ**
- **不変参照（`&`）は同時にいくつでも持てるが、可変参照と同時には持てない**

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // ← エラー！

    println!("{}, {}", r1, r2);
}
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

このルールのおかげで、「ある変数を書き換えている最中に、別の場所でも同時に読み書きしてしまう」という不具合（データ競合）が、コンパイル時に防がれる。複数のスレッドが同じデータを同時に書き換えて壊してしまう、といった事故は、C言語では実行時まで発覚しないことが多いが、Rustではコンパイルが通った時点でそれが起こらないと保証される。

## 練習問題

`&`を使って、文字列の長さを返す関数`str_len`を作り、渡した文字列がそのあとも使えることを確認してみよう。

**解答例を見る**

```rust
fn main() {
    let s = String::from("Rustは楽しい");
    println!("長さ: {}", str_len(&s));
    println!("元の文字列: {}", s); // 借用なので使える
}

fn str_len(text: &String) -> usize {
    text.len()
}
```

`.len()`はバイト数を返す点に注意（日本語の1文字は3バイト）。文字数を数えたい場合は`.chars().count()`を使う。

## exercises/06_borrowingの解答例

GitHubの`exercises/06_borrowing/`にある練習問題2問の解答例である。

### ex01_borrow：不変の借用

```rust
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}
```

`.split_whitespace()`で、スペース（や改行・タブ）区切りの単語に分割するイテレータを作り、`.count()`で個数を数える。

### ex02_mut_borrow：可変の借用

```rust
fn double_it(n: &mut i32) {
    *n *= 2;
}
```

`n`は`&mut i32`（整数への可変参照）である。参照先の値そのものを書き換えるには、`*n`のように`*`（デリファレンス）を付ける。`*n *= 2`は「参照先の値を2倍にして上書きする」という意味である。


## 保存して次に進もう

```bash
cd /workspaces/rust-practice
git add .
git commit -m "Complete lesson 6: clone and borrowing"
git push
```

体験が済んだら、`exercises/06_borrowing/`の練習問題にも挑戦してみよう（`cargo run`で一覧表示）。

## 次回予告

次回は、文字列や配列の一部分だけを借用する**スライス**について学ぶ。これで所有権シリーズは完結となる。
