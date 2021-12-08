#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
    non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, main, ptr_wrapping_offset_from,
      register_tool)]
extern "C" {
#[no_mangle]
fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
#[no_mangle]
fn rand() -> libc::c_int;
#[no_mangle]
fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
#[no_mangle]
fn free(__ptr: *mut libc::c_void);
#[no_mangle]
fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tnode {
pub splitchar: libc::c_char,
pub lokid: Tptr,
pub eqkid: Tptr,
pub hikid: Tptr,
}
// TERNARY SEARCH TREE ALGS
pub type Tptr = *mut tnode;
pub type Tnode = tnode;
#[no_mangle]
pub unsafe extern "C" fn vecswap(mut i: libc::c_int, mut j: libc::c_int,
                            mut n: libc::c_int,
                            mut x: *mut *mut libc::c_char) {
loop  {
   let fresh0 = n;
   n = n - 1;
   if !(fresh0 > 0 as libc::c_int) { break ; }
   let mut t: *mut libc::c_char = *x.offset(i as isize);
   let ref mut fresh1 = *x.offset(i as isize);
   *fresh1 = *x.offset(j as isize);
   let ref mut fresh2 = *x.offset(j as isize);
   *fresh2 = t;
   i += 1;
   j += 1
};
}
#[no_mangle]
pub unsafe extern "C" fn ssort1(mut x: *mut *mut libc::c_char,
                           mut n: libc::c_int, mut depth: libc::c_int) {
let mut a: libc::c_int = 0;
let mut b: libc::c_int = 0;
let mut c: libc::c_int = 0;
let mut d: libc::c_int = 0;
let mut r: libc::c_int = 0;
let mut v: libc::c_int = 0;
if n <= 1 as libc::c_int { return }
a = rand() % n;
let mut t: *mut libc::c_char = *x.offset(0 as libc::c_int as isize);
let ref mut fresh3 = *x.offset(0 as libc::c_int as isize);
*fresh3 = *x.offset(a as isize);
let ref mut fresh4 = *x.offset(a as isize);
*fresh4 = t;
v =
   *(*x.offset(0 as libc::c_int as isize)).offset(depth as isize) as
       libc::c_int;
b = 1 as libc::c_int;
a = b;
d = n - 1 as libc::c_int;
c = d;
loop  {
   while b <= c &&
             {
                 r =
                     *(*x.offset(b as isize)).offset(depth as isize) as
                         libc::c_int - v;
                 (r) <= 0 as libc::c_int
             } {
       if r == 0 as libc::c_int {
           let mut t_0: *mut libc::c_char = *x.offset(a as isize);
           let ref mut fresh5 = *x.offset(a as isize);
           *fresh5 = *x.offset(b as isize);
           let ref mut fresh6 = *x.offset(b as isize);
           *fresh6 = t_0;
           a += 1
       }
       b += 1
   }
   while b <= c &&
             {
                 r =
                     *(*x.offset(c as isize)).offset(depth as isize) as
                         libc::c_int - v;
                 (r) >= 0 as libc::c_int
             } {
       if r == 0 as libc::c_int {
           let mut t_1: *mut libc::c_char = *x.offset(c as isize);
           let ref mut fresh7 = *x.offset(c as isize);
           *fresh7 = *x.offset(d as isize);
           let ref mut fresh8 = *x.offset(d as isize);
           *fresh8 = t_1;
           d -= 1
       }
       c -= 1
   }
   if b > c { break ; }
   let mut t_2: *mut libc::c_char = *x.offset(b as isize);
   let ref mut fresh9 = *x.offset(b as isize);
   *fresh9 = *x.offset(c as isize);
   let ref mut fresh10 = *x.offset(c as isize);
   *fresh10 = t_2;
   b += 1;
   c -= 1
}
r = if a <= b - a { a } else { (b) - a };
vecswap(0 as libc::c_int, b - r, r, x);
r =
   if d - c <= n - d - 1 as libc::c_int {
       (d) - c
   } else { (n - d) - 1 as libc::c_int };
vecswap(b, n - r, r, x);
r = b - a;
ssort1(x, r, depth);
if *(*x.offset(r as isize)).offset(depth as isize) as libc::c_int !=
      0 as libc::c_int {
   ssort1(x.offset(r as isize), a + n - d - 1 as libc::c_int,
          depth + 1 as libc::c_int);
}
r = d - c;
ssort1(x.offset(n as isize).offset(-(r as isize)), r, depth);
}
#[no_mangle]
pub unsafe extern "C" fn ssort1main(mut x: *mut *mut libc::c_char,
                               mut n: libc::c_int) {
ssort1(x, n, 0 as libc::c_int);
}
// ssort2 -- Faster Version of Multikey Quicksort
#[no_mangle]
pub unsafe extern "C" fn vecswap2(mut a: *mut *mut libc::c_char,
                             mut b: *mut *mut libc::c_char,
                             mut n: libc::c_int) {
loop  {
   let fresh11 = n;
   n = n - 1;
   if !(fresh11 > 0 as libc::c_int) { break ; }
   let mut t: *mut libc::c_char = *a;
   let fresh12 = a;
   a = a.offset(1);
   *fresh12 = *b;
   let fresh13 = b;
   b = b.offset(1);
   *fresh13 = t
};
}
#[no_mangle]
pub unsafe extern "C" fn med3func(mut a: *mut *mut libc::c_char,
                             mut b: *mut *mut libc::c_char,
                             mut c: *mut *mut libc::c_char,
                             mut depth: libc::c_int)
-> *mut *mut libc::c_char {
let mut va: libc::c_int = 0;
let mut vb: libc::c_int = 0;
let mut vc: libc::c_int = 0;
va = *(*a).offset(depth as isize) as libc::c_int;
vb = *(*b).offset(depth as isize) as libc::c_int;
if va == vb { return a }
vc = *(*c).offset(depth as isize) as libc::c_int;
if vc == va || vc == vb { return c }
return if va < vb {
          if vb < vc { b } else if va < vc { c } else { a }
      } else if vb > vc { b } else if va < vc { a } else { c };
}
#[no_mangle]
pub unsafe extern "C" fn inssort(mut a: *mut *mut libc::c_char,
                            mut n: libc::c_int, mut d: libc::c_int) {
let mut pi: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pj: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
pi = a.offset(1 as libc::c_int as isize);
loop  {
   n -= 1;
   if !(n > 0 as libc::c_int) { break ; }
   pj = pi;
   while pj > a {
       // Inline strcmp: break if *(pj-1) <= *pj
       s = (*pj.offset(-(1 as libc::c_int as isize))).offset(d as isize);
       t = (*pj).offset(d as isize);
       while *s as libc::c_int == *t as libc::c_int &&
                 *s as libc::c_int != 0 as libc::c_int {
           s = s.offset(1);
           t = t.offset(1)
       }
       if *s as libc::c_int <= *t as libc::c_int { break ; }
       t = *pj;
       *pj = *pj.offset(-(1 as libc::c_int as isize));
       let ref mut fresh14 = *pj.offset(-(1 as libc::c_int as isize));
       *fresh14 = t;
       pj = pj.offset(-1)
   }
   pi = pi.offset(1)
};
}
#[no_mangle]
pub unsafe extern "C" fn ssort2(mut a: *mut *mut libc::c_char,
                           mut n: libc::c_int, mut depth: libc::c_int) {
let mut d: libc::c_int = 0;
let mut r: libc::c_int = 0;
let mut partval: libc::c_int = 0;
let mut pa: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pb: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pc: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pl: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pm: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut pn: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
if n < 10 as libc::c_int { inssort(a, n, depth); return }
pl = a;
pm = a.offset((n / 2 as libc::c_int) as isize);
pn = a.offset((n - 1 as libc::c_int) as isize);
if n > 30 as libc::c_int {
   // On big arrays, pseudomedian of 9
   d = n / 8 as libc::c_int;
   pl =
       med3func(pl, pl.offset(d as isize),
                pl.offset((2 as libc::c_int * d) as isize), depth);
   pm =
       med3func(pm.offset(-(d as isize)), pm, pm.offset(d as isize),
                depth);
   pn =
       med3func(pn.offset(-((2 as libc::c_int * d) as isize)),
                pn.offset(-(d as isize)), pn, depth)
}
pm = med3func(pl, pm, pn, depth);
t = *a;
*a = *pm;
*pm = t;
partval = *(*a).offset(depth as isize) as libc::c_int;
pb = a.offset(1 as libc::c_int as isize);
pa = pb;
pd = a.offset(n as isize).offset(-(1 as libc::c_int as isize));
pc = pd;
loop  {
   while pb <= pc &&
             {
                 r =
                     *(*pb).offset(depth as isize) as libc::c_int -
                         partval;
                 (r) <= 0 as libc::c_int
             } {
       if r == 0 as libc::c_int {
           t = *pa;
           *pa = *pb;
           *pb = t;
           pa = pa.offset(1)
       }
       pb = pb.offset(1)
   }
   while pb <= pc &&
             {
                 r =
                     *(*pc).offset(depth as isize) as libc::c_int -
                         partval;
                 (r) >= 0 as libc::c_int
             } {
       if r == 0 as libc::c_int {
           t = *pc;
           *pc = *pd;
           *pd = t;
           pd = pd.offset(-1)
       }
       pc = pc.offset(-1)
   }
   if pb > pc { break ; }
   t = *pb;
   *pb = *pc;
   *pc = t;
   pb = pb.offset(1);
   pc = pc.offset(-1)
}
pn = a.offset(n as isize);
r =
   if pa.wrapping_offset_from(a) as libc::c_long <=
          pb.wrapping_offset_from(pa) as libc::c_long {
       pa.wrapping_offset_from(a) as libc::c_long
   } else { pb.wrapping_offset_from(pa) as libc::c_long } as libc::c_int;
vecswap2(a, pb.offset(-(r as isize)), r);
r =
   if pd.wrapping_offset_from(pc) as libc::c_long <=
          pn.wrapping_offset_from(pd) as libc::c_long -
              1 as libc::c_int as libc::c_long {
       pd.wrapping_offset_from(pc) as libc::c_long
   } else {
       (pn.wrapping_offset_from(pd) as libc::c_long) -
           1 as libc::c_int as libc::c_long
   } as libc::c_int;
vecswap2(pb, pn.offset(-(r as isize)), r);
r = pb.wrapping_offset_from(pa) as libc::c_long as libc::c_int;
if r > 1 as libc::c_int { ssort2(a, r, depth); }
if *(*a.offset(r as isize)).offset(depth as isize) as libc::c_int !=
      0 as libc::c_int {
   ssort2(a.offset(r as isize),
          (pn.offset(pa.wrapping_offset_from(a) as libc::c_long as
                         isize).wrapping_offset_from(pd) as libc::c_long
               - 1 as libc::c_int as libc::c_long) as libc::c_int,
          depth + 1 as libc::c_int);
}
r = pd.wrapping_offset_from(pc) as libc::c_long as libc::c_int;
if r > 1 as libc::c_int {
   ssort2(a.offset(n as isize).offset(-(r as isize)), r, depth);
};
}
#[no_mangle]
pub unsafe extern "C" fn ssort2main(mut a: *mut *mut libc::c_char,
                               mut n: libc::c_int) {
ssort2(a, n, 0 as libc::c_int);
}
#[no_mangle]
pub static mut root: Tptr = 0 as *const tnode as *mut tnode;
// Insert 1 -- Simple Insertion Algorithm
#[no_mangle]
pub unsafe extern "C" fn insert1(mut p: Tptr, mut s: *mut libc::c_char)
-> Tptr {
if p.is_null() {
   p = malloc(::std::mem::size_of::<Tnode>() as libc::c_ulong) as Tptr;
   (*p).splitchar = *s;
   (*p).hikid = 0 as Tptr;
   (*p).eqkid = (*p).hikid;
   (*p).lokid = (*p).eqkid
}
if (*s as libc::c_int) < (*p).splitchar as libc::c_int {
   (*p).lokid = insert1((*p).lokid, s)
} else if *s as libc::c_int == (*p).splitchar as libc::c_int {
   if *s as libc::c_int != 0 as libc::c_int {
       s = s.offset(1);
       (*p).eqkid = insert1((*p).eqkid, s)
   }
} else { (*p).hikid = insert1((*p).hikid, s) }
return p;
}
#[no_mangle]
pub unsafe extern "C" fn cleanup1(mut p: Tptr) {
if !p.is_null() {
   cleanup1((*p).lokid);
   cleanup1((*p).eqkid);
   cleanup1((*p).hikid);
   free(p as *mut libc::c_void);
};
}
#[no_mangle]
pub static mut buffer: Tptr = 0 as *const tnode as *mut tnode;
#[no_mangle]
pub static mut bufn: libc::c_int = 0;
#[no_mangle]
pub static mut freen: libc::c_int = 0;
#[no_mangle]
pub static mut freearr: [*mut libc::c_void; 10000] =
[0 as *const libc::c_void as *mut libc::c_void; 10000];
#[no_mangle]
pub static mut storestring: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn insert2(mut s: *mut libc::c_char) {
let mut d: libc::c_int = 0;
let mut instr: *mut libc::c_char = s;
let mut pp: Tptr = 0 as *mut tnode;
let mut p: *mut Tptr = 0 as *mut Tptr;
p = &mut root;
pp = *p;
while pp == *p {
   d = *s as libc::c_int - (*pp).splitchar as libc::c_int;
   if d == 0 as libc::c_int {
       let fresh15 = s;
       s = s.offset(1);
       if *fresh15 as libc::c_int == 0 as libc::c_int { return }
       p = &mut (*pp).eqkid
   } else if d < 0 as libc::c_int {
       p = &mut (*pp).lokid
   } else { p = &mut (*pp).hikid }
}
loop  {
   // *p = (Tptr) malloc(sizeof(Tnode));
   let fresh16 = bufn;
   bufn = bufn - 1;
   if fresh16 == 0 as libc::c_int {
       buffer =
           malloc((1000 as libc::c_int as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Tnode>()
                                                       as libc::c_ulong))
               as Tptr;
       let fresh17 = freen;
       freen = freen + 1;
       freearr[fresh17 as usize] = buffer as *mut libc::c_void;
       bufn = 1000 as libc::c_int - 1 as libc::c_int
   }
   let fresh18 = buffer;
   buffer = buffer.offset(1);
   *p = fresh18;
   pp = *p;
   (*pp).splitchar = *s;
   (*pp).hikid = 0 as Tptr;
   (*pp).eqkid = (*pp).hikid;
   (*pp).lokid = (*pp).eqkid;
   let fresh19 = s;
   s = s.offset(1);
   if *fresh19 as libc::c_int == 0 as libc::c_int {
       if storestring != 0 { (*pp).eqkid = instr as Tptr }
       return
   }
   p = &mut (*pp).eqkid
};
}
#[no_mangle]
pub unsafe extern "C" fn cleanup2() {
let mut i: libc::c_int = 0;
i = 0 as libc::c_int;
while i < freen { free(freearr[i as usize]); i += 1 };
}
// Search Algorithms
#[no_mangle]
pub unsafe extern "C" fn search1(mut s: *mut libc::c_char) -> libc::c_int {
let mut p: Tptr = 0 as *mut tnode;
p = root;
while !p.is_null() {
   if (*s as libc::c_int) < (*p).splitchar as libc::c_int {
       p = (*p).lokid
   } else if *s as libc::c_int == (*p).splitchar as libc::c_int {
       let fresh20 = s;
       s = s.offset(1);
       if *fresh20 as libc::c_int == 0 as libc::c_int {
           return 1 as libc::c_int
       }
       p = (*p).eqkid
   } else { p = (*p).hikid }
}
return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn search2(mut s: *mut libc::c_char) -> libc::c_int {
let mut d: libc::c_int = 0;
let mut sc: libc::c_int = 0;
let mut p: Tptr = 0 as *mut tnode;
sc = *s as libc::c_int;
p = root;
while !p.is_null() {
   d = sc - (*p).splitchar as libc::c_int;
   if d == 0 as libc::c_int {
       if sc == 0 as libc::c_int { return 1 as libc::c_int }
       s = s.offset(1);
       sc = *s as libc::c_int;
       p = (*p).eqkid
   } else if d < 0 as libc::c_int {
       p = (*p).lokid
   } else { p = (*p).hikid }
}
return 0 as libc::c_int;
}
// Advanced searching: Partial match, near words
#[no_mangle]
pub static mut nodecnt: libc::c_int = 0;
#[no_mangle]
pub static mut srcharr: [*mut libc::c_char; 100000] =
[0 as *const libc::c_char as *mut libc::c_char; 100000];
#[no_mangle]
pub static mut srchtop: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn pmsearch(mut p: Tptr, mut s: *mut libc::c_char) {
if p.is_null() { return }
nodecnt += 1;
if *s as libc::c_int == '.' as i32 ||
      (*s as libc::c_int) < (*p).splitchar as libc::c_int {
   pmsearch((*p).lokid, s);
}
if *s as libc::c_int == '.' as i32 ||
      *s as libc::c_int == (*p).splitchar as libc::c_int {
   if (*p).splitchar as libc::c_int != 0 && *s as libc::c_int != 0 {
       pmsearch((*p).eqkid, s.offset(1 as libc::c_int as isize));
   }
}
if *s as libc::c_int == 0 as libc::c_int &&
      (*p).splitchar as libc::c_int == 0 as libc::c_int {
   let fresh21 = srchtop;
   srchtop = srchtop + 1;
   srcharr[fresh21 as usize] = (*p).eqkid as *mut libc::c_char
}
if *s as libc::c_int == '.' as i32 ||
      *s as libc::c_int > (*p).splitchar as libc::c_int {
   pmsearch((*p).hikid, s);
};
}
#[no_mangle]
pub unsafe extern "C" fn nearsearch(mut p: Tptr, mut s: *mut libc::c_char,
                               mut d: libc::c_int) {
if p.is_null() || d < 0 as libc::c_int { return }
nodecnt += 1;
if d > 0 as libc::c_int ||
      (*s as libc::c_int) < (*p).splitchar as libc::c_int {
   nearsearch((*p).lokid, s, d);
}
if (*p).splitchar as libc::c_int == 0 as libc::c_int {
   if strlen(s) as libc::c_int <= d {
       let fresh22 = srchtop;
       srchtop = srchtop + 1;
       srcharr[fresh22 as usize] = (*p).eqkid as *mut libc::c_char
   }
} else {
   nearsearch((*p).eqkid,
              if *s as libc::c_int != 0 {
                  s.offset(1 as libc::c_int as isize)
              } else { s },
              if *s as libc::c_int == (*p).splitchar as libc::c_int {
                  d
              } else { (d) - 1 as libc::c_int });
}
if d > 0 as libc::c_int ||
      *s as libc::c_int > (*p).splitchar as libc::c_int {
   nearsearch((*p).hikid, s, d);
};
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
-> libc::c_int {
let mut arr: [*mut libc::c_char; 3] =
   [b"apple\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char,
    b"cat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"boy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
ssort1main(arr.as_mut_ptr(), 3 as libc::c_int);
let mut i: libc::c_int = 0 as libc::c_int;
while i < 3 as libc::c_int {
   printf(b"%s \x00" as *const u8 as *const libc::c_char,
          arr[i as usize]);
   i += 1
}
return 0;
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
                                   *mut *mut libc::c_char) as i32)
}
}
