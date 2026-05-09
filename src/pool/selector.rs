#[derive(Debug, Default)]
pub struct StickyKeySelector {
    current_id: Option<String>,
}

impl StickyKeySelector {
    pub fn new() -> Self {
        Self { current_id: None }
    }

    pub fn current_id(&self) -> Option<&String> {
        self.current_id.as_ref()
    }

    pub fn set_current(&mut self, id: String) {
        self.current_id = Some(id);
    }

    pub fn reset(&mut self) {
        self.current_id = None;
    }
}
