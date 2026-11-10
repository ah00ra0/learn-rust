# پاسخ تمرین‌ها (بخش ۱): فصل‌های ۲ تا ۱۲

این صفحه پاسخ تمرین‌های انتهای فصل‌های ۲ تا ۱۲ را دارد. پیشنهاد جدی: **اول خودتان تمرین را حل کنید**، حتی اگر ناقص بود، و بعد پاسخ را ببینید. یادگیری Rust بیشتر از هر چیز از جنگیدن با کامپایلر می‌آید، نه از خواندن راه‌حل آماده.

چند نکته درباره‌ی پاسخ‌ها:

- هر پاسخ فقط **یک** راه‌حل ممکن است. اگر راه‌حل شما متفاوت اما درست بود، اشکالی ندارد.
- همه‌ی بلوک‌های `rust` این صفحه با `mdbook test` کامپایل و اجرا می‌شوند. جایی که از `assert!` و `assert_eq!` استفاده شده، یعنی نتیجه واقعاً بررسی شده است.
- تمرین‌هایی که ذاتاً «خطای کامپایلر را بخوانید» هستند با برچسب `compile_fail` آمده‌اند: این بلوک‌ها **عمداً** کامپایل نمی‌شوند و mdBook همین را تأیید می‌کند.
- فصل ۱ (نصب) تمرین ندارد.

---

## [فصل ۲: Hello World و Cargo](02-hello-cargo.md)

**تمرین ۱ تا ۴.** این تمرین‌ها در ترمینال انجام می‌شوند:

```bash
cargo new my_first
cd my_first
cargo run
```

فایل `src/main.rs` را این‌طور تغییر دهید (نام خودتان را بگذارید و سه خط چاپ کنید):

```rust
fn main() {
    println!("سلام، من آهورا هستم!");
    println!("این خط دوم است.");
    println!("و این خط سوم.");
}
```

برای بیلد release:

```bash
cargo build --release
```

فایل اجرایی در `target/release/my_first` است (در ویندوز `target\release\my_first.exe`). اگر `CARGO_TARGET_DIR` یا تنظیمات workspace تغییر داده شده باشد، محل `target` هم تغییر می‌کند. دستور `cargo build --release --message-format short` یا اجرای `cargo metadata` محل دقیق را نشان می‌دهد.

---

## [فصل ۳: متغیرها و تغییرپذیری](03-variables.md)

**تمرین ۱.** تغییر دادن متغیر immutable خطای `E0384` می‌دهد:

```rust,compile_fail
fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    println!("x = {x}");
}
```

پیام کامپایلر می‌گوید «نمی‌توان دو بار به متغیر immutable مقدار داد» و پیشنهاد می‌کند `let mut x` بنویسید. بخوانید: کامپایلر Rust معمولاً راه‌حل را هم می‌گوید.

**تمرین ۲.** با `mut`:

```rust
fn main() {
    let mut x = 5;
    x = 6;
    x += 1;
    assert_eq!(x, 7);
}
```

**تمرین ۳.** `const` باید نوع صریح داشته باشد و نامش به‌صورت `SCREAMING_SNAKE_CASE` است:

```rust
const PI: f64 = 3.14159;

fn main() {
    let radius = 5.0;
    let area = PI * radius * radius;
    println!("مساحت: {area}");
    assert!((area - 78.53975).abs() < 1e-9);
}
```

> **نکته:** در کد واقعی `std::f64::consts::PI` دقیق‌تر است و نیازی به تعریف ثابت خودتان نیست. اینجا فقط برای تمرین `const` خواسته شده بود.

**تمرین ۴.** shadowing اجازه می‌دهد با همان نام، نوع را عوض کنید (برخلاف `mut` که نوع را ثابت نگه می‌دارد):

```rust
fn main() {
    let value = "42";
    let value = value.parse::<i32>().unwrap(); // اینجا مقدار ثابت و مطمئن است
    assert_eq!(value + 1, 43);
}
```

> **هشدار:** `unwrap` اینجا فقط چون رشته‌ی ثابت `"42"` است قابل قبول است. برای ورودی کاربر از `match` یا `?` استفاده کنید ([فصل ۱۴](14-errors.md)).

---

## [فصل ۴: انواع داده](04-data-types.md)

**تمرین ۱.** `checked_add` در سرریز `None` برمی‌گرداند و panic نمی‌کند:

```rust
fn main() {
    let x: u8 = 255;
    assert_eq!(x.checked_add(1), None);
    assert_eq!(x.checked_add(0), Some(255));
    // گزینه‌های دیگر برای مدیریت سرریز:
    assert_eq!(x.wrapping_add(1), 0);
    assert_eq!(x.saturating_add(1), 255);
    assert_eq!(x.overflowing_add(1), (0, true));
}
```

**تمرین ۲.** tuple و destructuring:

```rust
fn main() {
    let user = ("علی", 29_u32, 87.5_f64);
    let (name, age, score) = user;
    println!("{name} ({age} ساله) امتیاز {score}");
    assert_eq!(user.1, 29);
}
```

**تمرین ۳.**

```rust
fn main() {
    let days = [
        "شنبه", "یکشنبه", "دوشنبه", "سه‌شنبه", "چهارشنبه", "پنجشنبه", "جمعه",
    ];
    // اندیس از صفر شروع می‌شود، پس روز سوم اندیس ۲ است
    println!("{}", days[2]);
    assert_eq!(days[2], "دوشنبه");
    assert_eq!(days.len(), 7);
}
```

**تمرین ۴.** Rust هیچ تبدیل عددی ضمنی انجام نمی‌دهد، حتی از نوع کوچک‌تر به بزرگ‌تر:

```rust,compile_fail
fn main() {
    let x: i32 = 5;
    let y: i64 = x; // error[E0308]: mismatched types
    println!("{y}");
}
```

سه راه درست:

```rust
fn main() {
    let x: i32 = 5;
    let a: i64 = i64::from(x); // ایمن: تبدیل بدون از دست رفتن اطلاعات (From)
    let b: i64 = x.into();     // همان، با inference
    let c = x as i64;          // cast صریح؛ برای تبدیل‌های «لب‌به‌لب» دقت کنید
    assert_eq!((a, b, c), (5, 5, 5));
}
```

قاعده‌ی سرانگشتی: وقتی تبدیل همیشه امن است `From/Into`، وقتی ممکن است شکست بخورد `TryFrom` (مثلاً `u8::try_from(300_i32)` یک `Err` می‌دهد) و `as` را فقط وقتی رفتار برش‌دادن (truncation) را می‌خواهید.

---

## [فصل ۵: توابع و کامنت‌ها](05-functions.md)

**تمرین ۱.**

```rust
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
}
```

به‌جای `9/5` که در اعداد صحیح می‌شود `1`، از `9.0 / 5.0` استفاده کردیم. اگر ورودی را `i32` بگیرید باید این نکته را در نظر بگیرید.

**تمرین ۲.** برگرداندن چند مقدار با tuple:

```rust
fn square_and_cube(n: i32) -> (i32, i32) {
    (n * n, n * n * n)
}

fn main() {
    let (sq, cu) = square_and_cube(3);
    assert_eq!((sq, cu), (9, 27));
}
```

**تمرین ۳.** فیبوناچی با `if` و `return` (نسخه‌ی تکراری، نه بازگشتی، چون بازگشتی ساده برای n بزرگ نمایی کند می‌شود):

```rust
fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut prev, mut curr) = (0u64, 1u64);
    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(50), 12_586_269_025);
}
```

> **نکته:** `fib(94)` از `u64` بیشتر می‌شود. در حالت debug با panic روبه‌رو می‌شوید؛ برای کد واقعی `checked_add` به کار ببرید.

**تمرین ۴.** doc comment با مثال. آنچه بین سه بک‌تیک داخل `///` می‌آید، هنگام `cargo test` (بخش doc tests) اجرا می‌شود. چون داخل یک پروژه‌ی کتابخانه‌ای باید باشد تا تست شود، نسخه‌ی کامل در پروژه‌ی [`exercises-solutions`](https://github.com/ah00ra0/learn-rust/tree/main/projects/exercises-solutions) است (`src/lib.rs`). شکل کلی:

```rust,ignore
/// دمای سانتی‌گراد را به فارنهایت تبدیل می‌کند.
///
/// # Examples
///
/// ```
/// use exercises_solutions::celsius_to_fahrenheit;
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
```

---

## [فصل ۶: کنترل جریان](06-control-flow.md)

**تمرین ۱.** FizzBuzz. ترتیب شرط‌ها مهم است: حالت «هر دو» باید اول بیاید. `match` روی tuple این را زیبا می‌کند:

```rust
fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

fn main() {
    for n in 1..=100 {
        println!("{}", fizzbuzz(n));
    }
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(9), "Fizz");
    assert_eq!(fizzbuzz(10), "Buzz");
    assert_eq!(fizzbuzz(7), "7");
}
```

**تمرین ۲.** فاکتوریل ۱۰ با `while`:

```rust
fn main() {
    let mut n: u64 = 10;
    let mut result: u64 = 1;
    while n > 1 {
        result *= n;
        n -= 1;
    }
    println!("10! = {result}");
    assert_eq!(result, 3_628_800);
}
```

**تمرین ۳.** `loop` می‌تواند با `break مقدار` مقدار برگرداند:

```rust
fn main() {
    let mut n = 0u32;
    let found = loop {
        n += 1;
        if n * n > 1000 {
            break n;
        }
    };
    assert_eq!(found, 32); // 32 * 32 = 1024 و 31 * 31 = 961
}
```

**تمرین ۴.** جدول ضرب:

```rust
fn main() {
    for i in 1..=9 {
        for j in 1..=9 {
            print!("{:3}", i * j);
        }
        println!();
    }
}
```

`{:3}` عدد را با عرض حداقل ۳ و راست‌چین چاپ می‌کند تا ستون‌ها تراز شوند.

---

## [فصل ۷: Ownership](07-ownership.md)

**تمرین ۱.** خطای move:

```rust,compile_fail
fn main() {
    let s1 = String::from("سلام");
    let s2 = s1; // مالکیت از s1 به s2 منتقل شد
    println!("{s1} {s2}"); // error[E0382]: borrow of moved value: `s1`
}
```

پیام می‌گوید `s1` از نوع `String` است که `Copy` نیست، پس `let s2 = s1` آن را **move** کرده است. با `clone` یک کپی عمیق جدا می‌سازیم:

```rust
fn main() {
    let s1 = String::from("سلام");
    let s2 = s1.clone();
    println!("{s1} {s2}");
    assert_eq!(s1, s2);
}
```

(راه دیگر: اگر فقط خواندن لازم است، `let s2 = &s1;` که قرض‌گیری است و کپی نمی‌کند.)

**تمرین ۲.** تابعی که مالکیت را می‌گیرد و پس می‌دهد:

```rust
fn add_exclamation(mut s: String) -> String {
    s.push('!');
    s
}

fn main() {
    let s = String::from("سلام");
    let s = add_exclamation(s); // shadowing؛ s قبلی move شده بود
    assert_eq!(s, "سلام!");
}
```

در عمل معمولاً `fn add_exclamation(s: &mut String)` طبیعی‌تر است تا مالکیت رفت‌وبرگشت نکند ([فصل ۸](08-borrowing.md)).

**تمرین ۳.** آرایه‌ی `[i32; 3]` نوعی است که `Copy` پیاده کرده؛ همه‌ی بایت‌هایش روی stack است و کپی بیت‌به‌بیت ارزان و امن است، پس `let b = a;` یک کپی می‌سازد و `a` همچنان معتبر می‌ماند. `Vec` اشاره‌گری به heap دارد؛ کپی بیت‌به‌بیت دو مالک برای یک بافر می‌سازد و باعث double free می‌شد، پس `Vec` فقط `Clone` است (کپی صریح و عمیق) و انتساب آن move است:

```rust
fn main() {
    let a = [1, 2, 3];
    let b = a; // Copy
    assert_eq!(a, b);

    let v = vec![1, 2, 3];
    let w = v; // Move
    // println!("{v:?}"); // اگر فعال شود: E0382
    assert_eq!(w, [1, 2, 3]);
}
```

---

## [فصل ۸: Reference و Borrowing](08-borrowing.md)

**تمرین ۱.** در عمل پارامتر را `&[i32]` بگیرید (چون `&Vec<i32>` را هم می‌پذیرد، در [فصل ۹](09-slices.md)) ولی چون تمرین `&Vec<i32>` خواسته:

```rust
fn sum(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(sum(&v), 10);
    assert_eq!(v.len(), 4); // v هنوز مال ماست
}
```

> **نکته:** `clippy` در اینجا `ptr_arg` را هشدار می‌دهد و پیشنهاد می‌کند `&[i32]` بگیرید؛ چون هر چه `&Vec` می‌پذیرد، `&[i32]` هم می‌پذیرد و بیشتر هم می‌پذیرد.

**تمرین ۲.**

```rust
fn push_end(v: &mut Vec<i32>, x: i32) {
    v.push(x);
}

fn main() {
    let mut v = vec![1, 2];
    push_end(&mut v, 3);
    assert_eq!(v, [1, 2, 3]);
}
```

**تمرین ۳.** دو `&mut` همزمان:

```rust,compile_fail
fn main() {
    let mut s = String::from("سلام");
    let r1 = &mut s;
    let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    r1.push('!');
    r2.push('?');
}
```

کامپایلر اشاره می‌کند «اولین قرض mutable اینجاست» و «دومین اینجاست» و «اولین بعداً استفاده شده». قاعده: در هر لحظه یا یک `&mut` یا چند `&`. این قانون data race را در زمان کامپایل غیرممکن می‌کند. با NLL، اگر `r1` پیش از ساختن `r2` آخرین استفاده‌اش را کرده باشد، مشکلی نیست.

**تمرین ۴.** بزرگ‌ترین عدد به‌صورت reference. چون آرایه ممکن است خالی باشد، `Option<&i32>` برمی‌گردانیم. با فقط یک پارامتر reference، lifetime elision خودش خروجی را به ورودی گره می‌زند و لازم نیست `'a` بنویسیم:

```rust
fn largest(nums: &[i32]) -> Option<&i32> {
    let mut best = nums.first()?;
    for n in nums {
        if n > best {
            best = n;
        }
    }
    Some(best)
}

fn main() {
    let data = [3, 9, 2, 7];
    assert_eq!(largest(&data), Some(&9));
    assert_eq!(largest(&[]), None);
}
```

(معادل کوتاه: `nums.iter().max()`.)

---

## [فصل ۹: Slice](09-slices.md)

**تمرین ۱.** دومین کلمه (برگرداندن `Option<&str>` چون ممکن است وجود نداشته باشد):

```rust
fn second_word(s: &str) -> Option<&str> {
    s.split_whitespace().nth(1)
}

fn main() {
    assert_eq!(second_word("سلام دنیای زیبا"), Some("دنیای"));
    assert_eq!(second_word("تنها"), None);
    assert_eq!(second_word("   "), None);
}
```

`split_whitespace` چند فاصله‌ی پشت‌سرهم و tab و خط جدید را هم درست مدیریت می‌کند و تکه‌هایی از همان رشته‌ی اصلی (بدون کپی) برمی‌گرداند.

**تمرین ۲.**

```rust
fn max_of(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().max()
}

fn main() {
    assert_eq!(max_of(&[4, 8, 1]), Some(8));
    assert_eq!(max_of(&[]), None);
}
```

**تمرین ۳.** `len()` تعداد **بایت** را برمی‌گرداند، نه تعداد حرف. `String` و `&str` در Rust همیشه UTF-8 هستند و هر حرف فارسی در UTF-8 دو بایت می‌گیرد:

```rust
fn main() {
    let s = "ایران";
    assert_eq!(s.len(), 10);            // 5 حرف × 2 بایت
    assert_eq!(s.chars().count(), 5);   // تعداد code point
}
```

به همین دلیل `&s[0..1]` روی این رشته panic می‌کند (مرز حرف را وسط می‌شکند)، ولی `&s[0..2]` معتبر است و `"ا"` را می‌دهد. برای شمردن «حرف» به معنای کاربری (grapheme cluster) باید از crate ‏`unicode-segmentation` کمک بگیرید؛ `chars()` هم هنوز گاهی با آنچه کاربر «یک حرف» می‌بیند فرق دارد.

---

## [فصل ۱۰: Struct](10-structs.md)

**تمرین ۱.** `Book`:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }

    fn summary(&self) -> String {
        format!("«{}» نوشته‌ی {} ({} صفحه)", self.title, self.author, self.pages)
    }
}

fn main() {
    let book = Book::new("بوف کور", "صادق هدایت", 120);
    println!("{}", book.summary());
    assert!(book.summary().contains("بوف کور"));
}
```

**تمرین ۲.** `Circle`:

```rust
use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let c = Circle { radius: 2.0 };
    assert!((c.area() - 12.566370614359172).abs() < 1e-9);
    assert!((c.circumference() - 12.566370614359172).abs() < 1e-9);
}
```

**تمرین ۳.** `Counter` (متد تغییردهنده `&mut self`، متد خواننده `&self`):

```rust
#[derive(Default)]
struct Counter {
    count: u32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }

    fn get(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter::default();
    c.increment();
    c.increment();
    assert_eq!(c.get(), 2);
}
```

توجه کنید که `c` باید `mut` باشد تا بتوان `increment` را صدا زد.

**تمرین ۴.** `Meters` به `Feet` (هر متر ≈ ۳٫۲۸۰۸۴ فوت). نوع‌های جدا برای واحدهای مختلف (الگوی newtype) جلوی اشتباه‌هایی مثل جمع متر با فوت را در زمان کامپایل می‌گیرد:

```rust
struct Meters(f64);
struct Feet(f64);

fn to_feet(m: &Meters) -> Feet {
    Feet(m.0 * 3.280_84)
}

fn main() {
    let m = Meters(100.0);
    let f = to_feet(&m);
    assert!((f.0 - 328.084).abs() < 1e-9);
}
```

(می‌توانستید `impl From<Meters> for Feet` هم بنویسید؛ در [فصل ۱۶](16-traits.md) می‌بینید.)

---

## [فصل ۱۱: Enum، match و Option](11-enums.md)

**تمرین ۱.** `Shape` و مساحت. برای مثلث با سه ضلع از فرمول هرون استفاده می‌کنیم:

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn main() {
    assert_eq!(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
    assert!((area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-12);
    assert!((area(&Shape::Circle(1.0)) - std::f64::consts::PI).abs() < 1e-12);
}
```

(برای مثلثی که نامعتبر است، مثل اضلاع `1, 1, 10`، مقدار زیر رادیکال منفی و نتیجه `NaN` می‌شود؛ در برنامه‌ی واقعی باید `Option` برگردانید یا با سازنده‌ی معتبرساز جلوی ساخت مثلث نامعتبر را بگیرید.)

**تمرین ۲.**

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    assert_eq!(divide(10.0, 4.0), Some(2.5));
    assert_eq!(divide(1.0, 0.0), None);
    // استفاده با combinator ها
    assert_eq!(divide(8.0, 2.0).map(|x| x * 10.0), Some(40.0));
    assert_eq!(divide(8.0, 0.0).unwrap_or(f64::NAN).is_nan(), true);
}
```

**تمرین ۳.** `TrafficLight`:

```rust
use std::time::Duration;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> Duration {
        match self {
            TrafficLight::Red => Duration::from_secs(30),
            TrafficLight::Yellow => Duration::from_secs(5),
            TrafficLight::Green => Duration::from_secs(25),
        }
    }
}

fn main() {
    assert_eq!(TrafficLight::Red.duration().as_secs(), 30);
    assert_eq!(TrafficLight::Yellow.duration().as_secs(), 5);
    assert_eq!(TrafficLight::Green.duration().as_secs(), 25);
}
```

حالا یک حالت جدید (مثلاً `Flashing`) اضافه می‌کنیم، بدون به‌روز کردن `match`:

```rust,compile_fail
use std::time::Duration;

enum TrafficLight {
    Red,
    Yellow,
    Green,
    Flashing, // حالت جدید
}

impl TrafficLight {
    fn duration(&self) -> Duration {
        match self {
            TrafficLight::Red => Duration::from_secs(30),
            TrafficLight::Yellow => Duration::from_secs(5),
            TrafficLight::Green => Duration::from_secs(25),
            // error[E0004]: non-exhaustive patterns: `&TrafficLight::Flashing` not covered
        }
    }
}
```

این ویژگی مهم‌ترین مزیت `match` است: کامپایلر **همه‌ی مکان‌هایی** را که باید با حالت جدید کنار بیایند نشان می‌دهد. اگر در `match` از `_ => ...` استفاده می‌کردید، این محافظت از بین می‌رفت؛ پس برای enum های خودتان که ممکن است رشد کنند، از `_` بپرهیزید.

---

## [فصل ۱۲: مجموعه‌ها](12-collections.md)

**تمرین ۱.** میانگین و میانه:

```rust
fn mean(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let sum: i64 = v.iter().map(|&x| i64::from(x)).sum();
    Some(sum as f64 / v.len() as f64)
}

fn median(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut sorted = v.to_vec(); // کپی می‌کنیم تا ورودی را تغییر ندهیم
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        Some(f64::from(sorted[mid]))
    } else {
        Some((f64::from(sorted[mid - 1]) + f64::from(sorted[mid])) / 2.0)
    }
}

fn main() {
    let data = [5, 1, 9, 3];
    assert_eq!(mean(&data), Some(4.5));
    assert_eq!(median(&data), Some(4.0)); // (3 + 5) / 2
    assert_eq!(median(&[7, 1, 3]), Some(3.0));
    assert_eq!(mean(&[]), None);
}
```

جمع را در `i64` می‌کنیم تا سرریز `i32` رخ ندهد.

**تمرین ۲.** Pig Latin. قاعده‌ی تمرین: کلمه‌ای که با بی‌صدا شروع شود، حرف اولش به انتها می‌رود و `ay` می‌گیرد. برای کلمه‌ی شروع‌شده با واکه (چیزی که تمرین نگفته) قرارداد معمول «`hay` به انتها» را می‌گیریم. حواسمان به UTF-8 است: به‌جای برش بایتی، از `chars()` استفاده می‌کنیم.

```rust
fn pig_latin_word(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    if "aeiouAEIOU".contains(first) {
        format!("{word}hay")
    } else {
        format!("{}{first}ay", chars.as_str())
    }
}

fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(pig_latin_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    assert_eq!(pig_latin_word("first"), "irstfay");
    assert_eq!(pig_latin_word("apple"), "applehay");
    assert_eq!(pig_latin("hello world"), "ellohay orldway");
    assert_eq!(pig_latin_word(""), "");
}
```

`chars.as_str()` باقی‌مانده‌ی iterator را به‌صورت `&str` می‌دهد (بدون allocate اضافه).

**تمرین ۳.** سیستم کارمندان. `HashMap<String, Vec<String>>`؛ با `entry().or_default()` اضافه می‌کنیم و برای چاپ مرتب، هم کلیدها و هم هر لیست را مرتب می‌کنیم (چون ترتیب `HashMap` تعریف‌نشده است). از `BTreeMap` هم می‌شد استفاده کرد تا کلیدها خودکار مرتب باشند، اما خود تمرین `HashMap` خواسته:

```rust
use std::collections::HashMap;

#[derive(Default)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn add(&mut self, department: &str, person: &str) {
        self.departments
            .entry(department.to_string())
            .or_default()
            .push(person.to_string());
    }

    /// فرمان‌هایی مانند "Add Sally to Engineering" را اجرا می‌کند.
    fn run_command(&mut self, cmd: &str) -> bool {
        let words: Vec<&str> = cmd.split_whitespace().collect();
        match words.as_slice() {
            ["Add", person, "to", department] => {
                self.add(department, person);
                true
            }
            _ => false,
        }
    }

    fn sorted_people(&self, department: &str) -> Vec<&str> {
        let mut people: Vec<&str> = self
            .departments
            .get(department)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default();
        people.sort();
        people
    }

    fn print_all(&self) {
        let mut names: Vec<&String> = self.departments.keys().collect();
        names.sort();
        for dept in names {
            println!("{dept}: {}", self.sorted_people(dept).join(", "));
        }
    }
}

fn main() {
    let mut company = Company::default();
    assert!(company.run_command("Add Sally to Engineering"));
    assert!(company.run_command("Add Amir to Engineering"));
    assert!(company.run_command("Add Reza to Sales"));
    assert!(!company.run_command("Remove Reza"));

    assert_eq!(company.sorted_people("Engineering"), ["Amir", "Sally"]);
    assert_eq!(company.sorted_people("Sales"), ["Reza"]);
    assert!(company.sorted_people("HR").is_empty());
    company.print_all();
}
```

الگوی slice در `match words.as_slice()` در [فصل ۲۲](22-patterns.md) کامل توضیح داده شده است.

**تمرین ۴.** شمارش تکرار هر حرف. اینجا هم «حرف» را `char` در نظر می‌گیریم. `BTreeMap` برای خروجی مرتب و قابل پیش‌بینی انتخاب شد:

```rust
use std::collections::BTreeMap;

fn char_counts(s: &str) -> BTreeMap<char, usize> {
    let mut counts = BTreeMap::new();
    for c in s.chars().filter(|c| !c.is_whitespace()) {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = char_counts("hello world");
    assert_eq!(counts[&'l'], 3);
    assert_eq!(counts[&'o'], 2);
    assert_eq!(counts.get(&'z'), None);

    let fa = char_counts("سلام");
    assert_eq!(fa[&'س'], 1);
    assert_eq!(fa[&'ا'], 1);
    for (c, n) in &counts {
        println!("{c}: {n}");
    }
}
```

اگر به‌جای `BTreeMap` از `HashMap` استفاده کنید، منطق همان است ولی ترتیب چاپ هر بار ممکن است فرق کند.

---

> **جمع‌بندی:** اگر در حل یک تمرین گیر کردید، به جای نگاه کردن به پاسخ، پیام کامپایلر را کلمه‌به‌کلمه بخوانید و به فصل مربوط برگردید. پاسخ تمرین‌های فصل‌های ۱۳ تا ۲۷ در [پاسخ تمرین‌ها (بخش ۲)](exercises-solutions-2.md) است.
