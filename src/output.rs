use termimad::crossterm::style::Color::*;
use termimad::*;

pub trait Output {
    fn write(&mut self, src: &str);
}

pub struct MockConsole {
    pub write_was_called: bool,
}
impl Output for MockConsole {
    fn write(&mut self, src: &str) {
        println!("{}", src);
        self.write_was_called = true;
    }
}

pub struct Console {
    pub skin: MadSkin,
}

impl Output for Console {
    fn write(&mut self, src: &str) {
        self.skin.print_text(src)
    }
}

// pub fn write<T: Output>(mut output: T, src: &str) {
//     output.write(src);
// }
