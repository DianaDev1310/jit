fn main() {
    println!("Enter the command.");
    let mut command = String::new();
    std::io::stdin().read_line(&mut command).ok();
    let ans = command.trim().to_string();

    println!("{}", ans);
}
