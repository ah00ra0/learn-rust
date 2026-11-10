# Unsafe Rust

تمام آنچه تاکنون نوشتید **Safe Rust** بود؛ کامپایلر ایمنی حافظه را تضمین می‌کند. اما گاهی لازم است از این قید بیرون بیایید:

- کار با سخت‌افزار و سیستم‌عامل
- فراخوانی کد C
- بهینه‌سازی‌های خاص
- پیاده‌سازی ساختارهایی که کامپایلر نمی‌تواند درستی‌شان را ثابت کند

برای این منظور بلوک `unsafe` وجود دارد.

> **مهم:** `unsafe` بررسی‌های ایمنی را **خاموش نمی‌کند**؛ فقط به شما اجازه می‌دهد پنج کار اضافی انجام دهید. **مسئولیت** درستی آن‌ها با شماست.

## پنج توانایی unsafe

1. دسترسی به مقدار raw pointer (dereference)
2. فراخوانی تابع یا متد `unsafe`
3. دسترسی یا تغییر `static mut`
4. پیاده‌سازی `unsafe trait`
5. دسترسی به فیلد `union`

## Raw Pointer

دو نوع: `*const T` و `*mut T`. تفاوت با reference:

- قوانین borrowing را رعایت نمی‌کنند (می‌توانند همزمان `*mut` و `*const` به یک جا باشند).
- ممکن است null یا نامعتبر باشند.
- خودکار آزاد نمی‌شوند.

ساختن raw pointer **ایمن** است، فقط **خواندن/نوشتن از طریق آن** unsafe است:

```rust
fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        *r2 += 1;
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }
}
```

## تابع unsafe

```rust
unsafe fn dangerous(p: *const i32) -> i32 {
    unsafe { *p }
}

fn main() {
    let x = 42;
    let v = unsafe { dangerous(&x) };
    println!("{v}");
}
```

## ساخت انتزاع ایمن روی کد unsafe

الگوی اصلی: **کد unsafe را کوچک نگه دارید** و پشت یک API ایمن پنهان کنید. مثلاً `split_at_mut` در کتابخانه‌ی استاندارد دو slice قابل‌تغییر و **جدا** از یک slice می‌دهد، کاری که borrow checker به‌تنهایی نمی‌تواند ثابتش کند:

```rust
use std::slice;

fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = my_split_at_mut(&mut v, 3);
    a[0] = 100;
    b[0] = 200;
    println!("{v:?}");
}
```

مسئولیت درستی با ماست: `assert!` تضمین می‌کند محدوده معتبر است و دو slice همپوشانی ندارند.

## فراخوانی کد C (FFI)

از Rust 2024 بلوک `extern` باید به‌صورت `unsafe extern` نوشته شود:

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let r = unsafe { abs(-3) };
    println!("{r}");
}
```

برای عرضه‌ی تابع Rust به C:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("از C صدا زده شدم!");
}

fn main() {
    call_from_c();
}
```

## static mut

متغیر سراسری قابل‌تغییر؛ به‌دلیل خطر data race، خواندن و نوشتنش unsafe است و **توصیه نمی‌شود**:

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    let v = unsafe { COUNTER };
    println!("{v}");
}
```

به‌جایش از `AtomicU32`، `Mutex` داخل `static` یا `OnceLock` استفاده کنید:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    COUNTER.fetch_add(3, Ordering::SeqCst);
    println!("{}", COUNTER.load(Ordering::SeqCst));
}
```

## unsafe trait

```rust
unsafe trait Zeroable {}

unsafe impl Zeroable for i32 {}

fn main() {
    fn takes<T: Zeroable>(_t: T) {}
    takes(5);
}
```

`Send` و `Sync` نمونه‌های واقعی‌اند.

## ابزارهای بررسی

- **Miri**: مفسری که رفتار تعریف‌نشده (UB) را در کد unsafe می‌گیرد: `cargo +nightly miri test`
- **Clippy** و lint های `unsafe_op_in_unsafe_fn`
- **cargo-geiger**: شمارش استفاده از unsafe در وابستگی‌ها

## بهترین روش‌ها

1. فقط وقتی لازم است از unsafe استفاده کنید.
2. بلوک را **کوچک** کنید.
3. بالای هر بلوک با کامنت `// SAFETY:` توضیح دهید چرا امن است.
4. پشت یک API ایمن پنهانش کنید.
5. تست و Miri را فراموش نکنید.

```rust
fn first_unchecked(v: &[i32]) -> i32 {
    assert!(!v.is_empty());
    // SAFETY: بالاتر بررسی کردیم که v خالی نیست
    unsafe { *v.get_unchecked(0) }
}

fn main() {
    println!("{}", first_unchecked(&[7, 8]));
}
```

## تمرین

1. با raw pointer مقدار یک متغیر را تغییر دهید.
2. تابع ایمن ی بنویسید که از `get_unchecked` استفاده کند و `SAFETY` را مستند کند.
3. درباره‌ی `Vec::from_raw_parts` در مستندات بخوانید و بگویید چه شروطی دارد.

در فصل بعد همه‌ی مفاهیم را در یک پروژه‌ی واقعی به کار می‌بریم: [پروژه‌ی mini grep](28-project-grep.md).
