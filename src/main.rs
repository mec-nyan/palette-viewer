//  ░░░░░█▀█░█▀█░█░░░█▀▀░▀█▀░▀█▀░█▀▀░░░█░█░▀█▀░█▀▀░█░█░█▀▀░█▀▄░░░░  //
//  ░░░░░█▀▀░█▀█░█░░░█▀▀░░█░░░█░░█▀▀░░░▀▄▀░░█░░█▀▀░█▄█░█▀▀░█▀▄░░░░  //
//  ░░░░░▀░░░▀░▀░▀▀▀░▀▀▀░░▀░░░▀░░▀▀▀░░░░▀░░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░░░░  //

mod palette;
mod view;

use clap::Parser;
use std::path::PathBuf;

use color_eyre::install;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
};

use crate::{
    palette::{Palette, parse_palette},
    view::ui,
};

#[derive(Parser, Debug)]
#[command(version, about = "View GIMP palettes in the terminal")]
struct Args {
    // Path to the selected palette.
    // TOOD: Maybe list available palettes.
    path: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    install()?;

    let args = Args::parse();

    let palette = parse_palette(&args.path);
    match palette {
        Ok(palette) => {
            ratatui::run(|terminal| app(terminal, palette))?;
        }
        Err(_) => {
            panic!("Not a GIMP palette!");
        }
    }

    Ok(())
}

fn app(terminal: &mut DefaultTerminal, palette: Palette) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            ui(frame, &palette);
        })?;

        if event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
