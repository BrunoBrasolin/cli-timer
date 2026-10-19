mod page;
mod server;
mod stopwatch;
mod storage;

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("127.0.0.1:{port}");

    if let Err(err) = server::run(&addr) {
        eprintln!("Failed to start server on {addr}: {err}");
        std::process::exit(1);
    }
}
