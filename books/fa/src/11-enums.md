# Enum، match و Option

**Enum** (شمارشی) نوعی است که مقدارش می‌تواند **یکی از چند حالت** ممکن باشد. enum ها در Rust بسیار قدرتمندتر از زبان‌هایی مثل C یا Java هستند، چون هر حالت می‌تواند **داده** هم همراه داشته باشد.

## تعریف ساده

```rust
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let d = Direction::North;
    println!("{d:?}");
}
```

## Enum با داده

هر حالت می‌تواند داده‌ی متفاوتی داشته باشد:

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // بدون داده
    Move { x: i32, y: i32 },    // مثل struct
    Write(String),              // مثل tuple با یک مقدار
    ChangeColor(u8, u8, u8),    // مثل tuple با سه مقدار
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("خروج"),
            Message::Move { x, y } => format!("حرکت به ({x}, {y})"),
            Message::Write(text) => format!("نوشتن: {text}"),
            Message::ChangeColor(r, g, b) => format!("رنگ: rgb({r},{g},{b})"),
        }
    }
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("سلام")),
        Message::ChangeColor(255, 0, 128),
    ];
    for m in &msgs {
        println!("{}", m.describe());
    }
}
```

## match

`match` مقدار را با چند **الگو** (pattern) مقایسه می‌کند و اولین الگوی منطبق را اجرا می‌کند:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Dime));
}
```

### match باید جامع (exhaustive) باشد

اگر یکی از حالت‌ها را فراموش کنید، کد **کامپایل نمی‌شود**. این یکی از بزرگ‌ترین مزیت‌های enum ها در Rust است؛ وقتی حالت جدیدی به enum اضافه کنید، کامپایلر همه‌ی جاهایی که باید به‌روز شوند را نشان می‌دهد.

```rust,compile_fail
enum Light { Red, Yellow, Green }

fn action(l: Light) -> &'static str {
    match l {
        Light::Red => "توقف",
        Light::Green => "حرکت",
        // ✗ خطا: Yellow پوشش داده نشده
    }
}
```

### الگوی catch-all

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("جایزه!"),
        7 => println!("باخت"),
        other => println!("حرکت {other} خانه"),  // مقدار را در متغیر می‌گیرد
    }

    match dice_roll {
        3 => println!("جایزه!"),
        _ => (),  // نادیده گرفتن بقیه
    }
}
```

### استخراج داده از enum

```rust
#[derive(Debug)]
enum UsState { Alabama, Alaska }

enum Coin {
    Quarter(UsState),
    Penny,
}

fn describe(coin: Coin) {
    match coin {
        Coin::Quarter(state) => println!("کوارتر از {state:?}"),
        Coin::Penny => println!("پنی"),
    }
}

fn main() {
    describe(Coin::Quarter(UsState::Alaska));
    describe(Coin::Penny);
    let _ = UsState::Alabama;
}
```

## Option: جایگزین null

بسیاری از زبان‌ها `null` دارند که "اختراع یک میلیارد دلاری اشتباه" نام گرفته است. Rust **null ندارد**. به‌جایش enum استاندارد `Option<T>` را دارد:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

```rust
fn main() {
    let some_number = Some(5);
    let some_text = Some("سلام");
    let absent: Option<i32> = None;
    println!("{some_number:?} {some_text:?} {absent:?}");
}
```

چرا بهتر از null است؟ چون `Option<i32>` و `i32` دو نوع **متفاوت** هستند. اگر مقدار ممکن است وجود نداشته باشد، کامپایلر شما را مجبور می‌کند حالت `None` را مدیریت کنید:

```rust,compile_fail
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y; // ✗ خطا: نمی‌توان i8 را با Option<i8> جمع کرد
}
```

### استفاده از Option

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{:?}", plus_one(Some(5)));
    println!("{:?}", plus_one(None));
}
```

متدهای مفید `Option`:

```rust
fn main() {
    let a = Some(4);
    let b: Option<i32> = None;

    println!("{}", a.unwrap_or(0));          // 4
    println!("{}", b.unwrap_or(0));          // 0
    println!("{:?}", a.map(|x| x * 2));      // Some(8)
    println!("{}", a.is_some());             // true
    println!("{:?}", a.and_then(|x| if x > 3 { Some(x) } else { None }));
    println!("{}", b.unwrap_or_default());   // 0
}
```

> **هشدار:** `unwrap()` اگر مقدار `None` باشد برنامه را panic می‌کند. در کد واقعی ترجیحاً از `match`، `if let`، `unwrap_or` یا عملگر `?` استفاده کنید.

## if let و let else

وقتی فقط یک الگو برایتان مهم است، `match` کامل زیادی است:

```rust
fn main() {
    let config_max = Some(3u8);

    // به‌جای match با _ => ()
    if let Some(max) = config_max {
        println!("حداکثر: {max}");
    }

    // با else
    let value: Option<u8> = None;
    if let Some(v) = value {
        println!("{v}");
    } else {
        println!("مقداری نیست");
    }
}
```

`let else` برای خروج زودهنگام عالی است:

```rust
fn describe(input: Option<&str>) -> String {
    let Some(text) = input else {
        return String::from("خالی");
    };
    format!("متن: {text}")
}

fn main() {
    println!("{}", describe(Some("سلام")));
    println!("{}", describe(None));
}
```

و `while let` برای حلقه تا زمانی که الگو منطبق باشد:

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
```

## تمرین

1. enum به نام `Shape` با حالت‌های `Circle(f64)`، `Rectangle(f64, f64)` و `Triangle(f64, f64, f64)` بسازید و تابع محاسبه‌ی مساحت را با `match` بنویسید.
2. تابعی بنویسید که دو عدد بگیرد و حاصل تقسیم را به‌صورت `Option<f64>` برگرداند (`None` هنگام تقسیم بر صفر).
3. enum به نام `TrafficLight` بنویسید و متدی برای برگرداندن مدت زمان هر چراغ نوشته و سپس یک حالت جدید اضافه کنید تا ببینید کامپایلر چه می‌گوید.

فصل بعد: [مجموعه‌ها](12-collections.md).
