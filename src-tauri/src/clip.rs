use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use arboard::Clipboard;


pub fn read_clipboard() -> Result<String, arboard::Error> {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text()
}

pub fn set_clipboard(text: &str) -> Result<(), arboard::Error>{
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)
}

pub fn copy_content() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&Settings::default())?;
    // select all
    enigo.key(Key::Control, Press)?;
    enigo.key(Key::Unicode('c'), Click)?;
    enigo.key(Key::Control, Release)?;
    Ok(())
}