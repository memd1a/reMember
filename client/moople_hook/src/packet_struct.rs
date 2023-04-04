use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::DerefMut,
    path::Path,
    ptr,
    sync::{
        atomic::{AtomicPtr, Ordering},
        LazyLock, Mutex,
    }, collections::HashMap,
};

use serde::{Deserialize, Serialize};

use crate::{
    config,
    socket::{CInPacket, CPacket},
};

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
    pub fn i8(off: usize, ret: usize) -> Self {
        Self {
            offset: off,
            ret_address: ret as u32,
            ty: PacketStructTy::I8,
        }
    }

    pub fn i16(off: usize, ret: usize) -> Self {
        Self {
            offset: off,
            ret_address: ret as u32,
            ty: PacketStructTy::I16,
        }
    }

    pub fn i32(off: usize, ret: usize) -> Self {
        Self {
            offset: off,
            ret_address: ret as u32,
            ty: PacketStructTy::I32,
        }
    }

    pub fn str(off: usize, ret: usize, ln: usize) -> Self {
        Self {
            offset: off,
            ret_address: ret as u32,
            ty: PacketStructTy::Str(ln as u32),
        }
    }

    pub fn buf(off: usize, ret: usize, ln: usize) -> Self {
        Self {
            offset: off,
            ret_address: ret as u32,
            ty: PacketStructTy::Buf(ln as u32),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PacketStruct {
    elements: Vec<PacketStructElem>,
    send_ret_addr: Option<u32>,
    ex_ret_addr: Option<u32>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PacketStructLog<'a> {
    strct: PacketStruct,
    data: Option<&'a [u8]>,
}

#[derive(Debug)]
pub struct PacketStructContext {
    pkt_ptr: AtomicPtr<CInPacket>,
    out_file: Mutex<BufWriter<File>>,
    packets: Mutex<HashMap<usize, PacketStruct>>
}

impl PacketStructContext {
    pub fn new(path: impl AsRef<Path>) -> Self {
        dbg!(path.as_ref());
        let out_file = Mutex::new(BufWriter::new(File::create(path).unwrap()));

        Self {
            packets: Mutex::default(),
            pkt_ptr: AtomicPtr::default(),
            out_file,
        }
    }

    pub fn clear(&self) {
        self.store_ptr(ptr::null_mut());
    }

    pub fn add_elem(&self, packet_ptr: usize, elem: PacketStructElem) {
        self.packets.lock().expect("packets")
            .entry(packet_ptr)
            .or_default()
            .elements
            .push(elem);
    }

    fn finish_take(&self, packet_ptr: usize) -> Option<PacketStruct> {
        self.packets.lock().expect("packets").remove(&packet_ptr)
    }

    pub fn store_ptr(&self, pkt_ptr: *mut CInPacket) {
        self.pkt_ptr.store(pkt_ptr, Ordering::SeqCst);
    }

    pub fn finish_incomplete(&self, pkt_addr: usize, ret_addr: usize) {
        let strct = self.finish_take(pkt_addr);
        let Some(mut strct) = strct else {
            log::error!("No packet for: {pkt_addr:?} @ {ret_addr:X}");
            return;
        };
        let data = unsafe {
            self.pkt_ptr
                .load(Ordering::SeqCst)
                .as_ref()
                .map(|data| data.get_data())
        };

        strct.ex_ret_addr = Some(ret_addr as u32);
        self.write_to_file(strct, data).unwrap();
    }

    pub fn finish(&self, pkt_addr: usize, ret_addr: usize, data: &[u8]) {
        let strct = self.finish_take(pkt_addr);
        let Some(mut strct) = strct else {
            log::error!("No packet for: {pkt_addr:?} @ {ret_addr:X}");
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
    pub fn flush(&self, packet_ptr: usize) -> anyhow::Result<()> {
        let pkt = self.packets.lock().expect("packets").remove(&packet_ptr);
        if let Some(pkt) = pkt {
            //TODO: maybe try to resolve pointer to the packet and get the data
            self.write_to_file(pkt, None)?;
        }

        Ok(())
    }
}


pub static SEND_PACKET_CTX: LazyLock<PacketStructContext> =
    LazyLock::new(|| PacketStructContext::new(config::packet_out_file()));
pub static RECV_PACKET_CTX: LazyLock<PacketStructContext> =
    LazyLock::new(|| PacketStructContext::new(config::packet_in_file()));
