trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range {
            canvas.wirte_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M')
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..selft.y
    }
}

pub struct Sink;

use std::io::{Write, Result};

impl Write for Sink {
    fn wirte(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait WriteHtml {
    fn write_html(&mut self, &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        Ok(())
    }
}

use serde::Serialize;
use serde_json;

pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    let writer = File::Create(config_filename())?;
    let mut serializer = serde_json::Serializer::new(writer);

    config.serialize(&mut serializer)?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
