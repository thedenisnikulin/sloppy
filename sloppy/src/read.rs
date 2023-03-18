use libc::{c_int, c_void, msghdr, size_t};
use libc::{c_uint, mmsghdr, sockaddr, socklen_t, ssize_t, timespec};
use std::time;

// types

pub type ReadFn = extern "C" fn(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
pub type RecvFn =
    unsafe extern "C" fn(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;

pub type RecvfromFn = unsafe extern "C" fn(
    socket: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
    addr: *mut sockaddr,
    addrlen: *mut socklen_t,
) -> ssize_t;

pub type RecvmmsgFn = unsafe extern "C" fn(
    sockfd: c_int,
    msgvec: *mut mmsghdr,
    vlen: c_uint,
    flags: c_int,
    timeout: *mut timespec,
) -> c_int;

pub type RecvmsgFn = unsafe extern "C" fn(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;

const TTS: time::Duration = time::Duration::from_millis(50);

// overriden funcs

#[no_mangle]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&fd) {
            //print!("slow read");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.read)(fd, buf, count)
}

#[no_mangle]
pub unsafe extern "C" fn recv(
    socket: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&socket) {
            //print!("slow read");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.recv)(socket, buf, len, flags)
}

#[no_mangle]
pub unsafe extern "C" fn recvfrom(
    socket: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
    addr: *mut sockaddr,
    addrlen: *mut socklen_t,
) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&socket) {
            //print!("slow read");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.recvfrom)(socket, buf, len, flags, addr, addrlen)
}

#[no_mangle]
pub unsafe extern "C" fn recvmmsg(
    sockfd: c_int,
    msgvec: *mut mmsghdr,
    vlen: c_uint,
    flags: c_int,
    timeout: *mut timespec,
) -> c_int {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&sockfd) {
            //print!("slow read");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.recvmmsg)(sockfd, msgvec, vlen, flags, timeout)
}

#[no_mangle]
pub unsafe extern "C" fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t {
    if let Ok(not_ok_fds) = crate::unix_sock_fd_map.read() {
        if crate::is_network_socket(&fd) {
            //print!("slow read");
            std::thread::sleep(TTS);
        }
    }
    (crate::fns.recvmsg)(fd, msg, flags)
}
