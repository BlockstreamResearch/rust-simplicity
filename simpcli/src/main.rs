// Simplicity "Human-Readable" Language
// Written in 2023 by
//   Andrew Poelstra <simplicity@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

use simplicity::human_encoding::Forest;
use simplicity::node::CommitNode;
use simplicity::{self, BitIter};

use base64::engine::general_purpose::STANDARD;
use std::env;
use std::str::FromStr;

/// What set of jets to use in the program.
// FIXME this should probably be configurable.
type DefaultJet = simplicity::jet::Elements;

fn usage(process_name: &str) -> Result<(), String> {
    eprintln!("Usage:");
    eprintln!("  {} disassemble <base64>", process_name);
    eprintln!();
    eprintln!("For commands which take an optional expression, the default value is \"main\".");
    eprintln!();
    eprintln!("Run `{} help` to display this message.", process_name);
    Err("invalid usage".into())
}

enum Command {
    Disassemble,
    Help,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "disassemble" => Ok(Command::Disassemble),
            "help" => Ok(Command::Help),
            x => Err(format!("unknown command {}", x)),
        }
    }
}

impl Command {
    fn takes_optional_exprname(&self) -> bool {
        match *self {
            Command::Disassemble => false,
            Command::Help => false,
        }
    }
}

fn main() -> Result<(), String> {
    let mut args = env::args();
    let process_name = args.next().unwrap();
    let process_name = match process_name.rfind('/') {
        Some(idx) => &process_name[idx + 1..],
        None => &process_name[..],
    };

    // Parse command-line args into (command, first_arg, expression)
    let command = match args.next() {
        Some(cmd) => match Command::from_str(&cmd) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("Error: {}.", e);
                eprintln!();
                return usage(&process_name);
            }
        },
        None => return usage(&process_name),
    };
    let first_arg = match args.next() {
        Some(s) => s,
        None => return usage(&process_name),
    };
    let _expression = if command.takes_optional_exprname() {
        args.next().unwrap_or("main".to_owned())
    } else {
        String::new()
    };
    if args.next().is_some() {
        usage(&process_name)?;
    }

    // Execute command
    match command {
        Command::Disassemble => {
            let v = base64::Engine::decode(&STANDARD, first_arg.as_bytes())
                .map_err(|e| format!("failed to parse base64: {}", e))?;
            let mut iter = BitIter::from(v.into_iter());
            let commit = CommitNode::decode(&mut iter)
                .map_err(|e| format!("failed to decode program: {}", e))?;
            let prog = Forest::<DefaultJet>::from_program(commit);
            println!("{}", prog.string_serialize());
        }
        Command::Help => {
            let _ = usage(&process_name);
        }
    }

    Ok(())
}
