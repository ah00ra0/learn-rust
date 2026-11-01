# Async و await

برنامه‌نویسی **async** برای کارهایی است که بیشتر وقتشان صرف **انتظار** می‌شود: درخواست شبکه، خواندن از دیسک، پایگاه‌داده. به‌جای اینکه برای هر کار یک thread سنگین بسازیم، هزاران کار را روی تعداد کمی thread مدیریت می‌کنیم.

## Thread در برابر Async

| | Thread | Async |
|---|---|---|
| مناسب | کار سنگین CPU، موازی‌سازی | کارهای I/O زیاد (سرور وب و ...) |
| هزینه‌ی هر واحد | حافظه‌ی نسبتاً زیاد (stack جدا) | بسیار کم |
| مثال | پردازش تصویر | سرور با ۱۰۰ هزار اتصال همزمان |

## Future

در Rust، تابع `async` فوراً اجرا نمی‌شود. یک **Future** برمی‌گرداند؛ مقداری که "در آینده" آماده می‌شود. Future ها **تنبل** هستند و تا `await` یا poll نشوند کاری نمی‌کنند:

```rust
async fn compute() -> u32 {
    42
}

fn main() {
    let _future = compute();   // هنوز هیچ کاری نشده
    println!("future ساخته شد ولی اجرا نشد");
}
```

trait ی ساده‌شده‌ی `Future`:

```rust,ignore
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

مقدار `Poll` یا `Ready(مقدار)` یا `Pending` است.

## await

داخل تابع `async` با `.await` منتظر Future دیگر می‌مانیم، بدون اینکه thread را مسدود کنیم:

```rust,ignore
async fn fetch_user(id: u32) -> String {
    format!("user-{id}")
}

async fn show() {
    let user = fetch_user(1).await;
    println!("{user}");
}
```

## Runtime

کتابخانه‌ی استاندارد **runtime ندارد**؛ یعنی چیزی که Future ها را اجرا و زمان‌بندی کند در خود زبان نیست. باید یک crate انتخاب کنید. محبوب‌ترین: **Tokio**. گزینه‌های دیگر: `async-std` و `smol`.

```bash
cargo add tokio --features full
```

```rust,ignore
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("شروع");
    sleep(Duration::from_millis(500)).await;
    println!("بعد از نیم ثانیه");
}
```

ماکروی `#[tokio::main]` تابع `main` را به یک runtime وصل می‌کند.

## اجرای همزمان چند کار

```rust,ignore
use std::time::Duration;
use tokio::time::sleep;

async fn task(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} تمام شد")
}

#[tokio::main]
async fn main() {
    // پشت سر هم: ۳۰۰ + ۲۰۰ میلی‌ثانیه
    let a = task("الف", 300).await;
    let b = task("ب", 200).await;
    println!("{a} / {b}");

    // همزمان: فقط ۳۰۰ میلی‌ثانیه (طولانی‌ترین)
    let (a, b) = tokio::join!(task("الف", 300), task("ب", 200));
    println!("{a} / {b}");

    // ساخت taskهای مستقل (مثل thread سبک)
    let h1 = tokio::spawn(async { task("ج", 100).await });
    let h2 = tokio::spawn(async { task("د", 100).await });
    println!("{} / {}", h1.await.unwrap(), h2.await.unwrap());
}
```

### Timeout و انتخاب اولین نتیجه

```rust,ignore
use std::time::Duration;
use tokio::time::{sleep, timeout};

#[tokio::main]
async fn main() {
    let result = timeout(Duration::from_millis(100), sleep(Duration::from_secs(5))).await;
    println!("{}", result.is_err());   // true: زمان تمام شد

    tokio::select! {
        _ = sleep(Duration::from_millis(50)) => println!("اولی برنده شد"),
        _ = sleep(Duration::from_millis(500)) => println!("دومی برنده شد"),
    }
}
```

## نکات مهم

- **کار سنگین CPU را داخل async انجام ندهید.** thread ی که runtime روی آن است مسدود می‌شود. از `tokio::task::spawn_blocking` استفاده کنید.
- **Mutex ی استاندارد را هنگام `.await` نگه ندارید.** از `tokio::sync::Mutex` استفاده کنید.
- Future ها باید `Send` باشند تا بین thread ها جابه‌جا شوند. `Rc` داخل async معمولاً مشکل می‌سازد.
- Cancellation: با رها کردن (drop) یک Future، اجرای آن متوقف می‌شود. این قدرتمند است ولی باید حواستان به حالت میانی باشد.
- هر دو سبک (thread و async) را می‌شود ترکیب کرد.

## مثال واقعی: درخواست HTTP

```rust,ignore
// Cargo.toml: reqwest = "0.12", tokio = { version = "1", features = ["full"] }
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
    println!("{} بایت", body.len());
    Ok(())
}
```

## تمرین

1. با Tokio برنامه‌ای بنویسید که ۱۰ taskِ همزمان با `sleep` ی تصادفی بسازد و نتیجه‌ی هر کدام را چاپ کند.
2. با `timeout` درخواستی را بعد از ۲ ثانیه لغو کنید.
3. تفاوت زمان اجرا بین `await` ی پشت سر هم و `join!` را اندازه بگیرید.

فصل بعد: [Unsafe Rust](27-unsafe.md).
