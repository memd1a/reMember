use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
};

use bytes::{BufMut, Bytes, BytesMut};
use futures::{channel::mpsc, ready, Sink, Stream};

#[repr(transparent)]
#[derive(Debug, Clone)]
struct FramedPipeBuf(Arc<Mutex<BytesMut>>);

impl FramedPipeBuf {
    fn new(cap: usize) -> Self {
        Self(Arc::new(Mutex::new(BytesMut::with_capacity(cap))))
    }

    fn take(&self, n: usize) -> Bytes {
        self.0.lock().expect("Reading frame").split_to(n).freeze()
    }
}
#[derive(Debug, Clone)]
pub struct FramedPipeSender {
    tx: mpsc::Sender<usize>,
    buf: FramedPipeBuf,
}

impl FramedPipeSender {
    fn push<B: AsRef<[u8]>>(&mut self, item: B) -> Result<(), mpsc::SendError> {
        let mut buf = self.buf.0.lock().expect("Writing frame");
        self.tx.start_send(item.as_ref().len())?;
        buf.put_slice(item.as_ref());
        Ok(())
    }

    pub fn try_send<B: AsRef<[u8]>>(&mut self, item: B) -> Result<(), mpsc::TrySendError<usize>> {
        let mut buf = self.buf.0.lock().expect("Writing frame");
        let item = item.as_ref();
        self.tx.try_send(item.len())?;
        buf.put_slice(item);
        Ok(())
    }

    pub fn try_send_all<B: AsRef<[u8]>>(
        &mut self,
        items: impl Iterator<Item = B>,
    ) -> Result<(), mpsc::TrySendError<usize>> {
        let mut buf = self.buf.0.lock().expect("Writing frame");
        for item in items {
            let item = item.as_ref();
            self.tx.try_send(item.len())?;
            buf.put_slice(item);
        }
        Ok(())
    }
}

impl<B: AsRef<[u8]>> Sink<B> for FramedPipeSender {
    type Error = mpsc::SendError;

    fn poll_ready(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.tx.poll_ready(cx)
    }

    fn start_send(mut self: std::pin::Pin<&mut Self>, item: B) -> Result<(), Self::Error> {
        Pin::new(&mut self).push(item)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx).poll_flush(cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx).poll_close(cx)
    }
}

#[derive(Debug)]
pub struct FramedPipeReceiver {
    rx: mpsc::Receiver<usize>,
    buf: FramedPipeBuf,
}

impl Stream for FramedPipeReceiver {
    type Item = Bytes;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let next_frame = ready!(Pin::new(&mut self.rx).poll_next(cx));
        Poll::Ready(next_frame.map(|frame| self.buf.take(frame)))
    }
}

pub fn framed_pipe(buf_cap: usize, channel_cap: usize) -> (FramedPipeSender, FramedPipeReceiver) {
    let buf = FramedPipeBuf::new(buf_cap);
    let (tx, rx) = mpsc::channel(channel_cap);

    (
        FramedPipeSender {
            buf: buf.clone(),
            tx,
        },
        FramedPipeReceiver { buf, rx },
    )
}

#[cfg(test)]
mod tests {
    use futures::{SinkExt, StreamExt};

    use crate::service::framed_pipe::framed_pipe;

    #[tokio::test]
    async fn echo_pipe() {
        let (tx, mut rx) = framed_pipe(1024 * 8, 128);

        const ECHO_DATA: [&'static [u8]; 4] = [&[0xFF; 4096], &[1, 2], &[], &[0x0; 1024]];

        for _ in 0..100 {
            for data in ECHO_DATA {
                tx.clone().send(data).await.unwrap();
            }

            for data in ECHO_DATA {
                let rx_data = rx.next().await.unwrap();
                assert_eq!(&rx_data, data);
            }
        }
    }
}
