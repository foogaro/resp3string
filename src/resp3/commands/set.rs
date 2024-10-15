use crate::resp3::utils::command::Command;

pub struct SetCommand {
    key: String,
    value: String,
}

impl SetCommand {
    pub fn new(key: String, value: String) -> Self {
        SetCommand { key, value }
    }
}

impl Command for SetCommand {
    fn get_parts(&self) -> Vec<&str> {
        vec!["SET", &self.key, &self.value]
    }
}
