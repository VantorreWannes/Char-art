use image::Luma;
use rusttype::{Font, Scale};

#[derive(Debug, Clone)]
pub struct AsStringOptions {
    shrink: Option<u32>,
    darken: Option<u8>,
}

impl AsStringOptions {
    pub fn new() -> Self {
        Self {
            shrink: None,
            darken: None,
        }
    }
    pub fn set_shrink(mut self, shrink: u32) -> Self {
        self.shrink = Some(shrink);
        self
    }

    pub fn set_darken(mut self, darken: u8) -> Self {
        self.darken = Some(darken);
        self
    }

    pub fn get_values(&self) -> (u32, u8) {
        let darken = self.darken.unwrap_or(0);
        let shrink = self.shrink.unwrap_or(1);
        (shrink, darken)
    }
}

impl Default for AsStringOptions {
    fn default() -> Self {
        Self::new()
    }
}
