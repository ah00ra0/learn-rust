# تست‌نویسی

تست‌نویسی در Rust بخشی از خود زبان و Cargo است. نیازی به نصب فریم‌ورک جداگانه نیست.

## اولین تست

تابعی با attribute ی `#[test]` یک تست است:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add(2, 2), 4);
    }
}

fn main() {
    println!("{}", add(1, 2));
}
```

اجرا:

```bash
cargo test
```

خروجی نمونه:

```text
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

- `#[cfg(test)]` یعنی این ماژول فقط هنگام `cargo test` کامپایل شود و در باینری نهایی نباشد.
- تست با **panic** شکست می‌خورد.
- هر تست در thread جداگانه و موازی اجرا می‌شود.

## ماکروهای assert

```rust
fn main() {
    assert!(1 + 1 == 2);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);

    // پیغام دلخواه
    let name = "Rust";
    assert!(name.starts_with('R'), "نام باید با R شروع شود، ولی {name} است");
}
```

برای مقایسه‌ی `assert_eq!` نوع‌ها باید `PartialEq` و `Debug` داشته باشند.

## بررسی panic

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("تقسیم بر صفر");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "تقسیم بر صفر")]
    fn divide_by_zero() {
        divide(1, 0);
    }
}

fn main() {
    println!("{}", divide(6, 3));
}
```

## تست با Result

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn parses_number() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }
}

fn main() {}
```

## کنترل اجرای تست‌ها

```bash
cargo test                       # همه
cargo test add                   # فقط تست‌هایی که نامشان شامل add است
cargo test -- --nocapture        # نمایش خروجی println
cargo test -- --test-threads=1   # اجرای ترتیبی
cargo test -- --ignored          # فقط تست‌های ignore شده
```

نادیده گرفتن یک تست:

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "کند است"]
    fn slow_test() {
        // ...
    }
}

fn main() {}
```

## انواع تست

### ۱. Unit Test

داخل همان فایل کد، در ماژول `tests`. می‌تواند به **توابع خصوصی** هم دسترسی داشته باشد چون ماژول فرزند است:

```rust
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}

fn main() {
    println!("{}", internal_adder(1, 1));
}
```

### ۲. Integration Test

در پوشه‌ی `tests/` کنار `src/`. هر فایل مثل یک crate جداگانه است و فقط به **API عمومی** کتابخانه دسترسی دارد:

```text
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

فایل `tests/integration_test.rs`:

```rust,ignore
use my_project::add;

#[test]
fn adds_correctly() {
    assert_eq!(add(2, 3), 5);
}
```

### ۳. Doc Test

مثال‌های داخل کامنت `///` هم اجرا می‌شوند و مستندات را همیشه درست نگه می‌دارند (همین کتاب هم با همین روش تست می‌شود):

```rust
/// جمع دو عدد.
///
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {}
```

(در پروژه‌ی واقعی `my_crate` نام کتابخانه‌ی شماست.)

## چند نکته‌ی عملی

- **تست را قبل از رفع باگ بنویسید:** اول تستی که باگ را بازتولید می‌کند، بعد درستش کنید.
- نام تست‌ها را توصیفی بگذارید: `returns_none_for_empty_input`.
- منطق تجاری را از I/O جدا کنید تا بدون فایل و شبکه قابل تست باشد.
- برای ماک کردن (mock) از trait ها استفاده کنید: وابستگی را با trait بگیرید و در تست پیاده‌سازی ساختگی بدهید.
- برای تست‌های مبتنی بر ویژگی (property-based) به crate ی `proptest` نگاه کنید.
- برای اندازه‌گیری سرعت از `criterion` استفاده کنید.
- برای پوشش کد (coverage) ابزار `cargo llvm-cov` را ببینید.

## تمرین

1. تابع `is_palindrome(s: &str) -> bool` بنویسید و ۵ تست برایش بنویسید (شامل رشته‌ی خالی).
2. تابعی بنویسید که در شرایطی panic کند و آن را با `should_panic` تست کنید.
3. در یک پروژه‌ی کتابخانه‌ای، پوشه‌ی `tests/` بسازید و یک integration test بنویسید.
4. به تابعی doc test اضافه کنید.

فصل بعد: [Trait Object و برنامه‌نویسی شی‌گرا](24-trait-objects.md).
