// 練習3：スコープ
//
// 【課題】
// このプログラムを実行する「前に」、A〜D にどんな値が出力されるか予想しよう。
// 予想したら、実行して答え合わせをしよう。
//
// 実行: cargo run --bin ex03_scope

const GREETING: &str = "こんにちは";

fn main() {
    println!("=== 練習3：スコープ ===");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("A: {}", x);
    }
    println!("B: {}", x);

    let y = {
        let a = 3;
        let b = 4;
        a * b
    };
    println!("C: {}", y);

    show_greeting();

    // 【チャレンジ】
    // 次の行のコメント（//）を外して実行するとエラーになる。なぜだろう？
    // 変数 a のスコープに注目しよう。
    // println!("a = {}", a);
}

fn show_greeting() {
    println!("D: {}", GREETING);
}
