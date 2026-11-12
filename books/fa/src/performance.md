# عملکرد و بهینه‌سازی

Rust سریع است، ولی **سریع‌بودن خودبه‌خود اتفاق نمی‌افتد**: کد کند در هر زبانی ممکن است. این فصل به شما چارچوب فکری و ابزار می‌دهد تا کد Rust را بر پایه‌ی **اندازه‌گیری**، نه حدس، سریع کنید.

> همه‌ی اعداد این فصل از اجرای واقعی [`projects/bench-demo`](https://github.com/ah00ra0/learn-rust/tree/main/projects/bench-demo) با `criterion` گرفته شده‌اند. **سیستم:** یک ماشین مجازی ۲ هسته‌ای (Intel Xeon 2.1GHz)، Rust 1.97، حالت `--release`. این ماشین پرنوسان است؛ اعداد را به‌عنوان **ترتیب بزرگی** بخوانید، نه مقدار دقیق. روی سیستم شما نسبت‌ها ممکن است فرق کند، و دقیقاً به همین دلیل باید خودتان اندازه بگیرید.

## قانون اول: اول اندازه‌گیری، بعد بهینه‌سازی

دو بدفهمی رایج:

- «این حلقه کند به نظر می‌رسد» ← معمولاً مشکل جای دیگری است.
- «`clone` بد است» ← گاهی ۱۰ نانوثانیه است و اهمیتی ندارد.

روش درست:

۱. **ابتدا درست بنویسید**، با ساده‌ترین الگوریتم مناسب.
۲. **اندازه بگیرید** (benchmark یا profiler) و گلوگاه واقعی را پیدا کنید.
۳. فقط همان بخش را بهینه کنید، **دوباره اندازه بگیرید** و تست‌ها را اجرا کنید.
۴. اگر بهبود معنادار نبود، تغییر را برگردانید (پیچیدگی هزینه دارد).

> ۹۰٪ زمان برنامه‌ها معمولاً در ۱۰٪ کد می‌گذرد. الگوریتم و ساختار داده (مثلاً `HashMap` به‌جای جستجوی خطی) تقریباً همیشه از ریزبهینه‌سازی مهم‌تر است.

## ۱. همیشه با `--release` اندازه بگیرید

حالت پیش‌فرض (`cargo build`) برای کامپایل سریع و خطایابی است و بهینه‌سازی ندارد. تفاوتش با release می‌تواند ده‌ها برابر باشد.

```bash
cargo build --release
cargo run --release
cargo bench
```

تنظیمات بیشتر در [Cargo پیشرفته](cargo-advanced.md): `lto`، `codegen-units = 1`، `panic = "abort"`، `opt-level`. مثلاً برای حداکثر سرعت:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

هزینه: کامپایل کندتر. اثر را **خودتان** اندازه بگیرید؛ روی برنامه‌ی کوچک گاهی چیزی عوض نمی‌شود.

## ۲. Benchmark با criterion

`cargo bench` ی داخلی هنوز در Rust پایدار نیست؛ استاندارد عملی crate ی **criterion** است: چندین بار اجرا می‌کند، آمار می‌گیرد، نوسان را حذف می‌کند و تغییرات را با اجرای قبلی مقایسه می‌کند.

```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.8"

[[bench]]
name = "bench"
harness = false        # criterion خودش main دارد
```

```rust,ignore
// benches/bench.rs
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_build(c: &mut Criterion) {
    let mut g = c.benchmark_group("vec_build_100k");
    g.bench_function("no_capacity", |b| b.iter(|| build_no_capacity(black_box(100_000))));
    g.bench_function("with_capacity", |b| b.iter(|| build_with_capacity(black_box(100_000))));
    g.finish();
}

criterion_group!(benches, bench_build);
criterion_main!(benches);
```

**`black_box` چیست؟** کامپایلر ممکن است محاسبه‌ای را که نتیجه‌اش استفاده نمی‌شود یا ورودی‌اش ثابت است حذف یا از قبل حساب کند. `std::hint::black_box` به او می‌گوید «این مقدار ناشناخته است» تا benchmark چیز دیگری را اندازه نگیرد.

اجرا (توجه: وقتی هم benchmark ی کتابخانه‌ای و هم benchmark ی `harness = false` دارید، آرگومان‌های criterion را با `--bench نام` بدهید):

```bash
cargo bench --bench bench -- --warm-up-time 1 --measurement-time 3
```

خروجی نمونه‌ی هر benchmark: `time: [حد پایین  میانگین  حد بالا]`.

### اندازه‌گیری سریع بدون criterion

برای یک بررسی سریع (نه نتیجه‌ی قابل‌اتکا):

```rust
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut total = 0u64;
    for i in 0..1_000_000u64 {
        total = total.wrapping_add(black_box(i) * 2);
    }
    let elapsed = start.elapsed();
    println!("{total} در {elapsed:?}");
}
```

## ۳. پروفایل: پیدا کردن گلوگاه

Benchmark می‌گوید «چقدر»؛ **Profiler** می‌گوید «کجا».

- **flamegraph:** `cargo install flamegraph` سپس `cargo flamegraph` نمودار شعله‌ای از زمان صرف‌شده در هر تابع می‌سازد (روی لینوکس از `perf` استفاده می‌کند). برای اینکه نام توابع در نمودار بیاید، در `Cargo.toml` بگذارید: `[profile.release] debug = true` (یا profile سفارشی که `inherits = "release"` دارد).
- **`perf stat` / `perf record`** (لینوکس)، **Instruments** (macOS)، **Visual Studio Profiler** (ویندوز).
- **`cargo build --timings`** برای زمان کامپایل، نه اجرا.
- **DHAT / `dhat-rs`** و **heaptrack** برای پروفایل تخصیص حافظه.
- **`cargo asm`** (cargo-show-asm) برای دیدن اسمبلی تولیدشده‌ی یک تابع.

## ۴. تخصیص حافظه (Allocation)

تخصیص روی heap نسبتاً گران است. بدترین حالت: تخصیص‌های ریز در حلقه‌ی داغ.

### `Vec::with_capacity`

```rust
fn main() {
    let mut a: Vec<u64> = Vec::new();
    let mut b: Vec<u64> = Vec::with_capacity(1000);
    assert_eq!(a.capacity(), 0);
    assert!(b.capacity() >= 1000);

    let b_ptr = b.as_ptr();
    for i in 0..1000 {
        a.push(i);
        b.push(i);
    }
    // b هرگز بازتخصیص نشد: آدرس بافر همان ماند
    assert_eq!(b.as_ptr(), b_ptr);
    assert!(a.capacity() >= 1000);
}
```

Vec با هر بار پر شدن، ظرفیتش را چند برابر می‌کند (رشد هندسی)، پس هزینه‌ی هر `push` به‌طور **سرشکن شده** ثابت است. بنابراین `with_capacity` در عمل **بهبود کوچک** می‌دهد. اندازه‌گیری ما برای ۱۰۰ هزار عنصر:

| روش | زمان تقریبی |
|---|---|
| `Vec::new()` + push | ≈ 113 µs |
| `Vec::with_capacity(n)` + push | ≈ 107 µs |

تفاوت کم و در حد نوسان این ماشین است. با این حال `with_capacity` رایگان و بی‌ضرر است و وقتی اندازه را می‌دانید باید بنویسید (کمتر حافظه‌ی اضافی و بدون بازتخصیص و کپی). تأثیر بزرگ‌تر آن وقتی دیده می‌شود که عناصر بزرگ‌تر یا تخصیص‌دهنده کندتر باشد.

### الگوهای کاهش تخصیص

- **بافر را بازاستفاده کنید:** `buf.clear()` ظرفیت را نگه می‌دارد.
- **به‌جای `String` در حلقه، `&str` یا `Cow`** (فصل [رشته‌ها](strings-unicode.md)).
- **`write!` به یک `String` مشترک** به‌جای `format!` در هر دور.
- **`collect` میانی غیرضروری را حذف کنید**؛ زنجیره‌ی iterator را تا مصرف نهایی نگه دارید.
- **`clone` را فقط وقتی لازم است بزنید**، ولی نگران `clone` ی کوچک هم نباشید.
- **انواع کوچک روی stack:** crate های `smallvec` و `arrayvec` برای بردارهای کوچک بدون heap.
- **تخصیص‌دهنده‌ی دیگر:** `mimalloc` یا `jemalloc` (از طریق `#[global_allocator]`) گاهی در برنامه‌های چندنخی تخصیص‌سنگین بهبود می‌دهد؛ اندازه‌گیری کنید.

### اندازه‌ی انواع در حافظه

دانستن اندازه به پیش‌بینی هزینه‌ی کپی و استفاده از کش کمک می‌کند:

```rust
use std::mem::size_of;

#[allow(dead_code)]
struct Reordered { a: u8, b: u32, c: u8 }       // Rust ترتیب را بهینه می‌کند
#[allow(dead_code)]
#[repr(C)]
struct CLayout { a: u8, b: u32, c: u8 }         // ترتیب اعلام‌شده، با padding

fn main() {
    let word = size_of::<usize>();

    // اشاره‌گرهای ساده یک کلمه‌اند؛ fat pointer ها دو کلمه
    assert_eq!(size_of::<&u8>(), word);
    assert_eq!(size_of::<Box<u8>>(), word);
    assert_eq!(size_of::<&[u8]>(), 2 * word);          // آدرس + طول
    assert_eq!(size_of::<&str>(), 2 * word);
    assert_eq!(size_of::<Box<dyn Fn()>>(), 2 * word);  // آدرس + vtable

    // String و Vec: آدرس + ظرفیت + طول
    assert_eq!(size_of::<String>(), 3 * word);
    assert_eq!(size_of::<Vec<u8>>(), 3 * word);

    // بهینه‌سازی niche: Option<Box<T>> هزینه‌ای اضافه ندارد
    assert_eq!(size_of::<Option<Box<u8>>>(), size_of::<Box<u8>>());
    assert_eq!(size_of::<Option<&u8>>(), size_of::<&u8>());
    assert!(size_of::<Option<u32>>() > size_of::<u32>()); // برای u32 جا برای tag لازم است

    // ترتیب فیلدها
    assert_eq!(size_of::<Reordered>(), 8);
    assert_eq!(size_of::<CLayout>(), 12);
}
```

نکات: Rust به‌طور پیش‌فرض ترتیب فیلدها را برای کاهش padding **بازچینی می‌کند** (مگر `#[repr(C)]`). enum های بزرگ به اندازه‌ی بزرگ‌ترین variant می‌شوند؛ اگر یک variant بسیار بزرگ دارید و بقیه کوچک، `Box` کردن آن می‌تواند اندازه‌ی کل را کم کند.

## ۵. چیدمان داده و کش پردازنده

پردازنده‌ی مدرن بسیار سریع‌تر از حافظه‌ی اصلی است؛ سرعت واقعی را **کش** تعیین می‌کند. داده‌ی پیوسته و خطی‌خوانده‌شده، خیلی سریع‌تر از داده‌ی پراکنده است.

### AoS در برابر SoA

فرض کنید یک میلیون ذره دارید و فقط **مجموع جرم‌ها** را می‌خواهید.

```rust,ignore
// AoS: آرایه‌ی struct ها؛ هر ذره ۵۶ بایت
pub struct ParticleAos { pub x: f32, pub y: f32, pub z: f32, pub mass: f32, pub id: u64, pub name_hash: [u64; 4] }

// SoA: ساختار آرایه‌ها؛ هر فیلد در یک Vec جدا
pub struct ParticlesSoa { pub x: Vec<f32>, pub y: Vec<f32>, pub z: Vec<f32>, pub mass: Vec<f32>, pub id: Vec<u64>, pub name_hash: Vec<[u64; 4]> }

pub fn total_mass_aos(p: &[ParticleAos]) -> f32 { p.iter().map(|p| p.mass).sum() }
pub fn total_mass_soa(p: &ParticlesSoa) -> f32 { p.mass.iter().sum() }
```

نتیجه‌ی اندازه‌گیری (۱ میلیون ذره):

| چیدمان | زمان تقریبی | داده‌ی خوانده‌شده |
|---|---|---|
| AoS | ≈ 2.10 ms | ۵۶ بایت به‌ازای هر ذره (کش پر از فیلدهای بی‌مصرف) |
| SoA | ≈ 0.73 ms | ۴ بایت به‌ازای هر ذره (فقط `mass`) |

حدود **۳ برابر** سریع‌تر، بدون هیچ ترفند الگوریتمی. SoA همچنین به کامپایلر اجازه‌ی vectorization (SIMD) می‌دهد. در مقابل، اگر معمولاً همه‌ی فیلدهای یک ذره را با هم می‌خوانید، AoS ساده‌تر و گاهی بهتر است. باز هم: **اندازه بگیرید**.

## ۶. حلقه‌ی دستی یا iterator؟

وعده‌ی «zero-cost abstraction» (فصل [Iterator](19-iterators.md)) یعنی زنجیره‌ی iterator ها **معمولاً** به حلقه‌ی دستی تبدیل می‌شود. اندازه‌گیری ما (۱ میلیون عدد):

| محاسبه | روش | زمان تقریبی |
|---|---|---|
| جمع عناصر | حلقه‌ی اندیس‌دار `data[i]` | ≈ 279 µs |
| جمع عناصر | `iter().fold` | ≈ 320 µs |
| جمع مربع‌های زوج‌ها | `filter().map().fold()` | ≈ 790 µs |
| جمع مربع‌های زوج‌ها | حلقه‌ی `for` با `if` | ≈ 649 µs |
| جمع مربع‌های زوج‌ها | `rayon::par_iter` | ≈ 446 µs |

درس‌ها:

- **حلقه‌ی اندیس‌دار و iterator ی ساده‌ی جمع تفاوت چشمگیری ندارند.** Bounds check در `data[i]` معمولاً توسط بهینه‌ساز حذف می‌شود چون `i < data.len()` اثبات می‌شود. پس از ترس از «اندیس کند است» دست بکشید و کد را خوانا بنویسید.
- «Zero-cost» یعنی **اغلب** هم‌سرعت؛ در مثال سوم ماشین ما حلقه‌ی دستی حدود ۲۰٪ سریع‌تر بود. این به شکل کد، نسخه‌ی کامپایلر و پردازنده بستگی دارد. اگر این گلوگاه شماست، هر دو را اندازه بگیرید.
- `rayon` روی ۲ هسته حدود ۱٫۵ برابر سریع‌تر از حلقه‌ی دستی شد (نه ۲ برابر): بخشی از زمان صرف تقسیم کار و هماهنگی می‌شود. برای کارهای **کوچک** rayon حتی ممکن است کندتر باشد.

## ۷. dyn Trait در برابر generic و enum

ارسال پویا (`Box<dyn Trait>`) انعطاف می‌دهد ولی هزینه دارد: heap برای هر شیء، اشاره‌گر دوتایی (vtable)، و **جلوگیری از inline و vectorization**. اندازه‌گیری برای ۱۰۰ هزار شکل:

| سناریو | روش | زمان تقریبی |
|---|---|---|
| فقط یک نوع (`Square`) | `Vec<Box<dyn Shape>>` | ≈ 610 µs |
| فقط یک نوع (`Square`) | generic: `&[Square]` | ≈ 72 µs |
| ترکیب دو نوع | `Vec<Box<dyn Shape>>` | ≈ 578 µs |
| ترکیب دو نوع | `Vec<enum AnyShape>` | ≈ 115 µs |

نسبت‌ها حدود **۸ برابر** و **۵ برابر** بودند. مفهوم:

- وقتی **مجموعه‌ی نوع‌ها بسته و معلوم** است، **enum** هم سریع‌تر است هم `match` را وادار به کامل بودن می‌کند (فصل [Enum](11-enums.md)).
- وقتی انواع در زمان کامپایل مشخص‌اند، **generic** (`T: Trait`) را بدهید (فصل [Generics](15-generics.md)).
- `dyn` را برای **افزونه‌پذیری واقعی** نگه دارید (کاربر بتواند نوع جدید اضافه کند) و بدانید در مسیرهای داغ هزینه دارد. در بیشتر برنامه‌ها (مثلاً لایه‌ی I/O) این هزینه کاملاً بی‌اهمیت است.

## ۸. سایر نکات عملی

### کپی‌های بی‌دلیل

- پارامتر `&str` / `&[T]` بگیرید، نه `String` / `Vec<T>` ([Slice](09-slices.md)).
- `iter()` و `for x in &v` به‌جای `into_iter()` وقتی نیاز به مالکیت ندارید.
- `Rc`/`Arc` کردن داده‌های بزرگ مشترک به‌جای clone.

### `#[inline]`

کامپایلر خودش بیشتر تصمیم‌ها را درست می‌گیرد. `#[inline]` را برای توابع **کوچک عمومی یک کتابخانه که از crate دیگر صدا زده می‌شوند** (چون inline بین crate ها بدون آن/LTO ممکن نیست) به‌کار ببرید. `#[inline(always)]` را بدون اندازه‌گیری نگذارید.

### SIMD

کامپایلر در حلقه‌های ساده‌ی روی slice ها خودکار vectorize می‌کند (به‌خصوص با SoA و بدون شاخه‌های پیچیده). برای کنترل دستی: `std::simd` (هنوز ناپایدار)، crate هایی مثل `wide` یا `std::arch`. برای استفاده از دستورهای CPU مقصد: `RUSTFLAGS="-C target-cpu=native"` (باینری فقط روی CPU های مشابه اجرا می‌شود).

### موازی‌سازی

برای کار CPU-bound روی داده‌های بزرگ: `rayon` (اغلب فقط با تبدیل `iter()` به `par_iter()`). برای I/O: async ([Async](26-async.md)). توجه به **هزینه‌ی هماهنگی** و **تقسیم‌ناپذیری** (قانون آمدال).

### Bounds check

اگر پروفایل نشان داد bounds check گلوگاه است: اول با `iter()`/`zip`/`chunks`/`windows` بازنویسی کنید (ایمن و اغلب سریع‌تر). `get_unchecked` (unsafe) آخرین راه است ([Unsafe](27-unsafe.md)).

### I/O

برای فایل و stdout از `BufReader`/`BufWriter` و قفل `stdout().lock()` استفاده کنید ([فصل I/O](formatting-and-io.md)). هر `println!` غیربافری یک system call هزینه دارد.

### الگوریتم و ساختار داده

- جستجوی مکرر در `Vec` ← `HashMap`/`HashSet`/`BTreeMap`.
- درج/حذف مکرر از ابتدا ← `VecDeque`.
- `HashMap` پیش‌فرض از SipHash (مقاوم در برابر حمله) استفاده می‌کند که کندتر از هش‌های ساده است؛ برای داده‌ی غیرمخرب می‌توان `rustc-hash` (FxHash) یا `ahash` را امتحان کرد.
- رشته‌های متعدد تکراری ← interning.

## ۹. زمان کامپایل

سرعت **توسعه** هم مهم است. Rust کامپایل کندی دارد، ولی راه‌هایی هست (جزئیات در [Cargo پیشرفته](cargo-advanced.md)):

- `cargo check` در حین کدنویسی
- `[profile.dev.package."*"] opt-level = 2` تا وابستگی‌ها در dev کند نباشند
- لینکر سریع (`lld` / `mold`)
- `sccache`
- کاهش وابستگی‌ها و feature های بی‌استفاده (`cargo machete`، `default-features = false`)
- تقسیم به چند crate در workspace
- استفاده‌ی بیش از حد از generic های تودرتو و macro های سنگین، زمان کامپایل را بالا می‌برد؛ گاهی `dyn` یا یک تابع غیر-generic داخلی (الگوی «outlined inner function») کمک می‌کند.
- `cargo build --timings` برای دیدن گلوگاه.

## چک‌لیست بهینه‌سازی

۱. آیا تست‌ها قبل و بعد از تغییر پاس می‌شوند؟
۲. با `--release` اندازه گرفتم؟
۳. گلوگاه را با profiler پیدا کردم، نه با حدس؟
۴. الگوریتم/ساختار داده درست است؟
۵. تخصیص‌ها در حلقه‌ی داغ را کم کردم؟
۶. چیدمان داده برای دسترسی غالب مناسب است؟
۷. بهبود را با benchmark تکرارپذیر اثبات کردم و ارزش پیچیدگی‌اش را دارد؟

## تمرین

1. پروژه‌ی `bench-demo` را اجرا کنید (`cargo bench --bench bench`) و نتایج را با جدول‌های این فصل مقایسه کنید. کدام نسبت‌ها روی سیستم شما فرق دارد؟
2. تابعی بنویسید که یک `Vec<String>` را به یک `String` با جداکننده تبدیل کند، یک بار با `+=` و `format!` در حلقه و یک بار با `join` / `write!` به یک بافر. هر دو را با criterion مقایسه کنید.
3. `size_of` را برای struct ی خودتان چاپ کنید، ترتیب فیلدها را عوض کنید و `#[repr(C)]` بگذارید؛ تفاوت را توضیح دهید.
4. برنامه‌ای بنویسید که فایل متنی بزرگی را بشمارد (تعداد کلمات)؛ نسخه‌ی اول با `read_to_string` و نسخه‌ی دوم با `BufReader::lines`؛ حافظه‌ی مصرفی و زمان را مقایسه کنید.
5. با `cargo flamegraph` یک برنامه‌ی ساده را پروفایل کنید و پرهزینه‌ترین تابع را شناسایی کنید.
