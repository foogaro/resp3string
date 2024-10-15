// src/lib.rs

pub mod resp3 {
    pub mod bindings {
        #[cfg(feature = "java")]
        mod java_21;
        #[cfg(feature = "python")]
        mod python_3;
    }
    pub mod commands {
        pub mod get;
        pub mod set;
    }
    pub mod utils {
        pub mod command;
        pub mod command_executor;
        pub mod redis_connection;
    }
    pub mod protocol;
}

#[cfg(test)]
mod tests {
    use crate::resp3::commands::get::GetCommand;
    use crate::resp3::commands::set::SetCommand;
    use crate::resp3::protocol::{BULK_STRING_PREFIX, CRLF};
    use crate::resp3::utils::command::Command;
    use crate::resp3::utils::redis_connection::RedisConnection;

    #[test]
    fn a_test_set_command() {
        // Step 1: Set up the Redis connection (ensure Redis is running on localhost:6379)
        let mut conn = RedisConnection::new("127.0.0.1:6379");

        let test_key = "test_key";
        let test_value = "test_value";

        // Step 2: Create and execute the SetCommand for a specific key-value pair
        let set_command = SetCommand::new(test_key.to_string(), test_value.to_string());
        let set_response = set_command.process_command(&mut conn);

        // Step 3: Assert that the SetCommand response is +OK (successful Redis SET response)
        assert_eq!(set_response.trim(), "+OK");
    }

    #[test]
    fn b_test_get_command() {
        // Step 1: Set up the Redis connection (ensure Redis is running on localhost:6379)
        let mut conn = RedisConnection::new("127.0.0.1:6379");

        let test_key = "test_key";
        let test_value = "test_value";

        // Step 2: Create and execute the GetCommand for the same key
        let get_command = GetCommand::new(test_key.to_string());
        let get_response = get_command.process_command(&mut conn);

        // Step 3: Assert that the GetCommand response is the expected value
        assert_eq!(get_response.trim(), String::new() + BULK_STRING_PREFIX + &test_value.len().to_string() + CRLF + &test_value.to_uppercase());
    }

    #[test]
    fn c_test_set_and_get_command() {
        // Step 1: Set up the Redis connection (ensure Redis is running on localhost:6379)
        let mut conn = RedisConnection::new("127.0.0.1:6379");

        let test_key = "test_key";
        let test_value = "test_value";

        // Step 2: Create and execute the SetCommand for a specific key-value pair
        let set_command = SetCommand::new(test_key.to_string(), test_value.to_string());
        let set_response = set_command.process_command(&mut conn);

        // Step 3: Assert that the SetCommand response is +OK (successful Redis SET response)
        assert_eq!(set_response.trim(), "+OK");

        // Step 4: Create and execute the GetCommand for the same key
        let get_command = GetCommand::new(test_key.to_string());
        let get_response = get_command.process_command(&mut conn);

        // Step 5: Assert that the GetCommand response is the expected value
        assert_eq!(get_response.trim(), String::new() + BULK_STRING_PREFIX + &test_value.len().to_string() + CRLF + &test_value.to_uppercase());
    }
}
