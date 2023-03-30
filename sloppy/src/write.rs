use std::os::unix::prelude::AsRawFd;
use std::time;

use libc::{c_int, c_void, mmsghdr, msghdr, size_t};
use libc::{sockaddr, socklen_t, ssize_t};

// types

pub type WriteFn = extern "C" fn(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

pub type SendFn =
    unsafe extern "C" fn(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;

pub type SendmsgFn = unsafe extern "C" fn(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;

pub type SendmmsgFn = unsafe extern "C" fn(
    sockfd: c_int,
    msgvec: *mut mmsghdr,
    vlen: size_t,
    flags: c_int,
) -> ssize_t;

pub type SendtoFn = unsafe extern "C" fn(
    socket: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t;

const TTS: time::Duration = time::Duration::from_millis(crate::TTS_MILLIS);

// overriden funcs

#[no_mangle]
unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    if !(0..3).contains(&fd) && !crate::is_seccomp() {
        if crate::is_network_socket(&fd) && !crate::is_irrelevant_sock_fam(&fd) {
            // print!("slow write");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.write)(fd, buf, count)
}

#[no_mangle]
unsafe extern "C" fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t {
    if !(0..3).contains(&socket) {
        if crate::is_network_socket(&socket) && !crate::is_irrelevant_sock_fam(&socket) {
            ////print!("slow write");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.send)(socket, buf, len, flags)
}

#[no_mangle]
unsafe extern "C" fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t {
    if !(0..3).contains(&fd) {
        if crate::is_network_socket(&fd) && !crate::is_irrelevant_sock_fam(&fd) {
            ////print!("slow write");
            std::thread::sleep(TTS);
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
    if !(0..3).contains(&socket) {
        if crate::is_network_socket(&socket) && !crate::is_irrelevant_sock_fam(&socket) {
            ////print!("slow write");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.sendto)(socket, buf, len, flags, addr, addrlen)
}

#[no_mangle]
unsafe extern "C" fn sendmmsg(
    sockfd: c_int,
    msgvec: *mut mmsghdr,
    vlen: size_t,
    flags: c_int,
) -> ssize_t {
    println!("AYO");
    if !(0..3).contains(&sockfd) {
        if crate::is_network_socket(&sockfd) && !crate::is_irrelevant_sock_fam(&sockfd) {
            ////print!("slow write");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.sendmmsg)(sockfd, msgvec, vlen, flags)
}

// TODO ignore local addresses like 192.* too
