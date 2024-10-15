use crate::resp3::utils::command::Command;

pub struct GetCommand {
    key: String,
}

impl GetCommand {
    pub fn new(key: String) -> Self {
        GetCommand { key }
    }
}

impl Command for GetCommand {
    fn get_parts(&self) -> Vec<&str> {
        vec!["GET", &self.key]
    }
}
