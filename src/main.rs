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

#[derive(Debug, Clone)]
struct UnicodePoints {
    points: Vec<u32>,
}
impl UnicodePoints {
    fn to_hex_u16(&self) -> String {
        // 0030_20e3
        self.points
            .iter()
            .map(|a| format!("{a:0>4x}"))
            .collect::<Vec<_>>()
            .join("_")
    }
}

#[derive(Debug, Clone)]
enum PngResolution {
    Px32,
    Px72,
    Px128,
    Px512,
}
#[derive(Debug, Clone)]
enum NotoGlyphType {
    Svg,
    Png(PngResolution),
}
impl NotoGlyphType {
    pub fn to_extension(&self) -> &'static str {
        match self {
            NotoGlyphType::Svg => "svg",
            NotoGlyphType::Png(_) => "png",
        }
    }
    pub fn to_subpath(&self) -> &'static str {
        match self {
            NotoGlyphType::Svg => "svg",
            NotoGlyphType::Png(v) => match v {
                PngResolution::Px32 => "png/32",
                PngResolution::Px72 => "png/72",
                PngResolution::Px128 => "png/128",
                PngResolution::Px512 => "png/512",
            },
        }
    }
}

struct NotoFont {}
impl NotoFont {
    const BASE_URL: &'static str = "https://raw.githubusercontent.com/googlefonts/noto-emoji";
    pub fn new() -> Self {
        Self {}
    }
    pub fn to_path(&self, v: &str) -> UnicodePoints {
        let mut points = vec![];
        for c in v.chars() {
            points.push(c as u32);
        }
        UnicodePoints { points }
    }
    pub fn to_url(&self, codepoints: &UnicodePoints, format: NotoGlyphType) -> String {
        // https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/heads/main/svg/emoji_u0023.svg
        // https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/tags/v2.051/svg/emoji_u0023.svg
        // https://raw.githubusercontent.com/googlefonts/noto-emoji/8998f5dd683424a73e2314a8c1f1e359c19e8742/svg/emoji_u0023.svg
        // Lets just use main.
        let ext = format.to_extension();
        let subpath = format.to_subpath();
        format!(
            "{}/refs/heads/main/{subpath}/emoji_u{}.{ext}",
            Self::BASE_URL,
            codepoints.to_hex_u16()
        )
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

    /// Retrieve the emoji from the noto font github repo at https://github.com/googlefonts/noto-emoji
    Noto { emoji: String },

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
        Commands::Noto { emoji } => {
            let noto = NotoFont::new();
            let path = noto.to_path(emoji);
            println!("Converted {emoji} to: {path:?}");
            println!(
                "url: {:?}",
                noto.to_url(&path, NotoGlyphType::Png(PngResolution::Px128))
            );
        }
    }
}
