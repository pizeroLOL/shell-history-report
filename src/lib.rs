use std::{
    collections::HashMap,
    io::{self, ErrorKind},
    path::PathBuf,
};

use shells::{bash::Bash, zsh::ZSH};

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

pub struct Outputs {
    pub shells: Vec<(String, usize)>,
    pub cmds: Vec<(String, usize)>,
}

pub fn merge_maps(vec: Vec<ShellHistoryOutput>) -> Outputs {
    let mut cmds = HashMap::new();
    let mut shells = HashMap::new();
    vec.iter()
        .flat_map(|history| {
            history
                .cmds
                .iter()
                .map(|(cmd, count)| (history.shell.to_string(), cmd, count))
        })
        .for_each(|(shell, cmd, &count)| {
            cmds.entry(cmd.to_string())
                .and_modify(|now| *now += count)
                .or_insert(count);
            shells
                .entry(shell)
                .and_modify(|now| *now += count)
                .or_insert(count);
        });
    let mut cmds = cmds.into_iter().collect::<Vec<_>>();
    cmds.sort_by_cached_key(|(_name, count)| *count);
    cmds.reverse();
    let mut shells = shells.into_iter().collect::<Vec<_>>();
    shells.sort_by_cached_key(|(_cmd, count)| *count);
    shells.reverse();
    Outputs { cmds, shells }
}

pub fn collect_hitorys() -> Result<Vec<ShellHistoryOutput>, Vec<String>> {
    let history_maps = [ZSH::new().get_history(), Bash::new().get_history()];
    let errors = history_maps
        .iter()
        .filter_map(|x| x.as_ref().err())
        .filter(|x| x.kind() != ErrorKind::NotFound)
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        let collect = errors.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        return Err(collect);
    }
    let outputs = history_maps
        .into_iter()
        .filter_map(|map| map.ok())
        .collect::<Vec<_>>();
    Ok(outputs)
}

#[cfg(test)]
mod test {
    use crate::{collect_hitorys, merge_maps};

    #[test]
    fn test_main() {
        let i = collect_hitorys().unwrap();
        merge_maps(i);
    }
}
