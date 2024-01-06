use shell_history_report::{collect_hitorys, merge_maps, Outputs};

static ROW_SPLIT: &str = "============================================";

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

fn start() {
    let index = "Index";
    let cmd = "Command";
    let count = "Count";
    println!("{ROW_SPLIT}");
    println!("  {cmd:20}  {index:>8}  {count:>8}");
    println!("{ROW_SPLIT}");
}

fn show_cmds(cmds: Vec<(String, usize)>) {
    cmds.iter()
        .take(20)
        .enumerate()
        .map(|(index, (cmd, count))| {
            let tmp = match cmd.len() > 18 {
                true => cmd.chars().take(18).collect::<String>() + "..",
                false => cmd.to_string(),
            };
            (index, tmp, count)
        })
        .for_each(|(index, cmd, count)| println!("  {cmd:20}  {:>8}  {count:>8}", index + 1));
    let others = cmds
        .iter()
        .skip(20)
        .map(|(_cmd, count)| count)
        .sum::<usize>();
    let dots = "..";
    let txt = "..others";
    println!("  {txt:20}  {dots:>8}  {others:>8}");
    println!("{ROW_SPLIT}");
}

fn show_shells(shells: &Vec<(String, usize)>) {
    shells
        .iter()
        .enumerate()
        .map(|(index, (shell, count))| {
            let tmp = match shell.len() > 18 {
                true => shell.chars().take(19).collect::<String>() + "..",
                false => shell.to_string(),
            };
            (index, tmp, count)
        })
        .for_each(|(index, shell, count)| println!("  {shell:20}  {:>8}  {count:>8}", index + 1));
    println!("{ROW_SPLIT}");
}

fn show_totals(shells: Vec<(String, usize)>) {
    let total = shells.iter().map(|(_, count)| count).sum::<usize>();
    let space = "";
    let text = "Total";
    println!("  {text:20}  {space:>8}  {total:>8}");
    println!("{ROW_SPLIT}");
}
