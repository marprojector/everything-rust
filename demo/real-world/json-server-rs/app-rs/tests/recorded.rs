//! The cases recorded from the original (`cases.jsonl`), run on the port directly: what
//! `everything-rust diff` checked through the bindings before the lift. They ran in a folder whose
//! `public/test.html` the app serves, so the test makes one.

use json_server_js::json;
use serde_json::Value;

const TEST_HTML: &str =
    "<!-- Testing automatic serving of files from the 'public/' directory -->\n";

#[test]
fn every_recorded_case_matches() {
    let text =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/cases.jsonl")).unwrap();
    let dir = std::env::temp_dir().join(format!("app-rs-recorded-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("public")).unwrap();
    std::fs::write(dir.join("public/test.html"), TEST_HTML).unwrap();
    std::env::set_current_dir(&dir).unwrap();
    let mut failures = Vec::new();
    let mut total = 0;
    for line in text.lines() {
        let case = json::parse(line).unwrap();
        let args = case["args"].as_array().unwrap();
        let got = futures::executor::block_on(app_rs::port::app_requests(
            args[0].clone(),
            args.get(1).cloned().unwrap_or(Value::Null),
        ));
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
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        failures.is_empty(),
        "{} of {total} differ:\n{}",
        failures.len(),
        failures[..failures.len().min(3)].join("\n")
    );
}
