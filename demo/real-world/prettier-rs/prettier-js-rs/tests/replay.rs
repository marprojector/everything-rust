//! Replays recorded Prettier CLI cases (`everything-rust app record` output) through the port.
//! Used as the test suite for mutation testing (`equiv/mutants/`). Skipped unless
//! `REPLAY_CASES` names the `app-cases` folder.
//!
//! A case fails when the port returns output that differs from Prettier's, or panics.
//! With `REPLAY_CLAIMED=<file>` (the names of the cases the unmutated port formats, one per line),
//! a case on that list that the port now refuses also fails: the front door would still give the
//! right bytes through Prettier, so this is reported separately as a fallback shift.

use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Caps the live heap so a mutant that loops while allocating aborts within seconds instead of
/// growing to tens of GB before `--timeout` fires (four such jobs at once froze a 96 GB machine).
/// The abort is a crash, which cargo-mutants counts as the mutant caught.
/// `REPLAY_HEAP_CAP_GB` overrides the default 8 GB.
struct CappedAlloc;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static CAP: AtomicUsize = AtomicUsize::new(8 << 30);

impl CappedAlloc {
    fn reserve(size: usize) {
        if LIVE.fetch_add(size, Ordering::Relaxed) + size > CAP.load(Ordering::Relaxed) {
            // Stderr is unbuffered, so this write does not allocate.
            let _ = std::io::Write::write_all(
                &mut std::io::stderr(),
                b"replay: heap cap exceeded, aborting\n",
            );
            std::process::abort();
        }
    }
}

unsafe impl GlobalAlloc for CappedAlloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        Self::reserve(l.size());
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        Self::reserve(l.size());
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Ordering::Relaxed);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new_size: usize) -> *mut u8 {
        if new_size > l.size() {
            Self::reserve(new_size - l.size());
        } else {
            LIVE.fetch_sub(l.size() - new_size, Ordering::Relaxed);
        }
        unsafe { System.realloc(p, l, new_size) }
    }
}

#[global_allocator]
static GLOBAL: CappedAlloc = CappedAlloc;

fn set_heap_cap() {
    if let Some(gb) = std::env::var("REPLAY_HEAP_CAP_GB")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
    {
        CAP.store(gb << 30, Ordering::Relaxed);
    }
}

/// Proves the cap trips: run with `--ignored heap_cap_aborts`; it should abort, not pass.
#[test]
#[ignore]
fn heap_cap_aborts() {
    set_heap_cap();
    let mut hog: Vec<Vec<u8>> = Vec::new();
    loop {
        hog.push(vec![1u8; 64 << 20]);
    }
}

#[test]
fn replay_recorded_cases() {
    set_heap_cap();
    let Ok(dir) = std::env::var("REPLAY_CASES") else {
        eprintln!("REPLAY_CASES not set; skipping");
        return;
    };
    let claimed: Option<HashSet<String>> = std::env::var("REPLAY_CLAIMED").ok().map(|f| {
        std::fs::read_to_string(f)
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    });
    // With REPLAY_ONLY_CLAIMED set, only the claimed cases run (coverage of the Rust-only runs).
    let only_claimed = std::env::var("REPLAY_ONLY_CLAIMED").is_ok();
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    entries.sort();
    let (mut mismatch, mut shifted, mut ok) = (Vec::new(), Vec::new(), 0usize);
    for path in entries {
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let toml = std::fs::read_to_string(&path).unwrap();
        if toml.contains("fs.base") || name.starts_with("config-") {
            continue;
        }
        let Some(args_line) = toml.lines().find(|l| l.starts_with("args")) else {
            continue;
        };
        let Some(start) = args_line.find('[') else {
            continue;
        };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&args_line[start..]) else {
            continue;
        };
        let argv: Vec<String> = v
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| x.as_str().map(String::from))
            .collect();
        let stem = path.with_extension("");
        let stdin = std::fs::read_to_string(stem.with_extension("stdin")).unwrap_or_default();
        let expected = std::fs::read_to_string(stem.with_extension("stdout")).unwrap_or_default();
        let status_ok = !toml.contains("status") || toml.contains("status.code = 0");
        let in_claimed = claimed.as_ref().is_some_and(|c| c.contains(&name));
        if only_claimed && !in_claimed {
            continue;
        }
        let Some(opts) = prettier_js_rs::Options::from_cli(&argv) else {
            if in_claimed {
                shifted.push(name);
            }
            continue;
        };
        match std::panic::catch_unwind(|| prettier_js_rs::format(&stdin, &opts)) {
            Ok(Ok(out)) if status_ok && out == expected => ok += 1,
            Ok(Ok(_)) | Err(_) => mismatch.push(name),
            Ok(Err(_)) => {
                if in_claimed {
                    shifted.push(name);
                }
            }
        }
    }
    eprintln!(
        "ok {ok} mismatch {} fallback-shift {}",
        mismatch.len(),
        shifted.len()
    );
    assert!(
        mismatch.is_empty(),
        "MISMATCH {:?}",
        &mismatch[..mismatch.len().min(10)]
    );
    assert!(
        shifted.is_empty(),
        "FALLBACK-SHIFT {:?}",
        &shifted[..shifted.len().min(10)]
    );
}
