#[derive(Debug)]
pub struct Options {
    pub width: usize,
    pub height: usize,
    pub max_depth: u8,
    pub fov: usize,
    pub file_name: String,
    pub window_title: String,
}
