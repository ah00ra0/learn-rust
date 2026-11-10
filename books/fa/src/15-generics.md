# Generics

**Generics** (انواع عمومی) اجازه می‌دهند کدی بنویسید که با **چند نوع مختلف** کار کند، بدون اینکه آن را برای هر نوع تکرار کنید. پیش‌تر هم با آن‌ها سر و کار داشته‌اید: `Vec<T>`، `Option<T>`، `Result<T, E>` و `HashMap<K, V>`.

## مسئله: تکرار کد

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest_i32(&[34, 50, 25]));
    println!("{}", largest_char(&['y', 'm', 'a']));
}
```

دو تابع دقیقاً یکسان‌اند و فقط نوعشان فرق دارد.

## توابع Generic

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest(&[34, 50, 25]));
    println!("{}", largest(&['y', 'm', 'a']));
    println!("{}", largest(&[1.5, 0.3]));
}
```

بخش `<T: PartialOrd>` یعنی: "T هر نوعی می‌تواند باشد، به شرط آنکه قابل مقایسه با `>` باشد." به این شرط **Trait Bound** می‌گویند. اگر آن را حذف کنید، کامپایلر خطا می‌دهد چون نمی‌داند `>` روی T تعریف شده یا نه.

## Struct و Enum های Generic

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Mixed<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Mixed { x: 5, y: 4.0 };
    println!("{integer:?} {float:?} {mixed:?}");
}
```

اگر بنویسید `Point { x: 5, y: 4.0 }` با `Point<T>` خطا می‌گیرید چون هر دو باید یک نوع باشند.

## متدهای Generic

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// پیاده‌سازی فقط برای یک نوع مشخص
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("{} {}", p.x(), p.distance_from_origin());
}
```

نکته: `impl<T>` یعنی "برای هر T". `impl Point<f64>` یعنی فقط برای حالتی که T برابر `f64` است.

## عملکرد: Monomorphization

آیا generics در زمان اجرا هزینه دارند؟ **خیر.** کامپایلر در زمان کامپایل برای هر نوعی که واقعاً استفاده شده، یک نسخه‌ی مجزا از کد تولید می‌کند (**monomorphization**). یعنی `Option<i32>` و `Option<f64>` در باینری نهایی دو نوع جداگانه‌اند و سرعت اجرا دقیقاً مثل نوشتن دستی هر دو است. هزینه‌اش فقط زمان کامپایل و اندازه‌ی کمی بزرگ‌تر باینری است.

## Trait Bound ها

```rust
use std::fmt::Display;

// سه شکل معادل
fn show1<T: Display>(item: T) {
    println!("{item}");
}

fn show2(item: impl Display) {
    println!("{item}");
}

fn show3<T>(item: T)
where
    T: Display,
{
    println!("{item}");
}

fn main() {
    show1(1);
    show2("سلام");
    show3(2.5);
}
```

وقتی شروط زیاد شود، `where` خواناتر است:

```rust
use std::fmt::{Debug, Display};

fn compare_and_print<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{t} {u:?}");
}

fn main() {
    compare_and_print(&"hi", &vec![1, 2]);
}
```

## Const Generics

می‌توانید روی **مقدار ثابت** هم generic بنویسید، مثلاً طول آرایه:

```rust
fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    let mut total = 0;
    for x in arr {
        total += x;
    }
    total
}

fn main() {
    println!("{}", sum_array([1, 2, 3]));
    println!("{}", sum_array([10, 20, 30, 40, 50]));
}
```

## تمرین

1. تابع generic به نام `smallest` بنویسید که کوچک‌ترین عنصر یک slice را برگرداند.
2. struct به نام `Pair<T>` با متد `larger(&self) -> &T` بسازید (به `PartialOrd` نیاز دارید).
3. تابعی بنویسید که یک `&[T]` بگیرد و عنصر آخر را به‌صورت `Option<&T>` برگرداند.

فصل بعد: [Trait](16-traits.md).
