#![cfg_attr(not(feature = "rustc-dep-of-std"), no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", feature(no_core))]
#![cfg_attr(feature = "rustc-dep-of-std", feature(lang_items))]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]
#![feature(untagged_unions)]

//! Public exports of libc functions
#![allow(non_camel_case_types)]

#[non_exhaustive]
pub enum c_void {}

#[cfg(feature = "rustc-dep-of-std")]
pub use rustc_std_workspace_core as core;
#[cfg(feature = "rustc-dep-of-std")]
use core::ops;

use core::prelude::v1::*;

pub type c_char = u8;
pub type time_t = i32;
pub type wchar_t = u16;
pub type c_long = i64;
pub type c_ulong = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type in_addr_t = u32;
pub type in6_addr_t = [u8; 0x10];
pub type in_port_t = u16;

pub type mode_t = c_uint;
pub type off_t = i64;

pub type pthread_t = u64;

#[repr(C)]
pub struct sem_t { // Unverified
    __size: [c_char; 16],
}

#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}

#[cfg(not(feature = "rustc-dep-of-std"))]
#[allow(non_snake_case, non_upper_case_globals, dead_code)]
pub mod FileOpenMode {
    pub const Write: *const u8 = "w\0".as_bytes().as_ptr();
    pub const Read: *const u8 = "r\0".as_bytes().as_ptr();
    pub const Append: *const u8 = "a\0".as_bytes().as_ptr();
    pub const ReadUpdate: *const u8 = "r+\0".as_bytes().as_ptr();
    pub const WriteUpdate: *const u8 = "w+\0".as_bytes().as_ptr();
    pub const AppendUpdate: *const u8 = "a+\0".as_bytes().as_ptr();
}

#[cfg_attr(not(feature = "rustc-dep-of-std"), derive(Debug, Clone, Copy))]
pub enum FILE {}

#[cfg_attr(not(feature = "rustc-dep-of-std"), derive(Debug, Clone, Copy))]
pub enum DIR {}

extern "C" {
    pub fn malloc(size: size_t) -> *const c_void;
    pub fn free(ptr: *const c_void);
    pub fn calloc(num: size_t, size: size_t) -> *const c_void;
    pub fn realloc(ptr: *const c_void, size: size_t) -> *const c_void;
    // fn aligned_alloc(align: usize, size: usize) -> *const c_void;
}

pub type pthread_key_t = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 5usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_list_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_list_t {
    pub __prev: *mut __pthread_list_t,
    pub __next: *mut __pthread_list_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pthread_attr_t {
    __size: [c_char; 0x38] // RE'd from nnSdk+0x4cf7f8 in smash 6.1.0
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __size: [0; 40]
};

pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;

// please be bsd/unix-like enough for this...
pub const EINTR: c_int = 4;
pub const EINVAL: c_int = 22;

extern "C" {
    #[link_name = "__pthread_key_create"]
    pub fn pthread_key_create(key: *mut pthread_key_t, func: extern fn(*mut c_void)) -> c_int;
    
    #[link_name = "__pthread_key_delete"]
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;

    #[link_name = "__pthread_mutex_lock"]
    pub fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> c_int;
    
    pub fn pthread_setspecific(
        key: pthread_key_t,
        value: *const c_void,
    ) -> c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;

    pub fn pthread_mutexattr_init(
        attr: *mut pthread_mutexattr_t
    ) -> c_int;

    pub fn pthread_mutexattr_settype(
        attr: *mut pthread_mutexattr_t, 
        _type: c_int
    ) -> c_int;

    pub fn pthread_mutexattr_destroy(
        attr: *mut pthread_mutexattr_t
    ) -> c_int;

    pub fn pthread_mutex_trylock(
        lock: *mut pthread_mutex_t
    ) -> c_int;

    pub fn pthread_mutex_unlock(
        lock: *mut pthread_mutex_t
    ) -> c_int;

    pub fn pthread_mutex_destroy(
        lock: *mut pthread_mutex_t
    ) -> c_int;

    pub fn pthread_mutex_init(
        lock: *mut pthread_mutex_t, 
        attr: *const pthread_mutexattr_t
    ) -> c_int;

    pub fn pthread_self() -> pthread_t;

    #[link_name = "__pthread_join"]
    pub fn pthread_join(
        native: pthread_t,
        value: *mut *mut c_void,
    ) -> c_int;

    pub fn pthread_detach(thread: pthread_t) -> c_int;

    pub fn pthread_attr_setstacksize(
        attr: *mut pthread_attr_t,
        stack_size: size_t,
    ) -> c_int;

    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;

    pub fn pthread_create(
        native: *mut pthread_t,
        attr: *const pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int;

    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;

    pub fn pthread_setname_np(
        t: pthread_t,
        name: *const c_schar,
        arg: *const c_void,
    ) -> c_int;
}

pub type socklen_t = c_uint;
pub type sa_family_t = c_ushort;
pub type __time_t = time_t;
pub type __suseconds_t = c_long;
pub type suseconds_t = c_long;
pub type nfds_t = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: in6_addr_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: u64
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *const c_void,
    pub iov_len: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [c_char; 118usize],
    pub __ss_align: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct addrinfo {
    pub ai_flags: c_int, /* AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST */
    pub ai_family: c_int,/* AF_xxx */
    pub ai_socktype: c_int, /* SOCK_xxx */
    pub ai_protocol: c_int, /* 0 or IPPROTO_xxx for IPv4 and IPv6 */
    pub ai_addrlen: socklen_t, /* length of ai_addr */
    pub ai_canonname: *const c_char,	/* canonical name for hostname */
    pub ai_addr: *const sockaddr,	/* binary address */
    pub ai_next: *mut addrinfo,	/* next node in linked list */
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_uint,
}

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const TCP_NODELAY: c_int = 1;
pub const TCP_MAXSEG: c_int = 2;

pub const IPPROTO_ICMP: c_int = 1;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_IPV6: c_int = 41;

pub const SO_ERROR: c_int = 0x1007;
pub const SOL_SOCKET: c_int = 0xffff;
pub const FIONBIO: c_uint = 0x8004667e;
pub const EINPROGRESS: c_int = 36;

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 24;

pub const POLLIN: c_short = 0x1;
pub const POLLPRI: c_short = 0x2;
pub const POLLOUT: c_short = 0x4;
pub const POLLERR: c_short = 0x8;
pub const POLLHUP: c_short = 0x10;
pub const POLLNVAL: c_short = 0x20;
pub const POLLRDNORM: c_short = 0x040;
pub const POLLWRNORM: c_short = 0x004;
pub const POLLRDBAND: c_short = 0x080;
pub const POLLWRBAND: c_short = 0x100;

pub const MSG_PEEK: c_int = 0x2;
pub const EAI_SYSTEM: c_int = 11;

// from libnx
pub const IPV6_ADD_MEMBERSHIP: c_int = 12;
pub const IP_ADD_MEMBERSHIP: c_int = 12;
pub const IP_DROP_MEMBERSHIP: c_int = 13;
pub const IPV6_DROP_MEMBERSHIP: c_int = 13;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SO_SNDTIMEO: c_int = 0x1005;
pub const SO_RCVTIMEO: c_int = 0x1006;
pub const SO_BROADCAST: c_int = 0x20;
pub const IP_TTL: c_int = 4;
pub const SO_REUSEADDR: c_int = 4;
pub const IPV6_V6ONLY: c_int = 27;
pub const IP_MULTICAST_TTL: c_int = 10;
pub const IP_MULTICAST_LOOP: c_int = 11;
pub const IPV6_MULTICAST_LOOP: c_int = 11;

pub const FIOCLEX: c_uint = 0x20006601;
pub const O_NONBLOCK: c_int = 0x4;
pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_DUPFD_CLOEXEC: c_int = 17;

pub const SO_KEEPALIVE: c_int = 0x0008;
pub const INADDR_ANY: c_int = 0;

extern "C" {
    #[link_name = "nnsocketIoctl"]
    pub fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;

    #[link_name = "nnsocketFreeAddrInfo"]
    pub fn freeaddrinfo(res: *mut addrinfo);

    #[link_name = "nnsocketGetAddrInfo"]
    pub fn getaddrinfo(
        node: *const c_schar, 
        service: *const c_char, 
        hints: *const addrinfo, 
        res: *mut *mut addrinfo
    ) -> c_int;

    #[link_name = "nnsocketFcntl"]
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;

    pub fn pread(
        fd: c_int, 
        buf: *mut c_void, 
        count: size_t, 
        offset: off_t
    ) -> ssize_t;
}

/*pub unsafe fn writev(
    fd: c_int,
    buf: *const iovec,
    count: c_int,
) -> ssize_t {
    let slice = core::slice::from_raw_parts(buf, count as usize);
    let mut total = 0;
    for iovec in slice.iter() {
        total += write(fd, iovec.iov_base, iovec.iov_len);
    }
    total
}

pub unsafe fn readv(fd: c_int, buf: *const iovec, count: c_int) -> ssize_t {
    let slice = core::slice::from_raw_parts(buf, count as usize);
    let mut total = 0;
    for iovec in slice.iter() {
        total += read(fd, iovec.iov_base, iovec.iov_len);
    }
    total
}

pub fn socketpair(
    __domain: c_int,
    __type: c_int,
    __protocol: c_int,
    __fds: *mut c_int,
) -> c_int {
    0
}*/

// sockets
extern "C" {
    #[link_name = "nnsocketSocket"]
    pub fn socket(
        __domain: c_int,
        __type: c_int,
        __protocol: c_int,
    ) -> c_int;
    #[link_name = "nnsocketBind"]
    pub fn bind(
        __fd: c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> c_int;
    #[link_name = "nnsocketGetSockName"]
    pub fn getsockname(
        __fd: c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> c_int;
    #[link_name = "nnsocketConnect"]
    pub fn connect(
        __fd: c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> c_int;
    #[link_name = "nnsocketGetPeerName"]
    pub fn getpeername(
        __fd: c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> c_int;
    #[link_name = "nnsocketSend"]
    pub fn send(
        __fd: c_int,
        __buf: *const c_void,
        __n: size_t,
        __flags: c_int,
    ) -> ssize_t;
    #[link_name = "nnsocketRecv"]
    pub fn recv(
        __fd: c_int,
        __buf: *mut c_void,
        __n: size_t,
        __flags: c_int,
    ) -> ssize_t;
    #[link_name = "nnsocketSendTo"]
    pub fn sendto(
        __fd: c_int,
        __buf: *const c_void,
        __n: size_t,
        __flags: c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    #[link_name = "nnsocketRecvFrom"]
    pub fn recvfrom(
        __fd: c_int,
        __buf: *mut c_void,
        __n: size_t,
        __flags: c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    #[link_name = "nnsocketSendMsg"]
    pub fn sendmsg(
        __fd: c_int,
        __message: *const msghdr,
        __flags: c_int,
    ) -> ssize_t;
    /*pub fn sendmmsg(
        __fd: c_int,
        __vmessages: *mut mmsghdr,
        __vlen: c_uint,
        __flags: c_int,
    ) -> c_int;*/
    #[link_name = "nnsocketRecvMsg"]
    pub fn recvmsg(
        __fd: c_int,
        __message: *mut msghdr,
        __flags: c_int,
    ) -> ssize_t;
    /*pub fn recvmmsg(
        __fd: c_int,
        __vmessages: *mut mmsghdr,
        __vlen: c_uint,
        __flags: c_int,
        __tmo: *mut timespec,
    ) -> c_int;*/
    #[link_name = "nnsocketGetSockOpt"]
    pub fn getsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *mut c_void,
        __optlen: *mut socklen_t,
    ) -> c_int;
    #[link_name = "nnsocketSetSockOpt"]
    pub fn setsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *const c_void,
        __optlen: socklen_t,
    ) -> c_int;

    #[link_name = "nnsocketListen"]
    pub fn listen(__fd: c_int, __n: c_int) -> c_int;
    #[link_name = "nnsocketAccept"]
    pub fn accept(
        __fd: c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> c_int;
    #[link_name = "nnsocketAccept"]
    pub fn accept4(
        __fd: c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
        __flags: c_int,
    ) -> c_int;
    #[link_name = "nnsocketShutdown"]
    pub fn shutdown(
        __fd: c_int,
        __how: c_int,
    ) -> c_int;
    #[link_name = "nnsocketSockAtMark"]
    pub fn sockatmark(__fd: c_int) -> c_int;
    #[link_name = "nnsocketPoll"]
    pub fn poll(
        fds: *mut pollfd,
        nfds: nfds_t,
        timeout: c_int
    ) -> c_int;
}

extern "C" {
    #[link_name = "__nnmusl_ErrnoLocation"]
    pub fn errno_loc() -> *mut i64;
}

pub fn gai_strerror(_: c_int) -> *const c_schar {
    "invalid error, no gai_strerror present\0".as_ptr() as _
}

extern "C" {
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn isblank(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    
    pub fn qsort(
        base: *mut c_void,
        num: size_t,
        size: size_t,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn setvbuf(
        stream: *mut FILE,
        buffer: *mut c_char,
        mode: c_int,
        size: size_t,
    ) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fwrite(
        ptr: *const c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn fread_unlocked(
        ptr: *mut c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);

    pub fn perror(s: *const c_char);
    pub fn atoi(s: *const c_char) -> c_int;

    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtol(
        s: *const c_char,
        endp: *mut *mut c_char,
        base: c_int,
    ) -> c_long;

    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;

    pub fn atexit(cb: extern "C" fn()) -> c_int;

    pub fn getenv(s: *const c_char) -> *mut c_char;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(
        dst: *mut c_char,
        src: *const c_char,
        n: size_t,
    ) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(
        s: *mut c_char,
        ct: *const c_char,
        n: size_t,
    ) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(
        s1: *const c_char,
        s2: *const c_char,
        n: size_t,
    ) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;

    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;

    pub fn wcslen(buf: *const wchar_t) -> size_t;
    pub fn wcstombs(
        dest: *mut c_char,
        src: *const wchar_t,
        n: size_t,
    ) -> size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(
        dest: *mut c_void,
        src: *const c_void,
        n: size_t,
    ) -> *mut c_void;

    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn fprintf(
        stream: *mut FILE,
        format: *const c_char,
        ...
    ) -> c_int;
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn snprintf(
        s: *mut c_char,
        n: size_t,
        format: *const c_char,
        ...
    ) -> c_int;
    pub fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;

    pub fn access(path: *const c_char, amode: c_int) -> c_int;

    pub fn chdir(dir: *const c_char) -> c_int;

    pub fn close(fd: c_int) -> c_int;

    pub fn getpid() -> pid_t;

    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;

    pub fn posix_memalign(
        memptr: *mut *mut c_void,
        align: size_t,
        size: size_t,
    ) -> c_int;

    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn sleep(secs: c_uint) -> c_uint;
    pub fn read(fd: c_int, buf: *const c_void, count: size_t)
        -> ssize_t;

    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;

    pub fn unlink(c: *const c_char) -> c_int;

    pub fn write(
        fd: c_int,
        buf: *const c_void,
        count: size_t,
    ) -> ssize_t;

    pub fn pwrite(
        fd: c_int,
        buf: *const c_void,
        count: size_t,
        offset: off_t,
    ) -> ssize_t;

    pub fn setenv(
        name: *const c_char,
        val: *const c_char,
        overwrite: c_int,
    ) -> c_int;

    pub fn unsetenv(name: *const c_char) -> c_int;

    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;

    pub fn sched_yield() -> c_int;

    pub fn mktime(tm: *mut tm) -> time_t;
    pub fn time(time: *mut time_t) -> time_t;
    pub fn gmtime(time_p: *const time_t) -> *mut tm;
    pub fn localtime(time_p: *const time_t) -> *mut tm;
    pub fn difftime(time1: time_t, time0: time_t) -> c_double;

    pub fn putenv(string: *mut c_char) -> c_int;

    pub fn setlocale(
        category: c_int,
        locale: *const c_char,
    ) -> *mut c_char;

    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;

    pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn getline(
        lineptr: *mut *mut c_char,
        n: *mut size_t,
        stream: *mut FILE,
    ) -> ssize_t;

    pub fn fdopendir(fd: c_int) -> *mut DIR;

    pub fn nanosleep(a: *const timespec, b: *mut timespec) -> c_int;
}

#[cfg(not(feature = "rustc-dep-of-std"))]
pub fn fwrite_slice<T: Sized>(slice: &[T], stream: *mut FILE) -> size_t {
    unsafe {
        fwrite(
            slice.as_ptr() as _,
            core::mem::size_of::<T>(),
            slice.len(),
            stream
        )
    }
}
