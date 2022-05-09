use std::io;

fn main() {
    println!("Enter the command.");

    let ans = get_input();

    println!("{}", ans);
}

fn get_input() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).ok();
    return command.trim().to_string();
}
