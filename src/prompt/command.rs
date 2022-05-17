use std::io::{self, Write};

pub trait Command {
    fn execute(&self) -> String;
}

pub struct EchoCommand {
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

fn output_string<W: Write>(w: &mut W, string: &str) -> io::Result<()> {
    writeln!(w, "{}", string)
}

#[cfg(test)]
mod tests {
    use crate::prompt::command::Command;
    use crate::prompt::command::EchoCommand;
    #[test]
    fn echo_works() {
        let cmd = EchoCommand {
            command: ["echo", "hoge"].iter().map(|&s| s.into()).collect(),
        };
        cmd.execute();
    }
}
