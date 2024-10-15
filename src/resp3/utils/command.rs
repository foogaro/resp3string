use crate::resp3::utils::redis_connection::RedisConnection;

pub trait Command {

    fn process_command(&self, conn: &mut RedisConnection) -> String {
        let formatted_command = self.format_resp_command();
        conn.send_command(&formatted_command)
    }

    fn format_resp_command(&self) -> String {
        let parts: Vec<&str> = self.get_parts();
        let mut resp_command = format!("*{}\r\n", parts.len());

        for part in parts {
            resp_command.push_str(&format!("${}\r\n{}\r\n", part.len(), part.to_uppercase()));
        }

        resp_command
    }

    fn get_parts(&self) -> Vec<&str>;
}
