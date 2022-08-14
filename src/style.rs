#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<u32> for Color {
    fn from(raw: u32) -> Self {
        Self(
            ((raw >> 16) & 255) as u8,
            ((raw >> 8) & 255) as u8,
            (raw & 255) as u8,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Style {
    color: Color,
    background: Option<Color>,
    border: Option<f64>,
}

impl Style {
    pub fn color(raw: u32) -> Self {
        Self {
            color: raw.into(),
            background: None,
            border: None,
        }
    }

    pub fn with_border(mut self, thickness: f64) -> Self {
        self.border = Some(thickness);
        self
    }

    pub fn with_background(mut self, background: u32) -> Self {
        self.background = Some(background.into());
        self
    }

    pub fn has_fill(&self) -> bool {
        self.get_frame().is_none() || (self.get_frame().is_some() && self.background.is_some())
    }

    pub fn get_frame(&self) -> Option<(&Color, f64)> {
        self.border.map(|thickness| (&self.color, thickness))
    }

    pub fn get_color(&self) -> &Color {
        if let Some((fg, _)) = self.get_frame() {
            return if let Some(bg) = &self.background {
                bg
            } else {
                fg
            };
        }
        &self.color
    }
}
