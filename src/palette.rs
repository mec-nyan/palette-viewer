use std::fs;

#[derive(Debug)]
pub struct Colour {
    // TODO: Remove public accessor and add corresponding methods.
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Palette {
    // TODO: Remove public accessor and add corresponding methods.
    pub name: String,
    pub columns: Option<usize>,
    pub colours: Vec<Colour>,
}

pub fn parse_palette(path: &std::path::Path) -> Result<Palette, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut lines = content.lines();

    // check palette header
    if lines.next() != Some("GIMP Palette") {
        return Err("Not a gimp palette.".to_string());
    }

    let mut name = String::new();
    let mut columns = None;
    let mut colours = Vec::new();

    for line in lines {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix("Name:") {
            name = rest.trim().to_string();
            // Inkscape default palette starts with an empty swatch for "no colour".
            // TODO: This may belong to the view...
            if name.to_lowercase().contains("inkscape") {
                colours.push(Colour {
                    r: 0,
                    g: 0,
                    b: 0,
                    name: Some("transparent".to_string()),
                });
            }
        } else if let Some(rest) = line.strip_prefix("Columns:") {
            columns = rest.trim().parse().ok();
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let r = parts[0].parse().unwrap_or(0);
                let g = parts[1].parse().unwrap_or(0);
                let b = parts[2].parse().unwrap_or(0);
                let name = parts.get(3..).map(|s| s.join(" "));

                colours.push(Colour { r, g, b, name });
            }
        }
    }

    Ok(Palette {
        name,
        columns,
        colours,
    })
}

pub fn get_columns(palette: &Palette) -> usize {
    palette.columns.unwrap_or(16)
}
