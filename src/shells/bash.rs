use std::{
    collections::HashMap,
    env::var,
    fs::read_to_string,
    io,
    path::{Path, PathBuf},
};

use crate::{find_cmd, ShellHistory, ShellHistoryOutput};

#[derive(Debug, Clone, Default)]
pub struct Bash {
    home: PathBuf,
}

impl Bash {
    pub fn new() -> Self {
        let home = var("HOME").unwrap_or_else(|_| var("USERPROFILE").expect("no home dir"));
        let home = Path::new(&home).to_path_buf();
        Self { home }
    }
}

impl ShellHistory for Bash {
    fn get_history(&self) -> Result<ShellHistoryOutput, io::Error> {
        #[inline]
        fn get_now_next(line: &str) -> Option<(&str, Option<&str>)> {
            let mut tmp = line.split(' ');
            Some((tmp.next()?, tmp.next()))
        }
        let mut hash_map = HashMap::new();
        let history_file = read_to_string(self.get_history_file_path())?;
        history_file
            .split('\n')
            .filter_map(|line| get_now_next(line))
            .filter_map(|(now, next)| find_cmd(now, next))
            .for_each(|cmd| {
                hash_map
                    .entry(cmd.to_string())
                    .and_modify(|now| *now += 1)
                    .or_insert(1);
            });
        let output = ShellHistoryOutput::new("bash", hash_map);
        Ok(output)
    }

    fn get_history_file_path(&self) -> PathBuf {
        self.home.join(".bash_history")
    }
}

#[cfg(test)]
mod test {
    use crate::ShellHistory;

    use super::Bash;

    #[test]
    fn test_bash_history() {
        Bash::new().get_history().unwrap();
    }
}
