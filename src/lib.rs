use luajit::ffi::lua_State;
use luajit::{State, c_int};

pub mod kafka_error;
mod protocol;
use protocol::codecs::{FromByte, ToByte};
use protocol::header::{RequestHeader};
use protocol::api_versions::{ApiVersionsResponse};

#[macro_use]
extern crate error_chain;

include!("bindings/module.rs");

use std::io::{Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: &mut TcpStream) {
    let mut size: i32 = 0;
    size.decode(&mut stream).unwrap();
    println!("Size {}", size);

    let hr: RequestHeader = RequestHeader::new(&mut stream);
    println!("hr {:?}", hr);

    if hr.api_key == 18 {
        let api_keys_response = ApiVersionsResponse::new(hr.correlation_id, 1, 1);

        let mut temp_buf = vec![];
        api_keys_response.encode(& mut temp_buf).unwrap();

        let size: i32 = temp_buf.len() as i32;

        println!("Temp buf {:?}", temp_buf);
        println!("Size of resp {}", size);
        size.encode(stream).unwrap();

        stream.write(&temp_buf[..]).unwrap();
    }
}

#[no_mangle]
pub extern fn brod_listen(l: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(l);
    let addr = state.to_str(1);
    let addr = match addr {
        Some(addr) => addr,
        None => panic!("No args received"),
    };

    let listener = TcpListener::bind(addr);
    let listener = match listener {
        Ok(listener) => listener,
        Err(_) => panic!(),
    };

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => panic!(),
        };

        handle_client(&mut stream);
    }

    0
}

#[no_mangle]
pub extern fn box_info(l: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(l);
    state.do_string(r#"box.cfg{}"#);
    state.do_string(r#"print(box.info())"#);
    0
}

#[no_mangle]
pub fn luaopen_brod(l: *mut lua_State) -> c_int
{
    let mut state = State::from_ptr(l);

    state.register("listen", brod_listen);
    state.register("box_info", box_info);
    return 1;
}
