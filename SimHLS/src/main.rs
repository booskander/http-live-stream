use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
pub mod file_manager;

use file_manager::file_manager::create_hls_stream;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct VideoStreamInfo {
    title: String,
    stream_url: String,
    resolution: String,
    duration: u64,
}

#[get("/stream/<title>")]
async fn create_stream(title: &str) -> Result<String, status::Custom<String>> {
    match create_hls_stream(title).await {
        Ok(_) => {
            let payload = VideoStreamInfo {
                title: String::from(title),
                stream_url: format!(
                    "Your streaming link: http://localhost:8000/api/video/{}/playlist.m3u8",
                    title
                ),
                resolution: String::from(""),
                duration: 19,
            };
            match serde_json::to_string(&payload) {
                Ok(json_payload) => Ok(json_payload),
                Err(_) => Err(status::Custom(
                    Status::InternalServerError,
                    String::from("Failed to serialize the response"),
                )),
            }
        }
        Err(_) => Err(status::Custom(
            Status::NotFound,
            String::from("Video not found"),
        )),
    }
}

#[launch]
async fn rocket() -> _ {
    println!("random code to trigger pipeline...");
    let allowed_origins = AllowedOrigins::all();
    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .to_cors()
        .expect("Error creating CORS fairing");
    rocket::build()
        .attach(cors)
        .mount("/api", routes![create_stream])
        .mount("/api/video", FileServer::from(relative!("video")))
        .configure(rocket::Config {
            port: 8000,
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
}
