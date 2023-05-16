use rustyline::Editor;

struct Command {
  name: &'static str,
  description: &'static str,
  handler: fn(&str) -> i32,
}

impl Command {
  const fn new(
    name: &'static str,
    description: &'static str,
    handler: fn(&str) -> i32,
  ) -> Command {
    Command {
      name,
      description,
      handler,
    }
  }

  fn handle(&self, args: &str) -> i32 {
    (self.handler)(args)
  }

  fn help(args: &str) -> i32 {
    if args == "" {
      for i in 0..CMD_TABLE.len() {
        println!("{} - {}", CMD_TABLE[i].name, CMD_TABLE[i].description);
      }
      return 0;
    }

    for i in 0..CMD_TABLE.len() {
      if args == CMD_TABLE[i].name {
        println!("{} - {}", CMD_TABLE[i].name, CMD_TABLE[i].description);
        return 0;
      }
    }

    println!("Unknown command '{}'", args);

    0
  }

  // use r# to tell the Rust compiler that this identifier should not be considered a keyword identifier.
  #[allow(unused_variables)]
  fn r#continue(args: &str) -> i32 {
    crate::cpu::exec();
    0
  }

  #[allow(unused_variables)]
  fn quit(args: &str) -> i32 {
    -1
  }
}

const CMD_TABLE: [Command; 3] = [
  Command::new("help", "Print this help message", Command::help),
  Command::new("c", "Continue the execution", Command::r#continue),
  Command::new("q", "Exit hemu", Command::quit),
];

pub fn sdb_mainloop() {
  let mut rl = Editor::<()>::new();
  if rl.load_history("history").is_err() {
    println!("No previous history.");
  }
  'out: loop {
    let readline = rl.readline("sdb> ");
    match readline {
      Ok(line) => {
        rl.add_history_entry(line.trim());

        let mut parts = line.trim().splitn(2, ' ');
        let input_cmd = parts.next().unwrap_or("");
        let input_args = parts.next().unwrap_or("");

        if input_cmd == "" {
          continue;
        }

        for cmd in CMD_TABLE.iter() {
          if input_cmd == cmd.name {
            if cmd.handle(input_args) < 0 {
              break 'out;
            }
            continue 'out;
          }
        }

        Command::help("");
      }
      Err(_) => {
        println!("Error!");
        break;
      }
    }
  }
}
