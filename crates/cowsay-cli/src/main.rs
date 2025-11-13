//! A command-line interface for generating cowsay-style ASCII art.
//!
//! This application is a clone of the classic `cowsay` utility, written in Rust.
//! It allows users to create messages spoken by an ASCII cow, with various
//! customization options for appearance and cow type.

use std::io::{self, Write};

use clap::Parser;
use cowsay::CowsayOption;

/// Represents the command-line arguments for the `cowsay-rs` application.
///
/// This struct uses `clap` to parse and manage command-line arguments,
/// providing options to customize the cowsay output, such as selecting
/// different cow appearances, specifying a cowfile, setting eye and tongue
/// strings, and controlling text wrapping.
#[derive(Debug, Parser)]
#[allow(clippy::struct_excessive_bools)]
#[command(version, about = "cowsay clone, with speed.", long_about = "")]
pub struct CowsayArgs {
    /// Borg appearance mode
    #[arg(short, long, action)]
    borg: bool,
    /// Dead appearance mode
    #[arg(short, long, action)]
    dead: bool,
    /// Greedy appearance mode
    #[arg(short, long, action)]
    greedy: bool,
    /// Sleepy appearance mode
    #[arg(short, long, action)]
    sleepy: bool,
    /// Tired appearance mode
    #[arg(short, long, action)]
    tired: bool,
    /// Wired appearance mode
    #[arg(short, long, action)]
    wired: bool,
    /// Young appearance mode
    #[arg(short, long, action)]
    young: bool,
    /// Selects an alternate cow picture. Cowfile may be either the name of a
    /// cow defined in a cowdir on the cowpath (without the '.cow' file extension), or the
    /// path to a cowfile (with the '.cow' file extension). `cowsay-rs -l` will list the
    /// names of available cows.
    #[arg(short, long)]
    file: Option<String>,
    /// Selects a random cow picture from the cowpath.
    #[arg(short, long, action)]
    random: bool,
    /// Specify the cow's eye characters.
    #[arg(short, long)]
    eyes: Option<String>,
    /// Specify the cow's tongue characters.
    #[arg(short = 'T', long)]
    tongue: Option<String>,
    /// Specify the text wrap column width. Default is 40.
    #[arg(short = 'W', long = "wrap")]
    wrap_column: Option<usize>,

    /// Text to be displayed inside the speech balloon.
    #[arg(required = true)]
    text: String,
}

fn main() -> io::Result<()> {
    let args = CowsayArgs::parse();

    let mut builder = CowsayOption::builder();
    builder = builder
        .with_borg(args.borg)
        .with_dead(args.dead)
        .with_greedy(args.greedy)
        .with_sleepy(args.sleepy)
        .with_tired(args.tired)
        .with_wired(args.wired)
        .with_young(args.young)
        .with_random(args.random);

    if let Some(ref file) = args.file {
        builder = builder.with_cowfile(file);
    }
    if let Some(ref eyes) = args.eyes {
        builder = builder.with_eyes(eyes);
    }
    if let Some(ref tongue) = args.tongue {
        builder = builder.with_tongue(tongue);
    }
    if let Some(wrap_column) = args.wrap_column {
        builder = builder.with_wrap(true).with_wrap_column(wrap_column);
    }

    let options = builder.build();

    let parser = match options.parser() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error creating cowsay parser: {e}");
            std::process::exit(1);
        }
    };

    if args.text.is_empty() {
        eprintln!("Error: No text provided for cowsay.");
        std::process::exit(1);
    }

    let cow = parser.say(Some(args.text.as_str()));

    let mut stdout = io::stdout();
    stdout.write_all(cow.as_bytes())
}
