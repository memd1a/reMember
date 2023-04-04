//TODO: add dropping, right now array + string are leaking memory
// need a solution to allocate via the rust-allocator + in-process allocator
// and then call the matching de-allocator

pub mod zxstr;
pub mod zarr;
