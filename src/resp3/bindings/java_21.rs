extern crate jni;

use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use std::ffi::CString;

// JNI wrapper to create RedisConnection in Java
#[no_mangle]
pub extern "C" fn createRedisConnection(
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
pub extern "C" fn createSetCommand(
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
pub extern "C" fn executeSetCommand(
    env: JNIEnv,
    _class: JClass,
    set_command_ptr: jlong,
    conn_ptr: jlong,
) -> jstring {
    let set_command: &SetCommand = unsafe { &*(set_command_ptr as *mut SetCommand) };
    let conn: &mut RedisConnection = unsafe { &mut *(conn_ptr as *mut RedisConnection) };

    let result = set_command.execute(conn);
    let output = CString::new(result).expect("CString::new failed");
    env.new_string(output.to_str().unwrap()).expect("Couldn't create Java string!").into_inner()
}

// JNI wrapper to free RedisConnection memory in Java
#[no_mangle]
pub extern "C" fn freeRedisConnection(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) {
    if conn_ptr != 0 {
        unsafe {
            Box::from_raw(conn_ptr as *mut RedisConnection);
        }
    }
}

// JNI wrapper to free SetCommand memory in Java
#[no_mangle]
pub extern "C" fn freeSetCommand(
    _env: JNIEnv,
    _class: JClass,
    set_command_ptr: jlong,
) {
    if set_command_ptr != 0 {
        unsafe {
            Box::from_raw(set_command_ptr as *mut SetCommand);
        }
    }
}
