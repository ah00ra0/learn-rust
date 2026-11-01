use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("خطا در آرگومان‌ها: {err}");
        eprintln!("استفاده: minigrep <عبارت> <فایل>");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("خطای برنامه: {e}");
        process::exit(1);
    }
}
