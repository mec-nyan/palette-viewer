//  ░░░░░█▀█░█▀█░█░░░█▀▀░▀█▀░▀█▀░█▀▀░░░█░█░▀█▀░█▀▀░█░█░█▀▀░█▀▄░░░░  //
//  ░░░░░█▀▀░█▀█░█░░░█▀▀░░█░░░█░░█▀▀░░░▀▄▀░░█░░█▀▀░█▄█░█▀▀░█▀▄░░░░  //
//  ░░░░░▀░░░▀░▀░▀▀▀░▀▀▀░░▀░░░▀░░▀▀▀░░░░▀░░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░░░░  //
//
// Palette viewer
//
// View GPL palette files on your terminal, plus much more!
//
// Wishlist:
// - Work in both inline and full screen modes.
// - Show full palette or sections of it.
// - Show colour information (hex, rgb, etc).
// - Select colours.
// - Allow for foreground and background selection.
// - Display an example of currently selected colour(s).
// - Yank to clipboard, select what to yank (again, hex, rgb, etc).
// - Move around swatches.
//
// Implementation:
//   Try to make a clean implementation using the ELM architecture (TEA).
//

mod app;
mod palette;
mod view;

use crate::app::app;

use clap::Parser;
use std::path::PathBuf;

use color_eyre::install;

use crate::palette::parse_palette;

#[derive(Parser, Debug)]
#[command(version, about = "View GIMP palettes in the terminal")]
struct Args {
    /// View palette in full screen mode
    #[arg(short, long)]
    full_screen: bool,

    /// Path to the selected palette.
    path: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    install()?;

    let args = Args::parse();

    let palette = parse_palette(&args.path)?;

    ratatui::run(|terminal| app(terminal, palette, args.full_screen))?;

    Ok(())
}
