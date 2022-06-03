mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    build_theme(&palette::Palette::luna(), Type::Dark, "Orbitized Luna")?;
    build_theme(&palette::Palette::sol(), Type::Light, "Orbitized Sol")?;

    Ok(())
}

fn build_theme(palette: &palette::Palette, ty: Type, name: &str) -> io::Result<()> {
    let mut builder = ThemeBuilder::new(name.to_string(), ty);
    imp::add_rules(&mut builder, palette);
    builder.build().save()?;

    Ok(())
}
