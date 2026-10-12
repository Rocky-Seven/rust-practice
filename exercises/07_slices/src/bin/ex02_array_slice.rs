// 練習2：配列スライス
//
// 【課題】
// average(nums) : i32のスライスを受け取り、平均値をf64で返す
//                 （空のスライスの場合は 0.0 を返す）
//
// ヒント: nums.len() で要素数、nums.iter().sum() で合計が求められる
//
// 実行  : cargo run --bin ex02_array_slice
// テスト: cargo test --bin ex02_array_slice

fn average(nums: &[i32]) -> f64 {
    // TODO: 平均値を計算して返そう
    todo!()
}

fn main() {
    let scores = [80, 90, 70, 100, 60];
    println!("平均: {:.1}", average(&scores));
    println!("一部分の平均: {:.1}", average(&scores[0..3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(average(&[2, 4, 6]), 4.0);
        assert_eq!(average(&[]), 0.0);
    }
}
