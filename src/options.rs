use window;

pub struct Options {
    pub iterations: usize,
    pub window:     window::Window
}

impl Default for Options {
    fn default() -> Self {
        Options {
            iterations: 250,
            window:     window::Window::default()
        }
    }
}
