extern {
    fn calloc(
        __nmemb : usize, __size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn gettimeofday(
        __tv : *mut timeval, __tz : *mut ::std::os::raw::c_void
    ) -> i32;
    fn printf(__format : *const u8, ...) -> i32;
    fn rand() -> i32;
    fn srand(__seed : u32);
    fn time(__timer : *mut isize) -> isize;
}

#[no_mangle]
pub unsafe extern  fn swap  (
    mut first : *mut i32, mut second : *mut i32
)   {

let mut temp : i32 = *first;
    *first = *second;
    *second = temp;

}

#[no_mangle]
pub unsafe extern  fn selectionSort  (
    mut arr : *mut i32, mut size : i32
)   {

let mut i : i32 = 0i32;
    while i < size - 1i32 {
	let mut min_index : i32 = i;
        let mut j : i32 = i + 1i32;
        while j < size {
	if *arr.offset(min_index as (isize)) > *arr.offset(j as (isize)) {
                min_index = j;
            }
            j = j + 1; 
}
        if min_index != i {
            swap(arr.offset(i as (isize)),arr.offset(min_index as (isize)));
        }
        i = i + 1; 
}

}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[derive(Copy)]
#[repr(C)]
pub struct timeval {
    pub tv_sec : isize,
    pub tv_usec : isize,
}

impl Clone for timeval {
    fn clone(&self) -> Self { *self }
}

unsafe extern  fn test  (mut len : i32)   {

if len == 0i32 {
        len = rand() % 500i32;
    }
    let size : i32 = len;
    let mut arr
        : *mut i32
        = calloc(size as (usize),
              ::std::mem::size_of::<i32>()) as (*mut i32);
    let mut i : i32 = 0i32;
    while i < size {
	*arr.offset(i as (isize)) = rand() % 10000i32 - 5000i32;
        i = i + 1; 
}
    let mut start_time: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut start_time as (*mut timeval),
        0i32 as (*mut ::std::os::raw::c_void));
    selectionSort(arr,size);
    let mut end_time: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut end_time as (*mut timeval),
        0i32 as (*mut ::std::os::raw::c_void));
    let mut total_time
        : isize
        = end_time.tv_usec / 1000isize + end_time.tv_sec * 1000isize - (start_time.tv_usec / 1000isize + start_time.tv_sec * 1000isize);
    printf((*b"Sorting with %d elements took %.2f seconds\n\0").as_ptr(),
        len,
        total_time as (f64) / 1000 as f64
    );
    free(arr as (*mut ::std::os::raw::c_void));

}

#[no_mangle]
pub unsafe extern  fn _c_main  () -> i32   {

srand(time(
            0i32 as (*mut ::std::os::raw::c_void) as (*mut isize)
        ) as (u32));
    test(100i32);
    test(1000i32);
    test(10000i32);
    test(30000i32);
    test(70000i32);
    test(100000i32);
    test(150000i32);
    0i32
}
