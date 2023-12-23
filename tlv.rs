#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn perror(__s: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
unsafe fn main_0() -> libc::c_int {
    let mut server_socket: libc::c_int = 0;
    let mut client_socket: libc::c_int = 0;
    let mut server_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut client_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut client_addr_len: socklen_t = ::std::mem::size_of::<sockaddr_in>()
        as libc::c_ulong as socklen_t;
    server_socket = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
    );
    if server_socket == -(1 as libc::c_int) {
        perror(b"Error creating socket\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(
        &mut server_addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    server_addr.sin_family = 2 as libc::c_int as sa_family_t;
    server_addr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    server_addr.sin_port = htons(80 as libc::c_int as uint16_t);
    if bind(
        server_socket,
        &mut server_addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        perror(b"Error binding socket\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if listen(server_socket, 10 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"Error listening\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(
        b"Server listening on port %d...\n\0" as *const u8 as *const libc::c_char,
        80 as libc::c_int,
    );
    loop {
        client_socket = accept(
            server_socket,
            &mut client_addr as *mut sockaddr_in as *mut sockaddr,
            &mut client_addr_len,
        );
        if client_socket == -(1 as libc::c_int) {
            perror(b"Error accepting connection\0" as *const u8 as *const libc::c_char);
        } else {
            handle_client(client_socket);
            close(client_socket);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_client(mut client_socket: libc::c_int) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut bytes_received: ssize_t = 0;
    bytes_received = recv(
        client_socket,
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        0 as libc::c_int,
    );
    if bytes_received == -(1 as libc::c_int) as libc::c_long {
        perror(b"Error receiving data\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut requested_file: *mut libc::c_char = b"index.html\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    if !(strstr(buffer.as_mut_ptr(), b"GET\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        let mut start: *mut libc::c_char = (strchr(buffer.as_mut_ptr(), '/' as i32))
            .offset(1 as libc::c_int as isize);
        let mut end: *mut libc::c_char = strchr(start, ' ' as i32);
        if !start.is_null() && !end.is_null() && start < end {
            let mut path_len: size_t = end.offset_from(start) as libc::c_long as size_t;
            requested_file = strndup(start, path_len);
        }
    }
    let mut file: *mut FILE = fopen(
        requested_file,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        perror(b"Error opening file\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut response_header: [libc::c_char; 200] = [0; 200];
    snprintf(
        response_header.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong,
        b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\0" as *const u8
            as *const libc::c_char,
    );
    send(
        client_socket,
        response_header.as_mut_ptr() as *const libc::c_void,
        strlen(response_header.as_mut_ptr()),
        0 as libc::c_int,
    );
    loop {
        bytes_received = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            file,
        ) as ssize_t;
        if !(bytes_received > 0 as libc::c_int as libc::c_long) {
            break;
        }
        send(
            client_socket,
            buffer.as_mut_ptr() as *const libc::c_void,
            bytes_received as size_t,
            0 as libc::c_int,
        );
    }
    fclose(file);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
