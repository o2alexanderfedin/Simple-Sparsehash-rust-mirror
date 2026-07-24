use super::*;
use crate::gen::include::simple_sparsehash_h::{
    SparseArray, SparseArrayGroup, SparseBucket, SparseDict,
};

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn sparse_array_init(element_size: u64, maximum: u32)
    -> *mut SparseArray {
    let mut i: u32 = 0 as u32;
    let mut arr: *mut SparseArray = 0 as *mut () as *mut SparseArray;

    /// CHECK YOUR SYSCALL RETURNS. Listen to djb.
    (arr =
        unsafe {
                calloc(1 as u64, core::mem::size_of::<SparseArray>() as u64)
            } as *mut SparseArray);
    if arr as *mut () == 0 as *mut () {
        return 0 as *mut () as *mut SparseArray;
    }
    /// This is a non-obvious hack I use. If we have const variables in a
    ///struct then to initialize them we can either cast them or use an
    ///initializer like this.
    ///Then we copy it into a heap-allocated blob. The compiler lets us
    ///do this.
    let mut stack_array: SparseArray =
        SparseArray {
            maximum: maximum as u64,
            groups: core::ptr::null_mut(),
        };
    unsafe {
        __builtin___memcpy_chk(arr as *mut (),
            &raw mut stack_array as *const (),
            core::mem::size_of::<SparseArray>() as u64,
            unsafe { __builtin_object_size(arr as *const (), 0) })
    };
    unsafe {
        (*arr).groups =
            unsafe {
                    calloc((unsafe { (*arr).maximum.wrapping_sub(1 as u64) } /
                                48 as u64).wrapping_add(1 as u64),
                        core::mem::size_of::<SparseArrayGroup>() as u64)
                } as *mut SparseArrayGroup
    };
    if unsafe { (*arr).groups } as *mut () == 0 as *mut () {
        unsafe { free(arr as *mut ()) };
        return 0 as *mut () as *mut SparseArray;
    }
    {
        i = 0 as u32;
        '__b0: loop {
            if !((i as u64) <
                            (unsafe { (*arr).maximum.wrapping_sub(1 as u64) } /
                                    48 as u64).wrapping_add(1 as u64)) {
                break '__b0;
            }
            '__c0: loop {
                let sag: &mut SparseArrayGroup =
                    unsafe { &mut *unsafe { (*arr).groups.add(i as usize) } };
                (*sag).elem_size = element_size as u64;
                break '__c0;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    return arr;
}

#[inline]
/// This is one of the popcount implementations from Wikipedia.
///http://en.wikipedia.org/wiki/Hamming_weight
extern "C" fn popcount_32(mut x: u32) -> u32 {
    let m1: u32 = 1431655765 as u32;
    let m2: u32 = 858993459 as u32;
    let m4: u32 = 252645135 as u32;
    x = x.wrapping_sub(x >> 1 & m1 as u32);
    x = (x & m2 as u32).wrapping_add(x >> 2 & m2 as u32);
    x = x.wrapping_add(x >> 4) & m4 as u32;
    x = x.wrapping_add(x >> 8);
    return x.wrapping_add(x >> 16) & 63 as u32;
}

/// This function is used to map an item's 'position' (the user-facing index
///into the array) with the 'offset' which is the actual position in the
///array, memory-wise.
///
///The way we do this is by counting the number of 1s in the bitmap from
///0 .. i-1 in the bitmap. The original implementation uses a big table for the
///popcount.
#[allow(unused_doc_comments)]
extern "C" fn position_to_offset(bitmap: *const u32, position: u32) -> u32 {
    let mut retval: u32 = 0 as u32;
    let mut pos: u32 = position as u32;
    let mut bitmap_iter: u32 = 0 as u32;
    {
        '__b1: loop {
            if !(pos as u64 >=
                            (core::mem::size_of::<u32>() as u64).wrapping_mul(8 as u64))
                {
                break '__b1;
            }
            '__c1: loop {
                retval =
                    retval.wrapping_add(popcount_32(unsafe {
                                *bitmap.add({
                                                let __p = &mut bitmap_iter;
                                                let __t = *__p;
                                                *__p = (*__p).wrapping_add(1);
                                                __t
                                            } as usize)
                            }));
                break '__c1;
            }
            pos =
                pos.wrapping_sub((core::mem::size_of::<u32>() as
                                u64).wrapping_mul(8 as u64) as u32);
        }
    }

    /// This last bit does the same thing as above, but takes care of the
    ///remainder that didn't fit cleanly into the 32 x 32 x 32 ... loop above. That
    ///is to say, it grabs the last 0 - 7 bits and adds the number of 1s in it to
    ///retval.
    return retval.wrapping_add(popcount_32(unsafe {
                            *bitmap.add(bitmap_iter as usize)
                        } & ((1 as u32) << pos).wrapping_sub(1))) as u32;
}

/// TODO: Figure out better names for charbit/modbit
#[allow(unused_doc_comments)]
extern "C" fn charbit(position: u32) -> u32 {

    /// Get enough bits to store 0 - 31.
    return (position >> 5) as u32;
}

#[allow(unused_doc_comments)]
extern "C" fn modbit(position: u32) -> u32 {

    /// Get the number of bits of this number that are 0 - 31,
    ///or something like that.
    return (1 << (position & 31 as u32)) as u32;
}

/// Simple check to see whether a slot in the array is occupied or not.
extern "C" fn is_position_occupied(bitmap: *const u32, position: u32) -> i32 {
    return (unsafe { *bitmap.add(charbit(position as u32) as usize) } &
                modbit(position as u32)) as i32;
}

extern "C" fn set_position(bitmap: *mut u32, position: u32) -> () {
    unsafe {
        *bitmap.add(charbit(position as u32) as usize) |=
            modbit(position as u32)
    };
}

/// Sparse Array
#[allow(unused_doc_comments)]
extern "C" fn _sparse_array_group_set(arr: &mut SparseArrayGroup, i: u32,
    val: *const (), vlen: u64) -> i32 {
    let mut offset: u32 = 0 as u32;
    let mut destination: *mut () = 0 as *mut ();
    if vlen as u64 > (*arr).elem_size { return 0 as i32; }

    /// So what needs to happen in this function:
    ///1. Convert the position (i) to the 'offset'
    ///2. Check to see if this slot is already occupied (bmtest).
    ///   overwrite the old element if this is the case.
    ///3. Otherwise, expand the array by a single element and increase
    ///   our bucket count (arr->count). Finally, OR the bit in our state
    ///   bitmap that shows this position is occupied.
    ///4. After doing all that, create a copy of val and stick it in the right
    ///   position in our array.
    (offset =
        position_to_offset(&raw mut (*arr).bitmap[0 as usize] as *mut u32 as
                *const u32, i as u32));
    if (is_position_occupied(&raw mut (*arr).bitmap[0 as usize] as *mut u32 as
                            *const u32, i as u32) == 0) as i32 != 0 {
        let to_move_siz: u64 =
            ((*arr).count.wrapping_sub(offset) as
                        u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                            as u64)) as u64;
        /// Reallocate the array to hold the new item
        let new_group: *mut () =
            unsafe {
                realloc((*arr).group,
                    ((*arr).count.wrapping_add(1 as u32) as
                            u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                                as u64)))
            };
        if new_group == 0 as *mut () { return 0 as i32; }
        if to_move_siz as u64 > 0 as u64 {
            unsafe {
                __builtin___memmove_chk(unsafe {
                            (new_group as
                                    *mut u8).add((offset.wrapping_add(1 as u32) as
                                            u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                                                as u64)) as usize)
                        } as *mut (),
                    unsafe {
                            (new_group as
                                    *mut u8).add((offset as
                                            u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                                                as u64)) as usize)
                        } as *const (), to_move_siz,
                    unsafe {
                        __builtin_object_size(unsafe {
                                    (new_group as
                                            *mut u8).add((offset.wrapping_add(1 as u32) as
                                                    u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                                                        as u64)) as usize)
                                } as *const (), 0)
                    })
            };
        }

        /// Increase the bucket count because we've expanded:
        {
            let __p = &mut (*arr).count;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        };
        (*arr).group = new_group;

        /// Remember to modify the bitmap:
        set_position(&raw mut (*arr).bitmap[0 as usize] as *mut u32,
            i as u32);
    }

    /// Copy the size into the position, fighting -pedantic the whole
    ///time.
    (destination =
        unsafe {
                ((*arr).group as
                        *mut u8).add((offset as
                                u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                                    as u64)) as usize)
            } as *mut ());
    unsafe {
        __builtin___memcpy_chk(destination, &raw const vlen as *const (),
            core::mem::size_of::<u64>() as u64,
            unsafe { __builtin_object_size(destination as *const (), 0) })
    };

    /// Here we mutate a variable because we're writing C and we don't respect
    ///anything.
    (destination =
        unsafe {
                (destination as
                        *mut u8).add(core::mem::size_of::<u64>() as usize)
            } as *mut ());
    unsafe {
        __builtin___memcpy_chk(destination, val, vlen,
            unsafe { __builtin_object_size(destination as *const (), 0) })
    };
    return 1 as i32;
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn sparse_array_set(arr: &SparseArray, i: u32,
    val: *const (), vlen: u64) -> i32 {
    if i as u64 > (*arr).maximum { return 0 as i32; }
    /// Since our hashtable is divided into many arrays, we need to pick the one
    ///relevant to `i` in this case:
    let operating_group: *mut SparseArrayGroup =
        unsafe { &mut *(*arr).groups.add((i / 48 as u32) as usize) };
    let position: i32 = (i % 48 as u32) as i32;
    return _sparse_array_group_set(unsafe { &mut *operating_group },
                position as u32, val, vlen as u64) as i32;
}

extern "C" fn _sparse_array_group_get(arr: &mut SparseArrayGroup, i: u32,
    outsize: *mut u64) -> *const () {
    let offset: u32 =
        position_to_offset(&raw mut (*arr).bitmap[0 as usize] as *mut u32 as
                    *const u32, i as u32) as u32;
    let item_siz: *const u8 =
        unsafe {
                ((*arr).group as
                        *mut u8).add((offset as
                                u64).wrapping_mul((*arr).elem_size.wrapping_add(core::mem::size_of::<u64>()
                                    as u64)) as usize)
            } as *const u8;
    let item: *const () =
        unsafe { item_siz.add(core::mem::size_of::<u64>() as usize) } as
            *const ();
    if (is_position_occupied(&raw mut (*arr).bitmap[0 as usize] as *mut u32 as
                            *const u32, i as u32) == 0) as i32 != 0 {
        return 0 as *mut () as *const ();
    }
    if unsafe { *(item_siz as *mut u64) } == 0 as u64 {
        return 0 as *mut () as *const ();
    }
    if !(outsize).is_null() {
        unsafe {
            __builtin___memcpy_chk(outsize as *mut (), item_siz as *const (),
                core::mem::size_of::<u64>() as u64,
                unsafe { __builtin_object_size(outsize as *const (), 0) })
        };
    }
    return item;
}

pub(crate) extern "C" fn sparse_array_get(arr: &SparseArray, i: u32,
    outsize: *mut u64) -> *const () {
    if i as u64 > (*arr).maximum { return 0 as *mut () as *const (); }
    let operating_group: *mut SparseArrayGroup =
        unsafe { &mut *(*arr).groups.add((i / 48 as u32) as usize) };
    let position: i32 = (i % 48 as u32) as i32;
    return _sparse_array_group_get(unsafe { &mut *operating_group },
            position as u32, outsize);
}

extern "C" fn _sparse_array_group_free(arr: &SparseArrayGroup) -> i32 {
    unsafe { free((*arr).group) };
    return 1 as i32;
}

pub(crate) extern "C" fn sparse_array_free(arr: *mut SparseArray) -> i32 {
    let mut i: u32 = 0 as u32;
    {
        '__b2: loop {
            if !((i as u64) <
                            (unsafe { (*arr).maximum.wrapping_sub(1 as u64) } /
                                    48 as u64).wrapping_add(1 as u64)) {
                break '__b2;
            }
            '__c2: loop {
                let sag: *mut SparseArrayGroup =
                    unsafe { &mut *unsafe { (*arr).groups.add(i as usize) } };
                _sparse_array_group_free(unsafe { &*sag });
                break '__c2;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    unsafe { free(unsafe { (*arr).groups } as *mut ()) };
    unsafe { free(arr as *mut ()) };
    return 1 as i32;
}

pub(crate) extern "C" fn sparse_dict_init() -> *mut SparseDict {
    let mut new: *mut SparseDict = 0 as *mut () as *mut SparseDict;
    '__b3: loop {
        '__c3: loop {
            new =
                unsafe {
                        calloc(1 as u64, core::mem::size_of::<SparseDict>() as u64)
                    } as *mut SparseDict;
            if new as *mut () == 0 as *mut () {
                return 0 as *mut () as *mut SparseDict;
            }
            unsafe { (*new).bucket_max = 32 as u64 };
            unsafe { (*new).bucket_count = 0 as u64 };
            unsafe {
                (*new).buckets =
                    sparse_array_init(core::mem::size_of::<SparseBucket>() as
                            u64, 32 as u32)
            };
            if unsafe { (*new).buckets } as *mut () == 0 as *mut () {
                break '__b3;
            }
            return new;
            break '__c3;
        }
        if !(false) { break '__b3; }
    }
    unsafe { free(new as *mut ()) };
    return 0 as *mut () as *mut SparseDict;
}

/// One of the simplest hashing functions, FNV-1a. See the wikipedia article for more info:
///http://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
extern "C" fn hash_fnv1a(key: *const i8, klen: u64) -> u64 {
    let iterations: i32 = klen as i32;
    let mut i: u8 = 0 as u8;
    let mut hash: u64 = fnv_offset_bias as u64;
    {
        i = 0 as u8;
        '__b4: loop {
            if !((i as i32) < iterations) { break '__b4; }
            '__c4: loop {
                hash = hash ^ unsafe { *key.add(i as usize) } as u64;
                hash = hash.wrapping_mul(fnv_prime as u64);
                break '__c4;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    return hash as u64;
}

extern "C" fn _create_and_insert_new_bucket(array: *mut SparseArray, i: u32,
    key: *const i8, klen: u64, value: &[u8], key_hash: u64) -> i32 {
    let mut copied_value: *mut () = 0 as *mut ();
    '__b5: loop {
        '__c5: loop {
            let mut copied_key: *mut i8 = 0 as *mut () as *mut i8;
            copied_value =
                unsafe { malloc(value.len() as u64 + klen as u64) };
            if copied_value == 0 as *mut () { break '__b5; }
            unsafe {
                __builtin___memcpy_chk(copied_value,
                    value.as_ptr() as *const (), value.len() as u64,
                    unsafe {
                        __builtin_object_size(copied_value as *const (), 0)
                    })
            };
            copied_key =
                unsafe { copied_value.add(value.len() as u64 as usize) } as
                    *mut i8;
            unsafe {
                __builtin___strncpy_chk(copied_key, key, klen,
                    unsafe {
                        __builtin_object_size(copied_key as *const (),
                            if 2 > 1 { 1 } else { 0 })
                    })
            };
            let mut bct: SparseBucket =
                SparseBucket {
                    key: copied_key,
                    klen: klen,
                    val: copied_value,
                    vlen: value.len() as u64,
                    hash: key_hash,
                };
            if (sparse_array_set(unsafe { &*array }, i as u32,
                                &raw mut bct as *const (),
                                core::mem::size_of::<SparseBucket>() as u64) == 0) as i32 !=
                    0 {
                break '__b5;
            }
            return 1 as i32;
            break '__c5;
        }
        if !(false) { break '__b5; }
    }
    unsafe { free(copied_value) };
    return 0 as i32;
}

#[allow(unused_doc_comments)]
extern "C" fn _rehash_and_grow_table(dict: &mut SparseDict) -> i32 {
    /// We've reached our chosen 'rehash the table' point, so
    ///we need to resize the table now.
    let mut i: u32 = 0 as u32;
    let mut buckets_rehashed: u32 = 0 as u32;
    let mut new_bucket_max: u64 = 0 as u64;
    let mut new_buckets: *mut SparseArray = core::ptr::null_mut();
    /// Loop through each bucket and stick it into the new array.
    let mut bucket_siz: u64 = 0 as u64;
    let mut bucket: *const SparseBucket = core::ptr::null();
    /// We found a bucket.
    let mut probed_val: u32 = 0 as u32;
    let mut num_probes: u32 = 0 as u32;
    let mut key_hash: u64 = 0 as u64;
    /// Quadratically probe along the hash table for an empty slot.
    let mut current_value_siz: u64 = 0 as u64;
    let mut current_value: *const () = core::ptr::null();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s7:
            {
            match __state {
                0 => {
                    i = 0 as u32;
                    buckets_rehashed = 0 as u32;
                    __state = 3;
                }
                2 => {
                    if !(new_buckets).is_null() {
                        __state = 36;
                    } else { __state = 35; }
                }
                3 => {
                    new_bucket_max =
                        (*dict).bucket_max.wrapping_mul(2 as u64) as u64;
                    __state = 4;
                }
                4 => {
                    new_buckets = 0 as *mut () as *mut SparseArray;
                    __state = 5;
                }
                5 => {
                    new_buckets =
                        sparse_array_init(core::mem::size_of::<SparseBucket>() as
                                u64, new_bucket_max as u32);
                    __state = 6;
                }
                6 => {
                    if new_buckets as *mut () == 0 as *mut () {
                        __state = 8;
                    } else { __state = 7; }
                }
                7 => { i = 0 as u32; __state = 10; }
                8 => { __state = 2; }
                9 => { sparse_array_free((*dict).buckets); __state = 31; }
                10 => {
                    if (i as u64) < (*dict).bucket_max {
                        __state = 11;
                    } else { __state = 9; }
                }
                11 => { bucket_siz = 0 as u64; __state = 13; }
                12 => {
                    {
                        let __p = &mut i;
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 10;
                }
                13 => {
                    bucket =
                        sparse_array_get(unsafe { &*(*dict).buckets }, i as u32,
                                &mut bucket_siz) as *const SparseBucket;
                    __state = 14;
                }
                14 => {
                    if bucket_siz != 0 as u64 &&
                            bucket as *mut () != 0 as *mut () {
                        __state = 16;
                    } else { __state = 15; }
                }
                15 => {
                    if buckets_rehashed as u64 == (*dict).bucket_count {
                        __state = 30;
                    } else { __state = 12; }
                }
                16 => {
                    probed_val = 0 as u32;
                    num_probes = 0 as u32;
                    __state = 17;
                }
                17 => {
                    key_hash = unsafe { (*bucket).hash } as u64;
                    __state = 18;
                }
                18 => { if 1 != 0 { __state = 20; } else { __state = 19; } }
                19 => {
                    if (sparse_array_set(unsafe { &*new_buckets },
                                        probed_val as u32, bucket as *const (),
                                        core::mem::size_of::<SparseBucket>() as u64) == 0) as i32 !=
                            0 {
                        __state = 29;
                    } else { __state = 28; }
                }
                20 => {
                    probed_val =
                        (key_hash.wrapping_add(num_probes.wrapping_mul(num_probes)
                                        as u64) & new_bucket_max.wrapping_sub(1 as u64) as u64) as
                            u32;
                    __state = 21;
                }
                21 => { current_value_siz = 0 as u64; __state = 22; }
                22 => {
                    current_value =
                        sparse_array_get(unsafe { &*new_buckets },
                            probed_val as u32, &mut current_value_siz);
                    __state = 23;
                }
                23 => {
                    if current_value_siz == 0 as u64 &&
                            current_value == 0 as *mut () as *const () {
                        __state = 25;
                    } else { __state = 24; }
                }
                24 => {
                    if num_probes as u64 > (*dict).bucket_count {
                        __state = 27;
                    } else { __state = 26; }
                }
                25 => { __state = 19; }
                26 => {
                    {
                        let __p = &mut num_probes;
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 18;
                }
                27 => { __state = 2; }
                28 => {
                    {
                        let __p = &mut buckets_rehashed;
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 15;
                }
                29 => { __state = 2; }
                30 => { __state = 9; }
                31 => { (*dict).buckets = new_buckets; __state = 32; }
                32 => {
                    (*dict).bucket_max = new_bucket_max as u64;
                    __state = 33;
                }
                33 => { return 1 as i32; }
                34 => { __state = 2; }
                35 => { return 0 as i32; }
                36 => { sparse_array_free(new_buckets); __state = 35; }
                _ => {}
            }
        }
    }

    /// We've reached our chosen 'rehash the table' point, so
    ///we need to resize the table now.
    /// Loop through each bucket and stick it into the new array.
    /// We found a bucket.
    /// Quadratically probe along the hash table for an empty slot.
    /// If the following ever happens, there are deeply troubling
    ///things that no longer make sense in the universe.
    /// Short circuit to see if we can quit early:
    /// Finally, swap out the old array with the new one:
    unreachable!();
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn sparse_dict_set(dict: *mut SparseDict,
    key: *const i8, klen: u64, value: *const (), vlen: u64) -> i32 {
    let mut key_hash: u64 = 0 as u64;
    let mut num_probes: u32 = 0 as u32;
    /// First check the array to see if we have an object already stored in
    ///'out' position.
    let mut current_value_siz: u64 = 0 as u64;
    /// Use quadratic probing here to insert into the table.
    ///Further reading: https://en.wikipedia.org/wiki/Quadratic_probing
    let mut probed_val: u32 = 0 as u32;
    let mut current_value: *const () = core::ptr::null();
    /// Awesome, the slot we want is empty. Insert as normal.
    /// We found a bucket. Check to see if it has the same key as we do.
    let mut existing_bucket: *const SparseBucket = core::ptr::null();
    /// Great, we probed along the hashtable and found a bucket with the same key as
    ///the key we want to insert. Replace it.
    let mut existing_key: *mut i8 = core::ptr::null_mut();
    let mut existing_val: *mut () = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s9:
            {
            match __state {
                0 => {
                    key_hash = hash_fnv1a(key, klen as u64) as u64;
                    __state = 3;
                }
                2 => { return 0 as i32; }
                3 => { num_probes = 0 as u32; __state = 4; }
                4 => { if 1 != 0 { __state = 6; } else { __state = 5; } }
                5 => {
                    {
                        let __p = unsafe { &mut (*dict).bucket_count };
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 26;
                }
                6 => { current_value_siz = 0 as u64; __state = 7; }
                7 => {
                    probed_val =
                        (key_hash.wrapping_add(num_probes.wrapping_mul(num_probes)
                                        as u64) &
                                unsafe { (*dict).bucket_max.wrapping_sub(1 as u64) } as u64)
                            as u32;
                    __state = 8;
                }
                8 => {
                    current_value =
                        sparse_array_get(unsafe { &*unsafe { (*dict).buckets } },
                            probed_val as u32, &mut current_value_siz);
                    __state = 9;
                }
                9 => {
                    if current_value_siz == 0 as u64 &&
                            current_value == 0 as *mut () as *const () {
                        __state = 11;
                    } else { __state = 12; }
                }
                10 => {
                    {
                        let __p = &mut num_probes;
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 23;
                }
                11 => {
                    if _create_and_insert_new_bucket(unsafe { (*dict).buckets },
                                probed_val as u32, key, klen as u64,
                                unsafe {
                                    let __p = value as *const u8 as *const u8;
                                    if __p.is_null() {
                                        &[]
                                    } else {
                                        core::slice::from_raw_parts(__p, vlen as u64 as usize)
                                    }
                                }, key_hash as u64) != 0 {
                        __state = 13;
                    } else { __state = 14; }
                }
                12 => {
                    existing_bucket =
                        current_value as *mut SparseBucket as *const SparseBucket;
                    __state = 15;
                }
                13 => { __state = 5; }
                14 => { __state = 2; }
                15 => {
                    if unsafe { (*existing_bucket).hash } as u64 == key_hash &&
                                unsafe { (*existing_bucket).klen } as u64 == klen &&
                            unsafe {
                                    strncmp(unsafe { (*existing_bucket).key } as *const i8, key,
                                        klen)
                                } == 0 {
                        __state = 16;
                    } else { __state = 10; }
                }
                16 => {
                    existing_key = unsafe { (*existing_bucket).key };
                    __state = 17;
                }
                17 => {
                    existing_val = unsafe { (*existing_bucket).val };
                    __state = 18;
                }
                18 => {
                    if _create_and_insert_new_bucket(unsafe { (*dict).buckets },
                                probed_val as u32, key, klen as u64,
                                unsafe {
                                    let __p = value as *const u8 as *const u8;
                                    if __p.is_null() {
                                        &[]
                                    } else {
                                        core::slice::from_raw_parts(__p, vlen as u64 as usize)
                                    }
                                }, key_hash as u64) != 0 {
                        __state = 19;
                    } else { __state = 20; }
                }
                19 => {
                    unsafe { free(existing_key as *mut ()) };
                    __state = 21;
                }
                20 => { __state = 2; }
                21 => { unsafe { free(existing_val) }; __state = 22; }
                22 => { return 1 as i32; }
                23 => {
                    if num_probes as u64 > unsafe { (*dict).bucket_count } {
                        __state = 24;
                    } else { __state = 4; }
                }
                24 => {
                    unsafe {
                        printf(c"Could not find an open slot in the table.\n".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    __state = 25;
                }
                25 => { __state = 2; }
                26 => {
                    if unsafe { (*dict).bucket_count } as f32 /
                                unsafe { (*dict).bucket_max } as f32 >= 80 as f32 / 100.0 {
                        __state = 28;
                    } else { __state = 27; }
                }
                27 => { return 1 as i32; }
                28 => {
                    return _rehash_and_grow_table(unsafe { &mut *dict }) as i32;
                }
                29 => { __state = 2; }
                _ => {}
            }
        }
    }

    /// First check the array to see if we have an object already stored in
    ///'out' position.
    /// Use quadratic probing here to insert into the table.
    ///Further reading: https://en.wikipedia.org/wiki/Quadratic_probing
    /// Awesome, the slot we want is empty. Insert as normal.
    /// We found a bucket. Check to see if it has the same key as we do.
    /// Great, we probed along the hashtable and found a bucket with the same key as
    ///the key we want to insert. Replace it.
    /// We return here because we don't want to execute the 'resize the table'
    ///logic. We overwrote a bucket instead of adding a new one, so we know
    ///we don't need to resize anything.
    /// If this ever happens something has gone very, very wrong.
    ///The hash table is full.
    /// See if we've hit our 'we should rehash the table' occupancy number:
    unreachable!();
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn sparse_dict_get(dict: &SparseDict, key: *const i8,
    klen: u64, outsize: *mut u64) -> *const () {
    let key_hash: u64 = hash_fnv1a(key, klen as u64) as u64;
    let mut num_probes: u32 = 0 as u32;
    loop {
        let mut current_value_siz: u64 = 0 as u64;
        let probed_val: u32 =
            (key_hash.wrapping_add(num_probes.wrapping_mul(num_probes) as u64)
                    & (*dict).bucket_max.wrapping_sub(1 as u64) as u64) as u32;
        let current_value: *const () =
            sparse_array_get(unsafe { &*(*dict).buckets }, probed_val as u32,
                &mut current_value_siz);
        if current_value_siz != 0 as u64 &&
                current_value != 0 as *mut () as *const () {
            /// We have to do a string comparison here because we use quadratic probing.
            ///The value we pulled from the underlying array could be anything.
            let existing_bucket: *mut SparseBucket =
                current_value as *mut SparseBucket;
            if unsafe { (*existing_bucket).hash } as u64 == key_hash &&
                        unsafe { (*existing_bucket).klen } as u64 == klen &&
                    unsafe {
                            strncmp(unsafe { (*existing_bucket).key } as *const i8, key,
                                klen)
                        } == 0 {
                if !(outsize).is_null() {
                    unsafe {
                        __builtin___memcpy_chk(outsize as *mut (),
                            unsafe { &raw const (*existing_bucket).vlen } as *const (),
                            core::mem::size_of::<u64>() as u64,
                            unsafe { __builtin_object_size(outsize as *const (), 0) })
                    };
                }
                return unsafe { (*existing_bucket).val } as *const ();
            }
        } else {

            /// We found nothing where we expected something.
            return 0 as *mut () as *const ();
        }
        {
            let __p = &mut num_probes;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        };
        if num_probes as u64 > (*dict).bucket_count {
            return 0 as *mut () as *const ();
        }
    }
    return 0 as *mut () as *const ();
}

pub(crate) extern "C" fn sparse_dict_free(dict: *mut SparseDict) -> i32 {
    let mut i: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b11: loop {
            if !((i as u64) < unsafe { (*dict).bucket_max }) { break '__b11; }
            '__c11: loop {
                let mut current_value_siz: u64 = 0 as u64;
                let current_value: *const () =
                    sparse_array_get(unsafe { &*unsafe { (*dict).buckets } },
                        i as u32, &mut current_value_siz);
                if current_value_siz != 0 as u64 &&
                        current_value != 0 as *mut () as *const () {
                    let existing_bucket: *const SparseBucket =
                        current_value as *mut SparseBucket as *const SparseBucket;
                    unsafe { free(unsafe { (*existing_bucket).val }) };
                }
                break '__c11;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p = (*__p).wrapping_add(1);
                __t
            };
        }
    }
    sparse_array_free(unsafe { (*dict).buckets });
    unsafe { free(dict as *mut ()) };
    return 1 as i32;
}

static fnv_prime: u64 = 1099511628211u64 as u64;

static fnv_offset_bias: u64 = 14695981039346656037u64 as u64;
