use ratatui::{prelude::*, widgets::*};

use crate::palette::{Palette, get_columns};

pub fn ui(frame: &mut Frame, palette: &Palette) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
        .split(frame.area());

    // Info:
    let palette_name = &palette.name;
    let info = Text::raw(format!("Palette: {}", palette_name));
    frame.render_widget(info, layout[0]);

    // visuals:
    let columns = get_columns(palette);
    let rows = (palette.colours.len() + columns - 1) / columns;

    let mut lines = Vec::new();

    for row in 0..rows {
        let mut spans = Vec::new();

        for col in 0..columns {
            let idx = row * columns + col;

            if let Some(colour) = palette.colours.get(idx) {
                let style = Style::default().bg(Color::Rgb(colour.r, colour.g, colour.b));
                spans.push(Span::styled("  ", style));
            } else {
                // empty swatches at the end.
                spans.push(Span::raw("  "));
            }
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, layout[1]);
}
