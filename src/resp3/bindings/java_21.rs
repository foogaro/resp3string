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
