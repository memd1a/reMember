use std::sync::{Arc, Mutex};

use bytes::{BytesMut, Buf};
use tokio::sync::mpsc;

type SharedBuf = Arc<Mutex<BytesMut>>;

#[derive(Debug, Clone)]
pub struct SessionBufSender {
    tx: mpsc::Sender<usize>,
    buf: Arc<Mutex<BytesMut>>,
}
#[derive(Debug)]
pub struct SessionBufReceiver {
    rx: mpsc::Receiver<usize>,
    buf: SharedBuf,
}

impl SessionBufSender {
    pub fn write(&self, data: &[u8]) -> anyhow::Result<()> {
        let mut buf = self.buf.lock().unwrap();
        buf.reserve(data.len());
        buf.extend_from_slice(data);
        self.tx.try_send(data.len())?;
        Ok(())
    }
}

impl SessionBufReceiver {
    pub async fn recv_len(&mut self) -> usize {
        self.rx.recv().await.expect("session buf should be never closed")
    }

    pub fn try_recv_len(&mut self) -> Option<usize> {
        self.rx.try_recv().ok()
    }

    pub fn copy_to(&self, dst: &mut [u8]) {
        //TODO ensure somehow this is triggered with the correct len
        let mut buf = self.buf.lock().unwrap();
        buf.copy_to_slice(dst);
    }
}