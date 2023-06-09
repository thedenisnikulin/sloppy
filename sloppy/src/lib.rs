#![feature(c_variadic)]

use libc::{c_int, dlsym, sendto, sockaddr, socklen_t, AF_UNIX, RTLD_NEXT, SO_TYPE};
use nix::sys::socket::{AddressFamily, SockaddrLike};
use once_cell::sync::Lazy;
use os_socketaddr::OsSocketAddr;
use std::{collections::HashSet, ffi::CString, os::unix::prelude::AsRawFd, sync::RwLock};

mod read;
mod write;

static fns: Lazy<SockFns> = unsafe {
    Lazy::new(|| SockFns::new_from_first_symbol().expect("cannot get socket functions symbols"))
};

const TTS_MILLIS: u64 = 50;

// TODO move each fn to separate type with their own constructors
struct SockFns {
    connect: ConnectFn,
    read: read::ReadFn,
    recv: read::RecvFn,
    recvfrom: read::RecvfromFn,
    recvmmsg: read::RecvmmsgFn,
    recvmsg: read::RecvmsgFn,
    write: write::WriteFn,
    send: write::SendFn,
    sendto: write::SendtoFn,
    sendmmsg: write::SendmmsgFn,
    sendmsg: write::SendmsgFn,
    prctl: PrctlFn,
}

impl SockFns {
    unsafe fn new_from_first_symbol() -> Result<SockFns, ()> {
        let connect_str = CString::new("connect").unwrap();
        let connect_ptr = dlsym(RTLD_NEXT, connect_str.as_ptr());
        if connect_ptr.is_null() {
            return Err(());
        }

        let connect_ptr: ConnectFn = std::mem::transmute(connect_ptr);

        let read_str = CString::new("read").unwrap();
        let read_ptr = dlsym(RTLD_NEXT, read_str.as_ptr());
        if read_ptr.is_null() {
            return Err(());
        }
        let read_ptr: read::ReadFn = std::mem::transmute(read_ptr);

        let recv_str = CString::new("recv").unwrap();
        let recv_ptr = dlsym(RTLD_NEXT, recv_str.as_ptr());
        if recv_ptr.is_null() {
            return Err(());
        }
        let recv_ptr: read::RecvFn = std::mem::transmute(recv_ptr);

        let recvfrom_str = CString::new("recvfrom").unwrap();
        let recvfrom_ptr = dlsym(RTLD_NEXT, recvfrom_str.as_ptr());
        if recvfrom_ptr.is_null() {
            return Err(());
        }
        let recvfrom_ptr: read::RecvfromFn = std::mem::transmute(recvfrom_ptr);

        let recvmmsg_str = CString::new("recvmmsg").unwrap();
        let recvmmsg_ptr = dlsym(RTLD_NEXT, recvmmsg_str.as_ptr());
        if recvmmsg_ptr.is_null() {
            return Err(());
        }
        let recvmmsg_ptr: read::RecvmmsgFn = std::mem::transmute(recvmmsg_ptr);

        let recvmsg_str = CString::new("recvmsg").unwrap();
        let recvmsg_ptr = dlsym(RTLD_NEXT, recvmsg_str.as_ptr());
        if recvmsg_ptr.is_null() {
            return Err(());
        }
        let recvmsg_ptr: read::RecvmsgFn = std::mem::transmute(recvmsg_ptr);

        let write_str = CString::new("write").unwrap();
        let write_str = dlsym(RTLD_NEXT, write_str.as_ptr());
        if write_str.is_null() {
            return Err(());
        }
        let write_ptr: write::WriteFn = std::mem::transmute(write_str);

        let send_str = CString::new("send").unwrap();
        let send_str = dlsym(RTLD_NEXT, send_str.as_ptr());
        if send_str.is_null() {
            return Err(());
        }
        let send_ptr: write::SendFn = std::mem::transmute(send_str);

        let sendmsg_str = CString::new("sendmsg").unwrap();
        let sendmsg_str = dlsym(RTLD_NEXT, sendmsg_str.as_ptr());
        if sendmsg_str.is_null() {
            return Err(());
        }
        let sendmsg_ptr: write::SendmsgFn = std::mem::transmute(sendmsg_str);

        let sendto_str = CString::new("sendto").unwrap();
        let sendto_str = dlsym(RTLD_NEXT, sendto_str.as_ptr());
        if sendto_str.is_null() {
            return Err(());
        }
        let sendto_ptr: write::SendtoFn = std::mem::transmute(sendto_str);

        let sendmmsg_str = CString::new("sendmmsg").unwrap();
        let sendmmsg_str = dlsym(RTLD_NEXT, sendmmsg_str.as_ptr());
        if sendmmsg_str.is_null() {
            return Err(());
        }
        let sendmmsg_ptr: write::SendmmsgFn = std::mem::transmute(sendmmsg_str);

        let prctl_str = CString::new("prctl").unwrap();
        let prctl_ptr = dlsym(RTLD_NEXT, prctl_str.as_ptr());
        if prctl_ptr.is_null() {
            return Err(());
        }
        let prctl_ptr: PrctlFn = std::mem::transmute(prctl_ptr);

        Ok(SockFns {
            connect: connect_ptr,
            read: read_ptr,
            recv: recv_ptr,
            recvfrom: recvfrom_ptr,
            recvmmsg: recvmmsg_ptr,
            recvmsg: recvmsg_ptr,
            write: write_ptr,
            send: send_ptr,
            sendmsg: sendmsg_ptr,
            sendmmsg: sendmmsg_ptr,
            sendto: sendto_ptr,
            prctl: prctl_ptr,
        })
    }
}

type ConnectFn =
    unsafe extern "C" fn(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;

type PrctlFn = unsafe extern "C" fn(option: c_int, ...) -> c_int;

#[no_mangle]
pub unsafe extern "C" fn connect(
    socket: c_int,
    address: *const sockaddr,
    address_len: socklen_t,
) -> c_int {
    (fns.connect)(socket, address, address_len)
}

// this needs to be here because without it some programs that use seccomp will crash with "prctl() failure"
#[no_mangle]
pub unsafe extern "C" fn prctl(option: c_int, args: ...) -> c_int {
    (fns.prctl)(option, args)
}

// WARNING avoid implicit "write" calls in all overriden fns (like print! macro)

fn is_network_socket<F: AsRawFd>(fd: &F) -> bool {
    let ret = match nix::sys::stat::fstat(fd.as_raw_fd()) {
        Ok(s) => s,
        _ => return false,
    };
    ret.st_mode & libc::S_IFMT == libc::S_IFSOCK
}

fn is_irrelevant_sock_fam<F: AsRawFd>(fd: &F) -> bool {
    let addr: nix::sys::socket::SockaddrStorage =
        match nix::sys::socket::getsockname(fd.as_raw_fd()) {
            Ok(a) => a,
            _ => return false,
        };

    let fam = match addr.family() {
        Some(f) => f,
        _ => return false,
    };

    //println!("NETFAM {:#?} FOR FD {}////", fam, fd.as_raw_fd());

    fam == AddressFamily::Unix || fam == AddressFamily::Netlink || fam == AddressFamily::Unspec
}

// TODO STOPPED HERE
fn is_seccomp() -> bool {
    let mode = unsafe { libc::prctl(libc::PR_GET_SECCOMP) } as u32;
    mode == libc::SECCOMP_MODE_STRICT || mode == libc::SECCOMP_MODE_FILTER
}
