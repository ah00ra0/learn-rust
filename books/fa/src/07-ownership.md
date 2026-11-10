# Ownership (مالکیت)

**Ownership** مهم‌ترین و متمایزترین ویژگی Rust است. با آن، Rust مدیریت حافظه را **بدون Garbage Collector** و **بدون نیاز به آزاد کردن دستی حافظه** انجام می‌دهد و در عین حال ایمن می‌ماند.

## مرور: Stack و Heap

- **Stack**: حافظه‌ای منظم و سریع (آخرین ورودی، اولین خروجی). فقط داده‌هایی با اندازه‌ی **ثابت و معلوم** در آن قرار می‌گیرند.
- **Heap**: حافظه‌ای انعطاف‌پذیر برای داده‌هایی که اندازه‌شان در زمان کامپایل معلوم نیست یا تغییر می‌کند. تخصیص در آن کندتر است و با اشاره‌گر (pointer) به آن دسترسی داریم.

مشکلاتی که در زبان‌های دیگر پیش می‌آید:

- **C/C++**: برنامه‌نویس باید دستی حافظه را آزاد کند. فراموش کردن = نشت حافظه. آزاد کردن دوباره = باگ امنیتی.
- **Java/Python/Go**: Garbage Collector حافظه را آزاد می‌کند اما هزینه‌ی زمان اجرا دارد.
- **Rust**: قوانین مالکیت در زمان **کامپایل** بررسی می‌شوند و هزینه‌ی اجرایی ندارند.

## سه قانون Ownership

1. هر مقدار در Rust یک **مالک** (owner) دارد.
2. در هر زمان فقط **یک** مالک وجود دارد.
3. وقتی مالک از **scope** خارج شود، مقدار **آزاد (drop)** می‌شود.

## نوع String

برای دیدن Ownership به نوعی با داده‌ی heap نیاز داریم. `String` همین‌طور است:

```rust
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
} // اینجا s از scope خارج می‌شود و Rust خودکار تابع drop را صدا می‌زند
```

Rust در انتهای `}` به‌طور خودکار حافظه را آزاد می‌کند. (این الگو در C++ با نام RAII شناخته می‌شود.)

## Move (انتقال مالکیت)

```rust,compile_fail
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;          // مالکیت از s1 به s2 منتقل می‌شود (move)

    println!("{s1}, world!"); // ✗ خطا: s1 دیگر معتبر نیست
}
```

چرا؟ `String` از سه بخش تشکیل شده: اشاره‌گر به heap، طول و ظرفیت. وقتی `s2 = s1` می‌نویسیم، این سه مقدار کپی می‌شوند ولی **داده‌ی heap کپی نمی‌شود**. اگر هر دو متغیر معتبر می‌ماندند، در پایان scope هر دو سعی می‌کردند همان حافظه را آزاد کنند (**double free**). Rust برای جلوگیری از این مشکل، `s1` را نامعتبر می‌کند.

## Clone (کپی عمیق)

اگر واقعاً می‌خواهید داده‌ی heap کپی شود:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}
```

`clone` ممکن است پرهزینه باشد، پس Rust آن را صریح می‌خواهد.

## Copy: داده‌های روی Stack

انواعی که اندازه‌ی ثابت دارند و کپی کردنشان ارزان است، **Copy** هستند و move نمی‌شوند:

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}"); // هر دو معتبرند
}
```

انواع Copy عبارت‌اند از:

- همه‌ی اعداد صحیح و اعشاری
- `bool` و `char`
- tuple هایی که فقط شامل انواع Copy هستند، مثل `(i32, i32)`
- reference های غیرقابل‌تغییر (`&T`)

## Ownership و توابع

پاس دادن مقدار به تابع هم مثل انتساب، آن را **move** یا **copy** می‌کند:

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);          // s اینجا move می‌شود
    // println!("{s}");          // ✗ خطا: دیگر معتبر نیست

    let x = 5;
    makes_copy(x);               // x از نوع Copy است
    println!("{x}");             // ✓ هنوز معتبر
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string اینجا drop می‌شود

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
```

مقدار بازگشتی هم مالکیت را منتقل می‌کند:

```rust
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{s1} {s3}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

اما اگر هر بار برای استفاده از یک مقدار در تابع مجبور باشیم آن را پس بگیریم، کار خسته‌کننده می‌شود:

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("طول '{s2}' برابر است با {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

راه‌حل بهتر، **Reference** و **Borrowing** است.

## Drop

وقتی مقدار از scope خارج شود، Rust تابع `drop` را صدا می‌زند. می‌توانید رفتار آن را برای نوع خودتان با trait `Drop` تعیین کنید:

```rust
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("{} آزاد شد", self.0);
    }
}

fn main() {
    let _a = Noisy("الف");
    let _b = Noisy("ب");
    println!("پایان main");
}
```

خروجی:

```text
پایان main
ب آزاد شد
الف آزاد شد
```

متغیرها به ترتیب **معکوس** تعریف drop می‌شوند. برای آزاد کردن زودتر از موعد می‌توانید `drop(x)` را صدا بزنید.

## خلاصه

| مفهوم | توضیح |
|---|---|
| Move | انتقال مالکیت؛ مقدار قبلی نامعتبر می‌شود |
| Copy | کپی ارزان روی stack؛ مقدار قبلی معتبر می‌ماند |
| Clone | کپی صریح و عمیق |
| Drop | آزادسازی خودکار در پایان scope |

## تمرین

1. کدی بنویسید که با move خطا بدهد، پیغام خطا را بخوانید و آن را با `clone` درست کنید.
2. تابعی بنویسید که یک `String` بگیرد و آن را بعد از اضافه کردن `"!"` برگرداند.
3. چرا `let a = [1, 2, 3]; let b = a;` مشکلی ندارد ولی با `Vec` همین کار move است؟

فصل بعد: [Reference و Borrowing](08-borrowing.md).
