use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Dead simple TUI flashcards.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct FlareArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show terms and prompt the user to type the corresponding terms.
    Guess(RunArgs),
    // /// Show terms and reveal the corresponding terms on a keypress.
    // Reveal(RunArgs),
    /// Show the location of the directory where `flare` will look for sets.
    SetDir,
}

#[derive(Args)]
pub struct RunArgs {
    /// Whether to show keys or values of the set first.
    #[arg(value_enum)]
    pub mode: Mode,

    /// The paths of the sets to draw terms from. If a path cannot be found relative to the
    /// current directory, it is looked for under the sets directory (see `flare set-dir`).
    #[arg(num_args = 1..)]
    pub sets: Vec<PathBuf>,

    /// Don't exit after completing the set; shuffle it and go again.
    #[arg(long)]
    pub endless: bool,

    /// Show how many terms remain in the set after each guess.
    #[arg(long, conflicts_with("endless"))]
    pub show_remaining: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
    Keys,
    Vals,
}
