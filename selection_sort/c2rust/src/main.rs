#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
    non_upper_case_globals, unused_assignments, unused_mut)]
//#![register_tool(c2rust)]
//#![feature(main, register_tool)]
extern "C" {
#[no_mangle]
fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
#[no_mangle]
fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
#[no_mangle]
fn rand() -> libc::c_int;
#[no_mangle]
fn srand(__seed: libc::c_uint);
#[no_mangle]
fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
#[no_mangle]
fn free(__ptr: *mut libc::c_void);
#[no_mangle]
fn time(__timer: *mut time_t) -> time_t;
#[no_mangle]
fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
-> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
pub tv_sec: __time_t,
pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
pub tz_minuteswest: libc::c_int,
pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
/* *
* @file
* @brief [Selection sort](https://en.wikipedia.org/wiki/Selection_sort)
* algorithm implementation.
*/
/* *
* Swapped two numbers using pointer
* @param first first pointer of first number
* @param second second pointer of second number
*/
#[no_mangle]
pub unsafe extern "C" fn swap(mut first: *mut libc::c_int,
                         mut second: *mut libc::c_int) {
let mut temp: libc::c_int = *first;
*first = *second;
*second = temp;
}
/* *
* Selection sort algorithm implements
* @param arr array to be sorted
* @param size size of array
*/
#[no_mangle]
pub unsafe extern "C" fn selectionSort(mut arr: *mut libc::c_int,
                                  mut size: libc::c_int) {
let mut i: libc::c_int = 0 as libc::c_int;
while i < size - 1 as libc::c_int {
   let mut min_index: libc::c_int = i;
   let mut j: libc::c_int = i + 1 as libc::c_int;
   while j < size {
       if *arr.offset(min_index as isize) > *arr.offset(j as isize) {
           min_index = j
       }
       j += 1
   }
   if min_index != i {
       swap(arr.offset(i as isize), arr.offset(min_index as isize));
   }
   i += 1
};
}
/* * Test function
* @returns None
*/
unsafe extern "C" fn test(mut len: libc::c_int) {
if len == 0 as libc::c_int {
   len = rand() % 500 as libc::c_int
} /* random array size */
let size: libc::c_int = len;
let mut arr: *mut libc::c_int =
   calloc(size as libc::c_ulong,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
       *mut libc::c_int;
/* generate size random numbers from -50 to 49 */
let mut i: libc::c_int = 0 as libc::c_int;
while i < size {
   *arr.offset(i as isize) =
       rand() % 10000 as libc::c_int - 5000 as libc::c_int;
   i += 1
   /* signed random numbers */
}
let mut start_time: timeval = timeval{tv_sec: 0, tv_usec: 0,};
gettimeofday(&mut start_time, 0 as *mut timezone);
selectionSort(arr, size);
let mut end_time: timeval = timeval{tv_sec: 0, tv_usec: 0,};
gettimeofday(&mut end_time, 0 as *mut timezone);
let mut total_time: libc::c_long =
   end_time.tv_usec / 1000 as libc::c_int as libc::c_long +
       end_time.tv_sec * 1000 as libc::c_int as libc::c_long -
       (start_time.tv_usec / 1000 as libc::c_int as libc::c_long +
            start_time.tv_sec * 1000 as libc::c_int as libc::c_long);
printf(b"Sorting with %d elements took %.2f seconds\n\x00" as *const u8 as
          *const libc::c_char, len,
      total_time as libc::c_double / 1000.0f64);
free(arr as *mut libc::c_void);
}
/* * Driver Code */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
-> libc::c_int {
/* Intializes random number generator */
srand(time(0 as *mut time_t) as libc::c_uint);
if argc > 1 as libc::c_int {
   test(atoi(*argv.offset(1 as libc::c_int as isize)));
} else {
   test(100 as libc::c_int);
   test(1000 as libc::c_int);
   test(10000 as libc::c_int);
   test(30000 as libc::c_int);
   test(70000 as libc::c_int);
   test(100000 as libc::c_int);
   test(150000 as libc::c_int);
}
return 0 as libc::c_int;
}
#[main]
pub fn main() {
let mut args: Vec<*mut libc::c_char> = Vec::new();
for arg in ::std::env::args() {
   args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
};
args.push(::std::ptr::null_mut());
unsafe {
   ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                               args.as_mut_ptr() as
                                   *mut *const libc::c_char) as i32)
}
}
