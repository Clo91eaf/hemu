use crate::cpu::{Cpu, CpuState};
use atoi::atoi;
use rustyline::Editor;
use crate::monitor::expr;

struct CommandTable {
  commands: [Command; 4],
}

impl CommandTable {
  pub fn new() -> CommandTable {
    CommandTable {
      commands: [
        Command::new("c", "Continue the execution", Command::r#continue),
        Command::new("q", "Exit hemu", Command::quit),
        Command::new("s", "Single step execution", Command::step),
        Command::new("info", "Print register and watches info", Command::info),
      ],
    }
  }
  #[allow(unused_variables)]
  fn help(&self, args: &str) -> i32 {
    if args == "" {
      for i in 0..self.commands.len() {
        println!(
          "{} - {}",
          self.commands[i].name, self.commands[i].description
        );
      }
      return 0;
    }

    for i in 0..self.commands.len() {
      if args == self.commands[i].name {
        println!(
          "{} - {}",
          self.commands[i].name, self.commands[i].description
        );
        return 0;
      }
    }

    println!("Unknown command '{}'", args);

    0
  }
}

struct Command {
  name: &'static str,
  description: &'static str,
  handler: fn(&str, &mut Cpu) -> i32,
}

impl Command {
  fn new(
    name: &'static str,
    description: &'static str,
    handler: fn(&str, &mut Cpu) -> i32,
  ) -> Command {
    Command {
      name,
      description,
      handler,
    }
  }

  fn handle(&self, args: &str, cpu: &mut Cpu) -> i32 {
    (self.handler)(args, cpu)
  }

  // use r# to tell the Rust compiler that this identifier should not be considered a keyword identifier.
  #[allow(unused_variables)]
  fn r#continue(args: &str, cpu: &mut Cpu) -> i32 {
    cpu.exec(usize::MAX);
    0
  }

  fn step(args: &str, cpu: &mut Cpu) -> i32 {
    cpu.exec(atoi::<usize>(args.as_bytes()).unwrap_or(1));
    0
  }

  fn info(args: &str, cpu: &mut Cpu) -> i32 {
    if args == "r" {
      cpu.dump_registers();
    } else if args == "w" {
      cpu.dump_watches();
    } else {
      println!("Unknown info '{}'", args);
    }

    0
  }

  #[allow(unused_variables)]
  fn quit(args: &str, cpu: &mut Cpu) -> i32 {
    cpu.state = CpuState::Quit;
    -1
  }
}

pub fn sdb_mainloop(cpu: &mut Cpu) {
  let cmd_table = CommandTable::new();
  let mut rl = Editor::<()>::new();
  'out: loop {
    let readline = rl.readline("(hemu) ");
    match readline {
      Ok(line) => {
        rl.add_history_entry(line.trim());

        let mut parts = line.trim().splitn(2, ' ');
        let input_cmd = parts.next().unwrap_or("");
        let input_args = parts.next().unwrap_or("");

        if input_cmd == "" {
          continue;
        }

        for cmd in cmd_table.commands.iter() {
          if input_cmd == cmd.name {
            if cmd.handle(input_args, cpu) < 0 {
              break 'out;
            }
            continue 'out;
          }
        }

        cmd_table.help("");
      }
      Err(_) => {
        println!("Error!");
        break;
      }
    }
  }
}

pub fn init_sdb() {
  expr::init_regex();
}