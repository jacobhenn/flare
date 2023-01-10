use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{bail, Context, Result};

/// The directory where sets will be looked for.
pub fn dir() -> Result<PathBuf> {
    let mut res = dirs::config_dir().context("couldn't find config dir on this OS")?;
    res.push("flare");
    res.push("sets");
    Ok(res)
}

#[derive(Debug)]
pub struct Set {
    pub terms: Vec<(String, String)>,
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            terms: s
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    let (key, val) = line.split_once("=>").with_context(|| {
                        format!("missing '=>' to separate key and val on line {i}")
                    })?;
                    let key = key.trim().to_owned();
                    let val = val.trim().to_owned();
                    if key == ":q" {
                        bail!("key is ':q' (reserved quitting command) on line {i}");
                    } else if val == ":q" {
                        bail!("value is ':q' (reserved quitting command) on line {i}");
                    }
                    Ok((key, val))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl Set {
    pub fn read(path: &Path) -> Result<Self> {
        // first, try to read the set path relative to the current directory.
        let path = if path.is_file() {
            Cow::Borrowed(path)
        } else {
            // if that doesn't exist, look under the set directory.
            let mut res = dir()?;
            res.push(path);
            if res.is_file() {
                Cow::Owned(res)
            } else {
                bail!("set file not found: neither {path:?} nor {res:?} exist")
            }
        };

        fs::read_to_string(&path)
            .with_context(|| format!("couldn't read file {path:?} to string"))?
            .parse()
            .with_context(|| format!("couldn't parse set at {path:?}"))
    }
}
