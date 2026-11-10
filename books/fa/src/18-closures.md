# Closure

**Closure** تابعی بی‌نام است که می‌تواند **متغیرهای محیط اطرافش را capture کند**. در زبان‌های دیگر به آن lambda هم می‌گویند.

## نوشتار

```rust
fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };
    let add_two = |x| x + 2;        // نوع و آکولاد اختیاری
    let hello = || println!("سلام");

    println!("{}", add_one(5));
    println!("{}", add_two(5));
    hello();
}
```

- پارامترها بین `| |` نوشته می‌شوند.
- معمولاً نوع‌ها را کامپایلر حدس می‌زند. اما با اولین فراخوانی، نوع **ثابت** می‌شود:

```rust,compile_fail
fn main() {
    let identity = |x| x;
    let s = identity(String::from("hello"));
    let n = identity(5);   // ✗ خطا: نوع قبلاً String تعیین شده
}
```

## Capture محیط

برخلاف توابع معمولی، closure می‌تواند به متغیرهای اطرافش دسترسی داشته باشد:

```rust
fn main() {
    let threshold = 10;
    let is_big = |n: i32| n > threshold;   // threshold را capture می‌کند

    println!("{} {}", is_big(5), is_big(15));
}
```

capture به سه شکل انجام می‌شود (کامپایلر خودش کم‌ترین سطح لازم را انتخاب می‌کند):

1. **قرض غیرقابل‌تغییر** (`&T`): فقط خواندن
2. **قرض قابل‌تغییر** (`&mut T`): تغییر
3. **گرفتن مالکیت** (move): مقدار به داخل closure منتقل می‌شود

```rust
fn main() {
    // ۱. فقط خواندن
    let list = vec![1, 2, 3];
    let only_reads = || println!("{list:?}");
    only_reads();
    println!("{list:?}");   // هنوز می‌توان استفاده کرد

    // ۲. تغییر
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    inc();
    println!("{count}");    // 2

    // ۳. گرفتن مالکیت
    let text = String::from("سلام");
    let consume = move || println!("{text}");
    consume();
    // println!("{text}");  // ✗ text به closure منتقل شد
}
```

## کلمه‌ی move

`move` مالکیت همه‌ی متغیرهای capture شده را به closure می‌دهد. وقتی لازم است که closure **از scope فعلی بیشتر عمر کند**، مثلاً در thread ها:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("از thread: {v:?}");
    });

    handle.join().unwrap();
}
```

## سه trait برای closure ها

بسته به اینکه closure با متغیرهای capture شده چه می‌کند، یکی یا چند تا از این trait ها را به‌طور خودکار پیاده می‌کند:

| Trait | معنی | تعداد فراخوانی |
|---|---|---|
| `FnOnce` | مقدار capture شده را مصرف (move) می‌کند | فقط یک بار |
| `FnMut` | می‌تواند مقدار capture شده را تغییر دهد | چند بار |
| `Fn` | فقط می‌خواند یا چیزی capture نمی‌کند | چند بار |

هر closure حداقل `FnOnce` است. اگر مصرف نکند `FnMut` هم هست. اگر تغییر هم ندهد `Fn` هم هست.

## closure به‌عنوان پارامتر

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn apply_twice(mut f: impl FnMut()) {
    f();
    f();
}

fn run_once(f: impl FnOnce() -> String) -> String {
    f()
}

fn main() {
    let factor = 3;
    println!("{}", apply(|x| x * factor, 5));

    let mut n = 0;
    apply_twice(|| n += 10);
    println!("{n}");

    let owned = String::from("مالکیت");
    println!("{}", run_once(move || owned));
}
```

## بازگرداندن closure

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn make_boxed(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|x| x * 2),
        _ => Box::new(|x| x + 1),
    }
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10));
    println!("{}", make_boxed("double")(8));
}
```

اگر بسته به شرط closure های مختلف برمی‌گردانید، به `Box<dyn Fn...>` نیاز دارید چون هر closure نوع منحصربه‌فرد خودش را دارد.

## Function Pointer

توابع معمولی هم می‌توانند به‌جای closure پاس داده شوند:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn main() {
    println!("{}", do_twice(add_one, 5));

    let strings: Vec<String> = vec![1, 2, 3].iter().map(ToString::to_string).collect();
    println!("{strings:?}");
}
```

## کاربرد رایج: مرتب‌سازی و فیلتر

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![
        Person { name: "علی".into(), age: 30 },
        Person { name: "سارا".into(), age: 22 },
        Person { name: "رضا".into(), age: 41 },
    ];

    people.sort_by_key(|p| p.age);
    println!("{people:?}");

    people.sort_by(|a, b| b.name.cmp(&a.name));
    println!("{people:?}");
}
```

## تمرین

1. closure ای بنویسید که یک عدد بگیرد و مربعش را برگرداند.
2. تابع `make_counter` بنویسید که closure ای برگرداند که هر بار صدا زده شود عدد بعدی را بدهد (راهنمایی: `FnMut` و `move`).
3. لیستی از رشته‌ها را با `sort_by_key` بر اساس طول مرتب کنید.

فصل بعد: [Iterator](19-iterators.md).
