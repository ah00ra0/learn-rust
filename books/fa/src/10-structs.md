# Struct

**Struct** (ساختار) اجازه می‌دهد چند مقدار مرتبط را با نام‌های معنی‌دار در یک نوع جدید کنار هم بگذارید.

## تعریف و ساخت

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("ali@example.com"),
        username: String::from("ali"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new@example.com");
    println!("{} {}", user1.username, user1.email);
}
```

- ترتیب فیلدها هنگام ساخت مهم نیست.
- برای تغییر یک فیلد، **کل** متغیر باید `mut` باشد (نمی‌توان فقط یک فیلد را mutable کرد).

### میان‌بر مقداردهی (Field Init Shorthand)

وقتی نام پارامتر با نام فیلد یکی باشد:

```rust
struct User {
    username: String,
    email: String,
}

fn build_user(email: String, username: String) -> User {
    User { email, username }
}

fn main() {
    let u = build_user("a@b.com".to_string(), "reza".to_string());
    println!("{}", u.username);
}
```

### Struct Update Syntax

ساخت نمونه‌ی جدید بر پایه‌ی نمونه‌ی قبلی:

```rust
#[derive(Debug)]
struct Config {
    width: u32,
    height: u32,
    title: String,
}

fn main() {
    let base = Config { width: 800, height: 600, title: String::from("پایه") };
    let wide = Config { width: 1920, ..base };
    println!("{wide:?}");
    // توجه: title از base به wide move شد، پس base.title دیگر معتبر نیست
}
```

## Tuple Struct و Unit-like Struct

```rust
struct Color(u8, u8, u8);
struct Point(f64, f64, f64);
struct AlwaysEqual; // بدون فیلد

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);
    let _marker = AlwaysEqual;
    println!("{} {}", red.0, origin.2);
}
```

`Color` و `Point` دو نوع **متفاوت** هستند حتی اگر ساختار داخلی‌شان شبیه باشد.

## چاپ struct با Debug

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("{rect:?}");   // Rectangle { width: 30, height: 50 }
    println!("{rect:#?}");  // چاپ چندخطی و زیبا
    dbg!(&rect);            // چاپ در stderr همراه شماره خط
}
```

خط `#[derive(Debug)]` را **attribute** می‌گویند و پیاده‌سازی خودکار trait به نام `Debug` را درخواست می‌کند.

## متدها (impl)

متد، تابعی است که داخل بلوک `impl` برای یک نوع تعریف می‌شود و اولین پارامترش `self` است:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("مساحت: {}", rect1.area());
    println!("جا می‌شود؟ {}", rect1.can_hold(&rect2));

    rect1.scale(2);
    println!("{rect1:?}");
}
```

سه شکل `self`:

| شکل | معنی |
|---|---|
| `&self` | فقط خواندن (رایج‌ترین) |
| `&mut self` | خواندن و تغییر |
| `self` | گرفتن مالکیت (مثلاً هنگام تبدیل به نوع دیگر) |

> **نکته:** Rust هنگام فراخوانی متد خودکار `&` ، `&mut` یا `*` را اضافه می‌کند (automatic referencing). به همین دلیل در Rust عملگر `->` مثل C++ لازم نیست.

## Associated Function (تابع وابسته)

توابعی در `impl` که `self` ندارند. با `::` صدا زده می‌شوند و معمولاً سازنده (constructor) هستند:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let r = Rectangle::new(3, 4);
    let s = Rectangle::square(5);
    println!("{} {}", r.width * r.height, s.width);
}
```

`Self` در `impl` یعنی همان نوعی که در حال پیاده‌سازی‌اش هستیم. هر نوع می‌تواند چند بلوک `impl` داشته باشد.

## الگوی Builder (پیشرفته‌تر)

برای ساخت اشیای با تنظیمات زیاد:

```rust
#[derive(Debug, Default)]
struct Server {
    host: String,
    port: u16,
    tls: bool,
}

impl Server {
    fn new(host: &str) -> Self {
        Self { host: host.to_string(), port: 80, ..Default::default() }
    }
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    fn tls(mut self, tls: bool) -> Self {
        self.tls = tls;
        self
    }
}

fn main() {
    let s = Server::new("example.com").port(443).tls(true);
    println!("{s:?}");
}
```

## تمرین

1. struct به نام `Book` با فیلدهای عنوان، نویسنده و تعداد صفحات بسازید و متدی برای چاپ خلاصه‌ی آن بنویسید.
2. struct به نام `Circle` با متدهای `area` و `circumference` بنویسید.
3. struct به نام `Counter` با متدهای `increment(&mut self)` و `get(&self)` بنویسید.
4. یک tuple struct به نام `Meters(f64)` بسازید و تابعی بنویسید که آن را به `Feet` تبدیل کند.

فصل بعد: [Enum، match و Option](11-enums.md).
