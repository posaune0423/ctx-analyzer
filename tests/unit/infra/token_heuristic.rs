use ctx_analyzer::domain::Confidence;
use ctx_analyzer::infra::CharsPer4Estimator;
use ctx_analyzer::ports::TokenEstimator;

#[test]
fn empty_string_is_zero_estimated() {
    let est = CharsPer4Estimator.estimate("");
    assert_eq!(est.tokens, 0);
    assert_eq!(est.confidence, Confidence::Estimated);
}

#[test]
fn ten_ascii_chars_round_up_to_three() {
    let est = CharsPer4Estimator.estimate(&"a".repeat(10));
    assert_eq!(est.tokens, 3);
}

#[test]
fn multibyte_counted_by_chars_not_bytes() {
    // 10 hiragana = 30 UTF-8 bytes but 10 chars => 3 tokens.
    let s = "あ".repeat(10);
    let est = CharsPer4Estimator.estimate(&s);
    assert_eq!(est.tokens, 3);
}

#[test]
fn exact_multiple_of_four_does_not_round_up() {
    let est = CharsPer4Estimator.estimate(&"x".repeat(8));
    assert_eq!(est.tokens, 2);
}
