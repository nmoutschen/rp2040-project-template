use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, text::Text};

pub struct Terminal<'f, C, S> {
    config: TerminalConfig<'f, C, S>,
    pos: Point,
}

impl<'f, C, S> Terminal<'f, C, S>
where
    C: PixelColor,
    S: DrawTarget<Color = C> + OriginDimensions,
    <S as embedded_graphics::draw_target::DrawTarget>::Error: core::fmt::Debug,
{
    /// Handle a batch of ASCII characters
    pub fn write(&mut self, c: &[u8]) {
        c.iter().for_each(|c| self.write_char(*c));
    }

    /// Handle a single ASCII character
    pub fn write_char(&mut self, c: u8) {
        match c {
            0x00..=0x07 => (),
            // Backspace
            0x08 => self.move_backward(1),
            // Tab
            0x09 => self.move_forward(4),
            // New line
            0x0A => self.move_next_line(),
            0x0B..=0x0C => (),
            // Carriage return
            0x0D => self.move_next_line(),
            0x0E..=0x1F => (),
            // Delete
            0x7F => self.move_backward(1),
            // Characters
            _ => self.print_char(c),
        }
    }

    /// Print a single ASCII character
    fn print_char(&mut self, c: u8) {
        // TODO: remove unwraps
        Text::new(
            &core::str::from_utf8(&[c]).unwrap_or("?"),
            self.pos,
            self.config.style,
        )
        .draw(&mut self.config.screen)
        .unwrap();

        self.move_forward(1);
    }

    /// Move the cursor backwards
    fn move_backward(&mut self, n: i32) {
        // TODO: clear characters
        let new_x = self.pos.x - n * self.config.style.font.character_size.width as i32;
        // Only move backwards if we're not at the left edge, otherwise, do nothing (for now).
        // TODO: Move to previous line if we're at the left edge
        if new_x >= self.min_x() {
            self.pos.x = new_x;
        }
    }

    /// Move the cursor by `count` characters
    ///
    /// If the cursor ends up outside the bounds of the screen, it will be moved to the next line.
    fn move_forward(&mut self, n: i32) {
        let new_x = self.pos.x + n * self.config.style.font.character_size.width as i32;
        if new_x >= self.max_x() {
            // Going to the next line
            self.move_next_line();
        } else {
            self.pos.x = new_x;
        }
    }

    // /// Move to the beginning of the line
    // fn move_start_line(&mut self) {
    //     self.pos.x = self.min_x();
    // }

    /// Move to the next line
    fn move_next_line(&mut self) {
        // Reset x position to the beginning of the line
        self.pos.x = self.min_x();

        let new_y = self.pos.y + self.config.style.font.character_size.height as i32;
        if new_y >= self.max_y() {
            // Looping to the beginning of the screen
            // TODO: Clear the display or scroll the screen
            self.pos.y = self.config.offset.y;
        } else {
            self.pos.y = new_y;
        }
    }

    /// Maximum X coordinate for the screen
    fn max_x(&self) -> i32 {
        self.config.offset.x + self.config.screen.size().width as i32
    }
    /// Minimum X coordinate for the screen
    fn min_x(&self) -> i32 {
        self.config.offset.x
    }

    /// Maximum Y coordinate for the screen
    fn max_y(&self) -> i32 {
        self.config.offset.y + self.config.screen.size().height as i32
    }
    // /// Minimum Y coordinate for the screen
    // fn min_y(&self) -> i32 {
    //     self.config.offset.y
    // }
}

/// Configuration of a `Terminal`
struct TerminalConfig<'f, C, S> {
    screen: S,
    offset: Point,
    style: MonoTextStyle<'f, C>,
}

/// Builder for the `Terminal`
pub struct TerminalBuilder<'f, C, S> {
    config: TerminalConfig<'f, C, S>,
}

impl<'f, C, S> TerminalBuilder<'f, C, S> {
    pub fn new(screen: S, style: MonoTextStyle<'f, C>) -> Self {
        Self {
            config: TerminalConfig {
                screen,
                offset: Point::new(0, 0),
                style,
            },
        }
    }

    pub fn with_offset(self, offset: Point) -> Self {
        Self {
            config: TerminalConfig {
                offset,
                ..self.config
            },
        }
    }

    pub fn build(self) -> Terminal<'f, C, S> {
        Terminal {
            pos: self.config.offset.clone(),
            config: self.config,
        }
    }
}
