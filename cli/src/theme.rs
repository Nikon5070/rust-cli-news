use crossterm::style::Color;
use termimad::{ MadSkin };


pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::Yellow);
    skin.italic.set_fg(Color::Rgb {
        r:28,
        g:28,
        b:28,
    });

    skin
}