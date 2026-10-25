//! Reference default (D17): the page keeps nothing between visits. This reads
//! every file under web/ (vendor/ aside) and fails if one names a browser
//! storage API. From Feedback Drafts and Status Trends.
//!
//! When the brief allows storage, delete this file and record the change in
//! BUILDER-SPEC.md. Then let the core write and read what is stored, and test
//! that an unreadable save is treated as a new one.

use std::fs;
use std::path::{Path, PathBuf};

const STORAGE_NAMES: &[&str] = &[
    "localStorage",
    "sessionStorage",
    "indexedDB",
    "IDBFactory",
    "document.cookie",
    "caches",
    "CacheStorage",
    "serviceWorker",
    "navigator.storage",
    "openDatabase",
];

fn storage_names_in(text: &str) -> Vec<&'static str> {
    STORAGE_NAMES
        .iter()
        .copied()
        .filter(|name| text.contains(name))
        .collect()
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(read) = fs::read_dir(&d) else {
            continue;
        };
        for entry in read {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                if path.file_name().map(|n| n != "vendor").unwrap_or(true) {
                    stack.push(path);
                }
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// Project change to D17 (T3, from SP-Q17; BUILDER-SPEC.md): one file may
/// keep the core's save text in localStorage, and nothing else may.
const ALLOWED_STORAGE: &[(&str, &str)] = &[("web/shared/save.js", "localStorage")];

fn allowed(path: &Path, name: &str) -> bool {
    let p = path.to_string_lossy().replace('\\', "/");
    ALLOWED_STORAGE
        .iter()
        .any(|(file, n)| p.ends_with(file) && *n == name)
}

#[test]
fn only_the_save_file_may_name_storage_and_only_local_storage() {
    assert!(allowed(Path::new("/x/web/shared/save.js"), "localStorage"));
    assert!(!allowed(Path::new("/x/web/shared/save.js"), "indexedDB"));
    assert!(!allowed(Path::new("/x/web/shared/board.js"), "localStorage"));
}

#[test]
fn the_page_names_no_browser_storage() {
    let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web");
    let mut found = Vec::new();
    for path in files_under(&web) {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for name in storage_names_in(&text) {
            if !allowed(&path, name) {
                found.push(format!("{}: {name}", path.display()));
            }
        }
    }
    assert!(
        found.is_empty(),
        "browser storage in the page (D17; the brief must allow it):\n{}",
        found.join("\n")
    );
}

#[test]
fn the_scanner_sees_storage() {
    assert_eq!(
        storage_names_in("window.localStorage.setItem('n', name)"),
        vec!["localStorage"]
    );
    assert_eq!(storage_names_in("indexedDB.open('x')"), vec!["indexedDB"]);
    assert!(storage_names_in("el(\"note\").value").is_empty());
}
