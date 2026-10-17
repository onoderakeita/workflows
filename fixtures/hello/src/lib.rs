//! 再利用ワークフローのセルフテスト用クレート。

/// 常に 2 を返す。
pub fn two() -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use super::two;

    #[test]
    fn two_is_two() {
        assert_eq!(two(), 2);
    }
}
