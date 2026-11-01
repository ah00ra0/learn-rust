# Lifetime

**Lifetime** (طول عمر) یکی از مفاهیمی است که مبتدی‌ها را می‌ترساند، اما ایده‌ی اصلی ساده است: **هر reference فقط تا زمانی معتبر است که چیزی که به آن اشاره می‌کند زنده باشد.** کامپایلر Rust این را بررسی می‌کند.

در بیشتر موارد lifetime ها **ضمنی** هستند و خودتان چیزی نمی‌نویسید. فقط وقتی کامپایلر نتواند حدس بزند، باید **توضیحشان بدهید.**

## جلوگیری از Dangling Reference

```rust,compile_fail
fn main() {
    let r;
    {
        let x = 5;
        r = &x;       // ✗ خطا: x به‌اندازه‌ی کافی عمر نمی‌کند
    }
    println!("{r}");  // r به حافظه‌ی آزادشده اشاره می‌کند!
}
```

کامپایلر با **Borrow Checker** این مقایسه را انجام می‌دهد: عمر `r` بزرگ‌تر از عمر `x` است، پس خطا.

## چرا گاهی به annotation نیاز داریم؟

```rust,compile_fail
fn longest(x: &str, y: &str) -> &str {  // ✗ خطا: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

کامپایلر نمی‌داند خروجی از `x` است یا `y`. در نتیجه نمی‌تواند عمر خروجی را بررسی کند.

## نشانه‌گذاری Lifetime

نشانه با `'` شروع می‌شود (معمولاً `'a`، `'b`):

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("{result}");
    }
}
```

معنی: "هر دو ورودی و خروجی حداقل به اندازه‌ی `'a` عمر می‌کنند، که در عمل برابر با **کوتاه‌ترین** عمر بین دو ورودی است."

نکته مهم: **annotation ها عمر هیچ چیز را تغییر نمی‌دهند.** فقط رابطه‌ی بین عمرها را به کامپایلر می‌گویند.

اگر نتیجه را بعد از مرگ `s2` استفاده کنیم:

```rust,compile_fail
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
    } // s2 اینجا می‌میرد
    println!("{result}"); // ✗ خطا: s2 به‌اندازه‌ی کافی عمر نمی‌کند
}
```

## نشانه‌ی lifetime تنها برای پارامتری که برمی‌گردد

اگر خروجی فقط به یکی از ورودی‌ها بستگی دارد، لازم نیست برای دیگری lifetime بدهید:

```rust
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn main() {
    println!("{}", first("الف", "ب"));
}
```

## Lifetime در Struct

وقتی struct یک reference نگه می‌دارد، باید lifetime بنویسید:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce(&self, announcement: &str) -> &str {
        println!("توجه: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("یکی بود یکی نبود. زیر گنبد کبود...");
    let first_sentence = novel.split('.').next().expect("جمله‌ای نیست");
    let e = Excerpt { part: first_sentence };
    println!("{} {}", e.part, e.level());
    println!("{}", e.announce("سلام"));
}
```

معنی: نمونه‌ی `Excerpt` نمی‌تواند بیشتر از متنی که به آن اشاره می‌کند عمر کند.

> **ساده‌ترین راه:** اگر نمی‌خواهید با lifetime درگیر شوید، به‌جای reference در struct از نوع‌های دارای مالک مثل `String` استفاده کنید.

## قواعد Lifetime Elision

چرا در `fn first_word(s: &str) -> &str` نیازی به annotation نبود؟ چون کامپایلر سه قاعده دارد که خودکار اعمال می‌شوند:

1. هر پارامتر reference یک lifetime مجزا می‌گیرد.
2. اگر **دقیقاً یک** lifetime ورودی وجود داشته باشد، به همه‌ی خروجی‌ها اعمال می‌شود.
3. اگر یکی از ورودی‌ها `&self` یا `&mut self` باشد، lifetime آن به خروجی‌ها داده می‌شود.

اگر بعد از این قواعد هنوز مبهم بود، خطا می‌گیرید و خودتان باید بنویسید.

## Lifetime ی 'static

عمر `'static` یعنی تا پایان برنامه زنده است. رشته‌های literal این‌طورند:

```rust
fn main() {
    let s: &'static str = "من در باینری برنامه ذخیره شده‌ام";
    println!("{s}");
}
```

> **هشدار:** وقتی کامپایلر پیشنهاد می‌دهد `'static` بگذارید، معمولاً راه‌حل درست نیست. بیشتر وقت‌ها مشکل طراحی است و باید مالکیت را درست کنید.

## ترکیب Generic، Trait و Lifetime

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("اعلان: {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("{}", longest_with_announcement("abc", "de", "شروع"));
}
```

## جمع‌بندی ذهنی

| سؤال | پاسخ |
|---|---|
| lifetime چیست؟ | بازه‌ای که reference معتبر است |
| annotation چه می‌کند؟ | رابطه‌ی بین عمرها را توضیح می‌دهد، عمر را عوض نمی‌کند |
| چه موقع لازم است؟ | وقتی خروجی reference است و از چند ورودی می‌تواند بیاید، یا struct ای reference نگه می‌دارد |
| اگر گیج شدم؟ | از `String` و `clone` استفاده کنید؛ بعداً بهینه‌اش کنید |

## تمرین

1. تابع `longest_word` بنویسید که از یک متن، بلندترین کلمه را به‌صورت `&str` برگرداند.
2. struct ی بنویسید که یک `&str` نگه می‌دارد و متدی دارد که اولین کلمه‌اش را برمی‌گرداند.
3. کدی بنویسید که خطای "borrowed value does not live long enough" بدهد و آن را درست کنید.

فصل بعد: [Closure](18-closures.md).
