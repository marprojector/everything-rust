//! The cases recorded from the original (`cases.jsonl`), run on the port directly: what
//! `everything-rust diff` checked through the bindings before the lift.

use json_server_js::json;
use serde_json::Value;

#[test]
fn every_recorded_case_matches() {
    let text =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/cases.jsonl")).unwrap();
    let mut failures = Vec::new();
    let mut total = 0;
    for line in text.lines() {
        let case = json::parse(line).unwrap();
        let args = case["args"].as_array().unwrap();
        let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Null);
        let got: Result<Value, String> = where_operators_rs::port::is_where_operator(
            arg(0).as_str().unwrap_or_default().to_string(),
        )
        .map(Value::Bool);
        total += 1;
        let same = match (&got, case.get("ok"), case.get("error")) {
            (Ok(v), Some(ok), _) => json::stringify(v, 0) == json::stringify(ok, 0),
            (Err(_), None, Some(_)) => true,
            _ => false,
        };
        if !same {
            failures.push(format!("{}\n  got {:?}", &line[..line.len().min(300)], got));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {total} differ:\n{}",
        failures.len(),
        failures[..failures.len().min(3)].join("\n")
    );
}
