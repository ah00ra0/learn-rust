# Concurrency: Thread، Channel و Mutex

یکی از وعده‌های Rust **Fearless Concurrency** یا "همزمانی بدون ترس" است: بسیاری از باگ‌های همزمانی (مثل data race) در Rust **خطای کامپایل** هستند، نه باگ نهفته در زمان اجرا.

> مدل‌های ownership و type system که برای ایمنی حافظه ساخته شدند، همان ابزارهایی هستند که ایمنی همزمانی را هم تضمین می‌کنند.

## ساخت Thread

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..4 {
            println!("thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("main: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();   // صبر تا پایان thread
}
```

- `thread::spawn` یک thread سیستم‌عامل می‌سازد.
- اگر `main` تمام شود، همه‌ی thread ها بسته می‌شوند. برای صبر کردن از `join` استفاده کنید.
- ترتیب اجرا قطعی نیست.

### move با thread

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{v:?}");
    });

    // println!("{v:?}"); // ✗ v به thread منتقل شد
    handle.join().unwrap();
}
```

بدون `move`، کامپایلر خطا می‌دهد چون ممکن است thread بیشتر از `v` عمر کند.

### مقدار برگشتی

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| (1..=100).sum::<u32>());
    let total = handle.join().unwrap();
    println!("{total}");
}
```

## Scoped Thread

اگر مطمئنید thread ها قبل از پایان scope تمام می‌شوند، می‌توانید **بدون move و بدون Arc**، داده‌ی محلی را قرض بدهید:

```rust
use std::thread;

fn main() {
    let mut nums = vec![1, 2, 3, 4];
    let (left, right) = nums.split_at_mut(2);

    thread::scope(|s| {
        s.spawn(|| left.iter_mut().for_each(|x| *x *= 10));
        s.spawn(|| right.iter_mut().for_each(|x| *x += 1));
    }); // اینجا خودکار join می‌شوند

    println!("{nums:?}");
}
```

## Message Passing با Channel

شعار Go: "با اشتراک حافظه ارتباط نگیرید؛ با ارتباط، حافظه را به اشتراک بگذارید." Rust هم channel دارد:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();   // mpsc: multiple producer, single consumer

    thread::spawn(move || {
        let msg = String::from("سلام از thread");
        tx.send(msg).unwrap();
        // println!("{msg}"); // ✗ مالکیت با send منتقل شد
    });

    let received = rx.recv().unwrap();   // مسدودکننده
    println!("{received}");
}
```

### چند پیام و چند فرستنده

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for i in 0..3 {
            tx.send(format!("الف{i}")).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 0..3 {
            tx2.send(format!("ب{i}")).unwrap();
        }
    });

    let mut all: Vec<String> = rx.iter().collect(); // تا وقتی همه‌ی فرستنده‌ها بسته شوند
    all.sort();
    println!("{all:?}");
}
```

متدها:

- `recv()`: منتظر می‌ماند.
- `try_recv()`: بدون انتظار، فوراً `Result` برمی‌گرداند.
- با پیمایش `for x in rx` تا بسته شدن کانال خوانده می‌شود.

## حافظه‌ی مشترک: Mutex

**Mutex** (Mutual Exclusion) اجازه می‌دهد در هر لحظه فقط یک thread به داده دسترسی داشته باشد:

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();   // قفل می‌گیرد
        *num = 6;
    } // اینجا قفل خودکار آزاد می‌شود (Drop)

    println!("{m:?}");
}
```

در Rust داده **داخل** Mutex است؛ بدون قفل گرفتن، راهی برای دسترسی به داده وجود ندارد. در C++ می‌شود فراموش کرد قفل بگیرید، ولی اینجا غیرممکن است.

### به اشتراک گذاشتن Mutex بین thread ها: Arc

`Rc` برای چند‌نخی ایمن نیست و کامپایلر اجازه نمی‌دهد. **`Arc`** (Atomic Reference Counted) نسخه‌ی thread-safe آن است:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("نتیجه: {}", *counter.lock().unwrap()); // 10
}
```

`Arc<Mutex<T>>` معادل `Rc<RefCell<T>>` در دنیای چند‌نخی است.

### RwLock

اگر خواندن زیاد و نوشتن کم است، `RwLock` چند خواننده‌ی همزمان یا یک نویسنده را مجاز می‌کند:

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("{} {}", *r1, *r2);
    }
    *lock.write().unwrap() += 1;
    println!("{:?}", lock.read().unwrap());
}
```

### Atomic ها

برای شمارنده‌ها و پرچم‌های ساده، Atomic ها از Mutex سریع‌ترند:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    c.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    println!("{}", counter.load(Ordering::SeqCst)); // 4000
}
```

## Send و Sync

دو trait نشانه (marker) که ایمنی چند‌نخی را تضمین می‌کنند:

- **`Send`**: مالکیت مقدار می‌تواند به thread دیگر منتقل شود.
- **`Sync`**: چند thread می‌توانند همزمان `&T` داشته باشند.

تقریباً همه‌ی انواع این دو را خودکار دارند. استثناها: `Rc` (نه Send نه Sync)، `RefCell` (نه Sync). کامپایلر اگر آن‌ها را به thread بدهید خطا می‌دهد:

```rust,compile_fail
use std::rc::Rc;
use std::thread;

fn main() {
    let rc = Rc::new(5);
    thread::spawn(move || {       // ✗ خطا: Rc نمی‌تواند بین thread ها ارسال شود
        println!("{rc}");
    });
}
```

## Deadlock

Rust از data race جلوگیری می‌کند ولی **از deadlock نه**. اگر دو thread دو قفل را به ترتیب مخالف بگیرند، هر دو برای همیشه منتظر می‌مانند. راه‌حل: همیشه قفل‌ها را به یک ترتیب ثابت بگیرید، قفل را کوتاه نگه دارید و از channel استفاده کنید.

## موازی‌سازی داده با Rayon

برای کارهای رایج، crate محبوب **rayon** پیمایش موازی را با یک تغییر کوچک ممکن می‌کند:

```rust,ignore
use rayon::prelude::*;

fn main() {
    let sum: u64 = (1..=1_000_000u64).into_par_iter().map(|x| x * x).sum();
    println!("{sum}");
}
```

## تمرین

1. ۴ thread بسازید که هر کدام مجموع بخشی از یک آرایه‌ی بزرگ را حساب کنند و نتایج را با channel برای `main` بفرستند.
2. شمارنده‌ای با `Arc<Mutex<i32>>` بسازید که ۱۰۰ thread هر کدام ۱۰۰ بار آن را زیاد کنند.
3. همان کار را با `AtomicUsize` انجام دهید و سرعت را مقایسه کنید.
4. سعی کنید `Rc` را به thread بدهید و خطای کامپایلر را بخوانید.

فصل بعد: [Pattern Matching به‌طور کامل](22-patterns.md).
