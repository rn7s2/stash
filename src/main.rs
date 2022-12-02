use std::str::FromStr;

enum BuiltInCommand {
    Cp(String, String),
    Ls,
    Mv(String, String),
    Rm(String),
}

struct ExternalCommand {
    exectuable: String,
    args: Vec<String>,
}

enum Command {
    BuiltIn(BuiltInCommand),
    External(ExternalCommand),
}

impl BuiltInCommand {
    fn exec(&self) {
        match self {
            BuiltInCommand::Cp(src, dest) => println!("cp src: {}, dest: {}", src, dest),
            BuiltInCommand::Ls => println!("ls"),
            BuiltInCommand::Mv(src, dest) => println!("mv src: {}, dest: {}", src, dest),
            BuiltInCommand::Rm(f) => println!("rm: {}", f),
        }
    }
}

impl ExternalCommand {
    fn exec(&self) {
        println!("exec: {}, args: {:?}", self.exectuable, self.args)
    }
}

impl Command {
    fn exec(&self) {
        match self {
            Command::BuiltIn(v) => v.exec(),
            Command::External(v) => v.exec(),
        }
    }
}

impl FromStr for BuiltInCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(" ").collect::<Vec<&str>>();
        if words.len() == 0 {
            Err("parsing build-in command error.".to_owned())
        } else {
            match words[0] {
                "cp" => Ok(BuiltInCommand::Cp(words[1].to_owned(), words[2].to_owned())),
                "ls" => Ok(BuiltInCommand::Ls),
                "mv" => Ok(BuiltInCommand::Mv(words[1].to_owned(), words[2].to_owned())),
                "rm" => Ok(BuiltInCommand::Rm(words[1].to_owned())),
                _ => Err("parsing build-in command error.".to_owned()),
            }
        }
    }
}

impl FromStr for ExternalCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(" ").collect::<Vec<&str>>();
        if words.len() == 0 {
            Err("parsing external command error.".to_owned())
        } else {
            Ok(ExternalCommand {
                exectuable: words[0].to_owned(),
                args: words[1..].iter().map(|word| (*word).to_owned()).collect(),
            })
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BuiltInCommand::from_str(s) {
            Ok(v) => Ok(Command::BuiltIn(v)),
            Err(_) => match ExternalCommand::from_str(s) {
                Ok(v) => Ok(Command::External(v)),
                Err(_) => Err("parsing command error.".to_owned()),
            },
        }
    }
}

fn main() {
    let mut line = String::new();
    loop {
        match std::io::stdin().read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let command = match Command::from_str(line.as_str().trim()) {
                    Ok(v) => v,
                    Err(_) => {
                        println!("invalid command.");
                        continue;
                    }
                };
                command.exec();
            }
            Err(_) => println!("read bytes are not valid UTF-8."),
        }
        line.clear();
    }
}
