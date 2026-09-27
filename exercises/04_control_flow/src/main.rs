// 第4回：制御構文 - 練習問題の一覧

fn main() {
    println!("=== 第4回：制御構文 - 練習問題 ===");
    println!();
    println!("【基本問題】");
    println!("  cargo run --bin ex01_if             if式（出力を予想しよう）");
    println!("  cargo run --bin ex02_loop            loop・while・for");
    println!("  cargo run --bin ex03_match           match式");
    println!();
    println!("【チャレンジ】");
    println!("  cargo run --bin challenge01_fizzbuzz  FizzBuzz関数");
    println!("  cargo run --bin challenge02_prime     素数判定関数");
    println!("  cargo run --bin challenge03_grade     成績判定関数（match版）");
    println!();
    println!("【ヒント】");
    println!("  ファイル内の todo!() を自分のコードに書き換えて完成させよう。");
    println!("  完成したかどうかは、次のコマンドで確認できる。");
    println!("  cargo test --bin ex02_loop");
}
