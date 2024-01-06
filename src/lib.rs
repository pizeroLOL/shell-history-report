use std::{collections::HashMap, io, path::PathBuf};

pub mod shells;

pub fn find_cmd<'a>(now: &'a str, next: Option<&'a str>) -> Option<&'a str> {
    match (now, next) {
        ("sudo", Some(next)) if !next.starts_with('-') => Some(next),
        (x, _) if x.starts_with(['\"', '\'']) => None,
        (x, _) => Some(x),
    }
}

#[derive(Debug, Clone)]
pub struct ShellHistoryOutput {
    pub cmds: HashMap<String, usize>,
    pub shell: String,
}

impl ShellHistoryOutput {
    pub fn new(shell: &str, cmds: HashMap<String, usize>) -> Self {
        Self {
            shell: shell.to_string(),
            cmds,
        }
    }
}

pub trait ShellHistory {
    fn get_history(&self) -> Result<ShellHistoryOutput, io::Error>;
    fn get_history_file_path(&self) -> PathBuf;
}
