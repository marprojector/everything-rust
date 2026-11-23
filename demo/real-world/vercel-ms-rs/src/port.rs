//! The Rust port of `src/index.ts`.
//!
//! Replace each `unimplemented!` with the port of the source function. Return `Err` where
//! the source throws or raises: the caller gets an error there too. After changing a
//! signature, run `everything-rust refresh` to regenerate the bindings. `everything-rust diff` checks
//! behavior against what the source recorded.

#![forbid(unsafe_code)]

use crate::js::{math_round, number_to_string};
use serde_json::{Value, json};

const S: f64 = 1000.0;
const M: f64 = S * 60.0;
const H: f64 = M * 60.0;
const D: f64 = H * 24.0;
const W: f64 = D * 7.0;
const Y: f64 = D * 365.25;
const MO: f64 = Y / 12.0;

/// Port of `ms` (`src/index.ts:50`).
pub fn ms(value: Value, options: Option<Value>) -> Result<Value, String> {
    if let Value::String(text) = &value {
        return Ok(number_value(parse(text.clone())?));
    }
    if let Some(n) = number_of(&value) {
        return format(n, options).map(Value::String);
    }
    Err(format!(
        "Value provided to ms() must be a string or number. value={value}"
    ))
}

/// Port of `parse` (`src/index.ts:71`).
pub fn parse(str: String) -> Result<f64, String> {
    let units = str.encode_utf16().count();
    if units == 0 || units > 100 {
        return Err(format!(
            "Value provided to ms.parse() must be a string with length between 1 and 99. value={}",
            Value::String(str)
        ));
    }
    let Some((number, unit)) = split(&str) else {
        return Ok(f64::NAN);
    };
    let n: f64 = number
        .parse()
        .map_err(|_| "unparsable number".to_string())?;
    let factor = match unit.to_ascii_lowercase().as_str() {
        "years" | "year" | "yrs" | "yr" | "y" => Y,
        "months" | "month" | "mo" => MO,
        "weeks" | "week" | "w" => W,
        "days" | "day" | "d" => D,
        "hours" | "hour" | "hrs" | "hr" | "h" => H,
        "minutes" | "minute" | "mins" | "min" | "m" => M,
        "seconds" | "second" | "secs" | "sec" | "s" => S,
        "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" | "" => return Ok(n),
        _ => return Ok(f64::NAN),
    };
    Ok(n * factor)
}

/// Port of `parseStrict` (`src/index.ts:156`).
pub fn parse_strict(value: Value) -> Result<f64, String> {
    match value {
        Value::String(text) => parse(text),
        other => Err(format!(
            "Value provided to ms.parse() must be a string with length between 1 and 99. value={other}"
        )),
    }
}

/// Port of `format` (`src/index.ts:225`).
pub fn format(ms: f64, options: Option<Value>) -> Result<String, String> {
    if !ms.is_finite() {
        return Err("Value provided to ms.format() must be of type number.".into());
    }
    Ok(if is_long(options.as_ref()) {
        fmt_long(ms)
    } else {
        fmt_short(ms)
    })
}

/// `options?.long`, truthy as JS sees it.
fn is_long(options: Option<&Value>) -> bool {
    options.is_some_and(|o| match o {
        Value::Object(map) if !is_marker(o) => map.get("long").is_some_and(truthy),
        _ => false,
    })
}

/// `format` over many values for `FormatBatch`: reads `options` once instead of once per value.
pub struct FormatIndex {
    items: Vec<f64>,
}

impl FormatIndex {
    pub fn new(items: Vec<f64>) -> Self {
        Self { items }
    }

    pub fn run(&self, options: Option<Value>) -> Result<Vec<String>, String> {
        use rayon::prelude::*;
        let long = is_long(options.as_ref());
        self.items
            .par_iter()
            .map(|&ms| {
                if !ms.is_finite() {
                    return Err("Value provided to ms.format() must be of type number.".to_string());
                }
                Ok(if long { fmt_long(ms) } else { fmt_short(ms) })
            })
            .collect()
    }
}

/// `-?\d*\.?\d+ *(unit)?` matched against the whole string, as the original's regex does:
/// the number text and the unit text (empty when there is none).
fn split(text: &str) -> Option<(&str, &str)> {
    let bytes = text.as_bytes();
    let mut i = usize::from(bytes.first() == Some(&b'-'));
    let digits = |from: usize| {
        bytes[from..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count()
    };
    let whole = digits(i);
    i += whole;
    if bytes.get(i) == Some(&b'.') {
        let fraction = digits(i + 1);
        if fraction == 0 {
            return None;
        }
        i += 1 + fraction;
    } else if whole == 0 {
        return None;
    }
    let number = &text[..i];
    let rest = text[i..].trim_start_matches(' ');
    Some((number, rest))
}

fn fmt_short(ms: f64) -> String {
    let abs = ms.abs();
    for (unit, suffix) in [
        (Y, "y"),
        (MO, "mo"),
        (W, "w"),
        (D, "d"),
        (H, "h"),
        (M, "m"),
        (S, "s"),
    ] {
        if abs >= unit {
            return format!("{}{suffix}", number_to_string(math_round(ms / unit)));
        }
    }
    format!("{}ms", number_to_string(ms))
}

fn fmt_long(ms: f64) -> String {
    let abs = ms.abs();
    for (unit, name) in [
        (Y, "year"),
        (MO, "month"),
        (W, "week"),
        (D, "day"),
        (H, "hour"),
        (M, "minute"),
        (S, "second"),
    ] {
        if abs >= unit {
            let plural = if abs >= unit * 1.5 { "s" } else { "" };
            return format!("{} {name}{plural}", number_to_string(math_round(ms / unit)));
        }
    }
    format!("{} ms", number_to_string(ms))
}

/// A number from the bridge: plain, or a `{"$num": ...}` marker for NaN, ±Infinity and -0.
fn number_of(v: &Value) -> Option<f64> {
    if let Some(n) = v.as_f64() {
        return Some(n);
    }
    match v.get("$num")?.as_str()? {
        "NaN" => Some(f64::NAN),
        "Infinity" => Some(f64::INFINITY),
        "-Infinity" => Some(f64::NEG_INFINITY),
        "-0" => Some(-0.0),
        _ => None,
    }
}

/// A number as the bridge carries it: JSON has no NaN, ±Infinity or -0.
fn number_value(n: f64) -> Value {
    if n.is_nan() {
        json!({ "$num": "NaN" })
    } else if n.is_infinite() {
        json!({ "$num": if n > 0.0 { "Infinity" } else { "-Infinity" } })
    } else if n == 0.0 && n.is_sign_negative() {
        json!({ "$num": "-0" })
    } else {
        json!(n)
    }
}

fn is_marker(v: &Value) -> bool {
    v.as_object()
        .is_some_and(|m| m.len() == 1 && m.keys().next().is_some_and(|k| k.starts_with('$')))
}

/// JavaScript truthiness of a value from the bridge.
fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|x| x != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(_) => true,
        Value::Object(_) => {
            if v.get("$undefined").is_some() {
                return false;
            }
            if let Some(n) = number_of(v) {
                return !(n.is_nan() || n == 0.0);
            }
            if let Some(b) = v.get("$bigint").and_then(Value::as_str) {
                return b != "0";
            }
            true
        }
    }
}
