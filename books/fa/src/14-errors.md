# مدیریت خطا: panic، Result و عملگر ?

Rust خطاها را به دو دسته تقسیم می‌کند:

- **خطاهای بازیابی‌ناپذیر** (unrecoverable): باگ‌هایی مثل دسترسی خارج از محدوده. با `panic!` برنامه متوقف می‌شود.
- **خطاهای بازیابی‌پذیر** (recoverable): مثل نبودن فایل. با نوع `Result<T, E>` مدیریت می‌شوند.

Rust **exception ندارد.** خطاها بخشی از نوع برگشتی تابع هستند و نمی‌توان فراموششان کرد.

## panic!

```rust,should_panic
fn main() {
    panic!("اوه! مشکل جدی پیش آمد");
}
```

با اجرای panic، برنامه پیغام خطا را چاپ می‌کند، پشته را باز می‌کند (unwinding) و خارج می‌شود. برای دیدن ردپای کامل:

```bash
RUST_BACKTRACE=1 cargo run
```

> در `Cargo.toml` می‌توانید با `panic = "abort"` در `[profile.release]` به‌جای unwinding فوراً برنامه را ببندید تا فایل باینری کوچک‌تر شود.

### چه زمانی panic کنیم؟

- وقتی برنامه در حالت **غیرممکن** قرار گرفته (نقض یک invariant)
- در مثال‌ها، نمونه‌های اولیه و تست‌ها
- وقتی ادامه دادن **ناامن** است

در بقیه‌ی موارد از `Result` استفاده کنید.

## Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

نمونه:

```rust
use std::fs::File;

fn main() {
    let result = File::open("nonexistent.txt");

    match result {
        Ok(_file) => println!("فایل باز شد"),
        Err(error) => println!("خطا: {error}"),
    }
}
```

### مدیریت انواع مختلف خطا

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(_) => println!("باز شد"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("فایل پیدا نشد"),
            ErrorKind::PermissionDenied => println!("اجازه دسترسی نیست"),
            other => println!("خطای دیگر: {other:?}"),
        },
    }
}
```

### unwrap و expect

میان‌برهایی که در صورت `Err` **panic** می‌کنند:

```rust,should_panic
fn main() {
    let n: i32 = "abc".parse().unwrap();       // panic با پیغام عمومی
    println!("{n}");
}
```

```rust,should_panic
fn main() {
    let n: i32 = "abc".parse().expect("باید یک عدد معتبر باشد"); // پیغام دلخواه
    println!("{n}");
}
```

در کد واقعی `expect` را به `unwrap` ترجیح دهید چون پیغام روشن‌تری می‌دهد.

### متدهای کاربردی Result

```rust
fn main() {
    let ok: Result<i32, String> = Ok(10);
    let err: Result<i32, String> = Err(String::from("بد"));

    println!("{}", ok.clone().unwrap_or(0));          // 10
    println!("{}", err.clone().unwrap_or(0));         // 0
    println!("{:?}", ok.clone().map(|x| x * 2));      // Ok(20)
    println!("{:?}", err.clone().map_err(|e| e.len()));  // Err(2)
    println!("{}", ok.is_ok());
    println!("{:?}", err.ok());                       // None
}
```

## انتشار خطا با عملگر ?

نوشتن `match` برای هر خطا خسته‌کننده است. عملگر `?` اگر مقدار `Ok` باشد آن را باز می‌کند و اگر `Err` باشد، **همان‌جا از تابع برمی‌گردد** و خطا را به فراخواننده می‌دهد:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("{name}"),
        Err(e) => println!("خطا: {e}"),
    }
}
```

نسخه‌ی کوتاه‌تر:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

fn main() {
    println!("{:?}", read_username_from_file().is_err());
}
```

`?` فقط در توابعی کار می‌کند که نوع برگشتی‌شان `Result` یا `Option` (یا چیزی سازگار) باشد. می‌توان `main` را هم این‌طور نوشت:

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let n: i32 = "123".parse()?;
    println!("{n}");
    Ok(())
}
```

`?` روی `Option` هم کار می‌کند:

```rust
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{:?} {:?}", first_char_upper("rust"), first_char_upper(""));
}
```

## نوع خطای اختصاصی

برای کتابخانه‌ها و برنامه‌های جدی، نوع خطای خودتان را تعریف کنید:

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Missing(String),
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "کلید {key} پیدا نشد"),
            ConfigError::InvalidNumber(e) => write!(f, "عدد نامعتبر: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

// این پیاده‌سازی اجازه می‌دهد ? خودکار ParseIntError را تبدیل کند
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::InvalidNumber(e)
    }
}

fn parse_port(value: Option<&str>) -> Result<u16, ConfigError> {
    let text = value.ok_or_else(|| ConfigError::Missing("port".to_string()))?;
    let port = text.parse::<u16>()?;  // ParseIntError → ConfigError با From
    Ok(port)
}

fn main() {
    println!("{:?}", parse_port(Some("8080")));
    println!("{}", parse_port(None).unwrap_err());
    println!("{}", parse_port(Some("abc")).unwrap_err());
}
```

> **نکته‌ی عملی:** crate های `thiserror` (برای کتابخانه‌ها) و `anyhow` (برای برنامه‌ها) نوشتن این کدها را بسیار ساده می‌کنند.

## راهنمای انتخاب

| وضعیت | ابزار |
|---|---|
| ممکن است چیزی وجود نداشته باشد | `Option<T>` |
| عملی ممکن است شکست بخورد و فراخواننده باید تصمیم بگیرد | `Result<T, E>` |
| باگ برنامه‌نویس / وضعیت غیرممکن | `panic!` |
| انتقال خطا به بالا | عملگر `?` |
| مثال و پروتوتایپ | `unwrap` / `expect` |

## تمرین

1. تابعی بنویسید که رشته‌ای را به `u32` تبدیل کند و `Result<u32, ParseIntError>` برگرداند.
2. تابعی بنویسید که محتوای یک فایل را بخواند و تعداد خطوط را برگرداند (با `?`).
3. enum خطای اختصاصی با دو حالت بسازید و `Display` را برایش پیاده‌سازی کنید.
4. تابع `main` را طوری بنویسید که `Result` برگرداند.

فصل بعد: [Generics](15-generics.md).
