use ratatui::{DefaultTerminal, crossterm::event};

use crate::{palette::Palette, view::ui};

pub fn app(terminal: &mut DefaultTerminal, palette: Palette, _: bool) -> std::io::Result<()> {
    // TOOD: Use the _ (bool) to select between inline and fullscreen modes.
    loop {
        terminal.draw(|frame| {
            ui(frame, &palette);
        })?;

        if event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
