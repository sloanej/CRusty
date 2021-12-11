extern {
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn malloc(__size : usize) -> *mut ::std::os::raw::c_void;
    fn printf(__format : *const u8, ...) -> i32;
    fn rand() -> i32;
    fn strlen(__s : *const u8) -> usize;
}

#[no_mangle]
pub unsafe extern fn vecswap(
    mut i : i32, mut j : i32, mut n : i32, mut x : *mut *mut u8
) {
    'loop0: loop {
        if !({
                 let _old = n;
                 n = n - 1;
                 _old
             } > 0i32) {
            break;
        }
        let mut t : *mut u8 = *x.offset(i as (isize));
        *x.offset(i as (isize)) = *x.offset(j as (isize));
        *x.offset(j as (isize)) = t;
        i = i + 1;
        j = j + 1;
    }
}

#[no_mangle]
pub unsafe extern fn ssort1(
    mut x : *mut *mut u8, mut n : i32, mut depth : i32
) {
    let mut a : i32;
    let mut b : i32;
    let mut c : i32;
    let mut d : i32;
    let mut r : i32 = 0;
    let mut v : i32;
    if n <= 1i32 {
    } else {
        a = rand() % n;
        let mut t : *mut u8 = *x.offset(0isize);
        *x.offset(0isize) = *x.offset(a as (isize));
        *x.offset(a as (isize)) = t;
        v = *(*x.offset(0isize)).offset(depth as (isize)) as (i32);
        a = {
                b = 1i32;
                b
            };
        c = {
                d = n - 1i32;
                d
            };
        'loop2: loop {
            if b <= c && ({
                              r = *(*x.offset(b as (isize))).offset(
                                       depth as (isize)
                                   ) as (i32) - v;
                              r
                          } <= 0i32) {
                if r == 0i32 {
                    let mut t : *mut u8 = *x.offset(a as (isize));
                    *x.offset(a as (isize)) = *x.offset(b as (isize));
                    *x.offset(b as (isize)) = t;
                    a = a + 1;
                }
                b = b + 1;
            } else {
                'loop3: loop {
                    if !(b <= c && ({
                                        r = *(*x.offset(c as (isize))).offset(
                                                 depth as (isize)
                                             ) as (i32) - v;
                                        r
                                    } >= 0i32)) {
                        break;
                    }
                    if r == 0i32 {
                        let mut t : *mut u8 = *x.offset(c as (isize));
                        *x.offset(c as (isize)) = *x.offset(d as (isize));
                        *x.offset(d as (isize)) = t;
                        d = d - 1;
                    }
                    c = c - 1;
                }
                if b > c {
                    break;
                }
                let mut t : *mut u8 = *x.offset(b as (isize));
                *x.offset(b as (isize)) = *x.offset(c as (isize));
                *x.offset(c as (isize)) = t;
                b = b + 1;
                c = c - 1;
            }
        }
        r = if a <= b - a { a } else { b - a };
        vecswap(0i32,b - r,r,x);
        r = if d - c <= n - d - 1i32 { d - c } else { n - d - 1i32 };
        vecswap(b,n - r,r,x);
        r = b - a;
        ssort1(x,r,depth);
        if *(*x.offset(r as (isize))).offset(
                depth as (isize)
            ) as (i32) != 0i32 {
            ssort1(x.offset(r as (isize)),a + n - d - 1i32,depth + 1i32);
        }
        r = d - c;
        ssort1(x.offset(n as (isize)).offset(-(r as (isize))),r,depth);
    }
}

#[no_mangle]
pub unsafe extern fn ssort1main(mut x : *mut *mut u8, mut n : i32) {
    ssort1(x,n,0i32);
}

#[no_mangle]
pub unsafe extern fn vecswap2(
    mut a : *mut *mut u8, mut b : *mut *mut u8, mut n : i32
) {
    'loop0: loop {
        if !({
                 let _old = n;
                 n = n - 1;
                 _old
             } > 0i32) {
            break;
        }
        let mut t : *mut u8 = *a;
        *{
             let _old = a;
             a = a.offset(1isize);
             _old
         } = *b;
        *{
             let _old = b;
             b = b.offset(1isize);
             _old
         } = t;
    }
}

#[no_mangle]
pub unsafe extern fn med3func(
    mut a : *mut *mut u8,
    mut b : *mut *mut u8,
    mut c : *mut *mut u8,
    mut depth : i32
) -> *mut *mut u8 {
    let mut va : i32;
    let mut vb : i32;
    let mut vc : i32;
    if {
           va = *(*a).offset(depth as (isize)) as (i32);
           va
       } == {
                vb = *(*b).offset(depth as (isize)) as (i32);
                vb
            } {
        a
    } else if {
                  vc = *(*c).offset(depth as (isize)) as (i32);
                  vc
              } == va || vc == vb {
        c
    } else if va < vb {
        (if vb < vc { b } else if va < vc { c } else { a })
    } else if vb > vc {
        b
    } else if va < vc {
        a
    } else {
        c
    }
}

#[no_mangle]
pub unsafe extern fn inssort(
    mut a : *mut *mut u8, mut n : i32, mut d : i32
) {
    let mut pi : *mut *mut u8;
    let mut pj : *mut *mut u8;
    let mut s : *mut u8;
    let mut t : *mut u8;
    pi = a.offset(1isize);
    'loop1: loop {
        if !({
                 n = n - 1;
                 n
             } > 0i32) {
            break;
        }
        pj = pi;
        'loop4: loop {
            if !(pj > a) {
                break;
            }
            s = (*pj.offset(-1isize)).offset(d as (isize));
            t = (*pj).offset(d as (isize));
            'loop6: loop {
                if !(*s as (i32) == *t as (i32) && (*s as (i32) != 0i32)) {
                    break;
                }
                s = s.offset(1isize);
                t = t.offset(1isize);
            }
            if *s as (i32) <= *t as (i32) {
                break;
            }
            t = *pj;
            *pj = *pj.offset(-1isize);
            *pj.offset(-1isize) = t;
            pj = pj.offset(-1isize);
        }
        pi = pi.offset(1isize);
    }
}

#[no_mangle]
pub unsafe extern fn ssort2(
    mut a : *mut *mut u8, mut n : i32, mut depth : i32
) {
    let mut d : i32;
    let mut r : i32 = 0;
    let mut partval : i32;
    let mut pa : *mut *mut u8;
    let mut pb : *mut *mut u8;
    let mut pc : *mut *mut u8;
    let mut pd : *mut *mut u8;
    let mut pl : *mut *mut u8;
    let mut pm : *mut *mut u8;
    let mut pn : *mut *mut u8;
    let mut t : *mut u8;
    if n < 10i32 {
        inssort(a,n,depth);
    } else {
        pl = a;
        pm = a.offset((n / 2i32) as (isize));
        pn = a.offset((n - 1i32) as (isize));
        if n > 30i32 {
            d = n / 8i32;
            pl = med3func(
                     pl,
                     pl.offset(d as (isize)),
                     pl.offset((2i32 * d) as (isize)),
                     depth
                 );
            pm = med3func(
                     pm.offset(-(d as (isize))),
                     pm,
                     pm.offset(d as (isize)),
                     depth
                 );
            pn = med3func(
                     pn.offset(-((2i32 * d) as (isize))),
                     pn.offset(-(d as (isize))),
                     pn,
                     depth
                 );
        }
        pm = med3func(pl,pm,pn,depth);
        t = *a;
        *a = *pm;
        *pm = t;
        partval = *(*a).offset(depth as (isize)) as (i32);
        pa = {
                 pb = a.offset(1isize);
                 pb
             };
        pc = {
                 pd = a.offset(n as (isize)).offset(-1isize);
                 pd
             };
        'loop4: loop {
            if pb <= pc && ({
                                r = *(*pb).offset(depth as (isize)) as (i32) - partval;
                                r
                            } <= 0i32) {
                if r == 0i32 {
                    t = *pa;
                    *pa = *pb;
                    *pb = t;
                    pa = pa.offset(1isize);
                }
                pb = pb.offset(1isize);
            } else {
                'loop5: loop {
                    if !(pb <= pc && ({
                                          r = *(*pc).offset(depth as (isize)) as (i32) - partval;
                                          r
                                      } >= 0i32)) {
                        break;
                    }
                    if r == 0i32 {
                        t = *pc;
                        *pc = *pd;
                        *pd = t;
                        pd = pd.offset(-1isize);
                    }
                    pc = pc.offset(-1isize);
                }
                if pb > pc {
                    break;
                }
                t = *pb;
                *pb = *pc;
                *pc = t;
                pb = pb.offset(1isize);
                pc = pc.offset(-1isize);
            }
        }
        pn = a.offset(n as (isize));
        r = if (pa as (isize)).wrapping_sub(
                   a as (isize)
               ) / ::std::mem::size_of::<*mut u8>(
                   ) as (isize) <= (pb as (isize)).wrapping_sub(
                                       pa as (isize)
                                   ) / ::std::mem::size_of::<*mut u8>() as (isize) {
                (pa as (isize)).wrapping_sub(
                    a as (isize)
                ) / ::std::mem::size_of::<*mut u8>() as (isize)
            } else {
                (pb as (isize)).wrapping_sub(
                    pa as (isize)
                ) / ::std::mem::size_of::<*mut u8>() as (isize)
            } as (i32);
        vecswap2(a,pb.offset(-(r as (isize))),r);
        r = if (pd as (isize)).wrapping_sub(
                   pc as (isize)
               ) / ::std::mem::size_of::<*mut u8>(
                   ) as (isize) <= (pn as (isize)).wrapping_sub(
                                       pd as (isize)
                                   ) / ::std::mem::size_of::<*mut u8>() as (isize) - 1isize {
                (pd as (isize)).wrapping_sub(
                    pc as (isize)
                ) / ::std::mem::size_of::<*mut u8>() as (isize)
            } else {
                (pn as (isize)).wrapping_sub(
                    pd as (isize)
                ) / ::std::mem::size_of::<*mut u8>() as (isize) - 1isize
            } as (i32);
        vecswap2(pb,pn.offset(-(r as (isize))),r);
        if {
               r = ((pb as (isize)).wrapping_sub(
                        pa as (isize)
                    ) / ::std::mem::size_of::<*mut u8>() as (isize)) as (i32);
               r
           } > 1i32 {
            ssort2(a,r,depth);
        }
        if *(*a.offset(r as (isize))).offset(
                depth as (isize)
            ) as (i32) != 0i32 {
            ssort2(
                a.offset(r as (isize)),
                ((pn.offset(
                      (pa as (isize)).wrapping_sub(
                          a as (isize)
                      ) / ::std::mem::size_of::<*mut u8>() as (isize)
                  ) as (isize)).wrapping_sub(
                     pd as (isize)
                 ) / ::std::mem::size_of::<*mut u8>() as (isize) - 1isize) as (i32),
                depth + 1i32
            );
        }
        if {
               r = ((pd as (isize)).wrapping_sub(
                        pc as (isize)
                    ) / ::std::mem::size_of::<*mut u8>() as (isize)) as (i32);
               r
           } > 1i32 {
            ssort2(a.offset(n as (isize)).offset(-(r as (isize))),r,depth);
        }
    }
}

#[no_mangle]
pub unsafe extern fn ssort2main(mut a : *mut *mut u8, mut n : i32) {
    ssort2(a,n,0i32);
}

#[no_mangle]
pub static mut root : *mut tnode = 0 as (*mut tnode);

#[derive(Copy)]
#[repr(C)]
pub struct tnode {
    pub splitchar : u8,
    pub lokid : *mut tnode,
    pub eqkid : *mut tnode,
    pub hikid : *mut tnode,
}

impl Clone for tnode {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn insert1(
    mut p : *mut tnode, mut s : *mut u8
) -> *mut tnode {
    if p == 0i32 as (*mut tnode) {
        p = malloc(::std::mem::size_of::<tnode>()) as (*mut tnode);
        (*p).splitchar = *s;
        (*p).lokid = {
                         (*p).eqkid = {
                                          (*p).hikid = 0i32 as (*mut tnode);
                                          (*p).hikid
                                      };
                         (*p).eqkid
                     };
    }
    if *s as (i32) < (*p).splitchar as (i32) {
        (*p).lokid = insert1((*p).lokid,s);
    } else if *s as (i32) == (*p).splitchar as (i32) {
        if *s as (i32) != 0i32 {
            (*p).eqkid = insert1(
                             (*p).eqkid,
                             {
                                 s = s.offset(1isize);
                                 s
                             }
                         );
        }
    } else {
        (*p).hikid = insert1((*p).hikid,s);
    }
    p
}

#[no_mangle]
pub unsafe extern fn cleanup1(mut p : *mut tnode) {
    if !p.is_null() {
        cleanup1((*p).lokid);
        cleanup1((*p).eqkid);
        cleanup1((*p).hikid);
        free(p as (*mut ::std::os::raw::c_void));
    }
}

#[no_mangle]
pub static mut buffer : *mut tnode = 0 as (*mut tnode);

#[no_mangle]
pub static mut bufn : i32 = 0i32;

#[no_mangle]
pub static mut freen : i32 = 0i32;

#[no_mangle]
pub static mut freearr
    : [*mut ::std::os::raw::c_void; 10000]
    = [0 as (*mut ::std::os::raw::c_void); 10000];

#[no_mangle]
pub static mut storestring : i32 = 0i32;

#[no_mangle]
pub unsafe extern fn insert2(mut s : *mut u8) {
    let mut _currentBlock;
    let mut d : i32;
    let mut instr : *mut u8 = s;
    let mut pp : *mut tnode;
    let mut p : *mut *mut tnode;
    p = &mut root as (*mut *mut tnode);
    pp = *p;
    'loop1: loop {
        if !(pp == *p) {
            _currentBlock = 2;
            break;
        }
        if {
               d = *s as (i32) - (*pp).splitchar as (i32);
               d
           } == 0i32 {
            if *{
                    let _old = s;
                    s = s.offset(1isize);
                    _old
                } as (i32) == 0i32 {
                _currentBlock = 15;
                break;
            }
            p = &mut (*pp).eqkid as (*mut *mut tnode);
        } else if d < 0i32 {
            p = &mut (*pp).lokid as (*mut *mut tnode);
        } else {
            p = &mut (*pp).hikid as (*mut *mut tnode);
        }
    }
    if _currentBlock == 2 {
        'loop2: loop {
            if {
                   let _old = bufn;
                   bufn = bufn - 1;
                   _old
               } == 0i32 {
                buffer = malloc(
                             1000usize.wrapping_mul(::std::mem::size_of::<tnode>())
                         ) as (*mut tnode);
                freearr[
                    {
                        let _old = freen;
                        freen = freen + 1;
                        _old
                    } as (usize)
                ] = buffer as (*mut ::std::os::raw::c_void);
                bufn = 1000i32 - 1i32;
            }
            *p = {
                     let _old = buffer;
                     buffer = buffer.offset(1isize);
                     _old
                 };
            pp = *p;
            (*pp).splitchar = *s;
            (*pp).lokid = {
                              (*pp).eqkid = {
                                                (*pp).hikid = 0i32 as (*mut tnode);
                                                (*pp).hikid
                                            };
                              (*pp).eqkid
                          };
            if *{
                    let _old = s;
                    s = s.offset(1isize);
                    _old
                } as (i32) == 0i32 {
                break;
            }
            p = &mut (*pp).eqkid as (*mut *mut tnode);
        }
        if storestring != 0 {
            (*pp).eqkid = instr as (*mut tnode);
        }
    }
}

#[no_mangle]
pub unsafe extern fn cleanup2() {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < freen) {
            break;
        }
        free(freearr[i as (usize)]);
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn search1(mut s : *mut u8) -> i32 {
    let mut _currentBlock;
    let mut p : *mut tnode;
    p = root;
    'loop1: loop {
        if p.is_null() {
            _currentBlock = 2;
            break;
        }
        if *s as (i32) < (*p).splitchar as (i32) {
            p = (*p).lokid;
        } else if *s as (i32) == (*p).splitchar as (i32) {
            if *{
                    let _old = s;
                    s = s.offset(1isize);
                    _old
                } as (i32) == 0i32 {
                _currentBlock = 8;
                break;
            }
            p = (*p).eqkid;
        } else {
            p = (*p).hikid;
        }
    }
    if _currentBlock == 2 { 0i32 } else { 1i32 }
}

#[no_mangle]
pub unsafe extern fn search2(mut s : *mut u8) -> i32 {
    let mut _currentBlock;
    let mut d : i32;
    let mut sc : i32;
    let mut p : *mut tnode;
    sc = *s as (i32);
    p = root;
    'loop1: loop {
        if p.is_null() {
            _currentBlock = 2;
            break;
        }
        if {
               d = sc - (*p).splitchar as (i32);
               d
           } == 0i32 {
            if sc == 0i32 {
                _currentBlock = 9;
                break;
            }
            sc = *{
                      s = s.offset(1isize);
                      s
                  } as (i32);
            p = (*p).eqkid;
        } else if d < 0i32 {
            p = (*p).lokid;
        } else {
            p = (*p).hikid;
        }
    }
    if _currentBlock == 2 { 0i32 } else { 1i32 }
}

#[no_mangle]
pub static mut nodecnt : i32 = 0i32;

#[no_mangle]
pub static mut srcharr : [*mut u8; 100000] = [0 as (*mut u8); 100000];

#[no_mangle]
pub static mut srchtop : i32 = 0i32;

#[no_mangle]
pub unsafe extern fn pmsearch(mut p : *mut tnode, mut s : *mut u8) {
    if p.is_null() {
    } else {
        nodecnt = nodecnt + 1;
        if *s as (i32) == b'.' as (i32) || *s as (i32) < (*p).splitchar as (i32) {
            pmsearch((*p).lokid,s);
        }
        if *s as (i32) == b'.' as (i32) || *s as (i32) == (*p).splitchar as (i32) {
            if (*p).splitchar != 0 && (*s != 0) {
                pmsearch((*p).eqkid,s.offset(1isize));
            }
        }
        if *s as (i32) == 0i32 && ((*p).splitchar as (i32) == 0i32) {
            srcharr[
                {
                    let _old = srchtop;
                    srchtop = srchtop + 1;
                    _old
                } as (usize)
            ] = (*p).eqkid as (*mut u8);
        }
        if *s as (i32) == b'.' as (i32) || *s as (i32) > (*p).splitchar as (i32) {
            pmsearch((*p).hikid,s);
        }
    }
}

#[no_mangle]
pub unsafe extern fn nearsearch(
    mut p : *mut tnode, mut s : *mut u8, mut d : i32
) { if p.is_null() || d < 0i32 {
    } else {
        nodecnt = nodecnt + 1;
        if d > 0i32 || *s as (i32) < (*p).splitchar as (i32) {
            nearsearch((*p).lokid,s,d);
        }
        if (*p).splitchar as (i32) == 0i32 {
            if strlen(s as (*const u8)) as (i32) <= d {
                srcharr[
                    {
                        let _old = srchtop;
                        srchtop = srchtop + 1;
                        _old
                    } as (usize)
                ] = (*p).eqkid as (*mut u8);
            }
        } else {
            nearsearch(
                (*p).eqkid,
                if *s != 0 { s.offset(1isize) } else { s },
                if *s as (i32) == (*p).splitchar as (i32) { d } else { d - 1i32 }
            );
        }
        if d > 0i32 || *s as (i32) > (*p).splitchar as (i32) {
            nearsearch((*p).hikid,s,d);
        }
    }
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    let mut arr
        : [*mut u8; 3]
        = [   (*b"apple\0").as_ptr() as (*mut u8),
              (*b"cat\0").as_ptr() as (*mut u8),
              (*b"boy\0").as_ptr() as (*mut u8)
          ];
    ssort1main(arr.as_mut_ptr(),3i32);
    let mut i : i32 = 0i32;
    'loop1: loop {
        if !(i < 3i32) {
            break;
        }
        printf((*b"%s \0").as_ptr(),arr[i as (usize)]);
        i = i + 1;
    }
    0
}
