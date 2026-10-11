// 練習1：不変の借用
//
// 【課題】
// word_count(text) : &strを借用で受け取り、スペース区切りの単語数を返す
//
// ヒント: .split_whitespace().count() で単語数が数えられる
//
// 実行  : cargo run --bin ex01_borrow
// テスト: cargo test --bin ex01_borrow

fn word_count(text: &str) -> usize {
    // TODO: 単語数を返そう
    todo!()
}

fn main() {
    let sentence = String::from("Rust is a lot of fun");
    println!("単語数: {}", word_count(&sentence));
    println!("元の文章: {}", sentence); // 借用なのでまだ使える
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("Rust is fun"), 3);
        assert_eq!(word_count("hello"), 1);
    }
}
