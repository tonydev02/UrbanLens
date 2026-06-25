use std::{
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

fn main() -> Result<(), String> {
    let host = env::var("API_HEALTHCHECK_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("API_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080);
    let address = format!("{host}:{port}")
        .parse::<SocketAddr>()
        .map_err(|error| format!("invalid healthcheck address: {error}"))?;

    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(2))
        .map_err(|error| format!("could not connect to API readiness endpoint: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| format!("could not set healthcheck read timeout: {error}"))?;
    stream
        .write_all(b"GET /ready HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .map_err(|error| format!("could not request readiness endpoint: {error}"))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|error| format!("could not read readiness response: {error}"))?;

    if response.starts_with("HTTP/1.1 200") || response.starts_with("HTTP/1.0 200") {
        Ok(())
    } else {
        Err("API readiness endpoint did not return HTTP 200".to_owned())
    }
}
