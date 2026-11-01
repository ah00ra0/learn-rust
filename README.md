# آموزش Rust (راست) به زبان فارسی 🦀

> **آموزش رایگان، کامل و متن‌باز زبان برنامه‌نویسی Rust از صفر تا پروژه‌ی واقعی، به فارسی.**

📖 **[خواندن آنلاین کتاب](https://ah00ra0.github.io/learn-rust/)**

اگر دنبال **آموزش Rust**، **یادگیری راست**، **آموزش Rust برای مبتدی‌ها** یا **دوره‌ی Rust فارسی** هستید، این کتاب را برای همین ساخته‌ایم. با خواندن آن، بدون نیاز به منبع دیگر، می‌توانید Rust را کامل یاد بگیرید.

## چه چیزهایی یاد می‌گیرید؟

| بخش | موضوعات |
|---|---|
| **شروع کار** | نصب Rust روی ویندوز، مک و لینوکس، Hello World، Cargo |
| **مبانی** | متغیر، نوع داده، تابع، if، loop، while، for |
| **مفاهیم اصلی** | Ownership، Borrowing، Reference، Slice |
| **ساختارهای داده** | Struct، Enum، match، Option، Vec، String، HashMap |
| **سازمان‌دهی** | Module، Crate، مدیریت خطا با Result و `?` |
| **انتزاع‌ها** | Generics، Trait، Lifetime |
| **سبک تابعی** | Closure، Iterator |
| **پیشرفته** | Smart Pointer، Concurrency (Thread, Channel, Mutex)، Pattern Matching، تست‌نویسی، Trait Object، Macro، Async/Await، Unsafe |
| **پروژه‌ی عملی** | ساخت ابزار جستجوی متن (mini grep) |
| **ضمیمه** | مسیر ادامه‌ی یادگیری، واژه‌نامه‌ی فارسی-انگلیسی |

## ویژگی‌ها

- ✅ **۲۹ فصل** از مقدماتی تا پیشرفته
- ✅ **همه‌ی مثال‌های کد تست‌شده‌اند** (با `mdbook test` کامپایل می‌شوند)
- ✅ اجرای کد در مرورگر با دکمه‌ی ▶ (Rust Playground)
- ✅ جستجو در کتاب، حالت تاریک، فونت فارسی (وزیرمتن) و چیدمان راست‌به‌چپ
- ✅ تمرین در پایان هر فصل
- ✅ پروژه‌ی کامل و قابل اجرا در [`projects/minigrep`](projects/minigrep)
- ✅ کاملاً رایگان و متن‌باز

## اجرای کتاب روی سیستم خودتان

```bash
cargo install mdbook
git clone https://github.com/ah00ra0/learn-rust
cd learn-rust
mdbook serve --open
```

برای تست همه‌ی مثال‌های کتاب:

```bash
mdbook test
```

برای ساخت نسخه‌ی نهایی (HTML در پوشه‌ی `docs`، همراه با sitemap):

```bash
./build.sh
```

## مشارکت

اشتباه تایپی، توضیح نامفهوم یا پیشنهاد فصل جدید دارید؟ [Issue باز کنید](https://github.com/ah00ra0/learn-rust/issues) یا Pull Request بفرستید. هر مثال کد جدید باید با `mdbook test` کامپایل شود.

اگر کتاب برایتان مفید بود، به این مخزن **⭐ Star** بدهید و برای دوستانتان بفرستید.

## منابع رسمی

[The Rust Book](https://doc.rust-lang.org/book/) · [Rust by Example](https://doc.rust-lang.org/rust-by-example/) · [Rustlings](https://github.com/rust-lang/rustlings) · [docs.rs](https://docs.rs) · [crates.io](https://crates.io)

---

**کلیدواژه‌ها:** آموزش Rust، آموزش راست، یادگیری Rust، Rust فارسی، زبان برنامه‌نویسی Rust، ownership، borrowing، lifetime، trait، cargo، آموزش رایگان برنامه‌نویسی
