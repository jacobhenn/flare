use crate::{
    args::{Mode, RunArgs},
    set::Set,
    stats::Stats,
};

use std::{ffi::OsStr, io};

use anyhow::{Context, Result};

use crossterm::{
    cursor,
    style::Stylize,
    terminal::{self, ClearType},
    QueueableCommand,
};
use rand::seq::SliceRandom;

pub fn run(args: RunArgs) -> Result<()> {
    let sets: Vec<Set> = args
        .sets
        .iter()
        .map(|s| Set::read(s))
        .collect::<Result<_>>()?;

    let set_names: Vec<&OsStr> = args
        .sets
        .iter()
        .map(|s| s.file_stem().unwrap_or(&OsStr::new("[unknown]")))
        .collect();

    let mut guesses_left: usize = sets.iter().map(|s| s.terms.len()).sum();

    let mut rng = rand::thread_rng();

    let mut stdout = io::stdout();

    let mut stats = Stats::new(&sets, &set_names);

    let (_, columns) = terminal::size()?;
    let (_, mut column) = cursor::position()?;
    if column >= columns - 1 {
        column -= 3;
    }

    'outer: loop {
        let mut indices: Vec<(usize, usize)> = (0..sets.len())
            .map(|set| (0..sets[set].terms.len()).map(move |term| (set, term)))
            .flatten()
            .collect();
        indices.shuffle(&mut rng);
        for (set, term) in indices {
            if !args.endless {
                guesses_left -= 1;
            }

            let (key, val) = &sets[set].terms[term];

            let (show, guess) = match args.mode {
                Mode::Keys => (val, key),
                Mode::Vals => (key, val),
            };

            println!("{show}");
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .context("failed to read stdin")?;
            let response = response.trim();

            stdout
                .queue(cursor::MoveTo(0, column))?
                .queue(terminal::Clear(ClearType::FromCursorDown))?;

            if response == ":q" {
                break 'outer;
            }

            if response == guess {
                print!("{}", "correct".green().bold());
                stats.record(set, term, true);
            } else {
                print!(
                    "{}{} {} => {}",
                    "incorrect".red().bold(),
                    ":".bold(),
                    show.as_str().bold(),
                    guess.as_str().bold(),
                );
                stats.record(set, term, false);
            }

            if sets.len() > 1 {
                print!(" - {}", set_names[set].to_string_lossy());
            }

            if args.show_remaining {
                println!(" - {} left", guesses_left);
            } else {
                println!();
            }
        }

        if !args.endless {
            break;
        }
    }

    stats.print();

    Ok(())
}
