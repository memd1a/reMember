use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::DerefMut,
    path::Path,
    sync::{LazyLock, Mutex, atomic::{AtomicPtr, Ordering}}, ptr,
};

use serde::{Deserialize, Serialize};

use crate::{PACKET_IN_FILE, PACKET_OUT_FILE, socket::{CInPacket, CPacket}};

/*
TODO exception handling
   for Decode: call    __CxxThrowException@8(might aswell check before decoding)

*/

#[derive(Debug, Serialize, Deserialize)]
pub enum PacketStructTy {
    I8,
    I16,
    I32,
    Buf(u32),
    Str(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketStructElem {
    ret_address: u32,
    ty: PacketStructTy,
    offset: usize,
}

impl PacketStructElem {
    pub fn i8(off: u32, ret: usize) -> Self {
        Self {
            offset: off as usize,
            ret_address: ret as u32,
            ty: PacketStructTy::I8,
        }
    }

    pub fn i16(off: u32, ret: usize) -> Self {
        Self {
            offset: off as usize,
            ret_address: ret as u32,
            ty: PacketStructTy::I16,
        }
    }

    pub fn i32(off: u32, ret: usize) -> Self {
        Self {
            offset: off as usize,
            ret_address: ret as u32,
            ty: PacketStructTy::I32,
        }
    }

    pub fn str(off: u32, ret: usize, ln: u32) -> Self {
        Self {
            offset: off as usize,
            ret_address: ret as u32,
            ty: PacketStructTy::Str(ln),
        }
    }

    pub fn buf(off: u32, ret: usize, ln: u32) -> Self {
        Self {
            offset: off as usize,
            ret_address: ret as u32,
            ty: PacketStructTy::Buf(ln),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PacketStruct {
    elements: Vec<PacketStructElem>,
    send_ret_addr: Option<u32>,
    ex_ret_addr: Option<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PacketStructLog<'a> {
    strct: PacketStruct,
    data: Option<&'a [u8]>,
}

#[derive(Debug)]
pub struct PacketStructContext {
    cur: Mutex<Option<PacketStruct>>,
    pkt_ptr: AtomicPtr<CInPacket>,
    out_file: Mutex<BufWriter<File>>,
}

impl PacketStructContext {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let out_file = Mutex::new(BufWriter::new(File::create(path).unwrap()));

        Self {
            cur: Mutex::default(),
            pkt_ptr: AtomicPtr::default(),
            out_file,
        }
    }

    pub fn clear(&self) {
        self.cur.lock().unwrap().take();
        self.store_ptr(ptr::null_mut());
    }

    pub fn add_elem(&self, _addr: usize, elem: PacketStructElem) {
        self.cur
            .lock()
            .unwrap()
            .get_or_insert_with(|| PacketStruct::default())
            .elements
            .push(elem);
    }

    fn finish_take(&self, _addr: usize) -> Option<PacketStruct> {
        self.cur.lock().unwrap().take()
    }

    pub fn store_ptr(&self, pkt_ptr: *mut CInPacket) {
        self.pkt_ptr.store(pkt_ptr, Ordering::SeqCst);
    }

    pub fn finish_incomplete(&self, pkt_addr: usize, ret_addr: usize) {
        let strct = self.finish_take(pkt_addr);
        let Some(mut strct) = strct else {
            println!("No packet for: {pkt_addr:?}");
            return;
        };
        let data = unsafe {
            self.pkt_ptr.load(Ordering::SeqCst).as_ref().map(|data| data.get_data())
        };


        strct.ex_ret_addr = Some(ret_addr as u32);
        self.write_to_file(strct, data).unwrap();
    }

    pub fn finish(&self, pkt_addr: usize, ret_addr: usize, data: &[u8]) {
        let strct = self.finish_take(pkt_addr);
        let Some(mut strct) = strct else {
            println!("No packet for: {pkt_addr:?}");
            return;
        };
        strct.send_ret_addr = Some(ret_addr as u32);
        self.write_to_file(strct, Some(data)).unwrap();
    }

    pub fn write_to_file(&self, strct: PacketStruct, data: Option<&[u8]>) -> anyhow::Result<()> {
        let mut file = self.out_file.lock().unwrap();
        serde_json::to_writer(file.deref_mut(), &PacketStructLog { strct, data })?;
        writeln!(file, ",")?;
        file.flush()?;

        Ok(())
    }

    /// Last resort flush when application is about to terminate
    pub fn flush(&self) -> anyhow::Result<()> {
        let pkt = self.cur.lock().unwrap().take();
        if let Some(pkt) = pkt {
            //TODO: maybe try to resolve pointer to the packet and get the data
            self.write_to_file(pkt, None)?;
        }

        Ok(())
    }
}

pub static SEND_PACKET_CTX: LazyLock<PacketStructContext> =
    LazyLock::new(|| PacketStructContext::new(PACKET_OUT_FILE));
pub static RECV_PACKET_CTX: LazyLock<PacketStructContext> =
    LazyLock::new(|| PacketStructContext::new(PACKET_IN_FILE));
