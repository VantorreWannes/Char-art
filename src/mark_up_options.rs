use std::{path::Path, fs, io};

use image::Luma;
use rusttype::{Font, Scale};

#[derive(Debug, Clone)]
pub struct MarkUpOptions<'a > {
    font: Option<Font<'a>>,
    scale: Option<Scale>,
    color: Option<Luma<u8>>,
}

impl MarkUpOptions<'_> {
    pub fn new() -> Self {
        Self {
            font: None,
            scale: None,
            color: None,
        }
    }

    pub fn set_font(mut self, font: &Path) -> io::Result<Self> {
        let font = fs::read(font)?;
        self.font = Some(Font::try_from_vec(font).ok_or::<io::Error>(io::ErrorKind::InvalidData.into())?);
        Ok(self)
    }

    pub fn set_scale(mut self, scale: f32) ->Self {
        self.scale = Some(Scale::uniform(scale));
        self
    }
    pub fn set_color(mut self, color: u8) -> Self {
        self.color = Some(Luma([color]));
        self
    }

    pub fn get_values(&self) -> (Font, Scale, Luma<u8>) {
        let font: Font = self.font.as_ref().unwrap_or(&Font::try_from_bytes(include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf")).unwrap()).to_owned();
        let scale = self.scale.unwrap_or(Scale::uniform(12.0));
        let color = self.color.unwrap_or(Luma([255]));
        (font, scale, color)
    }
}

impl Default for MarkUpOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}
