# Pattern Matching به‌طور کامل

**Pattern** (الگو) نحوی ویژه برای تطبیق با ساختار مقادیر است. تاکنون در `match` و `if let` دیده‌اید، اما الگوها در جاهای بیشتری به کار می‌روند.

## جاهایی که الگو می‌توان نوشت

### ۱. match

```rust
fn main() {
    let x = Some(5);
    match x {
        Some(n) if n > 3 => println!("بزرگ: {n}"),
        Some(n) => println!("کوچک: {n}"),
        None => println!("هیچ"),
    }
}
```

### ۲. let

هر `let` در واقع یک الگوست:

```rust
fn main() {
    let (x, y, z) = (1, 2, 3);
    let [a, b] = [10, 20];
    println!("{x} {y} {z} {a} {b}");
}
```

### ۳. پارامترهای تابع

```rust
fn print_coords(&(x, y): &(i32, i32)) {
    println!("({x}, {y})");
}

fn main() {
    print_coords(&(3, 5));
}
```

### ۴. حلقه‌ی for

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{index}: {value}");
    }
}
```

### ۵. if let ، while let و let else

```rust
fn main() {
    let favorite: Option<&str> = None;
    if let Some(color) = favorite {
        println!("{color}");
    } else {
        println!("رنگ مورد علاقه ندارد");
    }
}
```

## Refutable و Irrefutable

- **Irrefutable**: همیشه تطبیق می‌کند (مثل `let x = 5;`). در `let` و `for` و پارامتر تابع لازم است.
- **Refutable**: ممکن است تطبیق نکند (مثل `Some(x)`). در `if let` و `match` به کار می‌رود.

```rust,compile_fail
fn main() {
    let opt: Option<i32> = Some(3);
    let Some(x) = opt;   // ✗ خطا: الگوی refutable در let ساده، بدون else
}
```

## انواع الگو

### literal و چند الگو با |

```rust
fn main() {
    let x = 2;
    match x {
        1 => println!("یک"),
        2 | 3 => println!("دو یا سه"),
        _ => println!("دیگر"),
    }
}
```

### بازه‌ها

```rust
fn main() {
    let c = 'k';
    match c {
        'a'..='j' => println!("نیمه‌ی اول"),
        'k'..='z' => println!("نیمه‌ی دوم"),
        _ => println!("حرف نیست"),
    }

    let n = 7;
    match n {
        1..=5 => println!("کم"),
        6..=10 => println!("متوسط"),
        _ => println!("زیاد"),
    }
}
```

### Destructuring ساختارها

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;      // با نام جدید
    println!("{a} {b}");

    let Point { x, y } = p;            // میان‌بر
    println!("{x} {y}");

    match p {
        Point { x, y: 0 } => println!("روی محور x در {x}"),
        Point { x: 0, y } => println!("روی محور y در {y}"),
        Point { x, y } => println!("({x}, {y})"),
    }
}
```

### Destructuring enum ها و ساختارهای تو در تو

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb {r} {g} {b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("hsv {h} {s} {v}"),
        Message::Quit => println!("خروج"),
    }
}
```

### نادیده گرفتن مقادیر

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("{first} {last}"),   // .. یعنی بقیه
    }

    let (_, second, _) = (1, 2, 3);                        // _ یعنی نادیده
    println!("{second}");

    let _unused = 5;                                       // بدون هشدار unused
}
```

### الگوی slice

```rust
fn describe(v: &[i32]) -> String {
    match v {
        [] => "خالی".to_string(),
        [x] => format!("یک عنصر: {x}"),
        [first, .., last] => format!("از {first} تا {last}"),
    }
}

fn main() {
    println!("{}", describe(&[]));
    println!("{}", describe(&[5]));
    println!("{}", describe(&[1, 2, 3, 4]));
}
```

با `rest @ ..` هم می‌توانید باقی را بگیرید:

```rust
fn sum(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        [head, rest @ ..] => head + sum(rest),
    }
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4]));
}
```

### Match Guard

شرط اضافی با `if`:

```rust
fn main() {
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("برابر"),
        (x, y) if x + y == 0 => println!("قرینه"),
        (x, _) if x % 2 == 1 => println!("x فرد"),
        _ => println!("هیچ‌کدام"),
    }
}
```

### @ Binding

نام دادن به مقداری که همزمان تست می‌شود:

```rust
enum Msg {
    Hello { id: i32 },
}

fn main() {
    let msg = Msg::Hello { id: 5 };
    match msg {
        Msg::Hello { id: id_var @ 3..=7 } => println!("id در بازه: {id_var}"),
        Msg::Hello { id: 10..=12 } => println!("بازه‌ی دیگر"),
        Msg::Hello { id } => println!("id: {id}"),
    }
}
```

### الگوها و reference ها

```rust
fn main() {
    let reference = &4;

    match reference {
        &val => println!("با & : {val}"),
    }
    match *reference {
        val => println!("با * : {val}"),
    }

    let mut value = 5;
    match &mut value {
        m => *m += 10,
    }
    println!("{value}");

    let opt = Some(String::from("متن"));
    match &opt {
        Some(s) => println!("{s}"),   // s: &String ؛ opt هنوز معتبر است
        None => {}
    }
    println!("{opt:?}");
}
```

## matches! macro

```rust
fn main() {
    let x = Some(7);
    println!("{}", matches!(x, Some(n) if n > 5));

    let c = 'q';
    println!("{}", matches!(c, 'a'..='z'));
}
```

## تمرین

1. تابعی بنویسید که یک tuple `(i32, i32)` بگیرد و بگوید نقطه روی کدام ربع صفحه است (با match و guard).
2. تابع بازگشتی `max_of` بنویسید که با الگوی slice بزرگ‌ترین عنصر را پیدا کند.
3. با `matches!` بررسی کنید یک `char` رقم است یا نه.

فصل بعد: [تست‌نویسی](23-testing.md).
