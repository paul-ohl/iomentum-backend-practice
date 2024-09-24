pub struct Controller;

impl Controller {
    pub fn new() -> Self {
        Controller
    }

    pub fn health(&self) -> String {
        "OK".to_string()
    }
}
