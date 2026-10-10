// 練習2：エラーを直す練習
//
// 【課題】
// 以下のコードは、所有権のルール違反でコンパイルエラーになる。
// コメントアウトしてある2行目を有効化すると、エラーを再現できる。
// エラーにならないように、あなたの手で修正しよう
// （ヒント：値を複製するか、2つ目の値を別に用意する）
//
// 実行: cargo run --bin ex02_fix

fn main() {
    let name = String::from("花子");
    print_name(name.clone()); // ← ここを修正した（元は name だけだった）
    print_name(name);
}

fn print_name(name: String) {
    println!("名前: {}", name);
}
