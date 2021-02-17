// Dunno why we need this but Serde says so
#![feature(trivial_bounds)]
#![feature(iterator_fold_self)]

mod tree;
use tree::Node;

use anyhow::Result;
use structopt::StructOpt;

use std::{fs, path::PathBuf};
fn main() -> Result<()> {
    let options = DrawOptions::from_args();

    let mut input_path = PathBuf::from(if cfg!(debug_assertions) {
        env!("CARGO_MANIFEST_DIR")
    } else {
        "."
    });
    input_path.push(&options.input);
    let data = fs::read(input_path)?;
    let data = String::from_utf8_lossy(&data);
    let node: Node = serde_json::from_reader(json_comments::StripComments::new(data.as_bytes()))?;

    let drawn = &node.draw(&options);

    let mut output_path = PathBuf::from(if cfg!(debug_assertions) {
        env!("CARGO_MANIFEST_DIR")
    } else {
        "."
    });
    if let Some(it) = options.output {
        output_path.push(it)
    } else {
        let buf = PathBuf::from(options.input);
        output_path.push(format!(
            "{}.svg",
            buf.file_stem().unwrap().to_string_lossy()
        ));
    }

    fs::write(output_path, &drawn)?;

    Ok(())
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct DrawOptions {
    /// Input file path
    pub input: String,
    /// Output file path
    pub output: Option<String>,

    /// Horizontal space between terminal nodes on the tree, in pixels
    #[structopt(default_value = "100.0", long)]
    pub x_space: f32,
    /// Minimum vertical space between nodes on the tree, in pixels
    #[structopt(default_value = "50.0", long)]
    pub y_space: f32,
    /// Vertical space given between each property
    #[structopt(default_value = "20.0", long)]
    pub property_space: f32,
    /// Color of the branches
    #[structopt(default_value = "black", long)]
    pub branch_color: String,
    /// Font size of the main nodes in pixels
    #[structopt(default_value = "20.0", long)]
    pub node_font_size: f32,
    /// Font size of the property nodes in pixels
    #[structopt(default_value = "15.0", long)]
    pub prop_font_size: f32,
}

#[test]
fn make_schema() -> Result<()> {
    let outpath = if cfg!(debug_assertions) {
        concat!(env!("CARGO_MANIFEST_DIR"), "/node.schema.json")
    } else {
        "node.schema.json"
    };
    let schema = schemars::schema_for!(Node);
    let json = serde_json::to_string_pretty(&schema)?;
    fs::write(outpath, json)?;

    Ok(())
}
