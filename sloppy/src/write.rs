use std::time;

use libc::{c_int, c_void, msghdr, size_t};
use libc::{sockaddr, socklen_t, ssize_t};

// types

pub type WriteFn = extern "C" fn(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

pub type SendFn =
    unsafe extern "C" fn(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;

pub type SendmsgFn = unsafe extern "C" fn(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;

pub type SendtoFn = unsafe extern "C" fn(
    socket: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t;

const TTS: time::Duration = time::Duration::from_millis(50);

// overriden funcs

// TODO seems like is_network_socket causes errors when using send* functions

#[no_mangle]
unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&fd) {
            ////print!("slow write");
            //std::thread::sleep(TTS);
        }
    }
    (crate::fns.write)(fd, buf, count)
}

#[no_mangle]
unsafe extern "C" fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&socket) {
            ////print!("slow write");
            //std::thread::sleep(TTS);
        }
    }
    (crate::fns.send)(socket, buf, len, flags)
}

#[no_mangle]
unsafe extern "C" fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&fd) {
            ////print!("slow write");
            //std::thread::sleep(TTS);
        }
    }
    (crate::fns.sendmsg)(fd, msg, flags)
}

#[no_mangle]
unsafe extern "C" fn sendto(
    socket: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&socket) {
            ////print!("slow write");
            //std::thread::sleep(TTS);
        }
    }
    (crate::fns.sendto)(socket, buf, len, flags, addr, addrlen)
}
