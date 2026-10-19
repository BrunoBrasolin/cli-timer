use crate::{page, stopwatch::Stopwatch, storage};
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

const MAX_BODY: usize = 64 * 1024;

struct Request {
    method: String,
    path: String,
    body: String,
}

pub fn run(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Stopwatch running at http://{addr}");
    println!("Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle(stream) {
                    eprintln!("Request failed: {err}");
                }
            }
            Err(err) => eprintln!("Connection failed: {err}"),
        }
    }

    Ok(())
}

fn handle(stream: TcpStream) -> io::Result<()> {
    let request = read_request(&stream)?;

    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => {
            let stopwatches = storage::load_all()?;
            respond(&stream, "200 OK", "text/html; charset=utf-8", &page::render(&stopwatches))
        }
        ("POST", "/timers") => {
            let form = parse_form(&request.body);
            let name = form.get("name").map(|n| n.trim()).unwrap_or_default();
            if !name.is_empty() {
                let mut stopwatches = storage::load_all()?;
                stopwatches.entry(name.to_string()).or_insert_with(Stopwatch::new);
                storage::save_all(&stopwatches)?;
            }
            redirect_home(&stream)
        }
        ("POST", "/timers/action") => {
            let form = parse_form(&request.body);
            if let (Some(name), Some(action)) = (form.get("name"), form.get("action")) {
                let mut stopwatches = storage::load_all()?;
                apply(&mut stopwatches, name, action);
                storage::save_all(&stopwatches)?;
            }
            redirect_home(&stream)
        }
        _ => respond(&stream, "404 Not Found", "text/plain; charset=utf-8", "Not found"),
    }
}

fn apply(stopwatches: &mut HashMap<String, Stopwatch>, name: &str, action: &str) {
    if action == "delete" {
        stopwatches.remove(name);
        return;
    }

    let Some(stopwatch) = stopwatches.get_mut(name) else {
        return;
    };

    match action {
        "start" => stopwatch.start(),
        "pause" => stopwatch.pause(),
        "stop" => stopwatch.stop(),
        "reset" => stopwatch.reset(),
        _ => {}
    }
}

fn read_request(stream: &TcpStream) -> io::Result<Request> {
    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or("/");
    let path = target.split('?').next().unwrap_or("/").to_string();

    let mut content_length = 0;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 || header.trim_end().is_empty() {
            break;
        }
        if let Some((key, value)) = header.split_once(':') {
            if key.trim().eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
    }

    let mut body = vec![0; content_length.min(MAX_BODY)];
    reader.read_exact(&mut body)?;

    Ok(Request {
        method,
        path,
        body: String::from_utf8_lossy(&body).into_owned(),
    })
}

fn respond(mut stream: &TcpStream, status: &str, content_type: &str, body: &str) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;
    stream.flush()
}

fn redirect_home(mut stream: &TcpStream) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    )?;
    stream.flush()
}

/// Parses an `application/x-www-form-urlencoded` body.
fn parse_form(body: &str) -> HashMap<String, String> {
    body.split('&')
        .filter_map(|pair| pair.split_once('='))
        .map(|(key, value)| (url_decode(key), url_decode(value)))
        .collect()
}

fn url_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
                match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    Some(byte) => {
                        out.push(byte);
                        i += 2;
                    }
                    None => out.push(b'%'),
                }
            }
            byte => out.push(byte),
        }
        i += 1;
    }

    String::from_utf8_lossy(&out).into_owned()
}
