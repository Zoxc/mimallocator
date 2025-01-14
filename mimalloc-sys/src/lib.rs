//! Raw FFI wrapper over the mimalloc memory allocator
#![no_std]
use libc::{c_void, size_t};

extern "C" {
    pub fn mi_malloc(size: size_t) -> *mut c_void;
    pub fn mi_calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_expand(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_free(p: *mut c_void);
    pub fn mi_malloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
}
