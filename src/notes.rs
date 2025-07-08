

pub struct RusticNote {
    pub content: String,
}

impl RusticNote {
    pub fn new(content: String) -> Self {
        RusticNote { content }
    }
}

// todo move to a separate file
pub fn save_note(note: &RusticNote) {
    println!("Saving note: {}", note.content);
}
