use std::io::Write;

use crossterm::{ cursor, queue, style, Result };

use super::BoundingBox;

pub fn display<W: Write>(w: &mut W, bounds: BoundingBox) -> Result<()> {
    queue!(
        w,
        cursor::MoveTo(bounds.x, bounds.y),
        style::Print("heyo there"),
        cursor::MoveToNextLine(1),
    )?;

    w.flush()?;

    Ok(())
}
