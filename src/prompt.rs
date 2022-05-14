use std::io;
use std::io::Write;

pub fn start_jit_prompt() {
    loop {
        print_prompt();

        let cmd = get_input();
        println!("{}", cmd);

        let vec: Vec<&str> = cmd.split_whitespace().collect();
        println!("{:?}", vec);

        exec_command(vec);
    }
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

fn exec_command(cmd: Vec<&str>) {
    let cmd_type = parse_command();

    if cmd_type == "echo" {
        let command = EchoCommand {
            command: cmd.iter().map(|&s| s.into()).collect(),
        };
        command.execute();
    }
}

fn parse_command() -> String {
    let mut cmd_type = String::new();
    cmd_type = String::from("echo");
    return cmd_type;
}

// ----- Command Interface ----- //
pub trait Command {
    fn execute(&self) -> String;
}

struct EchoCommand {
    pub command: Vec<String>,
}

impl EchoCommand {
    pub fn new(cmd: Vec<String>) -> Self {
        EchoCommand { command: cmd }
    }
}

impl Command for EchoCommand {
    fn execute(&self) -> String {
        for i in 1..=self.command.len() - 1 {
            print!("{}", self.command[i]);
            if i < self.command.len() - 1 {
                print!(" ");
                std::io::stdout().flush().unwrap();
            }
        }
        println!();

        return String::from("echo");
    }
}
