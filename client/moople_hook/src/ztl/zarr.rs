use std::ptr;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct ZArray<T>(*const T);

#[derive(Debug)]
#[repr(C, packed)]
pub struct ZArrayData {
    ref_count: i32,
    cap: i32,
    byte_len: i32,
}

impl<T> ZArray<T> {
    pub fn empty() -> Self {
        unsafe { Self::from_ptr(ptr::null()) }
    }

    pub unsafe fn from_ptr(ptr: *const T) -> Self {
        Self(ptr)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn len(&self) -> usize {
        unsafe { self.get_arr_data() }
            .map(|d| (d.byte_len as usize) / std::mem::size_of::<T>())
            .unwrap_or(0)
    }

    pub unsafe fn get_arr_data_ptr(&self) -> *const ZArrayData {
        std::mem::transmute(self.0.byte_sub(0xC))
    }

    pub unsafe fn get_arr_data(&self) -> Option<&ZArrayData> {
        self.get_arr_data_ptr().as_ref()
    }

    pub fn get_data(&self) -> &[T] {
        let ln = self.len();
        if ln > 0 {
            unsafe { std::slice::from_raw_parts(self.0, ln) }
        } else {
            &[]
        }
    }
}