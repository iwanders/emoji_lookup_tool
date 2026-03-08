use std::path::PathBuf;

// This has a nice comparison of shortcodes.
// https://emojibase.dev/shortcodes/
//
// Unicode reference: https://github.com/unicode-org/cldr-json
// Slack: https://github.com/iamcal/emoji-data
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
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
    code_points: Vec<u32>,
}
impl UnicodePoints {
    /// To the noto hex string for the filename.
    fn to_hex_u16_noto(&self) -> String {
        // 0030_20e3
        let up_to = if self.code_points.last().unwrap() == &0xfe0f {
            self.code_points.len() - 1
        } else {
            self.code_points.len()
        };
        self.code_points
            .iter()
            .take(up_to)
            .map(|a| format!("{a:0>4x}"))
            .collect::<Vec<_>>()
            .join("_")
    }
    /// To the iamcal emoji data unified value.
    fn to_hex_unified(&self) -> String {
        // 0030-20E3
        self.code_points
            .iter()
            .map(|a| format!("{a:0>4X}"))
            .collect::<Vec<_>>()
            .join("-")
    }
    /// To escaped string hex thing.
    fn to_escaped_hex(&self) -> String {
        self.code_points
            .iter()
            .map(|a| format!("\\u{a:0>4x}"))
            .collect::<String>()
    }
}

#[derive(Debug, Copy, Clone, ValueEnum, Default)]
enum NotoGlyphType {
    Svg,
    #[default]
    Png,
    Png32,
    Png72,
    Png128,
    Png512,
}
impl NotoGlyphType {
    pub fn to_extension(&self) -> &'static str {
        match self {
            NotoGlyphType::Svg => "svg",
            _ => "png",
        }
    }
    pub fn to_subpath(&self) -> &'static str {
        match self {
            NotoGlyphType::Svg => "svg",
            NotoGlyphType::Png => "png/512",
            NotoGlyphType::Png32 => "png/32",
            NotoGlyphType::Png72 => "png/72",
            NotoGlyphType::Png128 => "png/128",
            NotoGlyphType::Png512 => "png/512",
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
        let mut code_points = vec![];
        for c in v.chars() {
            code_points.push(c as u32);
        }
        UnicodePoints { code_points }
    }

    pub fn file_name(&self, codepoints: &UnicodePoints, format: NotoGlyphType) -> String {
        let ext = format.to_extension();
        format!("emoji_u{}.{ext}", codepoints.to_hex_u16_noto())
    }
    pub fn to_url(&self, codepoints: &UnicodePoints, format: NotoGlyphType) -> String {
        // https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/heads/main/svg/emoji_u0023.svg
        // https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/tags/v2.051/svg/emoji_u0023.svg
        // https://raw.githubusercontent.com/googlefonts/noto-emoji/8998f5dd683424a73e2314a8c1f1e359c19e8742/svg/emoji_u0023.svg
        // Lets just use main.
        let subpath = format.to_subpath();
        format!(
            "{}/refs/heads/main/{subpath}/{}",
            Self::BASE_URL,
            self.file_name(codepoints, format)
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
    Noto {
        /// The emoji character to retrieve.
        emoji: Vec<String>,

        /// The format to retrieve, png defaults to png512.
        #[arg(short, long, value_enum)]
        format: Option<NotoGlyphType>,

        /// The output directory to write the retrieved glyph to.
        #[arg(short, long, default_value = "/tmp/")]
        out_dir: PathBuf,
    },

    Info {
        /// The emoji character to retrieve.
        emoji: Vec<String>,
    },

    /// Dump the entire emoji list, because why not, then you can grep to your heart's content.
    List,
}

fn parsed() -> Vec<Entry> {
    serde_json::from_str(EMOJI_DATA).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::Noto {
            emoji,
            format,
            out_dir,
        } => {
            let noto = NotoFont::new();
            for emoji in emoji.iter() {
                let path = noto.to_path(emoji);
                println!("{emoji} -> {path:?}");
                let format = format.unwrap_or_default();
                let url = noto.to_url(&path, format);
                let file_name = noto.file_name(&path, format);
                println!("  {:}", url);
                let res = reqwest::blocking::get(url)?;
                let data = res.bytes()?;

                let mut out_dir = out_dir.clone();
                fs::create_dir_all(&out_dir)?;
                out_dir.push(file_name);
                print!("  Writing to {:?}", out_dir);
                fs::write(out_dir, data)?;
                println!(" done!");
            }
        }
        Commands::Info { emoji } => {
            let noto = NotoFont::new();
            for emoji in emoji.iter() {
                let path = noto.to_path(emoji);
                println!("{emoji}");

                println!("           emoji: {:}", emoji);
                println!("  codepoints dec: {:?}", path.code_points);
                println!("  codepoints hex: {:0>4x?}", path.code_points);
                // \u1F3F4\u200D\u2620\uFE0F
                println!("  escaped hex   : {:}", path.to_escaped_hex());
                let url_png = noto.to_url(&path, NotoGlyphType::Png);
                let url_svg = noto.to_url(&path, NotoGlyphType::Svg);
                println!("        noto png: {:}", url_png);
                println!("        noto svg: {:}", url_svg);

                let p: Vec<Entry> = parsed();
                for e in p.iter() {
                    if path.to_hex_unified() == e.unified {
                        println!("{: >16}: {}", "name", e.name);
                        println!("{: >16}: {}", "category", e.category);
                        println!("{: >16}: {}", "subcategory", e.subcategory);
                        println!("{: >16}: {}", "short_name", e.short_name);
                        if e.short_names.len() != 1 {
                            println!("{: >16}: {:?}", "short_names", e.short_names);
                        }
                        println!("{: >16}: :{}:", "slack", e.short_name);
                    }
                }
            }
        }
    }
    Ok(())
}
