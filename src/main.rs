use std::fs::read_to_string;

use clap::Parser;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let text = read_to_string(args.file).unwrap();

    let mut json_config = HighlightConfiguration::new(
        tree_sitter_json::LANGUAGE.into(),
        "json",
        tree_sitter_json::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .unwrap();
    let highlight_names = ["keyword", "number", "string", "property.builtin"];
    json_config.configure(&highlight_names);

    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&json_config, text.as_bytes(), None, |_| None)
        .unwrap();

    for event in highlights {
        let event = event.unwrap();
        match event {
            HighlightEvent::Source { start, end } => {
                eprintln!("source: {start}-{end}");
            }
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {s:?}");
            }
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            }
        }
    }
}
