/*
 * Malamute - Rust ffi bindings
 *
 * This is experimental code aims to teach me how to integrate Rust with ZeroMQ/CLASS/zproject
 * based project well.
 *
 * Licensed under MIT license
 *
 */
#![allow (dead_code)]
#![feature(cstr_from_bytes)]

extern crate libc;

use std::ptr;
use libc::{c_char, c_int, c_void, uint32_t};

#[allow (non_camel_case_types)]
pub enum mlm_client_t {}
#[allow (non_camel_case_types)]
pub enum zmsg_t {}
#[allow (non_camel_case_types)]
pub enum zactor_t {}
#[allow (non_camel_case_types)]
pub enum zsock_t {}

#[link (name = "mlm")]
extern {

    pub fn mlm_client_new () -> *mut mlm_client_t;
    pub fn mlm_client_destroy (self_p: *mut *mut mlm_client_t);
    pub fn mlm_client_actor (_self: *mut mlm_client_t) -> *mut zactor_t;
    pub fn mlm_client_msgpipe (_self: *mut mlm_client_t) -> *mut zsock_t;
    pub fn mlm_client_connected (_self: *mut mlm_client_t) -> bool;
    pub fn mlm_client_connect (_self: *mut mlm_client_t, endpoint: *const c_char, timeout: uint32_t, address : *const c_char) -> c_int;
    pub fn mlm_client_set_plain_auth (_self: *mut mlm_client_t) -> c_int;
    pub fn mlm_client_set_producer (_self: *mut mlm_client_t, stream: *const c_char) -> c_int;
    pub fn mlm_client_set_consumer (_self: *mut mlm_client_t, stream: *const c_char, pattern: *const c_char) -> c_int;
    pub fn mlm_client_set_worker (_self: *mut mlm_client_t, address: *const c_char, pattern: *const c_char) -> c_int;
    pub fn mlm_client_send (_self: *mut mlm_client_t, subject: *const c_char, content: *mut *mut zmsg_t) -> c_int;
    pub fn mlm_client_sendto (_self: *mut mlm_client_t, address: *const c_char, subject: *const c_char, tracker: *const c_char, timeout: uint32_t, content: *mut *mut zmsg_t) -> c_int;
    pub fn mlm_client_sendfor (_self: *mut mlm_client_t, address: *const c_char, subject: *const c_char, tracker: *const c_char, timeout: uint32_t, content: *mut *mut zmsg_t) -> c_int;
    pub fn mlm_client_recv (_self: *mut mlm_client_t) -> *mut zmsg_t;
    pub fn mlm_client_command (_self: *mut mlm_client_t) -> *const c_char;
    pub fn mlm_client_status (_self: *mut mlm_client_t) -> c_int;
    pub fn mlm_client_reason (_self: *mut mlm_client_t) -> *const c_char;
    pub fn mlm_client_address (_self: *mut mlm_client_t) -> *const c_char;
    pub fn mlm_client_sender (_self: *mut mlm_client_t) -> *const c_char;
    pub fn mlm_client_subject (_self: *mut mlm_client_t) -> *const c_char;
    pub fn mlm_client_content (_self: *mut mlm_client_t) -> *mut zmsg_t;
    pub fn mlm_client_tracker (_self: *mut mlm_client_t) -> *const c_char;
    pub fn mlm_client_set_verbose (_self: *mut mlm_client_t, verbose: bool);

    pub fn mlm_server (pipe: *mut zsock_t, args: *mut c_void);

}

#[link (name = "czmq")]
extern {
    pub fn zmsg_new () -> *mut zmsg_t;
    pub fn zmsg_print (_self: *mut zmsg_t);
    pub fn zmsg_destroy (self_p: *mut *mut zmsg_t);
    pub fn zmsg_addstrf (_self: *mut zmsg_t, string: *const c_char);

    pub fn zactor_new (task: unsafe extern fn (pipe: *mut zsock_t, args: *mut c_void), args: *mut c_void) -> *mut zactor_t;
    pub fn zactor_destroy (self_p : *mut *mut zactor_t);

    pub fn zstr_sendx (dest: *mut c_void, string: *const c_char, ...) -> c_int;

    pub static mut zsys_interrupted: c_int;
}

// Rusty API
pub struct MlmClient {
    _ptr : *mut mlm_client_t,
}

impl MlmClient {
    pub fn new () -> Self {
        let ptr: *mut mlm_client_t;
        unsafe {
            ptr = mlm_client_new ();
            assert! (!ptr.is_null ());
        }
        MlmClient {_ptr: ptr}
    }

    pub unsafe fn raw_mut (&self) -> *mut mlm_client_t {
        self._ptr
    }
}

impl Drop for MlmClient {
    fn drop (&mut self) {
        unsafe {
            mlm_client_destroy (&mut self._ptr as *mut *mut mlm_client_t);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn safe_test() {

        use super::*;
        use std::ptr;
        use std::ffi;
        use libc::{c_void};

        //http://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr
        use std::io::Write;

        macro_rules! println_stderr(
            ($($arg:tt)*) => { {
                let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
                r.expect("failed printing to stderr");
            } }
        );

        let rust_client1 = MlmClient::new ();
        unsafe {
            println_stderr! ("BAF1");
            // constants
            let endpoint = ffi::CStr::from_bytes_with_nul (b"ipc://malamute-rust-ffi-safe\0").unwrap ();

            let BIND = ffi::CStr::from_bytes_with_nul (b"BIND\0").unwrap ();
            let VERBOSE = ffi::CStr::from_bytes_with_nul (b"VERBOSE\0").unwrap ();
            let STREAM = ffi::CStr::from_bytes_with_nul (b"STREAM\0").unwrap ();

            println_stderr! ("BAF2");
            // Malamute broker
            let mut server: *mut zactor_t = zactor_new (mlm_server, ptr::null_mut ());
            zstr_sendx (server as *mut c_void, BIND.as_ptr (), endpoint.as_ptr (), ptr::null_mut () as *mut c_void);
            zstr_sendx (server as *mut c_void, VERBOSE.as_ptr (), ptr::null_mut () as *mut c_void);

            println_stderr! ("BAF3");
            // Client 1
            let mut client1 : *mut mlm_client_t = rust_client1.raw_mut ();
            println_stderr! ("BAF4");
            let mut r = mlm_client_connect (client1,
                endpoint.as_ptr (),
                5000,
                ffi::CStr::from_bytes_with_nul (b"rust-ffi-client-1\0").unwrap ().as_ptr ());
            println_stderr! ("BAF5");
            assert! (r != -1);

            r = mlm_client_set_producer (
                client1,
                STREAM.as_ptr ()
                );
            assert! (r != -1);

            // Client 2
            let mut client2 : *mut mlm_client_t = mlm_client_new ();
            let mut r = mlm_client_connect (client2,
                endpoint.as_ptr (),
                5000,
                ffi::CStr::from_bytes_with_nul (b"rust-ffi-client-2\0").unwrap ().as_ptr ());
            assert! (r != -1);

            r = mlm_client_set_consumer (
                client2,
                STREAM.as_ptr (),
                ffi::CStr::from_bytes_with_nul (b".*\0").unwrap ().as_ptr ()
                );
            assert! (r != -1);

            let mut counter = 0;
            while zsys_interrupted == 0 {
                let mut msg = zmsg_new ();
                zmsg_addstrf (msg,
                    ffi::CStr::from_bytes_with_nul (b"Hello from rust-ffi\0").unwrap ().as_ptr ());
                r = mlm_client_send (
                    client1,
                    ffi::CStr::from_bytes_with_nul (b"[SUBJECT\0").unwrap ().as_ptr (),
                    &mut msg as *mut *mut zmsg_t
                    );
                let mut msg = mlm_client_recv (client2);

                println_stderr! (">>>>>>>>>>>>>>>>>>>sender={:?}\nsubject={:?}",
                    ffi::CStr::from_ptr (mlm_client_sender (client2)),
                    ffi::CStr::from_ptr (mlm_client_subject (client2))
                    );

                zmsg_print (msg);
                zmsg_destroy (&mut msg as *mut *mut zmsg_t);

                counter += 1;
                if counter > 10 {
                    break;
                }
            }

            //mlm_client_destroy (&mut client1 as *mut *mut mlm_client_t);
            mlm_client_destroy (&mut client2 as *mut *mut mlm_client_t);
            zactor_destroy (&mut server as *mut *mut zactor_t);
        }
        unsafe {
        println! ("rust_client1.ptr = {:?}", rust_client1.raw_mut ());
        };
    }

    #[test]
    fn unsafe_test() {

        use super::*;
        use std::ptr;
        use std::ffi;
        use libc::{c_void};

        //http://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr
        use std::io::Write;

        macro_rules! println_stderr(
            ($($arg:tt)*) => { {
                let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
                r.expect("failed printing to stderr");
            } }
        );

        unsafe {
            // constants
            let endpoint = ffi::CStr::from_bytes_with_nul (b"ipc://malamute-rust-ffi-unsafe\0").unwrap ();

            let BIND = ffi::CStr::from_bytes_with_nul (b"BIND\0").unwrap ();
            let VERBOSE = ffi::CStr::from_bytes_with_nul (b"VERBOSE\0").unwrap ();
            let STREAM = ffi::CStr::from_bytes_with_nul (b"STREAM\0").unwrap ();

            // Malamute broker
            let mut server: *mut zactor_t = zactor_new (mlm_server, ptr::null_mut ());
            zstr_sendx (server as *mut c_void, BIND.as_ptr (), endpoint.as_ptr (), ptr::null_mut () as *mut c_void);
            zstr_sendx (server as *mut c_void, VERBOSE.as_ptr (), ptr::null_mut () as *mut c_void);

            // Client 1
            let mut client1 : *mut mlm_client_t = mlm_client_new ();
            let mut r = mlm_client_connect (client1,
                endpoint.as_ptr (),
                5000,
                ffi::CStr::from_bytes_with_nul (b"rust-ffi-client-1\0").unwrap ().as_ptr ());
            assert! (r != -1);

            r = mlm_client_set_producer (
                client1,
                STREAM.as_ptr ()
                );
            assert! (r != -1);

            // Client 2
            let mut client2 : *mut mlm_client_t = mlm_client_new ();
            let mut r = mlm_client_connect (client2,
                endpoint.as_ptr (),
                5000,
                ffi::CStr::from_bytes_with_nul (b"rust-ffi-client-2\0").unwrap ().as_ptr ());
            assert! (r != -1);

            r = mlm_client_set_consumer (
                client2,
                STREAM.as_ptr (),
                ffi::CStr::from_bytes_with_nul (b".*\0").unwrap ().as_ptr ()
                );
            assert! (r != -1);

            let mut counter = 0;
            while zsys_interrupted == 0 {
                let mut msg = zmsg_new ();
                zmsg_addstrf (msg,
                    ffi::CStr::from_bytes_with_nul (b"Hello from rust-ffi\0").unwrap ().as_ptr ());
                r = mlm_client_send (
                    client1,
                    ffi::CStr::from_bytes_with_nul (b"[SUBJECT\0").unwrap ().as_ptr (),
                    &mut msg as *mut *mut zmsg_t
                    );
                let mut msg = mlm_client_recv (client2);

                println_stderr! (">>>>>>>>>>>>>>>>>>>sender={:?}\nsubject={:?}",
                    ffi::CStr::from_ptr (mlm_client_sender (client2)),
                    ffi::CStr::from_ptr (mlm_client_subject (client2))
                    );

                zmsg_print (msg);
                zmsg_destroy (&mut msg as *mut *mut zmsg_t);

                counter += 1;
                if counter > 10 {
                    break;
                }
            }

            mlm_client_destroy (&mut client1 as *mut *mut mlm_client_t);
            mlm_client_destroy (&mut client2 as *mut *mut mlm_client_t);
            zactor_destroy (&mut server as *mut *mut zactor_t);
        }

    }

}
