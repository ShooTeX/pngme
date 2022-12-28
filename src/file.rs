use crate::png::Png;

#[derive(Clone, Debug)]
pub struct File {
    png: Png,
    path: String,
}

impl File {
    pub fn new(png: Png, path: String) -> Self {
        Self { png, path }
    }

    pub fn png(&self) -> &Png {
        &self.png
    }

    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
}
