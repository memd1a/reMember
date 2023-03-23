use std::path::PathBuf;
use std::pin::Pin;

use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

use shrooming_svc::shrooming_launcher_server::ShroomingLauncher;
use shrooming_svc::{
    Empty, FileChunk, FileEntriesReply, FileEntry, FileRequest, HelloReply, HelloRequest,
};

pub mod shrooming_svc {
    tonic::include_proto!("shrooming"); // The string specified here must match the proto package name
}

struct FileDirEntry {
    pub name: String,
    pub hash: String,
}

pub struct LauncherService {
    serve_dir: PathBuf,
    serve_files: Vec<FileDirEntry>,
}

#[tonic::async_trait]
impl ShroomingLauncher for LauncherService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = shrooming_svc::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get_file_entries(
        &self,
        _request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<FileEntriesReply>, tonic::Status> {
        Ok(Response::new(FileEntriesReply {
            entries: self.serve_files.iter().map(|entry| FileEntry {
                name: entry.name.clone(),
                hash: entry.hash.clone(),
            }).collect(),
        }))
    }

    type GetFileStream = Pin<Box<dyn Stream<Item = Result<FileChunk, Status>> + Send + 'static>>;
    async fn get_file(
        &self,
        request: tonic::Request<FileRequest>,
    ) -> Result<tonic::Response<Self::GetFileStream>, tonic::Status> {
        let req = request.into_inner();
        let requested_file = req.name.as_str();
        if !self.serve_files.iter().any(|f| f.name == requested_file) {
            return Err(Status::not_found("File not found"));
        }

        // Get file reader stream
        let file_path = self.serve_dir.join(requested_file);
        let file = tokio::fs::File::open(file_path).await.unwrap();
        let reader_stream = ReaderStream::new(file);
        let file_chunk_stream = reader_stream.map(|chunk| {
            chunk
                .map(|chunk| FileChunk {
                    chunk_id: 0,
                    data: chunk.to_vec(),
                })
                .map_err(|err| Status::from_error(Box::new(err)))
        });

        Ok(Response::new(
            Box::pin(file_chunk_stream) as Self::GetFileStream
        ))
    }
}
