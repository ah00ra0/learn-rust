# Rust اصطلاحی (Idiomatic) و الگوهای طراحی

کد Rust «که کامپایل می‌شود» با کد Rust «اصطلاحی» فرق دارد. کد اصطلاحی همان است که جامعه‌ی Rust می‌نویسد و می‌خواند: از قابلیت‌های زبان (نوع‌ها، trait ها، iterator ها، ownership) به‌جای مبارزه با آن‌ها استفاده می‌کند. این فصل قراردادها، الگوهای طراحی رایج و ضدالگوهایی را که مبتدی‌ها زیاد می‌نویسند، نشان می‌دهد.

> قراردادهای این فصل از [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) و عرف جامعه گرفته شده‌اند. ابزار `cargo clippy` بیشتر آن‌ها را خودکار تذکر می‌دهد؛ آن را همیشه اجرا کنید.

## ۱. قراردادهای نام‌گذاری

| چیز | قالب | مثال |
|---|---|---|
| ماژول، تابع، متد، متغیر | `snake_case` | `read_file`, `user_name` |
| نوع (struct, enum, trait) | `UpperCamelCase` | `HttpClient`, `Iterator` |
| variant ی enum | `UpperCamelCase` | `Option::Some` |
| ثابت و static | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES` |
| lifetime | کوتاه و کوچک | `'a`, `'de` |
| پارامتر نوع | یک حرف بزرگ یا نام کوتاه | `T`, `K`, `V`, `E` |
| crate | `kebab-case` یا `snake_case` | `my-crate` |

کامپایلر خودش با هشدار `non_snake_case` و مشابه، این‌ها را می‌گیرد.

### پیشوندهای متد و هزینه‌ی تبدیل

| پیشوند | هزینه | مالکیت | مثال |
|---|---|---|---|
| `as_` | ارزان (همان داده، نمای دیگر) | `&self` → `&U` | `str::as_bytes`, `Vec::as_slice` |
| `to_` | گران‌تر (محاسبه یا کپی) | `&self` → مقدار جدید | `str::to_uppercase`, `to_string`, `to_vec` |
| `into_` | مصرف می‌کند | `self` → مقدار | `String::into_bytes`, `into_iter` |
| `is_` / `has_` | بولین | `&self` | `is_empty`, `has_key` |
| `with_` | سازنده با یک گزینه | | `Vec::with_capacity` |
| `try_` | ممکن است شکست بخورد | `Result` | `try_into`, `try_from` |
| `_mut` پسوند | نسخه‌ی قابل‌تغییر | `&mut self` | `iter_mut`, `get_mut` |

**getter ها پیشوند `get_` ندارند:** برای فیلد `name` متد را `name()` بنویسید، نه `get_name()`. (استثنا: `get` وقتی یک جستجوی ممکن‌شکست است مثل `HashMap::get`.)

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: impl Into<String>, age: u32) -> Self {
        User { name: name.into(), age }
    }
    fn name(&self) -> &str {          // نه get_name و نه &String
        &self.name
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    fn into_name(self) -> String {    // مصرف می‌کند
        self.name
    }
}

fn main() {
    let u = User::new("علی", 20);
    assert_eq!(u.name(), "علی");
    assert!(u.is_adult());
    assert_eq!(u.age(), 20);
    assert_eq!(u.into_name(), "علی");
}
```

## ۲. پارامترها: مرجع یا مالک؟

**قاعده:** تابع آنچه را نیاز دارد از ضعیف‌ترین نوع بپذیرد.

| به‌جای | بنویسید | چرا |
|---|---|---|
| `&String` | `&str` | `String`, `&str`, literal همه کار می‌کنند |
| `&Vec<T>` | `&[T]` | آرایه، Vec، slice |
| `&Box<T>` | `&T` | |
| `String` (فقط می‌خوانید) | `&str` | |
| `String` (نگه می‌دارید) | `impl Into<String>` یا `String` | |
| `&PathBuf` | `&Path` / `impl AsRef<Path>` | |

```rust
// ✗ محدودکننده
fn count_words_bad(s: &String) -> usize {
    s.split_whitespace().count()
}

// ✓ اصطلاحی
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn total(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn main() {
    let owned = String::from("a b c");
    assert_eq!(count_words_bad(&owned), 3);
    assert_eq!(count_words(&owned), 3);
    assert_eq!(count_words("literal text"), 2);

    assert_eq!(total(&[1, 2, 3]), 6);
    assert_eq!(total(&vec![4, 5]), 9);
}
```

(clippy این را با `ptr_arg` تذکر می‌دهد.)

## ۳. الگوی Newtype

یک struct تک‌فیلدی که **معنا و ایمنی نوع** اضافه می‌کند، بدون هزینه‌ی اجرایی:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(u64);
#[derive(Debug, Clone, Copy, PartialEq)]
struct OrderId(u64);

fn find_user(id: UserId) -> String {
    format!("کاربر #{}", id.0)
}

#[derive(Debug, PartialEq)]
struct Meters(f64);
#[derive(Debug, PartialEq)]
struct Feet(f64);

impl From<Feet> for Meters {
    fn from(f: Feet) -> Meters {
        Meters(f.0 * 0.3048)
    }
}

fn main() {
    let u = UserId(7);
    let _o = OrderId(7);
    assert_eq!(find_user(u), "کاربر #7");
    // find_user(_o);   // ✗ خطای کامپایل: OrderId، UserId نیست

    let m: Meters = Feet(10.0).into();
    assert!((m.0 - 3.048).abs() < 1e-9);
}
```

کاربردها: جلوگیری از اشتباه گرفتن آرگومان‌های هم‌نوع (`UserId` / `OrderId`)، پیاده‌کردن trait خارجی روی نوع خارجی (قاعده‌ی orphan)، اعتبارسنجی هنگام ساخت.

### «Parse, don't validate»

اعتبارسنجی را در سازنده انجام دهید تا مقدار نامعتبر **اصلاً قابل نمایش نباشد**:

```rust
#[derive(Debug)]
struct Email(String);

impl Email {
    fn parse(s: &str) -> Result<Email, String> {
        match s.split_once('@') {
            Some((user, domain)) if !user.is_empty() && domain.contains('.') => {
                Ok(Email(s.to_string()))
            }
            _ => Err(format!("ایمیل نامعتبر: {s}")),
        }
    }
    fn as_str(&self) -> &str {
        &self.0
    }
}

// این تابع مطمئن است ایمیل معتبر دارد؛ نیازی به بررسی دوباره نیست
fn send_welcome(to: &Email) -> String {
    format!("خوش آمدید {}", to.as_str())
}

fn main() {
    let e = Email::parse("ali@example.com").unwrap();
    assert_eq!(send_welcome(&e), "خوش آمدید ali@example.com");
    assert!(Email::parse("bad").is_err());
}
```

## ۴. الگوی Builder

برای ساخت اشیای دارای گزینه‌های زیاد (Rust پارامتر پیش‌فرض و overloading ندارد). دو سبک:

### الف) Builder مصرف‌کننده (`self → Self`) برای زنجیره‌ی خطی

```rust
#[derive(Debug, PartialEq)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    timeout_secs: u32,
}

struct RequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    timeout_secs: u32,
}

impl RequestBuilder {
    fn new(url: impl Into<String>) -> Self {
        RequestBuilder {
            url: url.into(),
            method: "GET".into(),
            headers: Vec::new(),
            timeout_secs: 30,
        }
    }
    fn method(mut self, m: &str) -> Self {
        self.method = m.to_string();
        self
    }
    fn header(mut self, k: &str, v: &str) -> Self {
        self.headers.push((k.to_string(), v.to_string()));
        self
    }
    fn timeout(mut self, secs: u32) -> Self {
        self.timeout_secs = secs;
        self
    }
    fn build(self) -> Result<Request, String> {
        if !self.url.starts_with("http") {
            return Err("آدرس باید با http شروع شود".into());
        }
        Ok(Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            timeout_secs: self.timeout_secs,
        })
    }
}

fn main() {
    let r = RequestBuilder::new("https://example.com")
        .method("POST")
        .header("x-key", "1")
        .timeout(5)
        .build()
        .unwrap();
    assert_eq!(r.method, "POST");
    assert_eq!(r.timeout_secs, 5);
    assert!(RequestBuilder::new("ftp://x").build().is_err());
}
```

مزیت: `build()` می‌تواند **اعتبارسنجی** کند و `Result` برگرداند.

### ب) جایگزین ساده: `Default` + struct update

اگر اعتبارسنجی ندارید و فیلدها `pub` هستند:

```rust
#[derive(Debug, Default)]
struct Options {
    verbose: bool,
    retries: u32,
    name: String,
}

fn main() {
    let o = Options { verbose: true, ..Default::default() };
    assert!(o.verbose);
    assert_eq!(o.retries, 0);
    assert_eq!(o.name, "");
}
```

## ۵. الگوی RAII / Guard

منبع را در **سازنده** بگیرید و در **`Drop`** آزاد کنید؛ پس هرگز فراموش نمی‌شود، حتی با `return` زودهنگام، `?` یا `panic`. فایل‌ها، قفل‌ها (`MutexGuard`)، اتصال‌ها همین‌طورند.

```rust
use std::cell::RefCell;

struct Logger {
    lines: RefCell<Vec<String>>,
}

/// وقتی ساخته شود «ورود» و وقتی از بین برود «خروج» ثبت می‌کند
struct Span<'a> {
    log: &'a Logger,
    name: &'static str,
}

impl<'a> Span<'a> {
    fn enter(log: &'a Logger, name: &'static str) -> Span<'a> {
        log.lines.borrow_mut().push(format!("ورود {name}"));
        Span { log, name }
    }
}

impl Drop for Span<'_> {
    fn drop(&mut self) {
        self.log.lines.borrow_mut().push(format!("خروج {}", self.name));
    }
}

fn work(log: &Logger, fail: bool) -> Result<(), String> {
    let _span = Span::enter(log, "work");
    if fail {
        return Err("شکست".into()); // خروج، حتی با return زودهنگام ثبت می‌شود
    }
    Ok(())
}

fn main() {
    let log = Logger { lines: RefCell::new(vec![]) };
    let _ = work(&log, true);
    let _ = work(&log, false);
    assert_eq!(
        *log.lines.borrow(),
        ["ورود work", "خروج work", "ورود work", "خروج work"]
    );
}
```

> **نکته:** نام متغیر guard را `_span` بگذارید، **نه** `_`. چون `let _ = ...` مقدار را فوراً drop می‌کند و guard همان لحظه تمام می‌شود.

## ۶. الگوی Strategy: closure یا trait

رفتار قابل‌تعویض را به‌صورت پارامتر بدهید. برای رفتار ساده، **closure** کافی است:

```rust
fn apply_discount(prices: &[f64], strategy: impl Fn(f64) -> f64) -> Vec<f64> {
    prices.iter().map(|&p| strategy(p)).collect()
}

fn main() {
    let prices = [100.0, 200.0];
    assert_eq!(apply_discount(&prices, |p| p * 0.9), [90.0, 180.0]);
    assert_eq!(apply_discount(&prices, |p| p - 10.0), [90.0, 190.0]);
}
```

وقتی رفتار حالت یا چند متد دارد، **trait**:

```rust
trait Compressor {
    fn name(&self) -> &str;
    fn compress(&self, input: &str) -> String;
}

struct Rle;
struct Identity;

impl Compressor for Identity {
    fn name(&self) -> &str { "identity" }
    fn compress(&self, input: &str) -> String { input.to_string() }
}

impl Compressor for Rle {
    fn name(&self) -> &str { "rle" }
    fn compress(&self, input: &str) -> String {
        let mut out = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            let mut n = 1;
            while chars.peek() == Some(&c) {
                chars.next();
                n += 1;
            }
            out.push_str(&format!("{n}{c}"));
        }
        out
    }
}

fn run(c: &dyn Compressor, s: &str) -> String {
    format!("{}: {}", c.name(), c.compress(s))
}

fn main() {
    assert_eq!(run(&Rle, "aaabcc"), "rle: 3a1b2c");
    assert_eq!(run(&Identity, "aaabcc"), "identity: aaabcc");
}
```

## ۷. الگوی Command با enum

به‌جای hierarchy ی کلاس‌ها، **enum** + `match`:

```rust
#[derive(Debug)]
enum Command {
    Push(i64),
    Add,
    Mul,
    Dup,
}

fn run(commands: &[Command]) -> Result<Vec<i64>, String> {
    let mut stack: Vec<i64> = Vec::new();
    for cmd in commands {
        match cmd {
            Command::Push(n) => stack.push(*n),
            Command::Dup => {
                let top = *stack.last().ok_or("پشته خالی است")?;
                stack.push(top);
            }
            Command::Add | Command::Mul => {
                let b = stack.pop().ok_or("عملوند کم است")?;
                let a = stack.pop().ok_or("عملوند کم است")?;
                stack.push(if matches!(cmd, Command::Add) { a + b } else { a * b });
            }
        }
    }
    Ok(stack)
}

fn main() {
    use Command::*;
    assert_eq!(run(&[Push(2), Push(3), Add, Dup, Mul]), Ok(vec![25]));
    assert_eq!(run(&[Add]), Err("عملوند کم است".to_string()));
}
```

مزیت بزرگ: وقتی variant جدیدی اضافه کنید، کامپایلر تمام `match` های ناکامل را نشان می‌دهد. الگوی **Visitor** در زبان‌های شی‌گرا معمولاً در Rust با enum + `match` (یا trait با متد برای هر نوع) جایگزین می‌شود.

## ۸. الگوی Type State

وضعیت‌های مجاز را در **نوع** کدگذاری کنید تا استفاده‌ی نادرست کامپایل نشود. مثال کامل در فصل [Trait Object](24-trait-objects.md) آمده است. نمونه‌ی کوتاه با نوع phantom:

```rust
use std::marker::PhantomData;

struct Open;
struct Closed;

struct Door<State> {
    opened_times: u32,
    _state: PhantomData<State>,
}

impl Door<Closed> {
    fn new() -> Self {
        Door { opened_times: 0, _state: PhantomData }
    }
    fn open(self) -> Door<Open> {
        Door { opened_times: self.opened_times + 1, _state: PhantomData }
    }
}

impl Door<Open> {
    fn close(self) -> Door<Closed> {
        Door { opened_times: self.opened_times, _state: PhantomData }
    }
    fn walk_through(&self) -> &'static str {
        "رد شدید"
    }
}

fn main() {
    let door = Door::new();
    let open = door.open();
    assert_eq!(open.walk_through(), "رد شدید");
    let closed = open.close();
    assert_eq!(closed.opened_times, 1);
    // closed.walk_through();   // ✗ خطای کامپایل: متد روی Door<Closed> وجود ندارد
}
```

## ۹. ضدالگوهای رایج (قبل ← بعد)

### ۱. `clone()` برای فرار از borrow checker

```rust
fn main() {
    let names = vec![String::from("الف"), String::from("ب")];

    // ✗ کپی کل بردار فقط برای پیمایش
    for n in names.clone() {
        let _ = n.len();
    }

    // ✓ قرض بگیرید
    for n in &names {
        let _ = n.len();
    }
    assert_eq!(names.len(), 2);
}
```

(کپی گاهی لازم است و اشکالی ندارد؛ مشکل وقتی است که **از سر ناآگاهی** برای «ساکت کردن» کامپایلر بزنید. اول بفهمید چرا خطا آمده.)

### ۲. حلقه‌ی اندیس‌دار

```rust
fn main() {
    let v = vec![10, 20, 30];

    // ✗
    let mut total = 0;
    for i in 0..v.len() {
        total += v[i];
    }

    // ✓
    let total2: i32 = v.iter().sum();
    assert_eq!(total, total2);

    // اگر هم اندیس و هم مقدار می‌خواهید
    for (i, x) in v.iter().enumerate() {
        assert_eq!(*x, v[i]);
    }
}
```

### ۳. `match` ی بی‌جهت طولانی روی Option/Result

```rust
fn main() {
    let maybe: Option<i32> = Some(3);

    // ✗
    let a = match maybe {
        Some(x) => x * 2,
        None => 0,
    };

    // ✓
    let b = maybe.map(|x| x * 2).unwrap_or(0);
    assert_eq!(a, b);

    // ✗
    let c = if maybe.is_some() { maybe.unwrap() } else { 0 };
    // ✓
    let d = maybe.unwrap_or(0);
    assert_eq!(c, d);

    // وقتی فقط یک حالت مهم است
    if let Some(x) = maybe {
        assert_eq!(x, 3);
    }
}
```

### ۴. `unwrap()` در کد کتابخانه

در کتابخانه، `unwrap`/`expect` خطای قابل‌بازیابی را به crash تبدیل می‌کند. به‌جایش `Result` برگردانید و `?` بزنید ([مدیریت خطا](14-errors.md)). `unwrap` برای مثال‌ها، تست‌ها و وقتی که **واقعاً** غیرممکن بودن شکست را اثبات می‌کنید قابل قبول است (با `expect("دلیل")`).

### ۵. رشته‌ای‌سازی (stringly-typed)

```rust
// ✗ هر رشته‌ای قابل ارسال است؛ غلط املایی در زمان اجرا
fn set_mode_bad(mode: &str) -> bool {
    mode == "fast"
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Fast,
    Safe,
}

// ✓ فقط مقادیر معتبر قابل ساخت‌اند
fn set_mode(mode: Mode) -> bool {
    mode == Mode::Fast
}

fn main() {
    assert!(set_mode_bad("fast"));
    assert!(!set_mode_bad("fsat")); // غلط املایی بی‌صدا
    assert!(set_mode(Mode::Fast));
    assert!(!set_mode(Mode::Safe));
}
```

### ۶. `bool` های بی‌معنا در پارامتر

```rust
#[derive(Clone, Copy)]
enum Visibility { Public, Private }
#[derive(Clone, Copy)]
enum Mutability { ReadOnly, ReadWrite }

// ✗ create_file("a", true, false)  ← چه معنایی دارد؟
// ✓ 
fn create_file(_name: &str, vis: Visibility, m: Mutability) -> (bool, bool) {
    (matches!(vis, Visibility::Public), matches!(m, Mutability::ReadWrite))
}

fn main() {
    assert_eq!(create_file("a", Visibility::Public, Mutability::ReadOnly), (true, false));
    let _ = (Visibility::Private, Mutability::ReadWrite);
}
```

### ۷. `Rc<RefCell<T>>` و `Arc<Mutex<T>>` در همه‌جا

اگر طراحی‌تان به آن‌ها نیاز دارد، آگاهانه بپذیریدش؛ ولی اگر فقط چون «کامپایلر غر می‌زند» اضافه می‌کنید، معمولاً ساختار داده را عوض کنید: داده‌ی مشترک را در **یک مالک** نگه دارید و به بقیه **اندیس/شناسه** (نه مرجع) بدهید. (فصل [ساختمان داده](data-structures-algorithms.md) این روش را دارد.)

### ۸. اصرار بر وراثت

Rust وراثت ندارد. **ترکیب (composition)** + trait: struct ی کوچک را در struct بزرگ‌تر بگذارید و رفتار مشترک را با trait و پیاده‌سازی پیش‌فرض بیان کنید.

## ۱۰. طراحی API کتابخانه

### مستندسازی

بخش‌های استاندارد کامنت `///`:

```rust
/// اولین کلمه‌ی متن را برمی‌گرداند.
///
/// # Examples
///
/// ```
/// let w = first_word("سلام دنیا");
/// assert_eq!(w, Some("سلام"));
/// ```
///
/// # Errors
/// (اگر `Result` برمی‌گرداند: چه زمانی خطا می‌دهد)
///
/// # Panics
/// (اگر ممکن است panic کند)
///
/// # Safety
/// (برای تابع unsafe: شرایطی که فراخواننده باید تضمین کند)
pub fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

fn main() {
    assert_eq!(first_word("a b"), Some("a"));
}
```

مثال‌های داخل `///` هنگام `cargo test` اجرا می‌شوند (فصل [تست‌نویسی](23-testing.md)).

### trait های رایج را پیاده کنید

نوع‌های عمومی را با `Debug`، `Clone`، `PartialEq`، `Default` و در صورت امکان `Hash/Eq/Ord` عرضه کنید تا کاربران محدود نشوند. این‌ها را بعداً **نمی‌توان** بدون شکستن سازگاری حذف کرد.

### `#[non_exhaustive]`

برای اینکه بتوانید بعداً variant یا فیلد اضافه کنید بدون اینکه تغییر major باشد:

```rust
#[non_exhaustive]
#[derive(Debug)]
pub enum ParseError {
    Empty,
    TooLong(usize),
}

fn main() {
    let e = ParseError::Empty;
    // در همان crate match کامل مجاز است؛ در crate های دیگر باید `_ =>` بگذارند
    match e {
        ParseError::Empty => {}
        ParseError::TooLong(_) => {}
    }
}
```

### Sealed Trait

برای trait ی که فقط شما اجازه‌ی پیاده‌سازی‌اش را می‌خواهید بدهید (تا بعداً متد اضافه کنید بدون شکستن کاربران):

```rust
mod private {
    pub trait Sealed {}
}

pub trait Format: private::Sealed {
    fn extension(&self) -> &'static str;
}

pub struct Json;
pub struct Toml;

impl private::Sealed for Json {}
impl private::Sealed for Toml {}
impl Format for Json {
    fn extension(&self) -> &'static str { "json" }
}
impl Format for Toml {
    fn extension(&self) -> &'static str { "toml" }
}

fn main() {
    assert_eq!(Json.extension(), "json");
    assert_eq!(Toml.extension(), "toml");
}
```

(کاربران بیرونی نمی‌توانند `Sealed` را پیاده کنند چون ماژول `private` برایشان قابل دسترس نیست، پس `Format` هم نمی‌توانند.)

### خطاها

- برای کتابخانه، enum خطای مشخص (با `thiserror`) بدهید؛ نه `Box<dyn Error>` و نه `String`. برای برنامه، `anyhow` راحت‌تر است ([خطا و لاگ](errors-logging-libs.md)).
- `Display` را کوتاه و با حرف کوچک شروع کنید و بدون نقطه‌ی پایانی (عرف std)، و `source()` را برای زنجیره‌ی خطا پیاده کنید.

### Generic در برابر `impl Trait` در پارامتر

`fn f(x: impl Display)` برای پارامتر ساده کوتاه‌تر است. اگر لازم است کاربر نوع را با turbofish مشخص کند یا چند پارامتر باید **یک نوع** باشند، `fn f<T: Display>(a: T, b: T)` بنویسید.

## ۱۱. ابزارهای اصطلاحی‌نویسی

```bash
cargo fmt                 # قالب یکسان؛ بحث سلیقه را حذف می‌کند
cargo clippy              # صدها lint برای اصطلاحی‌نویسی
cargo clippy -- -W clippy::pedantic    # سخت‌گیرانه‌تر (اختیاری)
```

lint های مفید clippy برای یادگیری:

| Lint | چه می‌گیرد |
|---|---|
| `needless_range_loop` | `for i in 0..v.len()` |
| `ptr_arg` | `&String` / `&Vec<T>` در پارامتر |
| `redundant_clone` | `clone` ی غیرضروری |
| `manual_map`, `manual_unwrap_or` | `match` ی قابل‌ساده‌سازی |
| `new_without_default` | `new()` بدون `Default` |
| `len_without_is_empty` | `len()` بدون `is_empty()` |
| `unwrap_used`, `expect_used` | (غیرپیش‌فرض) ممنوع کردن unwrap در کتابخانه |
| `missing_errors_doc`, `missing_panics_doc` | (pedantic) مستندات خطا |

## خلاصه‌ی اصول

۱. نوع‌ها را طوری بسازید که **حالت‌های نامعتبر قابل‌نمایش نباشند.**
۲. ضعیف‌ترین نوع پارامتر را بپذیرید (`&str`, `&[T]`).
۳. منبع را به **RAII** بسپارید.
۴. رفتارهای متنوع: closure → trait → enum، به همین ترتیب پیچیدگی.
۵. **ترکیب** به‌جای وراثت.
۶. خطا را **برگردانید**، panic نکنید.
۷. اول **clippy و fmt** را اجرا کنید.

## تمرین

1. struct ی `Temperature` با newtype بسازید که فقط مقادیر بالاتر از `-273.15` را بپذیرد (`parse`).
2. `Builder` برای `HttpServer` با فیلدهای `host`, `port`, `workers` بنویسید که در `build()` مقادیر را اعتبارسنجی کند.
3. یک guard بنویسید که هنگام drop، زمان سپری‌شده از ساختنش را در یک متغیر ثبت کند.
4. با الگوی type state، یک «فرم ثبت‌نام» مدل کنید که فقط وقتی `Validated` است بتوان `submit` کرد.
5. کد زیر را بازنویسی کنید تا clippy هیچ هشداری ندهد: `fn f(v: &Vec<String>) -> usize { let mut n = 0; for i in 0..v.len() { if v[i].len() > 3 { n += 1; } } n }`
