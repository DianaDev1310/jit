use std::io;
use std::io::Write;

fn main() {
    start_jit_prompt();
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

fn start_jit_prompt() {
    loop {
        print_prompt();
        let cmd = get_input();
        println!("{}", cmd);
    }
}
