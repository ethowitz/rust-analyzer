use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use anyhow::Result;
use glob::Pattern;
use tracing::warn;

use crate::AbsPathBuf;

#[derive(Debug)]
pub(crate) struct IgnoredCrateMatcher {
    patterns: Vec<Pattern>,
}

impl IgnoredCrateMatcher {
    pub(crate) fn new(path: AbsPathBuf) -> Result<Self> {
        let mut patterns = Vec::new();
        let raignore = path.join(".raignore");
        let file = match File::open(raignore) {
            Ok(file) => io::BufReader::new(file),
            Err(_) => return Ok(Self { patterns }),
        };

        for line in file.lines().map_while(Result::ok) {
            if line.trim().starts_with('#') {
                continue;
            }

            match Pattern::new(path.join(line).as_str()) {
                Ok(pat) => patterns.push(pat),
                Err(e) => {
                    warn!("failed to parse glob in .raignore: {e}");
                }
            }
        }

        Ok(Self { patterns })
    }

    pub(crate) fn is_match<T>(&self, manifest_path: T) -> bool
    where
        T: AsRef<Path>,
    {
        let Some(p) = manifest_path.as_ref().parent() else {
            return false;
        };

        self.patterns.iter().any(|pattern| pattern.matches_path(p))
    }
}
