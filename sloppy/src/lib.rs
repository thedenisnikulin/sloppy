use libc::{c_int, dlsym, sockaddr, socklen_t, RTLD_NEXT};
use once_cell::sync::Lazy;
use os_socketaddr::OsSocketAddr;
use std::{collections::HashSet, ffi::CString, os::unix::prelude::AsRawFd, sync::RwLock};

mod read;
mod write;

static fns: Lazy<SockFns> = unsafe {
    Lazy::new(|| SockFns::new_from_first_symbol().expect("cannot get socket functions symbols"))
};

static unix_sock_fd_map: Lazy<RwLock<HashSet<c_int>>> = Lazy::new(|| RwLock::new(HashSet::new()));

type ConnectFn =
    unsafe extern "C" fn(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;

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
    sendmsg: write::SendmsgFn,
    sendto: write::SendtoFn,
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
            sendto: sendto_ptr,
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn connect(
    socket: c_int,
    address: *const sockaddr,
    address_len: socklen_t,
) -> c_int {
    if address.is_null() {
        panic!("address is null");
    }

    let addr = OsSocketAddr::copy_from_raw(address, address_len).into_addr();

    if let Some(addr) = addr {
        if !addr.ip().is_loopback() {
            //print!("slow conn");
            //std::thread::sleep(ttw);
        }

        return (fns.connect)(socket, address, address_len);
    } else {
        //print!("unix conn");
        if let Ok(mut w) = unix_sock_fd_map.write() {
            w.insert(socket);
        } else {
            //print!("poisoned");
        }

        return (fns.connect)(socket, address, address_len);
    }
}

// WARNING avoid implicit "write" calls in all overriden fns (like //print! macro)

fn is_network_socket<F: AsRawFd>(fd: &F) -> bool {
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(fd.as_raw_fd(), &mut stat) } == 0 {
        print!(
            "(fd: {}, mode: {:#05x}_____)",
            fd.as_raw_fd(),
            stat.st_mode & libc::S_IFMT
        );
        return stat.st_mode & libc::S_IFMT == libc::S_IFSOCK;
    }

    false
}
