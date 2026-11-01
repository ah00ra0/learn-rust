# پروژه: ساخت ابزار جستجوی متن (mini grep)

در این فصل مفاهیم کتاب را در یک برنامه‌ی واقعی به کار می‌بریم: نسخه‌ی ساده‌ای از ابزار معروف `grep` که **یک عبارت را در یک فایل جستجو** می‌کند و خط‌های شامل آن را چاپ می‌کند.

```bash
minigrep frog poem.txt
IGNORE_CASE=1 minigrep TOO poem.txt
```

مفاهیمی که تمرین می‌کنیم: ماژول‌ها، struct، مدیریت خطا، `Result` و `?`، iterator ها، closure ها، lifetime، تست‌نویسی و متغیرهای محیطی.

> کد کامل این پروژه در پوشه‌ی [`projects/minigrep`](https://github.com/ah00ra0/learn-rust/tree/main/projects/minigrep) مخزن کتاب موجود و تست‌شده است.

## گام ۱: ساخت پروژه

```bash
cargo new minigrep
cd minigrep
```

برای آزمایش، فایلی به نام `poem.txt` در ریشه‌ی پروژه بسازید:

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

## گام ۲: معماری

یک اصل مهم: **منطق را از `main` جدا کنید.**

- `src/main.rs`: فقط خواندن آرگومان‌ها، فراخوانی منطق، و مدیریت خطای نهایی.
- `src/lib.rs`: تمام منطق اصلی. چون کتابخانه است، به‌راحتی **تست** می‌شود.

## گام ۳: Config

آرگومان‌های خط فرمان را در struct جمع می‌کنیم. تابع `build` به‌جای panic، `Result` برمی‌گرداند:

```rust,ignore
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // نام برنامه را رد می‌کنیم

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("عبارت جستجو داده نشده است"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("مسیر فایل داده نشده است"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}
```

نکات:

- `impl Iterator<Item = String>` یعنی هر iterator ی از `String` ها؛ در `main` همان `env::args()` است و در تست می‌توانیم لیست دلخواه بدهیم.
- `env::var("IGNORE_CASE").is_ok()` بررسی می‌کند متغیر محیطی تنظیم شده یا نه.

## گام ۴: تابع run

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

- `?` خطای خواندن فایل را به فراخواننده برمی‌گرداند.
- `Box<dyn Error>` یعنی هر نوع خطایی.

## گام ۵: توابع جستجو (با lifetime)

خط‌های برگشتی از **متن ورودی** هستند، پس lifetime ی خروجی به `contents` وصل است، نه `query`:

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

این‌جا از iterator و closure استفاده کردیم: نسبت به حلقه‌ی دستی کوتاه‌تر و به همان اندازه سریع است.

## گام ۶: main

```rust,ignore
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
```

- پیغام‌های خطا را با `eprintln!` به **stderr** می‌فرستیم تا اگر خروجی به فایل هدایت شد (`> out.txt`) خطاها داخلش نروند.
- `process::exit(1)` با کد خروج غیرصفر خارج می‌شود تا اسکریپت‌ها بفهمند خطایی رخ داده.

## گام ۷: تست‌ها

در انتهای `src/lib.rs`:

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn config_requires_two_arguments() {
        let args = vec![String::from("minigrep")].into_iter();
        assert!(Config::build(args).is_err());
    }
}
```

```bash
cargo test
```

## گام ۸: اجرا

```bash
cargo run -- frog poem.txt
```

```text
How public, like a frog
```

```bash
IGNORE_CASE=1 cargo run -- TOO poem.txt
```

```text
Are you nobody, too?
```

بدون `IGNORE_CASE` هم همین عبارت `too` پیدا می‌شود، اما `TOO` (با حروف بزرگ) فقط در حالت نادیده گرفتن حروف بزرگ و کوچک پیدا می‌شود. امتحانش کنید: `cargo run -- TOO poem.txt` بدون متغیر محیطی هیچ نتیجه‌ای نمی‌دهد.

و حالت خطا:

```bash
cargo run -- onlyone
```

```text
خطا در آرگومان‌ها: مسیر فایل داده نشده است
استفاده: minigrep <عبارت> <فایل>
```

برای ساخت نسخه‌ی بهینه و نصب:

```bash
cargo build --release
cargo install --path .
```

## پیشنهاد برای گسترش

۱. **چند فایل**: چند مسیر بگیرید و نام فایل را کنار هر خط چاپ کنید.
۲. **شماره‌ی خط**: خروجی را به شکل `12: متن خط` بسازید (راهنمایی: `enumerate`).
۳. **Regex**: crate ی `regex` را اضافه کنید.
۴. **پیمایش پوشه‌ها**: با `std::fs::read_dir` یا crate ی `walkdir` به‌صورت بازگشتی جستجو کنید.
۵. **آرگومان‌های حرفه‌ای**: به‌جای پردازش دستی از crate ی `clap` استفاده کنید.
۶. **موازی‌سازی**: با `rayon` فایل‌ها را همزمان پردازش کنید.
۷. **رنگ**: با crate ی `colored` عبارت یافت‌شده را رنگی چاپ کنید.

> اگر به ابزار حرفه‌ای جستجو علاقه دارید، کد منبع [ripgrep](https://github.com/BurntSushi/ripgrep) که با Rust نوشته شده، منبع آموزشی فوق‌العاده‌ای است.

فصل بعد: [مسیر بعدی و منابع یادگیری](29-next-steps.md).
