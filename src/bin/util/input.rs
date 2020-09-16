



pub enum InputMode {
    Normal,
    Editing,
}


/// App holds the state of the application
pub struct InputUI {
    /// Current value of the input box
    pub input_message: String,
    /// Current input mode
    pub input_mode: InputMode,
}


impl Default for InputUI {
    fn default() -> InputUI {
        InputUI {
            input_message: String::new(),
            input_mode: InputMode::Normal,
        }
    }
}
