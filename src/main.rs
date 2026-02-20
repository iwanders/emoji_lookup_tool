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
    category: &'static str,
    subcategory: &'static str,
}
impl Entry {
    pub fn contains_needle(&self, name: &str) -> bool {
        for s in self.short_names.iter() {
            if s.contains(name) {
                return true;
            }
        }
        if self.name.to_ascii_lowercase().contains(name) {
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

    pub fn sort_category_subcategory_name(left: &Self, right: &Self) -> std::cmp::Ordering {
        (left.category, left.subcategory, &left.name).cmp(&(
            right.category,
            right.subcategory,
            &right.name,
        ))
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
    /// Search emoji's, matching by the provided string in name and short names.
    Search { name: String },
    /// Dump the entire emoji list, because why not, then you can grep to your heart's content.
    List,
}

fn parsed() -> Vec<Entry> {
    serde_json::from_str(EMOJI_DATA).unwrap()
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search { name } => {
            let p: Vec<Entry> = parsed();
            for e in p.iter() {
                if e.contains_needle(name) {
                    println!("{}     {}", e.to_string(), e.name.to_lowercase());
                }
            }
        }
        Commands::List => {
            let mut p: Vec<Entry> = parsed();
            p.sort_by(Entry::sort_category_subcategory_name);
            for e in p.iter() {
                println!(
                    "{ }     {} ({}, {})",
                    e.to_string(),
                    e.name.to_lowercase(),
                    e.category,
                    e.subcategory
                );
            }
        }
    }
}
