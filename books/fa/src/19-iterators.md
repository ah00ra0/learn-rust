# Iterator

**Iterator** (پیمایشگر) الگویی برای پردازش دنباله‌ای از عناصر یکی‌یکی است. در Rust iterator ها **تنبل** (lazy) هستند: تا وقتی آن‌ها را مصرف نکنید، هیچ کاری انجام نمی‌دهند.

```rust
fn main() {
    let v = vec![1, 2, 3];
    let iter = v.iter();   // هنوز هیچ کاری نشده

    for x in iter {
        println!("{x}");
    }
}
```

## trait Iterator

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... ده‌ها متد دیگر با پیاده‌سازی پیش‌فرض
}
```

فقط `next` اجباری است. هر بار `Some(مقدار)` یا در پایان `None` برمی‌گرداند:

```rust
fn main() {
    let v = vec![10, 20];
    let mut it = v.iter();
    println!("{:?}", it.next()); // Some(10)
    println!("{:?}", it.next()); // Some(20)
    println!("{:?}", it.next()); // None
}
```

## سه روش ساخت iterator از مجموعه

| متد | نوع عناصر | توضیح |
|---|---|---|
| `.iter()` | `&T` | قرض غیرقابل‌تغییر |
| `.iter_mut()` | `&mut T` | قرض قابل‌تغییر |
| `.into_iter()` | `T` | گرفتن مالکیت |

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for x in v.iter_mut() {
        *x *= 10;
    }
    println!("{v:?}");

    let total: i32 = v.into_iter().sum();   // v مصرف می‌شود
    println!("{total}");
}
```

## Adapter ها (تبدیل‌کننده‌ها)

این متدها iterator جدیدی برمی‌گردانند و **تنبل‌اند**:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = v
        .iter()
        .filter(|&&x| x % 2 == 0)    // فقط زوج‌ها
        .map(|&x| x * x)             // مربع
        .collect();                  // مصرف و ساخت Vec

    println!("{result:?}");  // [4, 16, 36]
}
```

| Adapter | کار |
|---|---|
| `map(f)` | تبدیل هر عنصر |
| `filter(p)` | نگه داشتن عناصری که شرط را دارند |
| `enumerate()` | افزودن اندیس: `(i, x)` |
| `zip(other)` | جفت کردن با iterator دیگر |
| `take(n)` / `skip(n)` | n عنصر اول / رد کردن n عنصر |
| `chain(other)` | چسباندن دو iterator |
| `rev()` | معکوس |
| `flat_map(f)` / `flatten()` | صاف کردن لیست‌های تو در تو |
| `take_while(p)` / `skip_while(p)` | تا زمانی که شرط برقرار است |
| `windows(n)` / `chunks(n)` | (روی slice) پنجره یا تکه‌های n تایی |
| `peekable()` | امکان نگاه به عنصر بعدی |

```rust
fn main() {
    let names = ["علی", "سارا", "رضا"];
    let ages = [30, 22, 41];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{name}: {age}");
    }

    let nested = vec![vec![1, 2], vec![3], vec![4, 5]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("{flat:?}");

    let pairs: Vec<(usize, char)> = "abc".chars().enumerate().collect();
    println!("{pairs:?}");

    let v = [1, 2, 3, 4, 5];
    for w in v.windows(2) {
        println!("{w:?}");
    }
}
```

## Consumer ها (مصرف‌کننده‌ها)

این متدها iterator را **تا انتها پیمایش** می‌کنند و مقدار نهایی می‌دهند:

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("{}", v.iter().sum::<i32>());              // 31
    println!("{}", v.iter().product::<i32>());
    println!("{:?}", v.iter().max());                   // Some(9)
    println!("{:?}", v.iter().min());                   // Some(1)
    println!("{}", v.iter().count());                   // 8
    println!("{:?}", v.iter().position(|&x| x == 4));   // Some(2)
    println!("{}", v.iter().any(|&x| x > 8));           // true
    println!("{}", v.iter().all(|&x| x > 0));           // true
    println!("{:?}", v.iter().find(|&&x| x > 4));       // Some(5)
    println!("{:?}", v.last());

    let folded = v.iter().fold(0, |acc, x| acc + x);    // مثل reduce
    println!("{folded}");
}
```

## collect: ساخت مجموعه

`collect` می‌تواند به مجموعه‌های مختلف تبدیل کند؛ نوع مقصد را باید مشخص کنید:

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    let v = vec![1, 2, 2, 3];

    let a: Vec<i32> = v.iter().cloned().collect();
    let b: HashSet<i32> = v.iter().cloned().collect();
    let c: String = vec!["س", "ل", "ا", "م"].into_iter().collect();
    let d: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();

    println!("{a:?} {} {c} {}", b.len(), d["b"]);
}
```

جالب: `collect` می‌تواند `Result` را هم جمع کند. اگر یکی `Err` باشد، همان برمی‌گردد:

```rust
fn main() {
    let good: Result<Vec<i32>, _> = "1 2 3".split(' ').map(|s| s.parse::<i32>()).collect();
    let bad: Result<Vec<i32>, _> = "1 x 3".split(' ').map(|s| s.parse::<i32>()).collect();
    println!("{good:?}");
    println!("{}", bad.is_err());
}
```

## ساخت iterator اختصاصی

```rust
struct Countdown {
    n: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            None
        } else {
            let current = self.n;
            self.n -= 1;
            Some(current)
        }
    }
}

fn main() {
    let c = Countdown { n: 5 };
    let v: Vec<u32> = c.filter(|x| x % 2 == 1).collect();
    println!("{v:?}"); // [5, 3, 1]

    let sum: u32 = Countdown { n: 4 }.zip(Countdown { n: 4 }.skip(1)).map(|(a, b)| a * b).sum();
    println!("{sum}");
}
```

با پیاده‌سازی فقط `next`، همه‌ی متدهای adapter و consumer رایگان در اختیار شماست.

## iterator های بی‌نهایت

چون تنبل هستند می‌توان دنباله‌های بی‌پایان ساخت و فقط مقدار لازم را برداشت:

```rust
fn main() {
    let squares: Vec<u64> = (1..).map(|x| x * x).take_while(|&x| x < 100).collect();
    println!("{squares:?}");

    let evens: Vec<u32> = (0..).step_by(2).take(5).collect();
    println!("{evens:?}");
}
```

## کارایی: Zero-cost Abstraction

زنجیره‌ی iterator ها در Rust معمولاً **به‌اندازه‌ی حلقه‌ی دستی** سریع است، چون کامپایلر آن‌ها را به حلقه‌ی ساده تبدیل و بهینه می‌کند. پس بدون نگرانی از سبک تابعی استفاده کنید.

## مقایسه: حلقه یا iterator؟

```rust
fn main() {
    let words = ["rust", "is", "fast", "and", "safe"];

    // سبک حلقه
    let mut long_words = Vec::new();
    for w in &words {
        if w.len() > 3 {
            long_words.push(w.to_uppercase());
        }
    }

    // سبک iterator
    let long_words2: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect();

    println!("{long_words:?} {}", long_words == long_words2);
}
```

هر دو درست‌اند؛ هرکدام خواناتر است انتخاب کنید.

## تمرین

1. از لیست اعداد ۱ تا ۲۰، مجموع مربع اعداد فرد را حساب کنید.
2. لیستی از رشته‌ها را به `Vec<usize>` از طول‌هایشان تبدیل کنید.
3. با `fold` بزرگ‌ترین مقدار یک لیست را پیدا کنید.
4. iterator ای بنویسید که اعداد فیبوناچی تولید کند و ۱۰ تای اول را چاپ کنید.

فصل بعد: [Smart Pointer](20-smart-pointers.md).
