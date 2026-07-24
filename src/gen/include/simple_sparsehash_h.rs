use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SparseBucket {
    pub(crate) key: *mut i8,
    pub(crate) klen: u64,
    pub(crate) val: *mut (),
    pub(crate) vlen: u64,
    pub(crate) hash: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SparseArrayGroup {
    pub(crate) count: u32,
    pub(crate) elem_size: u64,
    pub(crate) group: *mut (),
    pub(crate) bitmap: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SparseArray {
    pub(crate) maximum: u64,
    pub(crate) groups: *mut SparseArrayGroup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SparseDict {
    pub(crate) bucket_max: u64,
    pub(crate) bucket_count: u64,
    pub(crate) buckets: *mut SparseArray,
}
