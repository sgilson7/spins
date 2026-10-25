//! What Spins keeps between visits (T3, from SP-Q17): the wins and losses
//! against each opponent at the tavern, and whether oak wheels have been
//! won. Nothing about the person. The core writes and reads the text; the
//! page only stores it (`web/shared/save.js`, the one file allowed to name
//! browser storage).

use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Record {
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    pub version: u32,
    /// one record per opponent, in the order of `api::OPPONENTS`
    pub records: Vec<Record>,
    pub oak: bool,
}

impl Progress {
    pub fn fresh(opponents: usize) -> Progress {
        Progress {
            version: VERSION,
            records: vec![Record::default(); opponents],
            oak: false,
        }
    }

    /// A save from an earlier visit, or a fresh start when there is none or
    /// it cannot be read. The flag says whether a save was refused.
    pub fn load(text: &str, opponents: usize) -> (Progress, bool) {
        if text.trim().is_empty() {
            return (Progress::fresh(opponents), false);
        }
        match serde_json::from_str::<Progress>(text) {
            Ok(mut p) if p.version == VERSION => {
                p.records.resize(opponents, Record::default());
                (p, false)
            }
            _ => (Progress::fresh(opponents), true),
        }
    }

    pub fn text(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_save_comes_back_as_it_was_written() {
        let mut p = Progress::fresh(3);
        p.records[2].wins = 4;
        p.oak = true;
        assert_eq!(Progress::load(&p.text(), 3), (p, false));
    }

    #[test]
    fn no_save_is_a_fresh_start_and_a_broken_one_is_refused() {
        assert_eq!(Progress::load("", 3), (Progress::fresh(3), false));
        assert_eq!(Progress::load("{not json", 3), (Progress::fresh(3), true));
        assert_eq!(
            Progress::load(r#"{"version":99,"records":[],"oak":true}"#, 3),
            (Progress::fresh(3), true)
        );
    }
}
