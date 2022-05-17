use std::io;
use std::io::Write;
mod command;
use command::Command;

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
    let cmd_type = cmd[0];

    if cmd_type == "echo" {
        let command = command::EchoCommand {
            command: cmd.iter().map(|&s| s.into()).collect(),
        };
        command.execute();
    }
}
