use std::{collections::HashMap, io::ErrorKind};

use shell_history_report::{
    shells::{bash::Bash, zsh::ZSH},
    ShellHistory, ShellHistoryOutput,
};

fn main() {
    let outputs = match collect_hitorys() {
        Ok(o) => o,
        Err(e) => {
            println!("{e:#?}");
            return;
        }
    };
    let Outputs { shells, cmds } = merge_maps(outputs);
    start();
    show_cmds(cmds);
    show_shells(&shells);
    show_totals(shells);
}

fn collect_hitorys() -> Result<Vec<ShellHistoryOutput>, Vec<String>> {
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

struct Outputs {
    pub shells: Vec<(String, usize)>,
    pub cmds: Vec<(String, usize)>,
}

fn merge_maps(vec: Vec<ShellHistoryOutput>) -> Outputs {
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

fn start() {
    let index = "Index";
    let cmd = "Command";
    println!("=========================================");
    println!("{cmd:16}  {index:<8}  Count");
    println!("=========================================");
}

fn show_cmds(cmds: Vec<(String, usize)>) {
    cmds.iter()
        .take(20)
        .enumerate()
        .for_each(|(index, (cmd, count))| println!("{cmd:16}  {:<8}  {count}", index + 1));
    let others = cmds
        .iter()
        .skip(20)
        .map(|(_cmd, count)| count)
        .sum::<usize>();
    let dots = "..";
    let txt = "..others";
    println!("{txt:16}  {dots:<8}  {others}");
    println!("=========================================");
}

fn show_shells(shells: &Vec<(String, usize)>) {
    shells
        .iter()
        .enumerate()
        .for_each(|(index, (shell, count))| println!("{shell:16}  {:<8}  {count}", index + 1));
    println!("=========================================");
}

fn show_totals(shells: Vec<(String, usize)>) {
    let total = shells.iter().map(|(_, count)| count).sum::<usize>();
    let space = "";
    let text = "Total";
    println!("{text:16}  {space:8}  {total}");
    println!("=========================================");
}
