// 練習1：if式
//
// 【課題】
// このプログラムを実行する「前に」、A〜Dにどんな値が出力されるか予想しよう。
// 予想したら、実行して答え合わせをしよう。
//
// 実行: cargo run --bin ex01_if

fn main() {
    println!("=== 練習1：if式 ===");

    let x = 8;

    let a = if x % 2 == 0 { "偶数" } else { "奇数" };
    println!("A: {}", a);

    let y = 3;
    let b = if y > 5 {
        "5より大きい"
    } else if y > 0 {
        "0より大きく5以下"
    } else {
        "0以下"
    };
    println!("B: {}", b);

    let z = -2;
    let c = if z >= 0 { z } else { -z };
    println!("C: {}", c);

    let flag = true;
    let d = if flag && z < 0 { "両方成立" } else { "不成立" };
    println!("D: {}", d);
}
