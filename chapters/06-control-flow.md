# کنترل جریان: if، loop، while و for

## شرط if

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("کوچک‌تر از ۵");
    } else if number < 10 {
        println!("بین ۵ و ۹");
    } else {
        println!("۱۰ یا بزرگ‌تر");
    }
}
```

نکات مهم:

- شرط **حتماً باید `bool`** باشد. برخلاف جاوااسکریپت یا C، عدد به‌طور خودکار به بولین تبدیل نمی‌شود.
- دور شرط پرانتز لازم نیست.

```rust,compile_fail
fn main() {
    let number = 3;
    if number { // ✗ خطا: انتظار bool داشت، i32 گرفت
        println!("سه");
    }
}
```

### if به‌عنوان expression

چون `if` یک expression است، می‌توانید مقدارش را در متغیر بریزید. همه‌ی شاخه‌ها باید **هم‌نوع** باشند:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");
}
```

## حلقه‌ها

### loop: حلقه‌ی بی‌پایان

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // مقدار را از حلقه برمی‌گردانیم
        }
    };

    println!("نتیجه: {result}"); // 20
}
```

- `break`: خروج از حلقه (می‌تواند مقدار برگرداند).
- `continue`: رفتن به تکرار بعدی.

### Label برای حلقه‌های تو در تو

```rust
fn main() {
    let mut count = 0;
    'outer: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;           // فقط از حلقه داخلی
            }
            if count == 2 {
                break 'outer;    // از حلقه بیرونی
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count = {count}");
}
```

### while

```rust
fn main() {
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("پرتاب!");
}
```

### for

رایج‌ترین حلقه در Rust است و روی هر چیزی که قابل پیمایش باشد کار می‌کند:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }
}
```

برای تکرار به تعداد مشخص از **Range** استفاده می‌کنیم:

```rust
fn main() {
    for i in 1..4 {          // 1, 2, 3  (انتهای بازه شامل نمی‌شود)
        println!("{i}");
    }
    for i in (1..=3).rev() { // 3, 2, 1  (=  یعنی شامل انتها)
        println!("{i}");
    }
}
```

اگر هم اندیس و هم مقدار می‌خواهید:

```rust
fn main() {
    let names = ["علی", "سارا", "رضا"];
    for (i, name) in names.iter().enumerate() {
        println!("{i}: {name}");
    }
}
```

> **چرا for بهتر از while برای پیمایش است؟** در `for` امکان خروج از محدوده‌ی آرایه وجود ندارد و کد کوتاه‌تر و سریع‌تر است (چون کامپایلر بررسی محدوده را حذف می‌کند).

## match (پیش‌نمایش)

ساختار `match` قدرتمندترین ابزار کنترل جریان در Rust است و در [فصل Enum](11-enums.md) به‌طور کامل بررسی می‌شود. نگاهی کوتاه:

```rust
fn main() {
    let n = 3;
    match n {
        1 => println!("یک"),
        2 | 3 => println!("دو یا سه"),
        4..=9 => println!("چهار تا نه"),
        _ => println!("چیز دیگر"),
    }
}
```

## تمرین

1. برنامه‌ای بنویسید که اعداد ۱ تا ۱۰۰ را چاپ کند. مضرب‌های ۳ را "Fizz" و مضرب‌های ۵ را "Buzz" و مضرب هر دو را "FizzBuzz" چاپ کند.
2. با `while` فاکتوریل ۱۰ را حساب کنید.
3. با `loop` و `break` اولین عددی که مربعش از ۱۰۰۰ بزرگ‌تر است را پیدا کنید.
4. با دو حلقه‌ی تو در تو جدول ضرب ۱ تا ۹ را چاپ کنید.

حالا به مهم‌ترین مفهوم Rust می‌رسیم: [Ownership](07-ownership.md).
