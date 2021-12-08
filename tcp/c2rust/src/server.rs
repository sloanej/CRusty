#![allow(warnings)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
    non_upper_case_globals, unused_assignments, unused_mut)]
//#![register_tool(c2rust)]
//#![feature(main, register_tool)]
extern "C" {
#[no_mangle]
fn perror(__s: *const libc::c_char);
#[no_mangle]
fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
#[no_mangle]
fn rand() -> libc::c_int;
#[no_mangle]
fn srand(__seed: libc::c_uint);
#[no_mangle]
fn exit(_: libc::c_int) -> !;
#[no_mangle]
fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
#[no_mangle]
fn socket(__domain: libc::c_int, __type: libc::c_int,
         __protocol: libc::c_int) -> libc::c_int;
#[no_mangle]
fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
-> libc::c_int;
#[no_mangle]
fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
#[no_mangle]
fn accept(__fd: libc::c_int, __addr: *mut sockaddr,
         __addr_len: *mut socklen_t) -> libc::c_int;
#[no_mangle]
fn htonl(__hostlong: uint32_t) -> uint32_t;
#[no_mangle]
fn htons(__hostshort: uint16_t) -> uint16_t;
#[no_mangle]
fn close(__fd: libc::c_int) -> libc::c_int;
#[no_mangle]
fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
-> ssize_t;
#[no_mangle]
fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
-> ssize_t;
#[no_mangle]
fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
-> libc::c_int;
#[no_mangle]
fn signal(__sig: libc::c_int, __handler: __sighandler_t)
-> __sighandler_t;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
pub tv_sec: __time_t,
pub tv_usec: __suseconds_t,
}
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type socklen_t = __socklen_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
pub tz_minuteswest: libc::c_int,
pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
/* *< port number to connect to */
/* *< shortname for sockaddr */
static mut scount: libc::c_int = 0 as libc::c_int;
static mut rcount: libc::c_int = 0 as libc::c_int;
static mut start_time: timeval = timeval{tv_sec: 0, tv_usec: 0,};
/* *
* Continuous loop to send and receive over the socket.
* Exits when "exit" is sent from commandline.
* @param sockfd socket handle number
*/
#[no_mangle]
pub unsafe extern "C" fn func(mut sockfd: libc::c_int) {
let mut buff: [libc::c_char; 100000] = [0; 100000];
//int n;
let mut payload: [libc::c_char; 100000] = [0; 100000];
srand(libc::time(0 as *mut i64) as libc::c_uint);
let mut i: libc::c_int = 0 as libc::c_int;
while i < 100000 as libc::c_int {
   payload[i as usize] =
       if rand() % 2 as libc::c_int == 0 as libc::c_int {
           'a' as i32
       } else { 'b' as i32 } as libc::c_char;
   i += 1
}
gettimeofday(&mut start_time, 0 as *mut timezone);
loop 
    // infinite loop for chat
    {
   bzero(buff.as_mut_ptr() as *mut libc::c_void,
         100000 as libc::c_int as libc::c_ulong);
   rcount += 1;
   read(sockfd, buff.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 100000]>() as
            libc::c_ulong);
   scount += 1;
   write(sockfd, payload.as_mut_ptr() as *const libc::c_void,
         100000 as libc::c_int as size_t);
};
}
unsafe extern "C" fn handler(mut _a: libc::c_int) {
let mut tmptime: timeval = timeval{tv_sec: 0, tv_usec: 0,};
gettimeofday(&mut tmptime, 0 as *mut timezone);
let mut total_time: libc::c_long =
   tmptime.tv_usec / 1000 as libc::c_int as libc::c_long +
       tmptime.tv_sec * 1000 as libc::c_int as libc::c_long -
       (start_time.tv_usec / 1000 as libc::c_int as libc::c_long +
            start_time.tv_sec * 1000 as libc::c_int as libc::c_long);
let mut sspeed: libc::c_float =
   ((scount as libc::c_float * 100000 as libc::c_int as libc::c_float /
         total_time as libc::c_float) as libc::c_double *
        (1000.0f64 / 1024 as libc::c_int as libc::c_double)) as
       libc::c_float;
let mut rspeed: libc::c_float =
   ((rcount as libc::c_float * 100000 as libc::c_int as libc::c_float /
         total_time as libc::c_float) as libc::c_double *
        (1000.0f64 / 1024 as libc::c_int as libc::c_double)) as
       libc::c_float;
printf(b"\n%d payloads sent, %.2f kb/s\n%d payloads received, %.2f kb/s\nTime taken: %ld\n\x00"
          as *const u8 as *const libc::c_char, scount,
      sspeed as libc::c_double, rcount, rspeed as libc::c_double,
      total_time);
exit(0 as libc::c_int);
}
/* * Driver code */
unsafe fn main_0() -> libc::c_int {
signal(2 as libc::c_int,
      Some(handler as unsafe extern "C" fn(_: libc::c_int) -> ()));
let mut sockfd: libc::c_int = 0;
let mut connfd: libc::c_int = 0;
let mut len: libc::c_uint = 0;
let mut servaddr: sockaddr_in =
   sockaddr_in{sin_family: 0,
               sin_port: 0,
               sin_addr: in_addr{s_addr: 0,},
               sin_zero: [0; 8],};
let mut cli: sockaddr_in =
   sockaddr_in{sin_family: 0,
               sin_port: 0,
               sin_addr: in_addr{s_addr: 0,},
               sin_zero: [0; 8],};
// socket create and verification
sockfd =
   socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
          0 as libc::c_int);
if sockfd == -(1 as libc::c_int) {
   perror(b"socket creation failed...\n\x00" as *const u8 as
              *const libc::c_char);
   exit(0 as libc::c_int);
} else {
   printf(b"Socket successfully created..\n\x00" as *const u8 as
              *const libc::c_char);
}
bzero(&mut servaddr as *mut sockaddr_in as *mut libc::c_void,
     ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
// assign IP, PORT
servaddr.sin_family = 2 as libc::c_int as sa_family_t;
servaddr.sin_addr.s_addr = htonl(0 as libc::c_int as in_addr_t);
servaddr.sin_port = htons(8080 as libc::c_int as uint16_t);
// Binding newly created socket to given IP and verification
if bind(sockfd, &mut servaddr as *mut sockaddr_in as *mut sockaddr,
       ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
           socklen_t) != 0 as libc::c_int {
   perror(b"socket bind failed...\n\x00" as *const u8 as
              *const libc::c_char);
   exit(0 as libc::c_int);
} else {
   printf(b"Socket successfully binded..\n\x00" as *const u8 as
              *const libc::c_char);
}
// Now server is ready to listen and verification
if listen(sockfd, 5 as libc::c_int) != 0 as libc::c_int {
   perror(b"Listen failed...\n\x00" as *const u8 as *const libc::c_char);
   exit(0 as libc::c_int);
} else {
   printf(b"Server listening..\n\x00" as *const u8 as
              *const libc::c_char);
}
len =
   ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_uint;
// Accept the data packet from client and verification
connfd =
   accept(sockfd, &mut cli as *mut sockaddr_in as *mut sockaddr,
          &mut len);
if connfd < 0 as libc::c_int {
   perror(b"server acccept failed...\n\x00" as *const u8 as
              *const libc::c_char);
   exit(0 as libc::c_int);
} else {
   printf(b"server acccept the client...\n\x00" as *const u8 as
              *const libc::c_char);
}
// Function for chatting between client and server
func(connfd);
// After chatting close the socket
close(sockfd);
return 0 as libc::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
