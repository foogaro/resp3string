use crate::resp3::utils::command::Command;
use crate::resp3::utils::redis_connection::RedisConnection;

pub struct CommandExecutor {
    conn: RedisConnection,
}

impl CommandExecutor {
    pub fn new(address: &str) -> Self {
        let conn = RedisConnection::new(address);
        CommandExecutor { conn }
    }

    pub fn execute<T: Command>(&mut self, command: T) -> String {
        command.process_command(&mut self.conn)
    }
}
