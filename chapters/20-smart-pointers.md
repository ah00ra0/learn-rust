# Smart Pointer: Box، Rc و RefCell

**Pointer** یعنی متغیری که آدرس حافظه را نگه می‌دارد. reference ها (`&T`) ساده‌ترین نوع pointer در Rust هستند و فقط **قرض** می‌گیرند. **Smart Pointer** ها ساختارهایی هستند که مثل pointer رفتار می‌کنند اما **متادیتای اضافه** و **قابلیت‌های بیشتر** دارند و معمولاً **مالک** داده‌اند. `String` و `Vec` هم از نظر فنی smart pointer هستند.

دو trait اصلی پشت آن‌ها:

- `Deref`: اجازه می‌دهد با `*` به مقدار داخلی برسیم و مثل reference رفتار کنند.
- `Drop`: کد پاک‌سازی هنگام خروج از scope.

## Box&lt;T&gt;: ذخیره روی Heap

ساده‌ترین smart pointer. مقدار را روی heap می‌گذارد و فقط یک pointer روی stack می‌ماند:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    println!("{}", *b + 1);
}
```

### کاربرد ۱: انواع بازگشتی

نوعی که خودش را شامل می‌شود اندازه‌ی بی‌نهایت دارد و کامپایل نمی‌شود:

```rust,compile_fail
enum List {
    Cons(i32, List),   // ✗ خطا: نوع بازگشتی با اندازه‌ی نامحدود
    Nil,
}
```

با `Box` اندازه‌ی ثابت می‌شود (فقط یک pointer):

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
}
```

### کاربرد ۲: داده‌ی بزرگ و انتقال مالکیت بدون کپی

### کاربرد ۳: trait object ها (فصل ۲۴)

```rust
trait Speak {
    fn speak(&self) -> String;
}
struct Cat;
struct Dog;
impl Speak for Cat { fn speak(&self) -> String { "میو".into() } }
impl Speak for Dog { fn speak(&self) -> String { "هاپ".into() } }

fn main() {
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Cat), Box::new(Dog)];
    for a in &animals {
        println!("{}", a.speak());
    }
}
```

## Deref

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("سلام {name}");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("راست"));
    hello(&m);   // &MyBox<String> → &String → &str  (Deref Coercion)
}
```

**Deref Coercion**: کامپایلر به‌طور خودکار `&MyBox<String>` را به `&String` و سپس `&str` تبدیل می‌کند. به همین دلیل می‌توانید `&String` را جایی که `&str` لازم است بدهید.

## Rc&lt;T&gt;: مالکیت چندگانه

گاهی یک مقدار باید **چند مالک** داشته باشد (مثل گره‌ای در گراف که چند یال به آن می‌رسد). `Rc` (Reference Counted) تعداد مالکان را می‌شمارد و وقتی صفر شد، مقدار را آزاد می‌کند:

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("مشترک"));
    println!("تعداد: {}", Rc::strong_count(&a)); // 1

    let b = Rc::clone(&a);    // فقط شمارنده زیاد می‌شود؛ داده کپی نمی‌شود
    println!("تعداد: {}", Rc::strong_count(&a)); // 2

    {
        let c = Rc::clone(&a);
        println!("تعداد: {} {c}", Rc::strong_count(&a)); // 3
    }
    println!("تعداد: {}", Rc::strong_count(&a)); // 2
    println!("{b}");
}
```

نکات:

- `Rc` فقط برای **تک‌نخی** (single-thread) است. برای چند‌نخی از `Arc` استفاده کنید (فصل ۲۱).
- با `Rc` فقط **دسترسی خواندنی** دارید.
- قرارداد: به‌جای `a.clone()` بنویسید `Rc::clone(&a)` تا معلوم باشد کپی ارزان است.

## RefCell&lt;T&gt; و Interior Mutability

قوانین borrowing معمولاً در **زمان کامپایل** بررسی می‌شوند. `RefCell<T>` همان قوانین را در **زمان اجرا** بررسی می‌کند. این به شما اجازه می‌دهد از طریق reference غیرقابل‌تغییر، داده را تغییر دهید (**Interior Mutability**):

```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1, 2, 3]);

    cell.borrow_mut().push(4);        // قرض قابل‌تغییر
    println!("{:?}", cell.borrow());  // قرض غیرقابل‌تغییر
}
```

اگر قوانین را نقض کنید، برنامه **panic** می‌کند (نه خطای کامپایل):

```rust,should_panic
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(5);
    let _a = cell.borrow_mut();
    let _b = cell.borrow_mut(); // panic: already borrowed
}
```

`Cell<T>` گزینه‌ی سبک‌تر برای انواع `Copy` است:

```rust
use std::cell::Cell;

fn main() {
    let c = Cell::new(10);
    c.set(c.get() + 1);
    println!("{}", c.get());
}
```

## ترکیب Rc و RefCell

الگوی رایج برای **داده‌ی مشترک و قابل‌تغییر** در برنامه‌ی تک‌نخی:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Account {
    balance: i32,
}

fn main() {
    let shared = Rc::new(RefCell::new(Account { balance: 100 }));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    a.borrow_mut().balance += 50;
    b.borrow_mut().balance -= 30;

    println!("{:?}", shared.borrow());
}
```

## Weak&lt;T&gt; و چرخه‌ی مرجع

اگر دو `Rc` به هم اشاره کنند، شمارنده هیچ‌وقت صفر نمی‌شود و حافظه **نشت** می‌کند (Rust نشت را ممنوع نمی‌کند، فقط ایمن نگهش می‌دارد). برای شکستن چرخه از `Weak` استفاده کنید که مالکیت نمی‌دهد:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("پدر برگ: {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("پدر برگ: {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    println!("strong={} weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("{}", branch.children.borrow().len() + leaf.value as usize);
}
```

## چه زمانی کدام؟

| نوع | مالکان | قابل‌تغییر | بررسی قوانین |
|---|---|---|---|
| `Box<T>` | یک | بله (با `mut`) | کامپایل |
| `Rc<T>` | چند | خیر | کامپایل |
| `RefCell<T>` | یک | بله | **اجرا** |
| `Rc<RefCell<T>>` | چند | بله | **اجرا** |
| `Arc<Mutex<T>>` | چند، چند‌نخی | بله | اجرا (قفل) |

> **توصیه:** اگر مجبور نیستید از `Rc<RefCell<T>>` استفاده نکنید. معمولاً طراحی بهتری با ownership ساده وجود دارد.

## تمرین

1. لیست پیوندی یک‌طرفه با `Box` بنویسید که متدهای `push` و `sum` داشته باشد.
2. با `Rc` یک متن را بین سه متغیر به اشتراک بگذارید و شمارنده را چاپ کنید.
3. struct ی بسازید که با `RefCell` تعداد فراخوانی یک متد `&self` را بشمارد.

فصل بعد: [Concurrency](21-concurrency.md).
