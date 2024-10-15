# resp3string

RESP, which stands for *RE*dis *S*erialization *P*rotocol, is the communication protocol used by clients to interact with a Redis server.

However, this repository isn’t focused on Redis itself. Instead, it’s an experiment aimed at building a single core library with bindings for multiple programming languages, all sharing the same implementation.

In my experience, each programming language tends to have its own library for implementing core logic. This leads to a proliferation of libraries that aren’t always aligned—some have features that others lack, which can be frustrating.

I wanted to explore how difficult it would be to implement the core logic once and expose it through bindings for various languages. Surprisingly, it's not particularly difficult—it’s more tedious than anything—but the effort is worth it for improving the developer experience.

Since I work for Redis, it’s hard to imagine running an experiment that isn’t related to it. That’s why my first experiment focuses on Redis, specifically on implementing the SET command. It’s a very basic version, without the TTL option and all, but it does have one twist: both the key and value are automatically transformed to UPPERCASE (that's why I'm not a designer).

# Core logic

The implementation should be highly portable, allowing it to run efficiently across various platforms and environments without needing significant changes to the codebase.
With that in mind, I focused on languages known for their portability and performance, like Golang and Rust, both of which are ideal for building cross-platform libraries that maintain speed and reliability.
Since I’ve already worked with Golang but haven’t had the chance to explore Rust yet, I decided to go with Rust to start learning something new.

## Rust

As I'm new to Rust, an introduction from my side would be arrogant, so I better lead you to the Rust site:
- [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Design
I’m a strong believer in the **SOLID** principles, and I’ve applied them to structure my code accordingly.

### Single Responsibility Principle (SRP)
Each command is responsible for handling only its specific logic. For each Redis command, there is a dedicated command object:
- The `SetCommand` object handles the Redis SET command;
- The `GetCommand` object handles the Redis GET command;
- And so on for other commands.

### Open/Closed Principle (OCP)
A generic `Command` object defines common methods and default implementations. This allows for easy extension, enabling new commands to be added without modifying the existing structure.

### Dependency Injection
For connection management, responsibility is externalized to a higher-level component, ensuring loose coupling and flexibility.
A `RedisConnection` object will manage the connection to the Redis server. This connection is utilized by a `CommandExecutor` object, which ensures that each specific command object, such as `SetCommand`, has seamless access to the connection when executing commands. This design centralizes connection management, allowing for efficient and consistent access across all command implementations.

## Implementation details

The goal is to build a modular library, which I believe is the most effective approach for this project.
To achieve this, the library will be structured as follows:

```rust
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
```

This modular setup allows for clean separation of concerns, making the library easier to maintain and extend. Each module focuses on a specific area, from bindings for different languages to core command implementations and utility functions.

### RedisConnection
```rust
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str;

pub struct RedisConnection {
    stream: TcpStream,
}

impl RedisConnection {
    pub fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address).expect("Could not connect to Redis server");
        RedisConnection { stream }
    }

    pub fn send_command(&mut self, command: &str) -> String {
        self.stream.write_all(command.as_bytes()).expect("Failed to write to Redis server");
        self.stream.flush().expect("Failed to flush the stream");

        let mut buffer = [0; 512];
        let bytes_read = self.stream.read(&mut buffer).expect("Failed to read from Redis server");
        let response = str::from_utf8(&buffer[..bytes_read]).expect("Failed to parse Redis response");
        response.to_string()
    }
    pub fn close(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }
}

impl Drop for RedisConnection {
    fn drop(&mut self) {
        println!("Dropping RedisConnection...");
        self.close();
    }
}
```

### Command
```rust
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
```

### CommandExecutor
```rust
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
```

### SetCommand

```rust
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
```

### GetCommand

```rust
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
```

# Bindings

Bindings are a way to allow code written in one programming language to be used by another language.
In the context of my library, bindings are essential for making the core functionality, written in Rust, accessible to different languages like Java and Python.

Bindings are useful when you want to write the core logic once and make it available across multiple platforms or languages.
This avoids the need to rewrite the same logic in different languages, reducing duplication and ensuring that the functionality remains consistent across all implementations.

In my case, I'm creating modules like `java_21` and `python_3` under the `bindings` directory.
These modules will contain the code necessary to expose your Rust library's functionality to Java and Python.
Using tools like JNI (Java Native Interface) for Java and PyO3 for Python, I create these bindings so that using my library in Java or Python can call the underlying Rust functions without worrying about the details of Rust itself.

## Java binding
```java
extern crate jni;

use crate::resp3::commands::get::GetCommand;
use crate::resp3::commands::set::SetCommand;
use crate::resp3::utils::redis_connection::RedisConnection;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use std::ffi::CString;
use crate::resp3::utils::command::Command;

// JNI wrapper to create RedisConnection in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_createRedisConnection(
    env: JNIEnv,
    _class: JClass,
    address: JString,
) -> jlong {
    let addr: String = env.get_string(address).expect("Couldn't get Java string!").into();
    let conn = Box::new(RedisConnection::new(&addr));
    Box::into_raw(conn) as jlong
}

// JNI wrapper to create SetCommand in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_createSetCommand(
    env: JNIEnv,
    _class: JClass,
    key: JString,
    value: JString,
) -> jlong {
    let key: String = env.get_string(key).expect("Couldn't get Java string!").into();
    let value: String = env.get_string(value).expect("Couldn't get Java string!").into();
    let set_command = Box::new(SetCommand::new(key, value));
    Box::into_raw(set_command) as jlong
}

// JNI wrapper to execute SetCommand in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_executeSetCommand(
    env: JNIEnv,
    _class: JClass,
    set_command_ptr: jlong,
    conn_ptr: jlong,
) -> jstring {
    let set_command: &SetCommand = unsafe { &*(set_command_ptr as *mut SetCommand) };
    let conn: &mut RedisConnection = unsafe { &mut *(conn_ptr as *mut RedisConnection) };

    let result = set_command.process_command(conn);
    let output = CString::new(result).expect("CString::new failed");
    env.new_string(output.to_str().unwrap()).expect("Couldn't create Java string!").into_inner()
}

// JNI wrapper to create GetCommand in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_createGetCommand(
    env: JNIEnv,
    _class: JClass,
    key: JString,
) -> jlong {
    let key: String = env.get_string(key).expect("Couldn't get Java string!").into();
    let get_command = Box::new(GetCommand::new(key));
    Box::into_raw(get_command) as jlong
}

// JNI wrapper to execute GetCommand in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_executeGetCommand(
    env: JNIEnv,
    _class: JClass,
    get_command_ptr: jlong,
    conn_ptr: jlong,
) -> jstring {
    let get_command: &GetCommand = unsafe { &*(get_command_ptr as *mut GetCommand) };
    let conn: &mut RedisConnection = unsafe { &mut *(conn_ptr as *mut RedisConnection) };

    let result = get_command.process_command(conn);
    let output = CString::new(result).expect("CString::new failed");
    env.new_string(output.to_str().unwrap()).expect("Couldn't create Java string!").into_inner()
}

// JNI wrapper to free RedisConnection memory in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_freeRedisConnection(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) {
    if conn_ptr != 0 {
        unsafe {
            let _ = Box::from_raw(conn_ptr as *mut RedisConnection);
        }
    }
}

// JNI wrapper to free SetCommand memory in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_freeSetCommand(
    _env: JNIEnv,
    _class: JClass,
    set_command_ptr: jlong,
) {
    if set_command_ptr != 0 {
        unsafe {
            let _ = Box::from_raw(set_command_ptr as *mut SetCommand);
        }
    }
}

// JNI wrapper to free GetCommand memory in Java
#[no_mangle]
pub extern "C" fn Java_TestRedis_freeGetCommand(
    _env: JNIEnv,
    _class: JClass,
    get_command_ptr: jlong,
) {
    if get_command_ptr != 0 {
        unsafe {
            let _ = Box::from_raw(get_command_ptr as *mut GetCommand);
        }
    }
}
```

## Python binding
```python
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
```

## Advantages
- Consistency: You only need to implement your core logic once in Rust, and it can be used by multiple languages.
- Performance: Since Rust is known for high performance, using Rust bindings in other languages can improve performance compared to native implementations in those languages.
- Maintainability: Bugs and features can be fixed or added in one place (the Rust code), and all supported languages benefit from the updates immediately.

By organizing my library this way, I'm ensuring that it's extensible and ready to integrate with different programming ecosystems through bindings.

# Build

The build process involves more than just compiling the Rust code; it also includes generating the necessary bindings. Unfortunately, combining multiple bindings in one build can cause interoperability issues, where objects from all bindings are exposed to every supported programming language.

The best approach is to perform a separate build for each binding. This ensures that only the relevant objects are available for the specific programming language, avoiding any conflicts or unwanted cross-language exposure.

## Java

```bash
cargo clean
cargo build --release --features java
```

This process generates the `libresp3string.so` library in the `target/release` folder. This library is the key component that needs to be loaded by the Java runtime in order to interface with and execute the Rust code. By loading this shared library, the Java application can call the underlying Rust functionality, enabling seamless integration between the two languages.

Here’s what you need to do on the Java side to interact with the Rust library:

```java
public class TestRedis {
    static {
        System.loadLibrary("resp3string");
    }

    public native long createRedisConnection(String address);
    public native long createSetCommand(String key, String value);
    public native String executeSetCommand(long setCommandPtr, long connPtr);
    public native void freeRedisConnection(long connPtr);
    public native void freeSetCommand(long setCommandPtr);

    public static void main(String[] args) {
        TestRedis rustRedis = new TestRedis();

        long conn = rustRedis.createRedisConnection("127.0.0.1:6379");

        long setCommand = rustRedis.createSetCommand("mykey", "myvalue");

        String response = rustRedis.executeSetCommand(setCommand, conn);
        System.out.println("Response from Redis: " + response);

        rustRedis.freeSetCommand(setCommand);
        rustRedis.freeRedisConnection(conn);
    }
}
```

This code integrates the Rust library into Java. It sets up a Redis connection, creates and executes a SET command, and then cleans up the resources. Now, to compile and run the Java program, use the following command:

```bash
java -Djava.library.path=target/release TestRedis
```

When executed, you should see a response similar to this:

```bash
Response from Redis: +OK

Dropping RedisConnection...
```

You can verify the results in Redis using redis-cli or a tool like RedisInsight. Here's how you can check if the key and value have been set:

```sql
> scan 0
1) "0"
2) 1) "MYKEY"

> get MYKEY
"MYVALUE"
```

As expected, the key MYKEY and value MYVALUE are stored in uppercase, just as defined by the Rust implementation.

The same process applies when using Python.


## Python

```bash
cargo clean
maturin develop --bindings pyo3
```

This process generates the `resp3string.cpython-39-x86_64-linux-gnu.so` library in the `env/lib/python3.9/site-packages/resp3string` folder. This library is the key component that needs to be loaded by the Python interpreter in order to interface with and execute the Rust code.

Here’s what you need to do on the Java side to interact with the Rust library:

```python
from resp3string import PyRedisConnection, PySetCommand, PyGetCommand

# Create a connection to Redis
conn = PyRedisConnection("127.0.0.1:6379")

# Send a command to Redis (e.g., PING)
response = conn.send_command("PING\r\n")

# Print the response from Redis
print(f"Response from Redis: {response}")

# Create a SetCommand to set a key-value pair in Redis
set_command = PySetCommand("mykey", "myvalue")

# Execute the SetCommand using the connection
response = set_command.execute(conn)

# Print the response from Redis
print(f"Response from Redis: {response}")

# Create a GetCommand to get a of a key fromRedis
get_command = PyGetCommand("mykey")

# Execute the GetCommand using the connection
response = get_command.execute(conn)

# Print the response from Redis
print(f"Response from Redis: {response}")
```

This code integrates the Rust library into Python. It sets up a Redis connection, creates and executes a PING command, a SET command, and a GET command.
Then cleans up the resources. Now, to run the Python script, use the following command:

```bash
python test_redis.py
```

When executed, you should see a response similar to this:

```bash
Response from Redis: +PONG

Response from Redis: +OK

Response from Redis: $7
MYVALUE

Dropping RedisConnection...
```

And once again, the key MYKEY and value MYVALUE are stored in uppercase, just as defined by the Rust implementation.

