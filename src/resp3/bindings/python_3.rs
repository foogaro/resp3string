mod python_bindings {
    use std::net::TcpStream;
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;
    use crate::resp3::commands::get::GetCommand;
    use crate::resp3::commands::set::SetCommand;
    use crate::resp3::utils::command::Command;
    use crate::resp3::utils::redis_connection::RedisConnection;

    #[pyclass]
    pub struct PyRedisConnection {
        conn: RedisConnection,
    }

    #[pymethods]
    impl PyRedisConnection {
        #[new]
        pub fn new(address: &str) -> PyResult<Self> {
            match TcpStream::connect(address) {
                Ok(_) => Ok(PyRedisConnection {
                    conn: RedisConnection::new(address), // Use the Rust implementation
                }),
                Err(_) => Err(PyRuntimeError::new_err("Failed to connect to Redis server")),
            }
        }

        pub fn send_command(&mut self, command: &str) -> PyResult<String> {
            // Use the Rust method to send a command and get a response
            Ok(self.conn.send_command(command))
        }

        pub fn close(&mut self) {
            self.conn.close();
        }
    }

    #[pyclass]
    pub struct PySetCommand {
        command: SetCommand,
    }

    #[pymethods]
    impl PySetCommand {
        #[new]
        pub fn new(key: String, value: String) -> Self {
            PySetCommand {
                command: SetCommand::new(key, value),
            }
        }

        pub fn execute(&self, conn: &mut PyRedisConnection) -> PyResult<String> {
            Ok(self.command.process_command(&mut conn.conn))
        }
    }

    #[pyclass]
    pub struct PyGetCommand {
        command: GetCommand,
    }

    #[pymethods]
    impl PyGetCommand {
        #[new]
        pub fn new(key: String) -> Self {
            PyGetCommand {
                command: GetCommand::new(key),
            }
        }

        pub fn execute(&self, conn: &mut PyRedisConnection) -> PyResult<String> {
            Ok(self.command.process_command(&mut conn.conn))
        }
    }

    #[pymodule]
    fn resp3string(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<PyRedisConnection>()?;
        m.add_class::<PySetCommand>()?;
        m.add_class::<PyGetCommand>()?;
        Ok(())
    }
}