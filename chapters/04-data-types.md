# انواع داده در Rust

Rust یک زبان **با نوع‌دهی ایستا** (statically typed) است؛ یعنی نوع همه‌ی متغیرها باید در زمان کامپایل مشخص باشد. در بیشتر موارد کامپایلر خودش نوع را حدس می‌زند (type inference)، اما گاهی باید صریح بنویسید:

```rust
fn main() {
    let guess: u32 = "42".parse().expect("این یک عدد نیست!");
    println!("{guess}");
}
```

بدون `: u32` کامپایلر نمی‌داند `parse` را به چه نوعی تبدیل کند.

انواع داده در Rust دو دسته‌اند: **اسکالر** (یک مقدار) و **مرکب** (چند مقدار).

## انواع اسکالر

### اعداد صحیح (Integer)

| طول | علامت‌دار | بدون علامت |
|---|---|---|
| 8 بیت | `i8` | `u8` |
| 16 بیت | `i16` | `u16` |
| 32 بیت | `i32` | `u32` |
| 64 بیت | `i64` | `u64` |
| 128 بیت | `i128` | `u128` |
| بسته به معماری | `isize` | `usize` |

- `i` یعنی **signed** (می‌تواند منفی باشد)، `u` یعنی **unsigned** (فقط صفر و مثبت).
- نوع پیش‌فرض اعداد صحیح `i32` است.
- `usize` برای اندیس آرایه‌ها و اندازه‌ی مجموعه‌ها استفاده می‌شود.

روش‌های نوشتن عدد:

```rust
fn main() {
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // فقط u8
    println!("{decimal} {hex} {octal} {binary} {byte}");
}
```

### سرریز عدد (Integer Overflow)

اگر مقداری از محدوده‌ی نوع خارج شود:

- در حالت **debug**: برنامه `panic` می‌کند.
- در حالت **release**: مقدار به‌صورت دوری برمی‌گردد (wrap).

برای کنترل صریح، متدهای آماده هست:

```rust
fn main() {
    let x: u8 = 250;
    println!("{}", x.wrapping_add(10));            // 4
    println!("{:?}", x.checked_add(10));           // None
    println!("{:?}", x.checked_add(5));            // Some(255)
    println!("{}", x.saturating_add(10));          // 255
    println!("{:?}", x.overflowing_add(10));       // (4, true)
}
```

### اعداد اعشاری (Float)

دو نوع `f32` و `f64` داریم. پیش‌فرض `f64` است.

```rust
fn main() {
    let x = 2.0;       // f64
    let y: f32 = 3.0;  // f32
    println!("{x} {y}");
}
```

### عملیات ریاضی

```rust
fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let int_div = 7 / 2;     // 3  (تقسیم صحیح)
    let remainder = 43 % 5;  // 3
    println!("{sum} {difference} {product} {quotient} {int_div} {remainder}");
}
```

> **دقت کنید:** Rust هیچ تبدیل نوع خودکاری بین اعداد انجام نمی‌دهد. `5_i32 + 2.0` خطا می‌دهد.

### تبدیل نوع با as

```rust
fn main() {
    let a: i32 = 300;
    let b = a as u8;      // 44  (بریده می‌شود!)
    let c = 3.99_f64 as i32; // 3
    let d = 65u8 as char;    // 'A'
    println!("{b} {c} {d}");
}
```

برای تبدیل ایمن‌تر از `From`/`Into` یا `TryFrom` استفاده می‌شود:

```rust
fn main() {
    let small: i32 = 100;
    let big: i64 = small.into();
    let result: Result<u8, _> = u8::try_from(300_i32);
    println!("{big} {}", result.is_err());
}
```

### Boolean

```rust
fn main() {
    let t = true;
    let f: bool = false;
    println!("{} {}", t && f, t || f);
}
```

### Char

نوع `char` یک **مقدار اسکالر یونیکد** است و ۴ بایت جا می‌گیرد. با **تک‌کوتیشن** نوشته می‌شود:

```rust
fn main() {
    let c = 'z';
    let heart = '❤';
    let farsi = 'ش';
    println!("{c} {heart} {farsi}");
}
```

(رشته‌ها با دو‌کوتیشن نوشته می‌شوند و نوعشان فرق دارد.)

## انواع مرکب

### Tuple

چند مقدار با **نوع‌های مختلف** را در یک گروه با طول ثابت نگه می‌دارد:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (x, y, z) = tup;
    println!("{x} {y} {z}");

    // دسترسی با نقطه و اندیس
    println!("{} {} {}", tup.0, tup.1, tup.2);
}
```

tuple خالی `()` را **unit** می‌نامند. توابعی که مقداری برنمی‌گردانند در واقع `()` برمی‌گردانند.

### Array

آرایه‌ها **طول ثابت** دارند و همه‌ی عناصرشان هم‌نوع‌اند. روی **stack** ذخیره می‌شوند:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months: [&str; 3] = ["farvardin", "ordibehesht", "khordad"];
    let zeros = [0; 5];  // [0, 0, 0, 0, 0]

    println!("{} {} {}", a[0], months[1], zeros.len());
}
```

اگر از اندیس معتبر بیرون بروید، برنامه در زمان اجرا `panic` می‌کند (به‌جای دسترسی به حافظه‌ی نامعتبر مثل C):

```rust,should_panic
fn main() {
    let a = [1, 2, 3];
    let index = a.len() + 2;
    println!("{}", a[index]); // panic: index out of bounds
}
```

> برای مجموعه‌ای که اندازه‌اش تغییر کند از `Vec` استفاده کنید (فصل مجموعه‌ها).

## رشته‌ها (نگاه اولیه)

دو نوع اصلی رشته داریم که در فصل Ownership دقیق‌تر بررسی می‌شوند:

- `&str`: رشته‌ی ثابت و قرض‌گرفته (مثل `"سلام"`).
- `String`: رشته‌ی قابل تغییر و دارای مالک که روی heap ذخیره می‌شود.

```rust
fn main() {
    let a: &str = "hello";
    let b: String = String::from("hello");
    let c = a.to_string();
    println!("{a} {b} {c}");
}
```

## تمرین

1. یک `u8` با مقدار 255 بسازید و ببینید `checked_add(1)` چه برمی‌گرداند.
2. یک tuple برای مشخصات یک کاربر (نام، سن، امتیاز) بسازید و آن را destructure کنید.
3. آرایه‌ای از ۷ نام روز هفته بسازید و روز سوم را چاپ کنید.
4. چرا `let x: i32 = 5; let y: i64 = x;` خطا می‌دهد؟ چطور درستش کنید؟

فصل بعد: [توابع و کامنت‌ها](05-functions.md).
