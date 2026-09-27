// 練習3：match式
//
// 【課題】
// 1. day_type(day) : 曜日番号(1=月〜7=日)を受け取り、
//                    1〜5なら"平日"、6・7なら"休日"、それ以外は"無効"を返す
// 2. sign(n)        : 整数を受け取り、正なら1、負なら-1、0なら0を返す
//
// 実行  : cargo run --bin ex03_match
// テスト: cargo test --bin ex03_match

fn day_type(day: u32) -> &'static str {
    // TODO: matchを使って判定しよう
    todo!()
}

fn sign(n: i32) -> i32 {
    // TODO: matchを使って判定しよう
    todo!()
}

fn main() {
    println!("=== 練習3：match式 ===");

    for day in 1..=7 {
        println!("{}曜日番号: {}", day, day_type(day));
    }

    println!("sign(5) = {}", sign(5));
    println!("sign(-3) = {}", sign(-3));
    println!("sign(0) = {}", sign(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_type() {
        assert_eq!(day_type(1), "平日");
        assert_eq!(day_type(5), "平日");
        assert_eq!(day_type(6), "休日");
        assert_eq!(day_type(7), "休日");
        assert_eq!(day_type(8), "無効");
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(5), 1);
        assert_eq!(sign(-3), -1);
        assert_eq!(sign(0), 0);
    }
}
