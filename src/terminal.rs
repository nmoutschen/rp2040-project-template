use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    prelude::*,
    text::Text,
};

// 64 character long string
static FILLER_STRING: &str = "                                                            ";

pub struct Terminal<'f, C, S> {
    config: TerminalConfig<'f, C, S>,
    pos: Point,
}

impl<'f, C, S> Terminal<'f, C, S>
where
    C: RgbColor,
    S: DrawTarget<Color = C> + OriginDimensions,
    <S as embedded_graphics::draw_target::DrawTarget>::Error: core::fmt::Debug,
{
    /// Handle a batch of ASCII characters
    pub fn write(&mut self, c: &[u8]) {
        c.iter().for_each(|c| self.write_char(*c));
    }

    /// Handle a single ASCII character
    pub fn write_char(&mut self, c: u8) {
        // Erase the cursor
        if self.config.cursor_color.is_some() {
            self.erase_chars(1);
        }

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

        // Redraw the cursor
        if self.config.cursor_color.is_some() {
            self.draw_cursor();
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

    /// Draw the cursor on the screen
    fn draw_cursor(&mut self) {
        if let Some(color) = self.config.cursor_color {
            let style_builder = MonoTextStyleBuilder::new()
                .font(self.config.style.font)
                .text_color(color);
            if let Some(bg_color) = self.config.style.background_color {
                style_builder.background_color(bg_color);
            }
            let style = style_builder.build();

            Text::new("_", self.pos, style)
                .draw(&mut self.config.screen)
                .unwrap();
        }
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

        self.erase_chars(n);
    }

    /// Move the cursor by `count` characters
    ///
    /// If the cursor ends up outside the bounds of the screen, it will be moved to the next line.
    fn move_forward(&mut self, n: i32) {
        let char_width = self.config.style.font.character_size.width as i32;

        let new_x = self.pos.x + n * char_width;
        if new_x + char_width > self.max_x() {
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
        let char_height = self.config.style.font.character_size.height as i32;

        // Reset x position to the beginning of the line
        self.pos.x = self.min_x();

        let new_y = self.pos.y + char_height;
        if new_y + char_height > self.max_y() {
            // Looping to the beginning of the screen
            // TODO: Clear the display or scroll the screen
            self.pos.y = self.config.offset.y;
        } else {
            self.pos.y = new_y;
        }

        self.erase_chars(FILLER_STRING.len() as i32);
    }

    fn erase_chars(&mut self, n: i32) {
        // Erase characters
        let color = match self.config.style.background_color {
            Some(color) => color,
            None => C::BLACK,
        };
        let style = MonoTextStyleBuilder::new()
            .font(self.config.style.font)
            .background_color(color)
            .build();
        Text::new(&FILLER_STRING[..n as usize], self.pos, style)
            .draw(&mut self.config.screen)
            .unwrap();
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
    cursor_color: Option<C>,
    style: MonoTextStyle<'f, C>,
}

/// Builder for the `Terminal`
pub struct TerminalBuilder<'f, C, S> {
    config: TerminalConfig<'f, C, S>,
}

impl<'f, C, S> TerminalBuilder<'f, C, S>
where
    C: RgbColor,
{
    // pub fn new(screen: S, style: MonoTextStyle<'f, C>) -> Self {
    pub fn new(screen: S) -> Self {
        Self {
            config: TerminalConfig {
                screen,
                offset: Point::new(0, 0),
                cursor_color: None,
                style: MonoTextStyleBuilder::new()
                    .font(&FONT_6X10)
                    .text_color(C::RED)
                    .background_color(C::BLACK)
                    .build(),
            },
        }
    }

    pub fn with_offset(mut self, offset: Point) -> Self {
        self.config.offset = offset;
        self
    }

    pub fn with_cursor(mut self, color: C) -> Self {
        self.config.cursor_color = Some(color);
        self
    }

    pub fn with_style(mut self, style: MonoTextStyle<'f, C>) -> Self {
        self.config.style = style;
        self
    }

    pub fn build(self) -> Terminal<'f, C, S> {
        Terminal {
            pos: self.config.offset.clone(),
            config: self.config,
        }
    }
}
