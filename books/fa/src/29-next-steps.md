# مسیر بعدی و منابع یادگیری

تبریک! اگر تا اینجا آمده‌اید، با تمام مفاهیم اصلی Rust آشنا شده‌اید. حالا بخش اصلی یادگیری شروع می‌شود: **ساختن چیزهای واقعی.**

## مرور آنچه یاد گرفتید

- نصب، Cargo، متغیر، نوع داده، تابع و کنترل جریان
- **Ownership، Borrowing و Slice** (قلب Rust)
- Struct، Enum، `match`، `Option` و مجموعه‌ها
- ماژول‌ها و مدیریت خطا با `Result` و `?`
- Generics، Trait و Lifetime
- Closure و Iterator
- Smart Pointer ها و Concurrency
- تست، Macro، Async و Unsafe

## نقشه‌ی پیشنهادی ادامه‌ی راه

### مرحله ۱: تمرین (۱ تا ۲ ماه)

- مسائل [Exercism Rust Track](https://exercism.org/tracks/rust) را حل کنید (با mentor رایگان).
- [Rustlings](https://github.com/rust-lang/rustlings): تمرین‌های کوچک تعاملی برای رفع خطاهای کامپایل.
- مسائل [Advent of Code](https://adventofcode.com) و [LeetCode](https://leetcode.com) را با Rust حل کنید.

### مرحله ۲: پروژه‌ی کوچک

چند ایده:

- ابزار خط فرمان: مدیریت کارها (todo)، تبدیل واحد، تغییر نام دسته‌ای فایل‌ها
- ماشین‌حساب یا مفسر یک زبان ساده
- بازی ساده با ترمینال (مثل مار یا Tetris) با crate ی `ratatui` یا `crossterm`
- سرور ساده‌ی HTTP یا ربات تلگرام

### مرحله ۳: انتخاب مسیر تخصصی

| حوزه | ابزارها و crate ها |
|---|---|
| **وب بک‌اند** | `axum`، `actix-web`، `sqlx`، `tokio`، `serde` |
| **خط فرمان** | `clap`، `ratatui`، `anyhow`، `indicatif` |
| **دسکتاپ** | `tauri`، `egui`، `iced`، `slint` |
| **بازی** | `bevy`، `macroquad`، `ggez` |
| **WebAssembly** | `wasm-bindgen`، `yew`، `leptos`، `dioxus` |
| **سیستمی و نهفته** | `embedded-hal`، `embassy`، `no_std` |
| **داده و ML** | `polars`، `ndarray`، `burn`، `candle` |
| **امنیت و شبکه** | `rustls`، `tokio`، `pnet` |

### مرحله ۴: مشارکت در پروژه‌های متن‌باز

- issue هایی با برچسب `good first issue` در GitHub را ببینید.
- در مخزن‌های crate هایی که استفاده می‌کنید مستندات یا باگ کوچکی را اصلاح کنید.
- کد منبع پروژه‌های خوب (مثل `ripgrep`، `bat`، `fd`) را بخوانید.

## منابع رسمی و معتبر (انگلیسی)

| منبع | توضیح |
|---|---|
| [The Rust Programming Language](https://doc.rust-lang.org/book/) | کتاب رسمی؛ معروف به "The Book" |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | یادگیری از طریق مثال |
| [Standard Library Docs](https://doc.rust-lang.org/std/) | مستندات کتابخانه‌ی استاندارد |
| [The Cargo Book](https://doc.rust-lang.org/cargo/) | همه‌چیز درباره‌ی Cargo |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | Unsafe Rust در عمق |
| [Rust Reference](https://doc.rust-lang.org/reference/) | مرجع دقیق زبان |
| [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/) | کتاب رسمی async |
| [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) | الگوهای طراحی |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | راهنمای طراحی کتابخانه |
| [docs.rs](https://docs.rs) | مستندات همه‌ی crate ها |
| [crates.io](https://crates.io) | مخزن کتابخانه‌ها |
| [Rust Playground](https://play.rust-lang.org) | اجرای کد در مرورگر |

## جامعه

- [Rust Users Forum](https://users.rust-lang.org)
- [Reddit: r/rust](https://reddit.com/r/rust)
- [Rust Discord](https://discord.gg/rust-lang)
- خبرنامه‌ی هفتگی [This Week in Rust](https://this-week-in-rust.org)

## نکات یادگیری موثر

۱. **هر روز کمی کد بنویسید**، حتی ۳۰ دقیقه.
۲. **پیغام‌های کامپایلر را کامل بخوانید.** معمولاً راه‌حل را دارند.
۳. در ابتدا `clone()` و `String` را بدون عذاب وجدان به کار ببرید؛ بعداً بهینه کنید.
۴. با `cargo clippy` کدتان را مرتب بررسی کنید.
۵. کد دیگران را بخوانید.
۶. چیزی را که یاد گرفتید **آموزش بدهید**؛ بهترین راه تثبیت یادگیری است.
۷. صبور باشید. بسیاری از مفاهیم Rust وقتی توی یک پروژه‌ی واقعی لازم می‌شوند "جا می‌افتند".

## کمک به این کتاب

این کتاب متن‌باز است. اگر اشتباه، پیشنهاد یا فصل جدیدی دارید:

- مخزن: [github.com/ah00ra0/learn-rust](https://github.com/ah00ra0/learn-rust)
- با دکمه‌ی ✏️ بالای هر صفحه می‌توانید همان صفحه را در GitHub ویرایش کنید.
- یک Issue باز کنید یا Pull Request بفرستید.
- اگر کتاب به شما کمک کرد، به مخزن **⭐ Star** بدهید و برای دوستانتان بفرستید. این ساده‌ترین راه حمایت از آن است.

موفق باشید و **Happy Rusting!** 🦀
