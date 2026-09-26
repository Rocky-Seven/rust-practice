// 第3回：関数とスコープ - 練習問題の一覧

fn main() {
    println!("=== 第3回：関数とスコープ - 練習問題 ===");
    println!();
    println!("【基本問題】");
    println!("  cargo run --bin ex01_basic          関数の基本（定義・パラメータ）");
    println!("  cargo run --bin ex02_return         戻り値（値を返す・タプル）");
    println!("  cargo run --bin ex03_scope          スコープ（出力を予想しよう）");
    println!();
    println!("【チャレンジ】");
    println!("  cargo run --bin challenge01_bmi          BMI計算関数");
    println!("  cargo run --bin challenge02_temperature  温度変換関数");
    println!("  cargo run --bin challenge03_circle       円の面積と円周");
    println!();
    println!("【ヒント】");
    println!("  ファイル内の todo!() を自分のコードに書き換えて完成させよう。");
    println!("  完成したかどうかは、次のコマンドで確認できる。");
    println!("  cargo test --bin ex01_basic");
}
