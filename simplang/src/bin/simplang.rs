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

use simplang::simplicity::bitcoin_hashes::hex::FromHex;
use simplang::simplicity::{self, dag::{InternalSharing, NoSharing}};
use simplang::Program;

use std::{env, fs};
use std::str::FromStr;

/// What set of jets to use in the program.
// FIXME this should probably be configurable.
type DefaultJet = simplicity::jet::Core;

fn usage(process_name: &str) -> Result<(), String> {
    println!("Usage:");
    println!("  {} assemble <filename>", process_name);
    println!("  {} cmr <filename> [expression name]", process_name);
    println!("  {} disassemble <hex>", process_name);
    println!("  {} relabel <filename>", process_name);
    println!("  {} type <filename> [expression name]", process_name);
    println!("  {} unshare <filename>", process_name);
    println!("  {} witnesses <filename>", process_name);
    println!();
    println!("For commands which take an optional expression, the default value is \"main\".");
    println!();
    println!("Run `{} help` to display this message.", process_name);
    Err("invalid usage".into())
}

enum Command {
    Assemble,
    Cmr,
    Disassemble,
    Help,
    Relabel,
    Type,
    Unshare,
    Witnesses,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "assemble" => Ok(Command::Assemble),
            "cmr" => Ok(Command::Cmr),
            "disassemble" => Ok(Command::Disassemble),
            "help" => Ok(Command::Help),
            "relabel" => Ok(Command::Relabel),
            "type" => Ok(Command::Type),
            "unshare" => Ok(Command::Unshare),
            "witnesses" => Ok(Command::Witnesses),
            x => Err(format!("unknown command {}", x)),
        }
    }
}

impl Command {
    fn takes_optional_exprname(&self) -> bool {
        match *self {
            Command::Assemble => false,
            Command::Cmr => true,
            Command::Disassemble => false,
            Command::Help => false,
            Command::Relabel => false,
            Command::Type => true,
            Command::Unshare => false,
            Command::Witnesses => false,
        }
    }
}

fn parse_file(name: &str) -> Result<Program<DefaultJet>, String> {
    let s = fs::read_to_string(name)
        .map_err(|e| format!("failed to read file {}: {}", name, e))?;
    match Program::parse(&s) {
        Ok(prog) => Ok(prog),
        Err(errs) => {
            for e in errs {
                println!("{}", e);
            }
            Err(format!("failed to parse file {}", name))
        },
    }
}

fn main() -> Result<(), String> {
    let mut args = env::args();
    let process_name = args.next().unwrap();

    // Parse command-line args into (command, first_arg, expression)
    let command = match args.next() {
        Some(cmd) => Command::from_str(&cmd)?,
        None => return usage(&process_name),
    };
    let first_arg = match args.next() {
        Some(s) => s,
        None => return usage(&process_name),
    };
    let expression = if command.takes_optional_exprname() {
        args.next().unwrap_or("main".to_owned())
    } else {
        String::new()
    };
    if args.next().is_some() {
        usage(&process_name)?;
    }

    // Execute command
    match command {
        Command::Assemble => {
            let prog = parse_file(&first_arg)?;

            let mut sink = vec![];
            let mut w = simplicity::BitWriter::from(&mut sink);
            prog.root().encode(&mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");

            for byte in sink {
                print!("{:02x}", byte);
            }
            println!();
        }
        Command::Cmr => {
            let prog = parse_file(&first_arg)?;
            match prog.by_name(&expression) {
                Some(node) => println!("{}", node.cmr()),
                None => return Err(format!("the node {} is not in the given program", expression)),
            }
        }
        Command::Disassemble => {
            let v = Vec::from_hex(&first_arg)
                .map_err(|e| format!("failed to parse hex: {}", e))?;
            let prog = Program::<DefaultJet>::from_bytes(&v)
                .map_err(|e| format!("failed to parse program: {}", e))?;
            println!("{}", prog.string_serialize());
        }
        Command::Help => {
            let _ = usage(&process_name);
        }
        Command::Type => {
            let prog = parse_file(&first_arg)?;
            match prog.by_name(&expression) {
                Some(node) => println!("{}", node.arrow()),
                None => return Err(format!("the node {} is not in the given program", expression)),
            }
        }
        Command::Relabel => {
            let prog = parse_file(&first_arg)?
                .relabelled::<InternalSharing>();
            println!("{}", prog.string_serialize());
        }
        Command::Unshare => {
            let prog = parse_file(&first_arg)?
                .relabelled::<NoSharing>();
            println!("{}", prog.string_serialize());
        }
        Command::Witnesses => {
            let mut prog = parse_file(&first_arg)?;
            for (key, val) in &*prog.witnesses() {
                println!("{}: {}", key, val.0);
            }
        }
    }

    Ok(())
}

