use rustyline::Editor;

struct Command {
  name: &'static str,
  description: &'static str,
  handler: fn(&str) -> i32,
}

impl Command {
  const fn new(name: &'static str, description: &'static str, handler: fn(&str) -> i32) -> Command {
    Command {
      name,
      description,
      handler,
    }
  }

  fn handle(&self, args: &str) -> i32 {
    print!("{}", self.description);
    (self.handler)(args)
  }
}

fn cmd_help(args: &str) -> i32 {
  0
}

fn cmd_c(args: &str) -> i32 {
  0
}

fn cmd_q(args: &str) -> i32 {
  0
}

const CMD_TABLE: [Command; 3] = [
  Command::new("help", "Print this help message", cmd_help),
  Command::new("c", "Continue the execution of the program", cmd_c),
  Command::new("q", "Exit hemu", cmd_q),
];

pub fn sdb_mainloop() {
  let mut rl = Editor::<()>::new();
  if rl.load_history("history").is_err() {
    println!("No previous history.");
  }
  loop {
    let readline = rl.readline("sdb> ");
    match readline {
      Ok(line) => {
        println!("Line: {}", line);
        rl.add_history_entry(line.trim());
        for cmd in CMD_TABLE.iter() {
          if line.trim() == cmd.name {
            cmd.handle("");
          }
        }
      },
      Err(_) => {
        println!("Error!");
        break;
      },
    }
  }
}