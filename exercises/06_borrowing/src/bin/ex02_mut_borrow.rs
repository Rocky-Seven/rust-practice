// 練習2：可変の借用
//
// 【課題】
// double_it(n) : &mut i32 を受け取り、その値を2倍に書き換える
//                （戻り値はなし。参照先を直接変更する）
//
// 実行  : cargo run --bin ex02_mut_borrow
// テスト: cargo test --bin ex02_mut_borrow

fn double_it(n: &mut i32) {
    // TODO: *n を2倍にしよう（参照先の値を書き換えるには * を付ける）
    todo!()
}

fn main() {
    let mut value = 21;
    double_it(&mut value);
    println!("2倍後: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_it() {
        let mut n = 10;
        double_it(&mut n);
        assert_eq!(n, 20);

        let mut m = 0;
        double_it(&mut m);
        assert_eq!(m, 0);
    }
}
