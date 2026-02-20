// This has a nice comparison of shortcodes.
// https://emojibase.dev/shortcodes/
//
// Unicode reference: https://github.com/unicode-org/cldr-json
// Slack: https://github.com/iamcal/emoji-data
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

const EMOJI_DATA: &str = include_str!("../thirdparty/iamcal_emoji-data_emoji.json");

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: String,
    short_name: &'static str,
    short_names: Vec<&'static str>,
    unified: &'static str,
}
impl Entry {
    pub fn contains_needle(&self, name: &str) -> bool {
        for s in self.short_names.iter() {
            if s.contains(name) {
                return true;
            }
        }
        if self.short_name.to_ascii_lowercase().contains(name) {
            return true;
        }
        false
    }

    pub fn to_string(&self) -> String {
        let mut z = String::with_capacity(4);
        for p in self.unified.split("-") {
            let single_byte = u32::from_str_radix(p, 16).unwrap();
            z += &std::char::from_u32(single_byte).unwrap().to_string();
        }
        z
    }
}

#[derive(Parser)]
#[command(version, about, long_about = "Search emoji's.")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search emoji's.
    Search { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search { name } => {
            let p: Vec<Entry> = serde_json::from_str(EMOJI_DATA).unwrap();
            for e in p.iter() {
                if e.contains_needle(name) {
                    println!("{}     {} ", e.to_string(), e.name);
                }
            }
        }
    }
}
