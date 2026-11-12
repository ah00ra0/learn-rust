# Cargo پیشرفته

در [فصل دوم](02-hello-cargo.md) با دستورهای پایه‌ی Cargo آشنا شدید. Cargo بسیار بیشتر از `build` و `run` است: مدیریت پروژه‌های چندبسته‌ای (workspace)، قابلیت‌های اختیاری (feature)، تنظیم دقیق بهینه‌سازی (profile)، اسکریپت build، انتشار و ابزارهای کیفیت. این فصل همه را با یک پروژه‌ی واقعی نشان می‌دهد.

> پروژه‌ی همراه: [`projects/workspace-demo`](https://github.com/ah00ra0/learn-rust/tree/main/projects/workspace-demo). تمام خروجی‌های این فصل از اجرای واقعی با Cargo 1.97 گرفته شده‌اند.

## ساختار workspace نمونه

```text
workspace-demo/
├── Cargo.toml            ← فقط [workspace]؛ بدون [package] (virtual workspace)
└── crates/
    ├── textkit/          ← کتابخانه‌ی آمار متن (با feature اختیاری serde)
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── app/              ← برنامه‌ی اجرایی که textkit را به کار می‌برد
        ├── Cargo.toml
        ├── build.rs
        └── src/main.rs
```

## Workspace

وقتی پروژه‌تان چند crate دارد (مثلاً کتابخانه + CLI + سرور)، آن‌ها را در یک **workspace** جمع می‌کنید. مزایا:

- یک `Cargo.lock` و یک پوشه‌ی `target/` مشترک (کامپایل مشترک وابستگی‌ها)
- نسخه‌ی یکسان وابستگی‌ها در همه‌ی crate ها
- اجرای یک‌جای دستورها: `cargo test --workspace`

```toml
# Cargo.toml (ریشه)
[workspace]
resolver = "3"
members = ["crates/textkit", "crates/app"]
default-members = ["crates/app"]     # cargo run / build بدون -p همین را می‌سازد

# اطلاعات مشترک: crate ها با version.workspace = true استفاده می‌کنند
[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"

# وابستگی‌ها یک بار با نسخه تعریف می‌شوند
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
textkit = { path = "crates/textkit", version = "0.1.0" }
```

و در هر crate:

```toml
# crates/app/Cargo.toml
[package]
name = "app"
version.workspace = true
edition.workspace = true
license.workspace = true
publish = false           # این crate را هرگز در crates.io منتشر نکن

[dependencies]
textkit = { workspace = true, features = ["serde"] }
serde_json.workspace = true
```

> **نکته:** در یک **virtual workspace** (ریشه‌ای که `[package]` ندارد) باید `resolver` را صریح بنویسید. در یک پکیج عادی با edition 2024، resolver ی نسخه‌ی 3 خودکار انتخاب می‌شود. این resolver هنگام انتخاب نسخه‌ی وابستگی‌ها `rust-version` (MSRV) پکیج‌ها را هم در نظر می‌گیرد.

### دستورهای رایج workspace

```bash
cargo build --workspace            # همه‌ی اعضا
cargo test  --workspace            # تست همه
cargo test  -p textkit             # فقط یک عضو
cargo run   -p app                 # اجرای یک عضو
cargo clippy --workspace --all-targets -- -D warnings
```

## Feature ها

**Feature** قابلیت اختیاری است: کد یا وابستگی‌ای که فقط در صورت درخواست کامپایل می‌شود. کاربردها: کاهش زمان کامپایل و اندازه، پشتیبانی اختیاری از کتابخانه‌های دیگر (مثل serde)، قابلیت‌های آزمایشی.

```toml
# crates/textkit/Cargo.toml
[dependencies]
serde = { workspace = true, optional = true }   # وابستگی اختیاری

[features]
default = []
# feature ی به نام serde که وابستگی serde را فعال می‌کند
serde = ["dep:serde"]
```

و در کد با `cfg`:

```rust,ignore
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Stats {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
}

#[cfg(feature = "serde")]
#[test]
fn stats_serialize_with_feature() {
    let json = serde_json::to_string(&stats("hi there")).unwrap();
    assert_eq!(json, r#"{"chars":8,"words":2,"lines":1}"#);
}
```

- `dep:serde` باعث می‌شود نام `serde` به‌عنوان feature ضمنی ساخته نشود و فقط همین feature صریح وجود داشته باشد (روش مدرن).
- `#[cfg_attr(شرط, attribute)]` فقط در صورت برقرار بودن شرط attribute را اعمال می‌کند.

فعال کردن از خط فرمان:

```bash
cargo build --features serde
cargo build --all-features
cargo build --no-default-features
```

### یکی‌سازی Feature ها (Feature Unification)

نکته‌ی مهمی که بسیاری را غافلگیر می‌کند: **features به‌صورت افزایشی (additive) هستند**؛ وقتی چند crate در یک build مشترکاً به یک وابستگی نیاز دارند، **اجتماع** feature های درخواستی همه فعال می‌شود. نتیجه‌ی عملی در پروژه‌ی ما:

```bash
cargo test -p textkit
```

```text
test tests::counts_unicode_chars ... ok
test tests::reverses ... ok
test result: ok. 2 passed; 0 failed; ...
```

فقط ۲ تست؛ چون هیچ‌کس `serde` را فعال نکرده، تست مربوط به آن اصلاً کامپایل نشده. ولی در

```bash
cargo test --workspace
```

۳ تست اجرا می‌شود، چون `app` در `textkit` فیچر `serde` را روشن کرده و آن برای کل build فعال است. می‌توانید دلیلش را ببینید:

```bash
cargo tree -e features -i serde
```

```text
serde v1.0.229
├── serde feature "default"
│   └── textkit v0.1.0 (...)
│       ├── textkit feature "default"
│       │   └── app v0.1.0 (...)
│       └── textkit feature "serde"
│           └── app v0.1.0 (...) (*)
```

**قواعد طراحی:**

- feature ها نباید رفتار را «عوض» کنند؛ فقط چیزی **اضافه** کنند. وگرنه فعال شدنشان توسط یک crate دیگر، رفتار crate شما را برای همه عوض می‌کند.
- هیچ‌وقت feature ی به نام «no-xxx» نسازید که قابلیت را خاموش کند.
- `default` را کوچک نگه دارید.
- `cargo hack --feature-powerset` (ابزار `cargo-hack`) همه‌ی ترکیب‌ها را تست می‌کند.

## Profile ها

Profile مجموعه‌ی تنظیمات کامپایلر است. دو profile اصلی: `dev` (برای `cargo build`) و `release` (برای `--release`)؛ علاوه بر `test` و `bench`. تنظیم پروژه‌ی نمونه:

```toml
[profile.release]
opt-level = 3          # سطح بهینه‌سازی: 0، 1، 2، 3، "s" (اندازه)، "z" (حداقل اندازه)
lto = "thin"           # Link-Time Optimization: false | "thin" | "fat"
codegen-units = 1      # کمتر = بهینه‌سازی بیشتر ولی کامپایل کندتر
panic = "abort"        # به‌جای unwinding برنامه را می‌بندد؛ باینری کوچک‌تر
strip = true           # حذف سمبل‌ها و اطلاعات debug از باینری

# وابستگی‌ها را حتی در حالت dev بهینه کن (کد خودتان debug می‌ماند)
[profile.dev.package."*"]
opt-level = 2
```

| تنظیم | اثر | هزینه |
|---|---|---|
| `opt-level = 3` | سریع‌ترین کد | کامپایل کندتر |
| `opt-level = "s"` / `"z"` | باینری کوچک‌تر | کمی کندتر |
| `lto = "fat"` | بهینه‌سازی بین همه‌ی crate ها | زمان لینک بسیار بیشتر |
| `lto = "thin"` | تعادل خوب | کمی بیشتر از بدون LTO |
| `codegen-units = 1` | امکان بهینه‌سازی بیشتر | build موازی‌تر نیست |
| `panic = "abort"` | باینری کوچک‌تر و کمی سریع‌تر | `catch_unwind` کار نمی‌کند؛ Drop در panic اجرا نمی‌شود |
| `strip = true` | باینری کوچک‌تر | stack trace ها بدون نام تابع |
| `debug = true` در release | برای profiling | باینری بزرگ‌تر |

برای نمونه با تنظیمات بالا و `APP_BUILD_TAG=v1 cargo build --release`، باینری `app` روی همین سیستم **حدود ۳۵۴ کیلوبایت** شد (مقدار تقریبی و وابسته به سیستم و نسخه‌ی کامپایلر).

> **قاعده:** قبل از بهینه‌سازی تنظیمات، **اندازه‌گیری** کنید. جزئیات در فصل [عملکرد](performance.md).

### Profile سفارشی

```toml
[profile.release-debug]
inherits = "release"
debug = true             # symbol ها برای perf و flamegraph
strip = false
```

```bash
cargo build --profile release-debug
```

## وابستگی‌ها

### انواع منبع

```toml
[dependencies]
serde = "1"                                         # از crates.io
textkit = { path = "../textkit" }                   # پوشه‌ی محلی
mylib = { git = "https://github.com/user/mylib", branch = "main" }
renamed = { package = "real-name", version = "2" }  # تغییر نام

[dev-dependencies]    # فقط برای تست، مثال و benchmark
serde_json = "1"

[build-dependencies]  # فقط برای build.rs
cc = "1"
```

### نسخه‌دهی و semver

در `Cargo.toml` نوشتن `"1.2.3"` یعنی `^1.2.3` یعنی «هر نسخه‌ی سازگار با 1.2.3» (`>=1.2.3, <2.0.0`):

| نوشتار | بازه |
|---|---|
| `"1.2.3"` یا `"^1.2.3"` | `>=1.2.3, <2.0.0` |
| `"0.2.3"` | `>=0.2.3, <0.3.0` (برای 0.x، تغییر minor ناسازگار است) |
| `"~1.2.3"` | `>=1.2.3, <1.3.0` |
| `"=1.2.3"` | دقیقاً همان |
| `"*"` | هر نسخه (در crates.io ممنوع است) |

قواعد semver: تغییر **major** = ناسازگار، **minor** = قابلیت جدید سازگار، **patch** = رفع باگ.

### Cargo.lock

`Cargo.lock` نسخه‌ی **دقیق** همه‌ی وابستگی‌ها را ثبت می‌کند تا build تکرارپذیر باشد.

- برای **برنامه‌های اجرایی**: حتماً commit کنید.
- برای **کتابخانه‌ها**: بحث قدیمی «نباید commit شود» بود؛ مستندات فعلی Cargo این را منعطف‌تر کرده است. در هر صورت، کاربران کتابخانه‌ی شما فایل lock شما را استفاده نمی‌کنند.

```bash
cargo update                    # همه به آخرین نسخه‌ی مجاز
cargo update -p serde           # فقط یکی
cargo update -p serde --precise 1.0.200   # نسخه‌ی دقیق
cargo tree                      # درخت وابستگی‌ها
cargo tree -d                   # فقط وابستگی‌های تکراری (نسخه‌های مختلف)
cargo tree -i serde             # چه کسی serde را می‌کشد؟
```

خروجی واقعی `cargo tree` پروژه‌ی نمونه (بخش اول):

```text
app v0.1.0 (.../crates/app)
├── serde_json v1.0.154
│   ├── itoa v1.0.18
│   ├── memchr v2.8.3
│   ├── serde_core v1.0.229
│   └── zmij v1.0.23
└── textkit v0.1.0 (.../crates/textkit)
    └── serde v1.0.229
        ├── serde_core v1.0.229
        └── serde_derive v1.0.229 (proc-macro)
```

### MSRV

`rust-version = "1.80"` در `[package]` کمترین نسخه‌ی Rust پشتیبانی‌شده را اعلام می‌کند؛ Cargo اگر کامپایلر قدیمی‌تر باشد خطای روشن می‌دهد و resolver 3 هنگام انتخاب نسخه‌ها آن را رعایت می‌کند.

## اسکریپت build (build.rs)

فایل `build.rs` در کنار `Cargo.toml` **قبل از** کامپایل crate اجرا می‌شود. کاربردها: تولید کد، کامپایل کد C (با crate ی `cc`)، پیدا کردن کتابخانه‌های سیستمی، تزریق اطلاعات build. نمونه‌ی پروژه:

```rust,ignore
// crates/app/build.rs
use std::process::Command;

fn main() {
    // فقط اگر build.rs یا این متغیر عوض شد دوباره اجرا شود
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=APP_BUILD_TAG");

    let tag = std::env::var("APP_BUILD_TAG").unwrap_or_else(|_| "dev".to_string());
    let rustc = Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into()))
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    // این متغیرها در زمان کامپایل به env!() در دسترس می‌شوند
    println!("cargo::rustc-env=BUILD_TAG={tag}");
    println!("cargo::rustc-env=BUILD_RUSTC={}", rustc.trim());
}
```

```rust,ignore
// crates/app/src/main.rs
println!("tag={} rustc={}", env!("BUILD_TAG"), env!("BUILD_RUSTC"));
```

اجرای واقعی:

```bash
echo "سلام دنیا
Rust" | cargo run -q
```

```text
{
  "chars": 15,
  "words": 3,
  "lines": 2
}
tag=dev rustc=rustc 1.97.0 (2d8144b78 2026-07-07)
```

(تعداد ۱۵ کاراکتر شامل کاراکترهای خط جدید است.) و با تگ دلخواه:

```bash
APP_BUILD_TAG=v1 cargo build --release
```

اجرای باینری ساخته‌شده، `tag=v1` چاپ می‌کند.

> **نکات:** خطوط `cargo::...` دستورهایی به Cargo هستند (قالب جدید با دو دونقطه؛ قالب قدیمی `cargo:` تک‌دونقطه بود). همیشه `rerun-if-changed` بدهید، وگرنه build.rs در هر build اجرا می‌شود. build.rs را سبک نگه دارید چون روی زمان کامپایل همه‌ی کاربران اثر دارد.

## Lints در Cargo.toml

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[workspace.lints.clippy]
unwrap_used = "warn"
pedantic = { level = "warn", priority = -1 }

# در هر crate:
# [lints]
# workspace = true
```

## ابزارهای مفید

برخی با Cargo می‌آیند، بقیه با `cargo install` نصب می‌شوند:

| دستور | کار | همراه Rust؟ |
|---|---|---|
| `cargo check` | بررسی سریع بدون ساخت | بله |
| `cargo fmt` / `cargo clippy` | قالب و lint | بله (با rustup) |
| `cargo doc --open` | ساخت و نمایش مستندات | بله |
| `cargo add` / `cargo remove` | مدیریت وابستگی در Cargo.toml | بله |
| `cargo tree` | درخت وابستگی | بله |
| `cargo fix` | اعمال پیشنهادهای کامپایلر؛ `cargo fix --edition` برای مهاجرت edition | بله |
| `cargo build --timings` | گزارش HTML زمان build | بله |
| `cargo expand` | نمایش خروجی macro ها | `cargo install cargo-expand` |
| `cargo audit` | بررسی آسیب‌پذیری‌های شناخته‌شده‌ی وابستگی‌ها (RustSec) | `cargo install cargo-audit` |
| `cargo deny` | مجوزها، منع crate ها، وابستگی‌های تکراری | `cargo install cargo-deny` |
| `cargo machete` | پیدا کردن وابستگی‌های بی‌استفاده | `cargo install cargo-machete` |
| `cargo nextest` | اجرای سریع‌تر و دقیق‌تر تست‌ها | `cargo install cargo-nextest` |
| `cargo llvm-cov` | گزارش پوشش کد | `cargo install cargo-llvm-cov` |
| `cargo hack` | تست ترکیب‌های feature | `cargo install cargo-hack` |
| `cargo outdated` | وابستگی‌های قدیمی | `cargo install cargo-outdated` |

> `cargo install` پیش‌فرض آخرین نسخه‌ی crate را می‌سازد. برای تکرارپذیری `cargo install --locked نام` را بزنید تا از `Cargo.lock` خود آن crate استفاده شود.

## انتشار در crates.io

مراحل انتشار یک کتابخانه:

۱. در [crates.io](https://crates.io) با GitHub وارد شوید و یک API token بسازید، سپس `cargo login`.
۲. متادیتای لازم در `Cargo.toml`:

```toml
[package]
name = "textkit"                       # باید یکتا باشد
version = "0.1.0"
edition = "2024"
description = "یک جمله‌ی توصیفی"        # الزامی
license = "MIT OR Apache-2.0"          # یا license-file؛ الزامی
repository = "https://github.com/you/textkit"
readme = "README.md"
keywords = ["text", "unicode"]         # حداکثر ۵
categories = ["text-processing"]
rust-version = "1.80"
```

۳. بررسی: `cargo package --list` (چه فایل‌هایی می‌رود)، `cargo publish --dry-run`.
۴. مستندات را بنویسید (`///` و `//!`) و `cargo doc` و `cargo test --doc` را اجرا کنید.
۵. انتشار: `cargo publish`.

نکته‌های مهم:

- **انتشار برگشت‌ناپذیر است**؛ نسخه‌ی منتشرشده هرگز حذف یا بازنویسی نمی‌شود.
- اگر نسخه‌ای باگ جدی دارد، `cargo yank --version 0.1.0` آن را برای **پروژه‌های جدید** غیرقابل‌انتخاب می‌کند (ولی پروژه‌هایی که با `Cargo.lock` به آن قفل‌اند همچنان کار می‌کنند).
- وابستگی‌های `path` برای انتشار باید `version` هم داشته باشند (مثل `textkit` در workspace ما).
- crate ای که `publish = false` دارد نمی‌تواند منتشر شود؛ این محافظ خوبی برای برنامه‌های داخلی است.

## پیکربندی Cargo: `.cargo/config.toml`

فایلی در `~/.cargo/config.toml` (برای همه) یا `.cargo/config.toml` داخل پروژه:

```toml
[alias]
t = "test --workspace"
c = "clippy --workspace --all-targets -- -D warnings"

[build]
# target-dir = "/tmp/my-target"      # پوشه‌ی build مشترک

[env]
APP_BUILD_TAG = "ci"

[target.x86_64-unknown-linux-gnu]
# linker = "clang"
# rustflags = ["-C", "link-arg=-fuse-ld=lld"]   # لینکر سریع‌تر
```

سپس `cargo t` همان `cargo test --workspace` است.

## Cross-compile

```bash
rustup target add x86_64-unknown-linux-musl     # باینری کاملاً static در لینوکس
cargo build --release --target x86_64-unknown-linux-musl

rustup target list                              # همه‌ی هدف‌ها
```

نکات: برای هدف‌های دیگر معمولاً به **لینکر** مناسب هم نیاز دارید (در `config.toml` تنظیم می‌شود). ابزار `cross` (مبتنی بر Docker) این کار را ساده می‌کند. برای هدف WebAssembly به فصل [FFI، WASM و Embedded](ffi-wasm-embedded.md) نگاه کنید.

## تسریع build

- `cargo check` در حین توسعه.
- لینکر سریع‌تر (`lld` یا `mold`).
- `[profile.dev.package."*"] opt-level = 2` برای وابستگی‌ها، تا اجرای dev کند نباشد.
- `sccache` برای کش کامپایل بین پروژه‌ها.
- کم کردن feature های وابستگی‌ها (`default-features = false`).
- `cargo build --timings` برای دیدن گلوگاه.
- تقسیم crate های بزرگ به workspace.

## جمع‌بندی

| نیاز | ابزار |
|---|---|
| چند crate هماهنگ | workspace + `[workspace.dependencies]` |
| قابلیت اختیاری | `[features]` + `cfg` (به‌خاطر unification افزایشی طراحی کنید) |
| بهینه‌سازی | `[profile.*]`، اول اندازه‌گیری |
| کد تولیدشده/اطلاعات build | `build.rs` |
| کیفیت | clippy، fmt، audit، deny، nextest، hack |
| انتشار | `cargo publish` (برگشت‌ناپذیر) + `yank` |
| میان‌بر | `[alias]` در `.cargo/config.toml` |

## تمرین

1. در workspace نمونه یک crate سوم به نام `cli-utils` اضافه کنید که از `textkit` استفاده کند، و `cargo tree -i textkit` را اجرا کنید.
2. فیچر `unicode-words` به `textkit` اضافه کنید که شمارش کلمه را با `unicode-segmentation` انجام دهد؛ سپس با `cargo test -p textkit --features unicode-words` تست کنید.
3. با `cargo build --timings` گزارش build را بسازید و کندترین crate را پیدا کنید.
4. profile ی بسازید که اندازه‌ی باینری را حداقل کند (`opt-level = "z"`، `lto = "fat"`، `strip`) و اندازه را با تنظیمات پیش‌فرض مقایسه کنید.
5. `cargo publish --dry-run` را روی `textkit` اجرا کنید و خطاهای متادیتا را رفع کنید.
