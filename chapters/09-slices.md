# Slice

**Slice** یک reference به بخشی **پیوسته** از یک مجموعه است، بدون اینکه مالکیت آن را داشته باشد.

## مسئله: اولین کلمه‌ی یک رشته

تابعی می‌خواهیم که اولین کلمه (تا اولین فاصله) را برگرداند. اگر فقط اندیس را برگردانیم:

```rust
fn first_word_end(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let end = first_word_end(&s); // 5
    s.clear();                    // رشته خالی شد ولی end هنوز 5 است!
    println!("{end}");
}
```

مشکل این است که اندیس `end` هیچ ارتباطی با خود رشته ندارد و بعد از تغییر رشته بی‌معنی می‌شود. Slice این مشکل را در زمان کامپایل حل می‌کند.

## String Slice

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");

    // میان‌بُرها
    let a = &s[..5];   // از ابتدا
    let b = &s[6..];   // تا انتها
    let c = &s[..];    // کل رشته
    println!("{a} {b} {c}");
}
```

نوع slice رشته `&str` است. (همین نوع رشته‌های literal مثل `"hello"` هم هست؛ آن‌ها slice هایی به داده‌ی داخل خود برنامه‌ی باینری هستند.)

### بازنویسی first_word

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let my_string = String::from("hello world");
    println!("{}", first_word(&my_string));       // از String
    println!("{}", first_word(&my_string[..]));   // از slice
    println!("{}", first_word("hello world"));    // از literal
}
```

> **قاعده‌ی طلایی:** پارامتر تابع را `&str` بگیرید، نه `&String`. این کار تابع را انعطاف‌پذیرتر می‌کند.

حالا اگر بعد از گرفتن slice رشته را تغییر بدهیم، کامپایلر جلویمان را می‌گیرد:

```rust,compile_fail
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or(s)
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();                // ✗ خطا: s هنوز توسط word قرض گرفته شده
    println!("{word}");
}
```

## نکته‌ی مهم درباره‌ی یونیکد و متن فارسی

رشته‌های Rust همیشه **UTF-8** هستند. اندیس‌ها در slice ها **بایت** هستند، نه کاراکتر. حروف فارسی هر کدام ۲ بایت (و برخی نمادها بیشتر) جا می‌گیرند:

```rust
fn main() {
    let s = "سلام";
    println!("{}", s.len());          // 8 (بایت)
    println!("{}", s.chars().count()); // 4 (کاراکتر)
}
```

اگر slice را وسط یک کاراکتر ببُرید، برنامه panic می‌کند:

```rust,should_panic
fn main() {
    let s = String::from("سلام");
    let cut = s.len() / 8; // 1
    println!("{}", &s[0..cut]); // panic: وسط کاراکتر
}
```

برای کار با کاراکترها از `chars()` استفاده کنید:

```rust
fn main() {
    let s = "سلام دنیا";
    for c in s.chars() {
        print!("[{c}]");
    }
    println!();

    let first_two: String = s.chars().take(2).collect();
    println!("{first_two}");
}
```

## Slice آرایه و Vec

slice فقط برای رشته نیست:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    println!("{slice:?}");            // [2, 3]
    println!("{}", slice.len());      // 2
}
```

نوع `&[T]` یعنی "slice ای از T". توابعی که `&[T]` می‌گیرند با آرایه و `Vec` هر دو کار می‌کنند:

```rust
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn main() {
    let arr = [1, 2, 3];
    let v = vec![10, 20, 30, 40];
    println!("{} {} {}", sum(&arr), sum(&v), sum(&v[1..3]));
}
```

## تمرین

1. تابعی بنویسید که دومین کلمه‌ی یک رشته را برگرداند.
2. تابعی بنویسید که `&[i32]` بگیرد و بزرگ‌ترین عنصر را به‌صورت `Option<i32>` برگرداند.
3. بررسی کنید `"ایران".len()` چه عددی می‌دهد و چرا.

فصل بعد: [Struct](10-structs.md).
