# قالب‌بندی خروجی و ورودی/خروجی

تا اینجا فقط از `println!("{x}")` استفاده کردید. این فصل **همه‌ی** امکانات قالب‌بندی، سپس خواندن از ورودی، کار با فایل، آرگومان‌های خط فرمان و مسیرها را پوشش می‌دهد.

> در این فصل برای اینکه نتیجه‌ی هر قالب **تأیید شود**، از `assert_eq!(format!(...), "...")` استفاده می‌کنیم. خروجی مورد انتظار همان رشته‌ی سمت راست است.

## خانواده‌ی ماکروهای قالب‌بندی

| ماکرو | کار |
|---|---|
| `format!` | رشته‌ی `String` می‌سازد |
| `print!` / `println!` | چاپ در stdout (با/بدون خط جدید) |
| `eprint!` / `eprintln!` | چاپ در stderr |
| `write!` / `writeln!` | نوشتن در هر چیزی که `fmt::Write` یا `io::Write` پیاده کرده باشد |
| `panic!`, `assert!`, `unreachable!`, `todo!` | پیغام خطا هم با همین قالب‌بندی ساخته می‌شود |

همه‌ی آن‌ها یک **زبان قالب** مشترک دارند.

## آرگومان‌ها: موقعیتی، نام‌دار، ضمنی

```rust
fn main() {
    let name = "رضا";
    let age = 30;

    assert_eq!(format!("{} {}", name, age), "رضا 30");          // به ترتیب
    assert_eq!(format!("{1} {0}", name, age), "30 رضا");        // اندیس
    assert_eq!(format!("{n} {a}", n = name, a = age), "رضا 30"); // نام‌دار
    assert_eq!(format!("{name} {age}"), "رضا 30");              // ضمنی (capture متغیر)
    assert_eq!(format!("{0} {0} {0}", "x"), "x x x");           // تکرار
    assert_eq!(format!("{{literal}}"), "{literal}");            // escape آکولاد
}
```

> **محدودیت:** در قالب ضمنی فقط **نام متغیر** مجاز است، نه عبارت. `{a + b}` خطاست؛ بنویسید `{}` با آرگومان `a + b`.

## مشخصات قالب: `{:fill align sign # 0 width .precision type}`

ساختار کلی داخل آکولاد: `{نام:مشخصات}`. مشخصات به ترتیب (همه اختیاری): **پر کننده، تراز، علامت، `#`، `0`، عرض، `.دقت`، نوع**.

### عرض و تراز

```rust
fn main() {
    assert_eq!(format!("[{:6}]", 42), "[    42]");     // عدد: پیش‌فرض راست‌تراز
    assert_eq!(format!("[{:6}]", "ab"), "[ab    ]");   // رشته: پیش‌فرض چپ‌تراز
    assert_eq!(format!("[{:<6}]", 42), "[42    ]");    // چپ
    assert_eq!(format!("[{:>6}]", "ab"), "[    ab]");  // راست
    assert_eq!(format!("[{:^6}]", "ab"), "[  ab  ]");  // وسط
    assert_eq!(format!("[{:*^7}]", "ab"), "[**ab***]"); // با کاراکتر پرکننده
    assert_eq!(format!("[{:-<5}]", 1), "[1----]");
}
```

### عرض پویا

```rust
fn main() {
    let w = 8;
    let p = 2;
    assert_eq!(format!("[{:w$}]", 7), "[       7]");        // نام متغیر + $
    assert_eq!(format!("[{:1$}]", 7, 5), "[    7]");         // اندیس آرگومان + $
    assert_eq!(format!("[{:>w$.p$}]", 3.14159), "[    3.14]");
    assert_eq!(format!("[{:>1$}]", "x", 3), "[  x]");
    assert_eq!(format!("[{:.*}]", 3, 1.23456), "[1.235]");   // .* دقت را از آرگومان قبلی می‌گیرد
}
```

### علامت، صفر و پیشوند

```rust
fn main() {
    assert_eq!(format!("{:+}", 5), "+5");
    assert_eq!(format!("{:05}", 42), "00042");
    assert_eq!(format!("{:+05}", 42), "+0042");
    assert_eq!(format!("{:05}", -42), "-0042");
    assert_eq!(format!("{:#x}", 255), "0xff");
    assert_eq!(format!("{:#b}", 5), "0b101");
    assert_eq!(format!("{:#o}", 8), "0o10");
}
```

### دقت اعشار و رشته

```rust
fn main() {
    assert_eq!(format!("{:.2}", 3.14159), "3.14");
    assert_eq!(format!("{:8.3}", 3.14159), "   3.142");
    assert_eq!(format!("{:08.3}", 3.14159), "0003.142");
    assert_eq!(format!("{:.0}", 2.5), "2");       // گرد کردن به نزدیک‌ترین زوج در این مورد
    assert_eq!(format!("{:.3}", "abcdef"), "abc"); // روی رشته: برش به ۳ کاراکتر
    assert_eq!(format!("{}", 1.0), "1");          // Display برای f64 صفر اضافی نمی‌گذارد
    assert_eq!(format!("{:?}", 1.0), "1.0");      // Debug دقیق‌تر نشان می‌دهد
}
```

### نوع قالب

| مشخصه | معنی | مثال | نتیجه |
|---|---|---|---|
| (خالی) | `Display` | `{}` | `42` |
| `?` | `Debug` | `{:?}` | `"ab"` با کوتیشن |
| `#?` | `Debug` زیبا (چندخطی) | `{:#?}` | |
| `x` / `X` | هگز | `{:x}` | `ff` / `FF` |
| `o` | هشت‌هشتی | `{:o}` | `10` |
| `b` | دودویی | `{:b}` | `101` |
| `e` / `E` | علمی | `{:e}` | `1.5e3` |
| `p` | آدرس pointer | `{:p}` | `0x7ff...` |

```rust
fn main() {
    assert_eq!(format!("{:x}", 255), "ff");
    assert_eq!(format!("{:X}", 255), "FF");
    assert_eq!(format!("{:#010b}", 5), "0b00000101"); // عرض شامل پیشوند است
    assert_eq!(format!("{:e}", 1500.0), "1.5e3");
    assert_eq!(format!("{:?}", "a\"b"), "\"a\\\"b\"");   // Debug کاراکتر خاص را escape می‌کند
    assert_eq!(format!("{:?}", Some("x")), "Some(\"x\")");
    assert_eq!(format!("{:?}", (1, 'a')), "(1, 'a')");
    assert_eq!(format!("{:?}", vec![1, 2]), "[1, 2]");
}
```

### `{:#?}` برای ساختارهای تو در تو

```rust
#[derive(Debug)]
struct User {
    name: String,
    tags: Vec<&'static str>,
}

fn main() {
    let u = User { name: "علی".to_string(), tags: vec!["a", "b"] };
    let pretty = format!("{u:#?}");
    assert!(pretty.starts_with("User {\n    name: \"علی\",\n    tags: [\n        \"a\","));
    println!("{pretty}");
}
```

## قالب‌بندی برای نوع خودتان

### Display و Debug

قبلاً دیدید. نکته‌ی مهم: وقتی `{:>10}` را روی نوع خودتان می‌زنید، **پارامترهای قالب** (عرض، تراز) به `fmt` داده می‌شوند و فقط اگر خودتان رعایتشان کنید اثر دارند. ساده‌ترین راه:

```rust
use std::fmt;

struct Plain(&'static str);
struct Padded(&'static str);

impl fmt::Display for Plain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0) // write! پارامترهای بیرونی را نادیده می‌گیرد
    }
}

impl fmt::Display for Padded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.0) // pad عرض/تراز/دقت را رعایت می‌کند
    }
}

fn main() {
    assert_eq!(format!("[{:>5}]", Plain("ab")), "[ab]");
    assert_eq!(format!("[{:>5}]", Padded("ab")), "[   ab]");
}
```

### نوشتن Debug دستی با ابزارهای کمکی

```rust
use std::fmt;

struct Secret {
    user: String,
    password: String,
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Secret")
            .field("user", &self.user)
            .field("password", &"***") // رمز در لاگ نمی‌آید
            .finish()
    }
}

fn main() {
    let s = Secret { user: "ali".into(), password: "hunter2".into() };
    assert_eq!(format!("{s:?}"), "Secret { user: \"ali\", password: \"***\" }");
}
```

### ساخت جدول متنی

```rust
fn main() {
    let rows = [("سیب", 12.5, 3), ("پرتقال", 8.25, 10), ("موز", 20.0, 1)];

    let mut out = String::new();
    out.push_str(&format!("{:<10}|{:>8}|{:>4}\n", "نام", "قیمت", "تعداد"));
    out.push_str(&format!("{}\n", "-".repeat(24)));
    for (name, price, qty) in rows {
        out.push_str(&format!("{name:<10}|{price:>8.2}|{qty:>4}\n"));
    }
    println!("{out}");
    assert!(out.contains("   12.50|   3"));
}
```

> **هشدار فارسی/عربی:** عرض در `{:<10}` بر اساس **تعداد کاراکتر یونیکد** است، نه عرض نمایشی در ترمینال. نتیجه برای متن راست‌به‌چپ یا کاراکترهای ترکیبی ممکن است در ترمینال هم‌تراز نباشد. برای ترازبندی دقیق از crate هایی مثل `unicode-width` کمک بگیرید (بیشتر در فصل [رشته‌ها و یونیکد](strings-unicode.md)).

## نوشتن در String و در جریان‌ها با write!

برای ساخت یک رشته‌ی بزرگ بدون ساختن String های موقتی:

```rust
use std::fmt::Write; // این trait برای String

fn main() {
    let mut s = String::new();
    write!(s, "{}-", 1).unwrap();
    writeln!(s, "{}", 2).unwrap();
    assert_eq!(s, "1-2\n");
}
```

برای فایل و stdout، `std::io::Write` را import کنید (نام‌ها یکی است ولی traitها متفاوت‌اند؛ اگر هر دو لازم است از `as` استفاده کنید):

```rust
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::new();
    write!(buf, "سلام {}", 5)?;       // Vec<u8> هم io::Write است
    writeln!(buf)?;
    assert_eq!(String::from_utf8(buf).unwrap(), "سلام 5\n");

    let stdout = std::io::stdout();
    let mut out = stdout.lock();     // قفل یک‌بار؛ برای چاپ زیاد سریع‌تر
    writeln!(out, "خروجی با قفل")?;
    Ok(())
}
```

## چاپ زیاد: بافر و قفل

`println!` هر بار stdout را قفل می‌کند و (روی ترمینال) هر خط را flush می‌کند. برای چاپ صدها هزار خط:

```rust
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for i in 0..5 {
        writeln!(out, "خط {i}")?;
    }
    out.flush()?; // BufWriter در Drop هم flush می‌کند ولی خطاها را نادیده می‌گیرد
    Ok(())
}
```

## stdout در برابر stderr

- **stdout**: نتیجه‌ی برنامه (قابل pipe به برنامه‌ی بعدی).
- **stderr**: پیغام خطا و لاگ.

```bash
./prog > out.txt        # فقط stdout به فایل می‌رود، خطاها روی صفحه می‌آیند
./prog 2> err.txt
```

```rust
fn main() {
    println!("نتیجه");
    eprintln!("هشدار: فایل پیکربندی پیدا نشد");
}
```

## خواندن از ورودی (stdin)

### یک خط

```rust
use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    let n = io::stdin().read_line(&mut line)?; // خط جدید انتهای رشته می‌ماند!
    // n == 0 یعنی EOF
    let trimmed = line.trim();
    println!("خواندم {n} بایت: {trimmed}");
    Ok(())
}
```

### همه‌ی خطوط

```rust
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut sum = 0i64;
    for line in stdin.lock().lines() {
        let line = line?;
        if let Ok(n) = line.trim().parse::<i64>() {
            sum += n;
        }
    }
    println!("مجموع: {sum}");
    Ok(())
}
```

### تست‌پذیر کردن ورودی

تابعی که مستقیم `stdin` را می‌خواند سخت تست می‌شود. به‌جایش از `impl BufRead` بگیرید:

```rust
use std::io::{BufRead, Cursor};

fn sum_numbers(reader: impl BufRead) -> i64 {
    reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|l| l.trim().parse::<i64>().ok())
        .sum()
}

fn main() {
    // در تست: Cursor روی بایت‌ها
    assert_eq!(sum_numbers(Cursor::new("1\n2\nx\n 40 \n")), 43);

    // در برنامه‌ی واقعی:
    // sum_numbers(std::io::stdin().lock());
    println!("{}", sum_numbers(Cursor::new("5\n6")));
}
```

### خواندن عدد از کاربر با حلقه‌ی تکرار

```rust
use std::io::{self, BufRead, Write};

fn ask_number(mut input: impl BufRead, mut output: impl Write) -> io::Result<u32> {
    loop {
        write!(output, "یک عدد وارد کنید: ")?;
        output.flush()?; // چون print! خط جدید ندارد، prompt بدون flush دیده نمی‌شود
        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "ورودی تمام شد"));
        }
        match line.trim().parse() {
            Ok(n) => return Ok(n),
            Err(_) => writeln!(output, "عدد معتبر نیست")?,
        }
    }
}

fn main() -> io::Result<()> {
    let mut out = Vec::new();
    let n = ask_number(io::Cursor::new("abc\n42\n"), &mut out)?;
    assert_eq!(n, 42);
    assert!(String::from_utf8(out).unwrap().contains("عدد معتبر نیست"));
    Ok(())
}
```

## کار با فایل‌ها

### خواندن و نوشتن کامل

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir().join("rust_book_io_demo");
    fs::create_dir_all(&dir)?;
    let path = dir.join("a.txt");

    fs::write(&path, "سلام\nدنیا\n")?;            // ایجاد یا بازنویسی
    let text = fs::read_to_string(&path)?;        // کل فایل به‌صورت UTF-8
    assert_eq!(text, "سلام\nدنیا\n");

    let bytes = fs::read(&path)?;                 // بایت خام
    assert_eq!(bytes.len(), text.len());

    fs::remove_dir_all(&dir)?;
    Ok(())
}
```

### خواندن خط‌به‌خط با بافر (برای فایل‌های بزرگ)

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("rust_book_lines.txt");

    {
        let mut w = BufWriter::new(File::create(&path)?);
        for i in 1..=3 {
            writeln!(w, "خط شماره {i}")?;
        }
    } // w در اینجا flush و بسته می‌شود

    let reader = BufReader::new(File::open(&path)?);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[1], "خط شماره 2");

    std::fs::remove_file(&path)?;
    Ok(())
}
```

### افزودن به انتهای فایل و OpenOptions

```rust
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("rust_book_append.log");
    let _ = std::fs::remove_file(&path);

    for msg in ["الف", "ب"] {
        let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
        writeln!(f, "{msg}")?;
    }
    assert_eq!(std::fs::read_to_string(&path)?, "الف\nب\n");
    std::fs::remove_file(&path)
}
```

### خطاهای I/O

نوع `io::Error` یک `kind()` دارد:

```rust
use std::io::ErrorKind;

fn main() {
    let err = std::fs::read_to_string("/this/does/not/exist").unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
    println!("{err}");
}
```

## مسیرها: Path و PathBuf

`Path` شبیه `&str` و `PathBuf` شبیه `String` است (نوع قرض‌گرفته و نوع مالک). **مسیر را هرگز با الحاق رشته نسازید**؛ جداکننده در ویندوز و یونیکس فرق دارد.

```rust
use std::path::{Path, PathBuf};

fn main() {
    let mut p = PathBuf::from("projects");
    p.push("rust");
    p.push("main.rs");
    assert_eq!(p, Path::new("projects/rust/main.rs"));

    assert_eq!(p.file_name().unwrap(), "main.rs");
    assert_eq!(p.extension().unwrap(), "rs");
    assert_eq!(p.file_stem().unwrap(), "main");
    assert_eq!(p.parent().unwrap(), Path::new("projects/rust"));

    let q = p.with_extension("txt");
    assert_eq!(q, Path::new("projects/rust/main.txt"));

    // مسیر لزوماً UTF-8 نیست؛ تبدیل به &str ممکن است None بدهد
    assert_eq!(p.to_str(), Some("projects/rust/main.rs"));
    println!("{}", p.display());      // برای چاپ از display() استفاده کنید
}
```

پیمایش پوشه:

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir().join("rust_book_walk");
    fs::create_dir_all(dir.join("sub"))?;
    fs::write(dir.join("a.txt"), "1")?;
    fs::write(dir.join("sub/b.txt"), "2")?;

    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let kind = if entry.file_type()?.is_dir() { "dir" } else { "file" };
        names.push(format!("{kind}:{}", entry.file_name().to_string_lossy()));
    }
    names.sort(); // ترتیب read_dir مشخص نیست
    assert_eq!(names, ["dir:sub", "file:a.txt"]);

    fs::remove_dir_all(&dir)
}
```

> برای پیمایش بازگشتی پوشه‌ها از crate ی `walkdir` استفاده کنید.

## آرگومان‌های خط فرمان و متغیرهای محیطی

```rust
use std::env;

fn main() {
    // آرگومان‌ها: args().next() نام برنامه است
    let args: Vec<String> = env::args().collect();
    println!("{} آرگومان", args.len());

    // اگر آرگومان غیر UTF-8 ممکن است برسد: args_os()
    for a in env::args().skip(1) {
        println!("arg: {a}");
    }

    // متغیر محیطی
    match env::var("HOME") {
        Ok(v) => println!("HOME={v}"),
        Err(env::VarError::NotPresent) => println!("HOME تنظیم نیست"),
        Err(e) => println!("{e}"),
    }
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    println!("{level}");

    // مسیر جاری
    println!("{}", env::current_dir().unwrap().display());
}
```

برای برنامه‌ی جدی از [clap](cli-clap.md) به‌جای پردازش دستی آرگومان استفاده کنید.

## کد خروج برنامه

```rust,no_run
use std::process::ExitCode;

fn run() -> Result<(), String> {
    Err("چیزی خراب شد".to_string())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("خطا: {e}");
            ExitCode::from(2) // کد دلخواه
        }
    }
}
```

- `ExitCode` به شما اجازه می‌دهد destructor ها اجرا شوند و برنامه تمیز بسته شود.
- `std::process::exit(code)` فوراً خارج می‌شود و **Drop اجرا نمی‌شود** (بافرهای نوشته‌نشده می‌توانند از بین بروند).
- `fn main() -> Result<(), E>` هم ممکن است؛ در صورت `Err`، خطا با `Debug` چاپ می‌شود و کد خروج ۱ است.

## اجرای برنامه‌ی دیگر

```rust
use std::process::Command;

fn main() {
    let out = Command::new("echo").arg("hello").output().expect("اجرا نشد");
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "hello");
}
```

> `Command` هرگز آرگومان‌ها را از طریق shell تفسیر نمی‌کند، پس خطر command injection کمتر از `system()` در C است؛ ولی اگر `Command::new("sh").arg("-c").arg(user_input)` بنویسید، دوباره آسیب‌پذیر می‌شوید.

## خلاصه

| نیاز | ابزار |
|---|---|
| ساخت رشته | `format!`، `write!` به `String` |
| چاپ سریع‌ترین | `stdout().lock()` + `BufWriter` |
| خواندن خط | `read_line` یا `lines()` روی `BufRead` |
| تست‌پذیری | توابع را روی `impl BufRead` / `impl Write` بنویسید |
| فایل کوچک | `fs::read_to_string` / `fs::write` |
| فایل بزرگ | `BufReader` / `BufWriter` |
| مسیر | `Path` / `PathBuf`، نه الحاق رشته |
| آرگومان | `env::args` یا `clap` |
| خروج | `ExitCode` |

## تمرین

1. جدولی از اعداد ۱ تا ۱۰ با ستون‌های مربع، مکعب و ریشه‌ی دوم (۳ رقم اعشار) و تراز راست چاپ کنید.
2. نوع `Matrix` را `Display` کنید طوری که `{:>6.2}` را برای هر عنصر رعایت کند (راهنمایی: `f.precision()` و `f.width()`).
3. تابعی بنویسید که از `impl BufRead` متن بگیرد و تعداد کلمات هر خط را به `impl Write` بنویسد؛ برایش تست با `Cursor` و `Vec<u8>` بنویسید.
4. برنامه‌ای بنویسید که مسیر یک پوشه را بگیرد و فایل‌ها را بر اساس اندازه مرتب چاپ کند.
5. برنامه‌ای بنویسید که با `OpenOptions` هر بار اجرا یک خط با زمان به فایل لاگ اضافه کند (از `std::time::SystemTime` استفاده کنید).
