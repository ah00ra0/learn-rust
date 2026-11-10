# Trait

**Trait** مجموعه‌ای از رفتارها (متدها) است که یک نوع می‌تواند پیاده‌سازی کند. اگر با زبان‌های دیگر آشنا باشید، شبیه **interface** در Java و C# یا **protocol** در Swift است، اما قدرتمندتر.

## تعریف و پیاده‌سازی

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} ({})", self.headline, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("نسخه جدید منتشر شد"),
    };
    let article = NewsArticle {
        headline: String::from("خبر مهم"),
        location: String::from("تهران"),
    };
    println!("{}", tweet.summarize());
    println!("{}", article.summarize());
}
```

### قانون یتیم (Orphan Rule)

شما فقط وقتی می‌توانید trait را برای نوعی پیاده کنید که **یا trait یا نوع متعلق به crate شما** باشد. نمی‌توانید `Display` (از std) را برای `Vec<T>` (از std) پیاده کنید. این قانون جلوی تعارض بین crate ها را می‌گیرد.

## پیاده‌سازی پیش‌فرض

```rust
trait Greet {
    fn name(&self) -> String;

    fn greet(&self) -> String {
        format!("سلام {}!", self.name())  // پیاده‌سازی پیش‌فرض
    }
}

struct Person(String);
struct Robot;

impl Greet for Person {
    fn name(&self) -> String {
        self.0.clone()
    }
}

impl Greet for Robot {
    fn name(&self) -> String {
        String::from("ربات")
    }
    fn greet(&self) -> String {   // بازنویسی
        String::from("بیپ بوپ")
    }
}

fn main() {
    println!("{}", Person(String::from("علی")).greet());
    println!("{}", Robot.greet());
}
```

## Trait به‌عنوان پارامتر

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Post(String);

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("خلاصه: {}", self.0)
    }
}

// روش ۱: impl Trait
fn notify(item: &impl Summary) {
    println!("خبر فوری! {}", item.summarize());
}

// روش ۲: trait bound (برابر با بالا)
fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let p = Post(String::from("سلام"));
    notify(&p);
    notify2(&p);
}
```

### ترکیب چند trait

```rust
use std::fmt::{Debug, Display};

fn print_both<T: Display + Debug>(item: &T) {
    println!("{item} / {item:?}");
}

fn main() {
    print_both(&"متن");
}
```

## بازگرداندن نوعی که trait را پیاده می‌کند

```rust
trait Animal {
    fn sound(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("هاپ")
    }
}

fn make_animal() -> impl Animal {
    Dog
}

fn main() {
    println!("{}", make_animal().sound());
}
```

> محدودیت: `impl Trait` در نوع برگشتی فقط می‌تواند **یک نوع مشخص** برگرداند. اگر لازم است بسته به شرط نوع‌های مختلف برگردانید، به [Trait Object](24-trait-objects.md) نیاز دارید.

## Trait های مهم کتابخانه‌ی استاندارد

| Trait | کاربرد |
|---|---|
| `Debug` | چاپ برای برنامه‌نویس با `{:?}` |
| `Display` | چاپ برای کاربر با `{}` |
| `Clone` | کپی صریح با `.clone()` |
| `Copy` | کپی ضمنی و ارزان |
| `PartialEq` / `Eq` | مقایسه‌ی برابری با `==` |
| `PartialOrd` / `Ord` | مقایسه‌ی ترتیب با `<` و `>` |
| `Hash` | استفاده به‌عنوان کلید `HashMap` |
| `Default` | مقدار پیش‌فرض |
| `From` / `Into` | تبدیل نوع |
| `Iterator` | پیمایش |
| `Drop` | آزادسازی |
| `Add`, `Sub`, ... | سربارگذاری عملگرها |

بسیاری از آن‌ها را می‌توان با `#[derive(...)]` خودکار پیاده کرد:

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{}", a == b);
    println!("{:?}", Point::default());
}
```

## پیاده‌سازی Display

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{p}");
    let s = p.to_string();   // Display خودکار to_string را هم می‌دهد
    println!("{s}");
}
```

## سربارگذاری عملگر

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    println!("{:?}", a + b);
}
```

## From و Into

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let f: Fahrenheit = Celsius(100.0).into();   // با From، Into رایگان است
    println!("{}", f.0);
    let g = Fahrenheit::from(Celsius(0.0));
    println!("{}", g.0);
}
```

## Trait با نوع وابسته (Associated Type)

```rust
trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

struct Stack(Vec<i32>);

impl Container for Stack {
    type Item = i32;
    fn first(&self) -> Option<&i32> {
        self.0.first()
    }
}

fn main() {
    let s = Stack(vec![7, 8]);
    println!("{:?}", s.first());
}
```

`Iterator` هم همین‌طور است: `type Item;` را دارد.

## Supertrait

```rust
use std::fmt::Display;

trait Pretty: Display {
    fn pretty(&self) -> String {
        format!("*** {} ***", self)
    }
}

impl Pretty for i32 {}

fn main() {
    println!("{}", 42.pretty());
}
```

## تمرین

1. trait به نام `Shape` با متدهای `area` و `name` تعریف کنید و برای `Circle` و `Square` پیاده کنید.
2. تابع generic بنویسید که یک `&[T]` از انواع `Shape` بگیرد و مجموع مساحت را حساب کند.
3. `Display` را برای struct ای که ساختید پیاده کنید.
4. `Add` را برای `Money(i32)` پیاده کنید.

فصل بعد: [Lifetime](17-lifetimes.md).
