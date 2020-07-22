pub mod kafka_error;
mod protocol;

use protocol::codecs::{FromByte};
use protocol::header::{RequestHeader};

#[macro_use]
extern crate error_chain;

include!("bindings/module.rs");

use std::os::raw::c_char;
use std::ffi::CStr;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: &mut TcpStream) {
    let mut size: i32 = 0;
    size.decode(&mut stream).unwrap();
    println!("Size {}", size);

    let hr: RequestHeader = RequestHeader::new(&mut stream);
    println!("hr {:?}", hr);
}

#[no_mangle]
pub extern fn brod_listen(c_ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(c_ptr) };
    let addr = c_str.to_str();
    let addr = match addr {
        Ok(addr) => addr,
        Err(_) => panic!(),
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
}
