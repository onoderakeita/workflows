//! `clippy_args` 入力の検証用クレート。
//!
//! `-D warnings` のみでは `clippy::needless_return` により失敗し、
//! `clippy_args: -A clippy::needless_return` を渡すと通ることを
//! self-test で確かめる。

/// 常に 3 を返す（あえて不要な `return` を残す）。
pub fn three() -> u32 {
    return 3;
}
