include!("bindings/module.rs");
use std::os::raw::c_char;
use std::ffi::CStr;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

use luajit::{c_int, State, lua_fn};

fn handle_client(stream: &mut TcpStream) {
    let res = stream.write("Hello there, General Kenobi!\n".as_bytes());
    match res {
        Ok(res) => res,
        Err(e) => panic!(e),
    };
}

#[no_mangle]
pub extern fn hello() {
    println!("Hello there, General Kenobi!");
}

#[no_mangle]
pub extern fn rustproc(fiber_id: u64) {
    for _ in 0..5 {
        println!("rust fiber {}: before sleep", fiber_id);
        unsafe {
            fiber_sleep(0.1);
        }
        println!("rust fiber {}: after sleep", fiber_id);
    }
}

fn return_42(state: &mut State) -> c_int {
    state.push(42);

    1
}

#[no_mangle]
pub extern fn lua() {
    let mut state = State::new();
    state.open_libs();
    state.do_string(r#"print("Hello world!")"#);

    state.push(lua_fn!(return_42));
    state.set_global("return_42");
    state.do_string(r#"print(return_42())"#);
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