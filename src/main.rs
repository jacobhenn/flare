use crate::args::{Command, FlareArgs};

use anyhow::Result;

use clap::Parser;

mod args;

mod set;

mod guess;

mod reveal;

mod stats;

fn main() -> Result<()> {
    let args = FlareArgs::parse();

    match args.command {
        Command::Guess(args) => guess::run(args)?,
        // Command::Reveal(..) => todo!(),
        Command::SetDir => println!("{}", set::dir()?.display()),
    }

    Ok(())
}
