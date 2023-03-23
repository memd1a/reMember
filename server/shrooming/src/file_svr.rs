use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::{self, Empty, StreamBody},
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use hyper::{header, StatusCode};
use tokio_util::io::ReaderStream;

use crate::files::FileIndex;

pub struct FileSvr {
    index: FileIndex,
}

impl FileSvr {
    pub fn new(index: FileIndex) -> Self {
        Self { index }
    }

    async fn serve_ix(State(ix): State<Arc<Self>>) -> impl IntoResponse {
        Json(ix.index.get_index())
    }

    async fn serve_file(
        State(ix): State<Arc<Self>>,
        Path(name): Path<String>,
    ) -> impl IntoResponse {
        let Some(file) = ix.index.get(&name) else  {
            return Err(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap());
        };

        //TODO handle the path error here
        let file = tokio::fs::File::open(&file.path).await.unwrap();
        let file_len = file.metadata().await.unwrap().len();
        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        return Ok(Response::builder()
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .header(header::CONTENT_LENGTH, file_len)
            .body(body)
            .unwrap());
    }

    pub async fn serve(self, addr: SocketAddr) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/index", get(Self::serve_ix))
            .route("/file/:name", get(Self::serve_file))
            .with_state(Arc::new(self));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
