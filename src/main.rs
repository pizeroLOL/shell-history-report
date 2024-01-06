use shell_history_report::{collect_hitorys, merge_maps, Outputs};

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
