use std::io::{stdout, Write};

use anyhow::Result;

use crossterm::{
    cursor::MoveTo,
    terminal,
    queue,
    style::Print,
};

#[derive(Debug)]
pub struct Popup {
    pub width: u16,
    pub x: u16,
    pub y: u16,
    pub lines: Vec<String>,
}

impl Popup {
    pub fn new(lines: Vec<String>) -> Result<Popup> {
        let (cols, rows) = terminal::size()?;
        let x = cols / 4;
        let y = rows / 4;
        Ok(Popup {
            width: x * 2,
            x,
            y,
            lines,
        })
    }

    pub fn display(&self) -> Result<()> {
        let mut stdout = stdout();
        
        print_range(&mut stdout, Some((self.x, self.y)), self.width, '-')?;

        queue!(
            stdout,
            MoveTo(self.x, self.y + 1),
            Print('|')
        )?;
        print_range(&mut stdout, None, self.width - 2, ' ')?;
        queue!(
            stdout,
            Print('|')
        )?;

        for (i, line) in self.lines.iter().enumerate() {
            // TODO make sure it doesn't overflow
            let total_padding = self.width - line.len() as u16 - 2;
            queue!(
                stdout,
                MoveTo(self.x, self.y + 2 + i as u16),
                Print('|'),
            )?;
            print_range(&mut stdout, None, total_padding / 2, ' ')?;
            queue!(
                stdout,
                Print(line),
            )?;
            print_range(&mut stdout, None, total_padding - total_padding / 2, ' ')?;
            queue!(
                stdout,
                Print('|'),
            )?;
        }

        queue!(
            stdout,
            MoveTo(self.x, self.y + 2 + self.lines.len() as u16),
            Print('|')
        )?;
        print_range(&mut stdout, None, self.width - 2, ' ')?;
        queue!(
            stdout,
            Print('|')
        )?;

        print_range(&mut stdout, Some((self.x, self.y + 3 + self.lines.len() as u16)), self.width, '-')?;

        stdout.flush()?;

        Ok(())
    }
}

fn print_range(output: &mut impl Write, pos: Option<(u16, u16)>, width: u16, c: char) -> Result<()> {
    if let Some((x, y)) = pos {
        queue!(
            output,
            MoveTo(x, y),
        )?;
    }

    for _ in 0..width {
        queue!(
            output,
            Print(c),
        )?;
    }

    Ok(())
}
