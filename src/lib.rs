#![allow(unused_imports, dead_code)]

mod gen;
mod simple_sparsehash;
mod test;
use crate::test::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn calloc(__count: u64, __size: u64) -> *mut ();
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___memcpy_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn free(_: *mut ()) -> ();
    fn realloc(__ptr: *mut (), __size: u64) -> *mut ();
    fn __builtin___memmove_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn malloc(__size: u64) -> *mut ();
    fn __builtin___strncpy_chk(_: *mut i8, _: *const i8, _: u64, _: u64) -> *mut i8;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn printf(_: *const i8, ...) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn __builtin___snprintf_chk(_: *mut i8, _: u64, _: i32, _: u64, _: *const i8, ...) -> i32;
    fn __builtin_unreachable() -> ();
}
