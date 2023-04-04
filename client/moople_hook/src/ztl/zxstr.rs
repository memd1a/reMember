use std::ptr;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ZXStringData {
    ref_count: i32,
    cap: i32,
    byte_len: i32,
}

#[repr(C, packed)]
pub struct ZXString<T>(pub *const T);
pub type ZXString8 = ZXString<u8>;
pub type ZXString16 = ZXString<u16>;

impl<T> ZXString<T> {
    pub fn str_len(&self) -> usize {
        let ln = self.len();
        if ln > 0 {
            //Remove null terminator
            ln
        } else {
            0
        }
    }

    pub fn empty() -> Self {
        unsafe { Self::from_ptr(ptr::null()) }
    }

    pub unsafe fn from_ptr(ptr: *const T) -> Self {
        Self(ptr)
    }

    pub fn len(&self) -> usize {
        unsafe { self.get_str_data() }
            .map(|d| (d.byte_len as usize) / std::mem::size_of::<T>())
            .unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub unsafe fn get_str_data_ptr(&self) -> *const ZXStringData {
        std::mem::transmute(self.0.byte_sub(0xC))
    }

    pub unsafe fn get_str_data(&self) -> Option<&ZXStringData> {
        self.get_str_data_ptr().as_ref()
    }

    pub fn get_data(&self) -> &[T] {
        let ln = self.len();
        if ln > 0 {
            unsafe { std::slice::from_raw_parts(self.0, ln) }
        } else {
            &[]
        }
    }

    pub fn get_data_str(&self) -> &[T] {
        let data = self.get_data();
        let str_len = self.str_len();

        &data[..str_len]
    }
}



impl ZXString<u8> {
    pub fn get_str(&self) -> Option<&str> {
        std::str::from_utf8(self.get_data_str()).ok()
    }
}

impl ZXString<u16> {
    pub fn get_str_owned(&self) -> String {
        String::from_utf16_lossy(self.get_data_str())
    }
}