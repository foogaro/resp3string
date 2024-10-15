// src/resp3/protocol.rs

// RESP3 protocol symbols
pub const SIMPLE_STRING_PREFIX: &str = "+";
pub const BULK_STRING_PREFIX: &str = "$";
pub const ERROR_PREFIX: &str = "-";
pub const ARRAY_PREFIX: &str = "*";

// Line endings
pub const CR: &str = "\r";
pub const LF: &str = "\n";
pub const CRLF: &str = "\r\n";
