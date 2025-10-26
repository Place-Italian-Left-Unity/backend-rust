use crate::{api::ArtListData, errors::Error, program_constants::ProgramConstants};

mod api;
mod errors;
mod program_constants;

static ALL_PROGRAM_CONSTANTS: std::sync::LazyLock<ProgramConstants> =
    std::sync::LazyLock::new(ProgramConstants::lazy_evaluate);

pub fn shortened_response_new<R: std::io::Read>(
    status_code: u16,
    content_type: impl Into<Vec<u8>> + AsRef<[u8]>,
    data_length: usize,
    data: R,
) -> tiny_http::Response<R> {
    tiny_http::Response::new(
        tiny_http::StatusCode(status_code),
        vec![tiny_http::Header::from_bytes(b"Content-Type", content_type).unwrap()],
        data,
        Some(data_length),
        None,
    )
}

#[tokio::main]
async fn main() {
    // 0x07 is BEL
    println!("\x07+++ Starting Backend +++");

    let server = tiny_http::Server::http(ALL_PROGRAM_CONSTANTS.listening_address.as_str()).unwrap();

    println!("+++ Starting Server +++");

    let server = std::sync::Arc::new(server);
    let mut handles = Vec::with_capacity(ALL_PROGRAM_CONSTANTS.server_threads as usize);

    for server_thread_number in 0..ALL_PROGRAM_CONSTANTS.server_threads {
        println!("+++ Started Thread {server_thread_number} +++");
        let server = server.clone();

        let guard = std::thread::spawn(async move || {
            loop {
                let rq = server.recv().unwrap();
                let url = rq.url();

                /* Handle Templates Request */
                if url.starts_with("/templates/") {
                    let file_name = url.split("/templates/").last().unwrap();
                    let path = format!("{}{file_name}", ALL_PROGRAM_CONSTANTS.templates_path);
                    let read_path = std::fs::read(path);
                    let response = match read_path {
                        Ok(data) => shortened_response_new(
                            200,
                            b"image/png",
                            data.len(),
                            std::io::Cursor::new(data),
                        ),
                        Err(e) => Error::IoError(e).to_response(),
                    };
                    let _ = rq.respond(response);
                    continue;
                }

                match url {
                    "/api/art/list" => {
                        let data: Vec<ArtListData> = ALL_PROGRAM_CONSTANTS
                            .templates_data
                            .iter()
                            .map(ArtListData::from_template_data)
                            .collect();
                        let data = serde_json::to_string(&data).unwrap();
                        let response = shortened_response_new(
                            200,
                            b"application/json",
                            data.len(),
                            std::io::Cursor::new(data),
                        );
                        let _ = rq.respond(response);
                    }
                    _ => continue,
                }
            }
        });

        handles.push(guard);
    }

    if let Some(h) = handles.into_iter().next() {
        h.join().unwrap().await;
    }
}
