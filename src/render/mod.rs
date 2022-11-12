use std::io::{Result, Write};

pub trait Render<T: Write> {
    fn render(&self, output: T) -> Result<()>;
}
