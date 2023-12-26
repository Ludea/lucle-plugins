use axum::{
    body::Bytes,
    extract::{BodyStream, Path as axumPath},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::post,
    BoxError, Router,
};
use futures::{Stream, TryStreamExt};
use std::fs;
use std::io;
use std::path::Path;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower_http::services::ServeDir;

const UPLOADS_DIRECTORY: &str = "uploads";

#[no_mangle]
pub fn add_route(router: Router) -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("binaries/dist").not_found_service(service);

    router
        .nest_service("/", serve_dir.clone())
        .route("/upload/:file_name", post(save_request_body))
        .fallback_service(serve_dir)
}

pub fn get_list_folders(path: &Path) -> Vec<String> {
    let mut folders: Vec<String> = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            folders = entries
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
        }
        Err(err) => println!("Path error : {}", err),
    }
    folders
}

async fn save_request_body(
    axumPath(file_name): axumPath<String>,
    body: BodyStream,
) -> Result<(), (StatusCode, String)> {
    stream_to_file(&file_name, body).await
}

async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
