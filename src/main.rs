use std::io;
use std::io::Write;

fn main() {
    print_prompt();

    let ans = get_input();

    println!("{}", ans);
}

fn get_input() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).ok();
    return command.trim().to_string();
}

fn print_prompt() {
    print!("jit$ ");
    std::io::stdout().flush().unwrap();
}
