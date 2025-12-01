use std::{fs, io::{BufRead, BufReader}};
use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{PathBuf};

pub struct Ignore {
    pub ignore_patterns: GlobSet,
}

impl Ignore {
    pub fn from(path: &str) -> Result<Self> {
        let mut builder = GlobSetBuilder::new();
        
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let pat = line?.trim().to_string();
            if pat.is_empty() || pat.starts_with('#') {
                continue;
            }
            let glob = Glob::new(&pat)
                .map_err(|e| anyhow::anyhow!("Invalid glob pattern: {}: {}", pat, e))?;
            builder.add(glob);
        };
        Ok(Self { ignore_patterns: builder.build().unwrap() })
    }
    pub fn is_ignored(&self, path: &PathBuf) -> bool {
        self.ignore_patterns.is_match(path)
    }
}