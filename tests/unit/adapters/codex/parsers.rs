use std::io::BufReader;

use ctx_analyzer::adapters::codex::parsers;

#[test]
fn parses_two_valid_lines_records_one_warning() {
    let input = concat!(
        r#"{"timestamp":"2026-04-09T15:54:54.177Z","type":"session_meta","payload":{"id":"abc"}}"#,
        "\n",
        r#"{"timestamp":"2026-04-09T15:54:54.177Z","type":"event_msg","payload":{"type":"task_started","turn_id":"t1"}}"#,
        "\n",
        "this line is not json",
        "\n",
    );
    let reader = BufReader::new(input.as_bytes());
    let (envelopes, warnings) = parsers::read_from_reader(reader).expect("parse should succeed");
    assert_eq!(envelopes.len(), 2, "valid lines parsed");
    assert_eq!(warnings.len(), 1, "one malformed line recorded");
    assert_eq!(warnings[0].line, 3);
}

#[test]
fn skips_blank_lines_silently() {
    let input = "\n\n";
    let reader = BufReader::new(input.as_bytes());
    let (envelopes, warnings) = parsers::read_from_reader(reader).unwrap();
    assert!(envelopes.is_empty());
    assert!(warnings.is_empty());
}
