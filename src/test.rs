use super::*;
use crate::gen::include::simple_sparsehash_h::{SparseArray, SparseDict};
use crate::simple_sparsehash::{
    sparse_array_free, sparse_array_get, sparse_array_init, sparse_array_set, sparse_dict_free,
    sparse_dict_get, sparse_dict_init, sparse_dict_set,
};

pub(crate) extern "C" fn test_empty_array_does_not_blow_up() -> i32 {
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    arr = sparse_array_init(core::mem::size_of::<u64>() as u64, 32 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 29) };
        return 0;
    }
    if ((sparse_array_get(unsafe { &*arr }, 0 as u32, 0 as *mut () as *mut u64)).is_null() as i32
        == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 31) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 33) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_cannot_set_outside_bounds() -> i32 {
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    let test_num: u64 = 666 as u64;
    arr = sparse_array_init(core::mem::size_of::<u64>() as u64, 32 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 41) };
        return 0;
    }
    if !(sparse_array_set(
        unsafe { &*arr },
        35 as u32,
        &raw const test_num as *const (),
        core::mem::size_of::<u64>() as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 43) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 45) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_cannot_get_outside_bounds() -> i32 {
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    arr = sparse_array_init(core::mem::size_of::<u64>() as u64, 32 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 52) };
        return 0;
    }
    if ((sparse_array_get(unsafe { &*arr }, 35 as u32, 0 as *mut () as *mut u64)).is_null() as i32
        == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 54) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 56) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_cannot_set_bigger_elements() -> i32 {
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    let test_num: u64 = 666 as u64;
    arr = sparse_array_init(core::mem::size_of::<i8>() as u64, 100 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 64) };
        return 0;
    }
    if !(sparse_array_set(
        unsafe { &*arr },
        0 as u32,
        &raw const test_num as *const (),
        core::mem::size_of::<u64>() as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 66) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 68) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_array_set_backwards() -> i32 {
    let mut i: i32 = 0;
    let array_size: i32 = 120 as i32;
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    arr = sparse_array_init(core::mem::size_of::<i32>() as u64, array_size as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 78) };
        return 0;
    }
    {
        i = array_size - 1;
        '__b12: loop {
            if !(i >= 0) {
                break '__b12;
            }
            '__c12: loop {
                let mut returned: *const i32 = 0 as *mut () as *const i32;
                let mut siz: u64 = 0 as u64;
                if (sparse_array_set(
                    unsafe { &*arr },
                    i as u32,
                    &raw mut i as *const (),
                    core::mem::size_of::<i32>() as u64,
                ) == 0) as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 83) };
                    return 0;
                }
                returned = sparse_array_get(unsafe { &*arr }, i as u32, &mut siz) as *mut i32;
                if (returned).is_null() as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 85) };
                    return 0;
                }
                if !(unsafe { *returned } == i) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 86) };
                    return 0;
                }
                if !(siz == core::mem::size_of::<i32>() as u64) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 87) };
                    return 0;
                }
                break '__c12;
            }
            i -= 1;
        }
    }
    {
        i = array_size - 1;
        '__b13: loop {
            if !(i >= 0) {
                break '__b13;
            }
            '__c13: loop {
                let mut returned: *const i32 = 0 as *mut () as *const i32;
                let mut siz: u64 = 0 as u64;
                returned = sparse_array_get(unsafe { &*arr }, i as u32, &mut siz) as *mut i32;
                if !(unsafe { *returned } == i) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 94) };
                    return 0;
                }
                if !(siz == core::mem::size_of::<i32>() as u64) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 95) };
                    return 0;
                }
                break '__c13;
            }
            i -= 1;
        }
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 98) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_array_set() -> i32 {
    let mut i: i32 = 0;
    let array_size: i32 = 130 as i32;
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    arr = sparse_array_init(core::mem::size_of::<i32>() as u64, array_size as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 107) };
        return 0;
    }
    {
        i = 0;
        '__b14: loop {
            if !(i < array_size) {
                break '__b14;
            }
            '__c14: loop {
                let mut returned: *const i32 = 0 as *mut () as *const i32;
                let mut siz: u64 = 0 as u64;
                if (sparse_array_set(
                    unsafe { &*arr },
                    i as u32,
                    &raw mut i as *const (),
                    core::mem::size_of::<i32>() as u64,
                ) == 0) as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 112) };
                    return 0;
                }
                returned = sparse_array_get(unsafe { &*arr }, i as u32, &mut siz) as *mut i32;
                if !(unsafe { *returned } == i) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 114) };
                    return 0;
                }
                if !(siz == core::mem::size_of::<i32>() as u64) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 115) };
                    return 0;
                }
                break '__c14;
            }
            i += 1;
        }
    }
    {
        i = 0;
        '__b15: loop {
            if !(i < array_size) {
                break '__b15;
            }
            '__c15: loop {
                let mut returned: *const i32 = 0 as *mut () as *const i32;
                let mut siz: u64 = 0 as u64;
                returned = sparse_array_get(unsafe { &*arr }, i as u32, &mut siz) as *mut i32;
                if !(unsafe { *returned } == i) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 123) };
                    return 0;
                }
                if !(siz == core::mem::size_of::<i32>() as u64) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 124) };
                    return 0;
                }
                break '__c15;
            }
            i += 1;
        }
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 127) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_array_set_high_num() -> i32 {
    let test_num: i32 = 65555555 as i32;
    let index: i32 = (48 - 1) as i32;
    let mut returned: *const i32 = 0 as *mut () as *const i32;
    let mut siz: u64 = 0 as u64;
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    arr = sparse_array_init(core::mem::size_of::<i32>() as u64, 140 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 139) };
        return 0;
    }
    if (sparse_array_set(
        unsafe { &*arr },
        index as u32,
        &raw const test_num as *const (),
        core::mem::size_of::<i32>() as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 141) };
        return 0;
    }
    returned = sparse_array_get(unsafe { &*arr }, index as u32, &mut siz) as *mut i32;
    if (returned).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 143) };
        return 0;
    }
    if !(unsafe { *returned } == test_num) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 144) };
        return 0;
    }
    if !(siz == core::mem::size_of::<i32>() as u64) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 145) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 147) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_array_set_overwrites_old_values() -> i32 {
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    let test_num: i32 = 666 as i32;
    let test_num2: i32 = 1024 as i32;
    arr = sparse_array_init(core::mem::size_of::<i32>() as u64, 150 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 156) };
        return 0;
    }
    if (sparse_array_set(
        unsafe { &*arr },
        0 as u32,
        &raw const test_num as *const (),
        core::mem::size_of::<i32>() as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 158) };
        return 0;
    }
    if (sparse_array_set(
        unsafe { &*arr },
        0 as u32,
        &raw const test_num2 as *const (),
        core::mem::size_of::<i32>() as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 159) };
        return 0;
    }
    if !(unsafe {
        *(sparse_array_get(unsafe { &*arr }, 0 as u32, 0 as *mut () as *mut u64) as *const i32)
    } as i32
        == 1024) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 161) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 163) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_array_get() -> i32 {
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;
    let test_num: i32 = 666 as i32;
    let mut item_size: u64 = 0 as u64;
    arr = sparse_array_init(core::mem::size_of::<i32>() as u64, 200 as u32);
    if (arr).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 172) };
        return 0;
    }
    if (sparse_array_set(
        unsafe { &*arr },
        0 as u32,
        &raw const test_num as *const (),
        core::mem::size_of::<i32>() as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 174) };
        return 0;
    }
    if !(unsafe { *(sparse_array_get(unsafe { &*arr }, 0 as u32, &mut item_size) as *const i32) }
        as i32
        == 666) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 175) };
        return 0;
    }
    if !(item_size == core::mem::size_of::<i32>() as u64) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 176) };
        return 0;
    }
    if (sparse_array_free(arr) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 178) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_dict_set() -> i32 {
    let mut dict: *mut SparseDict = 0 as *mut () as *mut SparseDict;
    dict = sparse_dict_init();
    if (dict).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 185) };
        return 0;
    }
    if (sparse_dict_set(
        dict,
        c"key".as_ptr() as *mut i8 as *const i8,
        unsafe { strlen(c"key".as_ptr() as *mut i8 as *const i8) } as u64,
        c"value".as_ptr() as *mut i8 as *const (),
        unsafe { strlen(c"value".as_ptr() as *mut i8 as *const i8) } as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 187) };
        return 0;
    }
    if (sparse_dict_free(dict) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 189) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn test_dict_get() -> i32 {
    let mut dict: *mut SparseDict = 0 as *mut () as *mut SparseDict;
    let mut outsize: u64 = 0 as u64;
    let mut value: *const i8 = 0 as *mut () as *const i8;
    dict = sparse_dict_init();
    if (dict).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 199) };
        return 0;
    }
    if (sparse_dict_set(
        dict,
        c"key".as_ptr() as *mut i8 as *const i8,
        unsafe { strlen(c"key".as_ptr() as *mut i8 as *const i8) } as u64,
        c"value".as_ptr() as *mut i8 as *const (),
        unsafe { strlen(c"value".as_ptr() as *mut i8 as *const i8) } as u64,
    ) == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 201) };
        return 0;
    }
    value = sparse_dict_get(
        unsafe { &*dict },
        c"key".as_ptr() as *mut i8 as *const i8,
        unsafe { strlen(c"key".as_ptr() as *mut i8 as *const i8) } as u64,
        &mut outsize,
    ) as *const i8;
    if (value).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 205) };
        return 0;
    }
    if !(outsize == unsafe { strlen(c"value".as_ptr() as *mut i8 as *const i8) }) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 206) };
        return 0;
    }
    if !(unsafe { strncmp(value, c"value".as_ptr() as *mut i8 as *const i8, outsize) } == 0) as i32
        != 0
    {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 207) };
        return 0;
    }
    if (sparse_dict_free(dict) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 209) };
        return 0;
    }
    return 1;
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn test_dict_lots_of_set() -> i32 {
    let mut dict: *mut SparseDict = 0 as *mut () as *mut SparseDict;
    let mut i: i32 = 0;
    dict = sparse_dict_init();
    if (dict).is_null() as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 218) };
        return 0;
    }
    let iterations: i32 = 1000000 as i32;
    {
        i = 0;
        '__b16: loop {
            if !(i < iterations) {
                break '__b16;
            }
            '__c16: loop {
                let mut key: [i8; 64] = [
                    0 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                unsafe {
                    __builtin___snprintf_chk(
                        &raw mut key[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 64]>() as u64,
                        0,
                        unsafe {
                            __builtin_object_size(
                                &raw mut key[0 as usize] as *mut i8 as *const (),
                                if 2 > 1 { 1 } else { 0 },
                            )
                        },
                        c"crazy hash%i".as_ptr() as *mut i8 as *const i8,
                        i,
                    )
                };
                let mut val: [i8; 64] = [
                    0 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                unsafe {
                    __builtin___snprintf_chk(
                        &raw mut val[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 64]>() as u64,
                        0,
                        unsafe {
                            __builtin_object_size(
                                &raw mut val[0 as usize] as *mut i8 as *const (),
                                if 2 > 1 { 1 } else { 0 },
                            )
                        },
                        c"value%i".as_ptr() as *mut i8 as *const i8,
                        i,
                    )
                };
                if (sparse_dict_set(
                    dict,
                    &raw mut key[0 as usize] as *mut i8 as *const i8,
                    unsafe { strlen(&raw mut key[0 as usize] as *mut i8 as *const i8) } as u64,
                    &raw mut val[0 as usize] as *mut i8 as *const (),
                    unsafe { strlen(&raw mut val[0 as usize] as *mut i8 as *const i8) } as u64,
                ) == 0) as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 228) };
                    return 0;
                }
                if !(unsafe { (*dict).bucket_count } == (i + 1) as u32 as u64) as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 229) };
                    return 0;
                }
                let mut outsize: u64 = 0 as u64;
                let retrieved_value: *const i8 = sparse_dict_get(
                    unsafe { &*dict },
                    &raw mut key[0 as usize] as *mut i8 as *const i8,
                    unsafe { strlen(&raw mut key[0 as usize] as *mut i8 as *const i8) } as u64,
                    &mut outsize,
                ) as *const i8;
                if (retrieved_value).is_null() as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 233) };
                    return 0;
                }
                if !(outsize == unsafe { strlen(&raw mut val[0 as usize] as *mut i8 as *const i8) })
                    as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 234) };
                    return 0;
                }
                if !(unsafe {
                    strncmp(
                        retrieved_value,
                        &raw mut val[0 as usize] as *mut i8 as *const i8,
                        outsize,
                    )
                } == 0) as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 235) };
                    return 0;
                }
                break '__c16;
            }
            i += 1;
        }
    }
    {
        i = iterations - 1;
        '__b17: loop {
            if !(i >= 0) {
                break '__b17;
            }
            '__c17: loop {
                let mut key: [i8; 64] = [
                    0 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                unsafe {
                    __builtin___snprintf_chk(
                        &raw mut key[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 64]>() as u64,
                        0,
                        unsafe {
                            __builtin_object_size(
                                &raw mut key[0 as usize] as *mut i8 as *const (),
                                if 2 > 1 { 1 } else { 0 },
                            )
                        },
                        c"crazy hash%i".as_ptr() as *mut i8 as *const i8,
                        i,
                    )
                };
                let mut val: [i8; 64] = [
                    0 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                unsafe {
                    __builtin___snprintf_chk(
                        &raw mut val[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 64]>() as u64,
                        0,
                        unsafe {
                            __builtin_object_size(
                                &raw mut val[0 as usize] as *mut i8 as *const (),
                                if 2 > 1 { 1 } else { 0 },
                            )
                        },
                        c"value%i".as_ptr() as *mut i8 as *const i8,
                        i,
                    )
                };
                /// CHECK YOUR SYSCALL RETURNS. Listen to djb.
                let mut outsize: u64 = 0 as u64;
                let retrieved_value: *const i8 = sparse_dict_get(
                    unsafe { &*dict },
                    &raw mut key[0 as usize] as *mut i8 as *const i8,
                    unsafe { strlen(&raw mut key[0 as usize] as *mut i8 as *const i8) } as u64,
                    &mut outsize,
                ) as *const i8;
                if (retrieved_value).is_null() as i32 != 0 {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 248) };
                    return 0;
                }
                if !(outsize == unsafe { strlen(&raw mut val[0 as usize] as *mut i8 as *const i8) })
                    as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 249) };
                    return 0;
                }
                if !(unsafe {
                    strncmp(
                        retrieved_value,
                        &raw mut val[0 as usize] as *mut i8 as *const i8,
                        outsize,
                    )
                } == 0) as i32
                    != 0
                {
                    unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 250) };
                    return 0;
                }
                break '__c17;
            }
            i -= 1;
        }
    }
    if (sparse_dict_free(dict) == 0) as i32 != 0 {
        unsafe { printf(c"%i: ".as_ptr() as *mut i8 as *const i8, 253) };
        return 0;
    }
    return 1;
}

pub(crate) extern "C" fn __main_inner(argc: i32, argv: *const *mut i8) -> Result<(), i32> {
    {
        let _ = argc;
    };
    {
        let _ = argv;
    };
    let mut test_return_val: i32 = 0;
    let mut tests_failed: i32 = 0;
    let mut tests_run: i32 = 0;
    test_return_val = test_cannot_set_bigger_elements();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_cannot_set_bigger_elements".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_cannot_set_bigger_elements".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_cannot_set_outside_bounds();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_cannot_set_outside_bounds".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_cannot_set_outside_bounds".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_cannot_get_outside_bounds();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_cannot_get_outside_bounds".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_cannot_get_outside_bounds".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_empty_array_does_not_blow_up();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_empty_array_does_not_blow_up".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_empty_array_does_not_blow_up".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_array_set();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_array_set".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_array_set".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_array_set_backwards();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_array_set_backwards".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_array_set_backwards".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_array_set_overwrites_old_values();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_array_set_overwrites_old_values".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_array_set_overwrites_old_values".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_array_set_high_num();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_array_set_high_num".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_array_set_high_num".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_array_get();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_array_get".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_array_get".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_dict_set();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_dict_set".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_dict_set".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_dict_get();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_dict_get".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_dict_get".as_ptr() as *mut i8,
            )
        };
    }
    test_return_val = test_dict_lots_of_set();
    if (test_return_val == 0) as i32 != 0 {
        tests_failed += 1;
        unsafe {
            printf(
                c"%c[%dmFailed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                31,
                27,
                0,
                c"test_dict_lots_of_set".as_ptr() as *mut i8,
            )
        };
    } else {
        tests_run += 1;
        unsafe {
            printf(
                c"%c[%dmPassed%c[%dm: %s\n".as_ptr() as *mut i8 as *const i8,
                27,
                32,
                27,
                0,
                c"test_dict_lots_of_set".as_ptr() as *mut i8,
            )
        };
    }
    unsafe {
        printf(
            c"\n-----\nTests passed: (%i/%i)\n".as_ptr() as *mut i8 as *const i8,
            tests_run,
            tests_run + tests_failed,
        )
    };
    return Ok(());
}
