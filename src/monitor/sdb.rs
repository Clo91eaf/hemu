use crate::cpu::{Cpu, CpuState};
use crate::cpu::memory::read_data;
use crate::monitor::expr;
use atoi::atoi;
use rustyline::Editor;

struct CommandTable {
  commands: [Command; 6],
}

impl CommandTable {
  pub fn new() -> CommandTable {
    CommandTable {
      commands: [
        Command::new("c", "Continue the execution", Command::r#continue),
        Command::new("q", "Exit hemu", Command::quit),
        Command::new("s", "Single step execution", Command::step),
        Command::new("info", "Print register and watches info", Command::info),
        Command::new("p", "Calculate the expression", Command::expr),
        Command::new("x", "Scan memory", Command::scan),
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
    cpu.exec(usize::MAX)
  }

  fn step(args: &str, cpu: &mut Cpu) -> i32 {
    cpu.exec(atoi::<usize>(args.as_bytes()).unwrap_or(1))
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

  fn expr(args: &str, cpu: &mut Cpu) -> i32 {
    println!("0x{:08x}", expr::expr(args.to_string(), cpu));
    0
  }

  #[allow(unused_variables)]
  fn scan(args: &str, cpu: &mut Cpu) -> i32 {
    let mut parts = args.splitn(2, ' ');
    let input_size = parts.next().unwrap_or("");
    let input_addr = expr::expr(parts.next().unwrap_or("").to_string(), cpu);
    (0..atoi::<usize>(input_size.as_bytes()).unwrap_or(1)).for_each(|i| {
      if i % 4 == 0 {
        print!("0x{:08x}: ", input_addr + i as u64 * 4);
      }
      print!("0x{:08x} ", read_data(input_addr + i as u64 * 4, 4));
      if i % 4 == 3 {
        println!();
      }
    });
    println!(); // buffer flush
    0
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
              log::info!("Quit hemu");
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

pub fn init_sdb() {}
