use crate::imago::nanokvm::capture::{Auth, LoginCredentials};
use std::time::Duration;
use wit_bindgen::generate;

generate!({
    generate_all,
});

fn main() {
    let username = required_env("NANOKVM_USERNAME");
    let password = required_env("NANOKVM_PASSWORD");
    let webhook_url = required_env("DISCORD_WEBHOOK_URL");

    let session =
        imago::nanokvm::capture::local(&Auth::Login(LoginCredentials { username, password }))
            .unwrap_or_else(|err| {
                eprintln!("failed to create capture session: {err}");
                std::process::exit(1);
            });

    loop {
        match session.capture_jpeg() {
            Ok(jpeg_stream) => match read_jpeg_stream(jpeg_stream) {
                Ok(jpeg_bytes) => {
                    if let Err(err) = send_to_discord(&webhook_url, &jpeg_bytes) {
                        eprintln!("{err}");
                    }
                }
                Err(err) => eprintln!("{err}"),
            },
            Err(err) => eprintln!("failed to capture JPEG frame: {err}"),
        }

        std::thread::sleep(Duration::from_secs(60));
    }
}

fn required_env(key: &str) -> String {
    let value = std::env::var(key).unwrap_or_else(|_| {
        eprintln!("{key} is not set");
        std::process::exit(1);
    });
    let trimmed = value.trim();
    if trimmed.is_empty() {
        eprintln!("{key} is empty");
        std::process::exit(1);
    }
    trimmed.to_string()
}

fn read_jpeg_stream(stream: wasi::io::streams::InputStream) -> Result<Vec<u8>, String> {
    let mut jpeg = Vec::new();
    loop {
        match stream.blocking_read(64 * 1024) {
            Ok(chunk) => {
                if chunk.is_empty() {
                    break;
                }
                jpeg.extend_from_slice(&chunk);
            }
            Err(wasi::io::streams::StreamError::Closed) => break,
            Err(err) => return Err(format!("failed to read JPEG stream: {err:?}")),
        }
    }

    if jpeg.is_empty() {
        return Err("captured JPEG stream was empty".to_string());
    }
    Ok(jpeg)
}

fn send_to_discord(webhook_url: &str, jpeg: &[u8]) -> Result<(), String> {
    let part = waki::multipart::Part::new("files[0]", jpeg.to_vec())
        .filename("capture.jpg")
        .mime_str("image/jpeg")
        .map_err(|err| format!("failed to construct multipart payload: {err}"))?;

    let form = waki::multipart::Form::new().part(part);

    let response = waki::Client::new()
        .post(webhook_url)
        .connect_timeout(Duration::from_secs(30))
        .multipart(form)
        .send()
        .map_err(|err| format!("failed to send Discord webhook: {err}"))?;

    let status = response.status_code();
    if !(200..300).contains(&status) {
        let body = response
            .body()
            .unwrap_or_else(|err| format!("<failed to read response body: {err}>").into_bytes());
        let body = String::from_utf8_lossy(&body);
        return Err(format!("Discord webhook returned {status}: {body}"));
    }

    Ok(())
}
