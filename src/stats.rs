use std::{ffi::OsStr, iter};

use crate::set::Set;

use crossterm::{style::Stylize, terminal};

#[derive(Clone, Copy)]
struct Guess {
    set: usize,
    term: usize,
    was_correct: bool,
}

pub struct Stats<'a> {
    sets: &'a [Set],
    set_names: &'a [&'a OsStr],
    guesses: Vec<Guess>,
}

impl<'a> Stats<'a> {
    pub fn new(sets: &'a [Set], set_names: &'a [&'a OsStr]) -> Self {
        Self {
            sets,
            set_names,
            guesses: Vec::new(),
        }
    }

    pub fn record(&mut self, set: usize, term: usize, was_correct: bool) {
        self.guesses.push(Guess {
            set,
            term,
            was_correct,
        })
    }

    fn print_guess_set<'b>(guesses: impl Iterator<Item = &'b Guess>) {
        let (terminal_width, terminal_height) = terminal::size().unwrap_or((80, 30));

        let mut total_guesses = 0;
        let mut correct_guesses = 0;

        for guess in guesses {
            total_guesses += 1;
            if guess.was_correct {
                correct_guesses += 1;
            }
        }

        if total_guesses == 0 {
            println!("(no guesses)");
            return;
        }

        println!("total guesses: {}", total_guesses.to_string().bold());
        println!("correct guesses: {}", correct_guesses.to_string().bold());

        println!("average correctness:");
        let progress_bar_width = terminal_width as usize * 2 / 3;
        let avg_correct_pct = correct_guesses * 100 / total_guesses;
        let dashes = correct_guesses * progress_bar_width / total_guesses;
        println!(
            "[{blank:->dashes$}{blank: >spaces$}] {pct}%",
            blank = "",
            spaces = progress_bar_width - dashes,
            pct = avg_correct_pct.to_string().bold()
        );
    }

    pub fn print(&self) {
        if self.sets.len() > 1 {
            for (i, name) in self.set_names.iter().enumerate() {
                println!("\n- for set {}:", name.to_string_lossy());
                Self::print_guess_set(self.guesses.iter().filter(|g| g.set == i))
            }
            println!("\n- total:");
        }
        Self::print_guess_set(self.guesses.iter());
    }
}
