# کتاب آشپزی خطاهای کامپایلر

بیشتر وقتی که مبتدی‌ها با Rust درگیرند، صرف خواندن خطاهای کامپایلر می‌شود. خبر خوب: این خطاها **تعداد محدودی الگوی تکراری** دارند. در این فصل ۲۹ خطای رایج را می‌بینید. برای هر کدام:

1. کدی که خطا می‌دهد
2. **پیغام واقعی کامپایلر** (خط اول خطا، از Rust 1.97 با edition 2024)
3. توضیح اینکه کامپایلر چه چیزی را محافظت می‌کند
4. راه‌حل (کامپایل‌شده و تست‌شده)

> **نکته:** هر مثال خطادار در این فصل با برچسب `compile_fail` و **کد دقیق خطا** (مثلاً `E0382`) نوشته شده است. یعنی تست‌های کتاب بررسی می‌کنند که همان کد خطا واقعاً تولید می‌شود. برای توضیح رسمی هر کد می‌توانید `rustc --explain E0382` را اجرا کنید.

## چطور پیغام کامپایلر را بخوانیم؟

```text
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:4:16
  |
2 |     let s = String::from("hi");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let t = s;
  |             - value moved here
4 |     println!("{s} {t}");
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let t = s.clone();
  |              ++++++++
```

۱. **خط اول**: کد و خلاصه‌ی خطا.
۲. **فلش `-->`**: فایل و شماره‌ی خط.
۳. **برچسب‌های زیر کد**: داستان ماجرا را به ترتیب تعریف می‌کنند ("اینجا ساخته شد"، "اینجا move شد"، "اینجا دوباره استفاده شد").
۴. **`help:`** در پایان: اغلب پیشنهاد دقیق اصلاح.

قاعده‌ی طلایی: **اولین خطا را درست کنید و دوباره کامپایل کنید.** خطاهای بعدی معمولاً پیامد همان اولی هستند.

---

## گروه ۱: Ownership و Move

### E0382: استفاده از مقدار منتقل‌شده

`error[E0382]: borrow of moved value: 's'`

```rust,compile_fail,E0382
fn main() {
    let s = String::from("hi");
    let t = s;                 // مالکیت به t رفت
    println!("{s} {t}");       // ✗ s دیگر معتبر نیست
}
```

**چرا؟** `String` نوع Copy نیست؛ انتساب آن مالکیت را منتقل می‌کند (فصل [Ownership](07-ownership.md)).

**راه‌حل:** یا قرض بگیرید، یا clone کنید، یا ساختار را طوری عوض کنید که فقط یک مالک لازم باشد.

```rust
fn main() {
    let s = String::from("hi");
    let t = &s;                // قرض، نه انتقال
    println!("{s} {t}");

    let u = s.clone();         // کپی صریح (فقط اگر واقعاً دو مالک لازم است)
    println!("{s} {u}");
}
```

**نوع شایع دیگر:** move داخل حلقه. 

```rust,compile_fail,E0382
fn consume(_s: String) {}

fn main() {
    let s = String::from("hi");
    for _ in 0..2 {
        consume(s);            // ✗ دور دوم s دیگر وجود ندارد
    }
}
```

راه‌حل: `consume(s.clone())`، یا تابع را طوری بنویسید که `&str` بگیرد.

### E0505: انتقال مقداری که هنوز قرض داده شده

`error[E0505]: cannot move out of 's' because it is borrowed`

```rust,compile_fail,E0505
fn consume(_s: String) {}

fn main() {
    let s = String::from("hi");
    let r = &s;
    consume(s);                // ✗ r هنوز زنده است
    println!("{r}");
}
```

**راه‌حل:** استفاده از `r` را قبل از انتقال تمام کنید.

```rust
fn consume(_s: String) {}

fn main() {
    let s = String::from("hi");
    let r = &s;
    println!("{r}");           // آخرین استفاده از r
    consume(s);                // حالا مشکلی نیست (NLL)
}
```

### E0507: خارج کردن مقدار از پشت reference

`error[E0507]: cannot move out of 'r.name' which is behind a shared reference`

```rust,compile_fail,E0507
struct P { name: String }

fn main() {
    let p = P { name: String::from("a") };
    let r = &p;
    let n = r.name;            // ✗ نمی‌توان مالکیت را از پشت & گرفت
    println!("{n}");
}
```

**چرا؟** `r` فقط قرض است. اگر `name` را بردارید، `p` ناقص می‌شود.

**راه‌حل:**

```rust
struct P { name: String }

fn main() {
    let p = P { name: String::from("a") };
    let r = &p;
    let n = &r.name;           // قرض بگیرید
    let m = r.name.clone();    // یا کپی کنید
    println!("{n} {m}");

    // اگر مالکیت p را دارید می‌توانید بردارید:
    let owned = p.name;
    println!("{owned}");
}
```

برای `Option` داخل struct از `.take()` یا `.as_ref()` استفاده کنید.

### E0381: استفاده از متغیر مقداردهی‌نشده

`error[E0381]: used binding 'x' isn't initialized`

```rust,compile_fail,E0381
fn main() {
    let x: i32;
    println!("{x}");           // ✗
}
```

**راه‌حل:** مقدار بدهید. Rust اجازه می‌دهد اعلام و مقداردهی جدا باشد، به شرطی که **در همه‌ی مسیرها** مقدار بگیرد:

```rust
fn main() {
    let x: i32;
    if std::env::args().count() > 5 {
        x = 1;
    } else {
        x = 2;
    }
    println!("{x}");
}
```

### E0384: انتساب دوباره به متغیر تغییرناپذیر

`error[E0384]: cannot assign twice to immutable variable 'x'`

```rust,compile_fail,E0384
fn main() {
    let x = 1;
    x = 2;                     // ✗
    println!("{x}");
}
```

**راه‌حل:** `let mut x = 1;` یا shadowing (`let x = 2;`). فصل [متغیرها](03-variables.md).

---

## گروه ۲: Borrowing

### E0499: دو قرض قابل‌تغییر همزمان

`error[E0499]: cannot borrow 's' as mutable more than once at a time`

```rust,compile_fail,E0499
fn main() {
    let mut s = String::from("hi");
    let a = &mut s;
    let b = &mut s;            // ✗ a هنوز زنده است
    a.push('x');
    b.push('y');
}
```

**چرا؟** دو نویسنده‌ی همزمان یعنی احتمال data race و باطل شدن pointer ها.

**راه‌حل:** بازه‌ی زندگی قرض‌ها را جدا کنید.

```rust
fn main() {
    let mut s = String::from("hi");
    {
        let a = &mut s;
        a.push('x');
    }                          // a تمام شد
    let b = &mut s;
    b.push('y');
    println!("{s}");
}
```

**حالت پرتکرار:** دو عنصر از یک `Vec` را همزمان تغییر دادن. راه‌حل: `split_at_mut` یا `swap`.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4];
    let (left, right) = v.split_at_mut(2);
    left[0] += right[0];
    v.swap(1, 3);
    println!("{v:?}");
}
```

### E0502: قرض قابل‌تغییر وقتی قرض خواندنی زنده است

`error[E0502]: cannot borrow 'v' as mutable because it is also borrowed as immutable`

```rust,compile_fail,E0502
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);                 // ✗ push ممکن است حافظه را جابه‌جا کند
    println!("{first}");
}
```

**راه‌حل‌ها:** مقدار را کپی کنید (برای انواع Copy)، یا ترتیب دستورها را عوض کنید.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = v[0];          // کپی عدد، نه reference
    v.push(4);
    println!("{first}");
}
```

**حالت پرتکرار:** پیمایش یک مجموعه و تغییر همان مجموعه داخل حلقه.

```rust,compile_fail,E0502
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        if *x == 2 {
            v.push(10);        // ✗ v در حال پیمایش است
        }
    }
}
```

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let to_add: Vec<i32> = v.iter().filter(|&&x| x == 2).map(|_| 10).collect();
    v.extend(to_add);          // اول جمع کنید، بعد تغییر بدهید
    v.retain(|&x| x != 1);     // یا از متدهای درجا استفاده کنید
    println!("{v:?}");
}
```

### E0506: انتساب به مقداری که قرض داده شده

`error[E0506]: cannot assign to 'x' because it is borrowed`

```rust,compile_fail,E0506
fn main() {
    let mut x = 5;
    let r = &x;
    x = 6;                     // ✗ r هنوز به x اشاره می‌کند
    println!("{r}");
}
```

راه‌حل: استفاده از `r` را قبل از انتساب تمام کنید، یا `let r = x;` (کپی).

### E0596: قرض قابل‌تغییر از چیزی که `mut` نیست

`error[E0596]: cannot borrow 'v' as mutable, as it is not declared as mutable`

```rust,compile_fail,E0596
fn main() {
    let v = Vec::new();
    v.push(1);                 // ✗
}
```

**راه‌حل:** `let mut v = Vec::new();`. همین خطا برای پارامتر تابع هم می‌آید: اگر تابع باید داده را تغییر دهد، پارامتر را `&mut Vec<i32>` بگیرید و هنگام فراخوانی `&mut v` بدهید.

> **نکته:** closure ای که مقدار capture شده را تغییر می‌دهد، خودش هم باید `mut` باشد (`let mut c = || s.push('a');`)، و کامپایلر برای هر دو جداگانه خطای E0596 می‌دهد.

### E0716: مقدار موقت قبل از استفاده می‌میرد

`error[E0716]: temporary value dropped while borrowed`

```rust,compile_fail,E0716
fn main() {
    let r: &str = String::from("hi").as_str();   // ✗ String موقتی در پایان همین خط می‌میرد
    println!("{r}");
}
```

**راه‌حل:** مقدار را در متغیر نگه دارید تا مالک داشته باشد.

```rust
fn main() {
    let owned = String::from("hi");
    let r: &str = owned.as_str();
    println!("{r}");
}
```

### E0597: مقدار به‌اندازه‌ی کافی عمر نمی‌کند

`error[E0597]: 'x' does not live long enough`

```rust,compile_fail,E0597
fn main() {
    let r;
    {
        let x = 5;
        r = &x;                // ✗ x در پایان این بلوک می‌میرد
    }
    println!("{r}");
}
```

**راه‌حل:** `x` را در scope بیرونی تعریف کنید، یا مقدار را کپی/move کنید.

```rust
fn main() {
    let x = 5;
    let r = &x;
    println!("{r}");
}
```

### E0515: برگرداندن reference به متغیر محلی

`error[E0515]: cannot return reference to local variable 's'`

```rust,compile_fail,E0515
fn make() -> &'static str {
    let s = String::from("hi");
    &s                         // ✗ s با پایان تابع می‌میرد
}

fn main() { make(); }
```

**راه‌حل:** مالکیت را برگردانید.

```rust
fn make() -> String {
    String::from("hi")
}

fn main() { println!("{}", make()); }
```

### E0373: closure ممکن است بیشتر از متغیرش عمر کند

`error[E0373]: closure may outlive the current function, but it borrows 'v', which is owned by the current function`

```rust,compile_fail,E0373
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let h = thread::spawn(|| println!("{v:?}"));   // ✗
    h.join().unwrap();
}
```

**چرا؟** thread ممکن است بعد از پایان `main` هم اجرا شود؛ پس نمی‌تواند به `v` قرض بدهد.

**راه‌حل:** `move` (فصل [Closure](18-closures.md) و [Concurrency](21-concurrency.md)).

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let h = thread::spawn(move || println!("{v:?}"));
    h.join().unwrap();
}
```

### E0521: داده‌ی قرض‌گرفته از تابع بیرون می‌زند

`error[E0521]: borrowed data escapes outside of function`

```rust,compile_fail,E0521
use std::thread;

fn show(s: &str) {
    thread::spawn(move || println!("{s}"));   // ✗ s فقط تا پایان show معتبر است
}

fn main() {}
```

**راه‌حل:** داده‌ی دارای مالک بفرستید، یا از `thread::scope` استفاده کنید.

```rust
use std::thread;

fn show(s: &str) {
    let owned = s.to_string();
    thread::spawn(move || println!("{owned}")).join().unwrap();
}

fn show_scoped(s: &str) {
    thread::scope(|sc| {
        sc.spawn(|| println!("{s}"));
    });
}

fn main() {
    show("a");
    show_scoped("b");
}
```

---

## گروه ۳: Lifetime

### E0106: lifetime مشخص نشده

`error[E0106]: missing lifetime specifier`

```rust,compile_fail,E0106
fn longest(a: &str, b: &str) -> &str {      // ✗ خروجی از a است یا b؟
    if a.len() > b.len() { a } else { b }
}

fn main() {}
```

**راه‌حل:** رابطه را بنویسید (فصل [Lifetime](17-lifetimes.md)).

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longest("abc", "de"));
}
```

اگر نمی‌خواهید با lifetime درگیر شوید، `String` برگردانید.

---

## گروه ۴: نوع‌ها و Trait ها

### E0308: نوع‌ها نمی‌خوانند

`error[E0308]: mismatched types`

```rust,compile_fail,E0308
fn main() {
    let x: i32 = "5";          // ✗ انتظار i32، گرفت &str
    println!("{x}");
}
```

**راه‌حل:** تبدیل صریح.

```rust
fn main() {
    let x: i32 = "5".parse().unwrap();
    println!("{x}");
}
```

**حالت پرتکرار:** `;` اضافه در انتهای تابع، که بدنه را `()` می‌کند (فصل [توابع](05-functions.md))، یا شاخه‌های `if`/`match` با نوع‌های متفاوت.

### E0277: trait لازم پیاده‌سازی نشده

`error[E0277]: cannot add 'f64' to 'i32'`

```rust,compile_fail,E0277
fn main() {
    let x = 1_i32 + 1.5_f64;   // ✗ Rust تبدیل خودکار ندارد
    println!("{x}");
}
```

```rust
fn main() {
    let x = f64::from(1_i32) + 1.5_f64;
    println!("{x}");
}
```

E0277 خانواده‌ی بزرگی است؛ هر وقت نوعی trait مورد نیاز را ندارد این خطا می‌آید. نمونه‌ها:

- `` `Foo` doesn't implement `Debug` `` ← `#[derive(Debug)]`
- `` the trait bound `T: Display` is not satisfied `` ← قید را به generic اضافه کنید: `T: Display`
- `` `Rc<...>` cannot be sent between threads safely `` ← `Arc` بگیرید
- `` the size for values of type `str` cannot be known `` ← از `&str` یا `String` استفاده کنید

```rust,compile_fail,E0277
struct Point { x: i32 }

fn main() {
    let p = Point { x: 1 };
    println!("{p:?}");         // ✗ Point، Debug ندارد
}
```

```rust
#[derive(Debug)]
struct Point { x: i32 }

fn main() {
    let p = Point { x: 1 };
    println!("{p:?}");
}
```

### E0282: کامپایلر نمی‌تواند نوع را حدس بزند

`error[E0282]: type annotations needed for 'Vec<_>'`

```rust,compile_fail,E0282
fn main() {
    let v = Vec::new();        // ✗ عنصرها از چه نوعی‌اند؟
}
```

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    let w = Vec::<u8>::new();         // یا turbofish
    let n = "5".parse::<i32>().unwrap();
    println!("{v:?} {w:?} {n}");
}
```

اغلب با `.collect()` و `.parse()` و `.into()` دیده می‌شود. نوع مقصد را بنویسید.

### E0369: عملگر روی نوع تعریف نشده

`error[E0369]: cannot add 'P' to 'P'`

```rust,compile_fail,E0369
struct P;

fn main() {
    let _ = P + P;             // ✗
}
```

**راه‌حل:** trait مربوط را پیاده کنید (`std::ops::Add`) یا `#[derive(PartialEq)]` برای `==`.

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct P(i32);

impl Add for P {
    type Output = P;
    fn add(self, o: P) -> P { P(self.0 + o.0) }
}

fn main() {
    println!("{:?}", P(1) + P(2));
}
```

### E0599: متد پیدا نشد

`error[E0599]: no method named 'missing' found for struct 'S' in the current scope`

```rust,compile_fail,E0599
struct S;

fn main() {
    S.missing();               // ✗
}
```

علت‌های رایج: غلط املایی، نوع اشتباه، trait ای که متد از آن می‌آید `use` نشده (مثل `use std::io::Read;`)، یا قید generic کافی نیست (متد فقط وقتی `T: Clone` است). کامپایلر معمولاً در `help:` همان trait را پیشنهاد می‌دهد.

### E0614: dereference روی چیزی که pointer نیست

`error[E0614]: type 'u32' cannot be dereferenced`

```rust,compile_fail,E0614
fn main() {
    let x = 5u32;
    println!("{}", *x);        // ✗ x یک عدد ساده است
}
```

**راه‌حل:** `*` را بردارید. (در حلقه‌ها گاهی `x` از نوع `&i32` است و `*x` لازم می‌شود؛ نوع را با پیغام خطا بخوانید.)

### E0061: تعداد آرگومان نادرست

`error[E0061]: this function takes 1 argument but 2 arguments were supplied`

```rust,compile_fail,E0061
fn f(_a: i32) {}

fn main() {
    f(1, 2);                   // ✗
}
```

Rust overloading ندارد؛ برای آرگومان اختیاری از `Option` یا الگوی builder استفاده کنید ([Struct](10-structs.md)).

---

## گروه ۵: Pattern ها

### E0004: match کامل نیست

`error[E0004]: non-exhaustive patterns: 'C::B' not covered`

```rust,compile_fail,E0004
enum C { R, G, B }

fn main() {
    let c = C::R;
    match c {
        C::R => {}
        C::G => {}             // ✗ B فراموش شد
    }
}
```

```rust
enum C { R, G, B }

fn main() {
    let c = C::R;
    match c {
        C::R => println!("r"),
        C::G => println!("g"),
        C::B => println!("b"),
    }
}
```

> **توصیه:** از `_ =>` به‌عنوان راحت‌ترین راه استفاده نکنید؛ وقتی حالت جدیدی به enum اضافه شود، کامپایلر دیگر شما را متوجه نمی‌کند.

### E0023: تعداد فیلد الگو با variant نمی‌خواند

`error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields`

```rust,compile_fail,E0023
enum E { A(i32, i32) }

fn main() {
    let e = E::A(1, 2);
    match e {
        E::A(x) => println!("{x}"),     // ✗
    }
}
```

راه‌حل: `E::A(x, _)` یا `E::A(x, ..)`.

### E0026: فیلد وجود ندارد

`error[E0026]: struct 'P' does not have a field named 'y'`

```rust,compile_fail,E0026
struct P { x: i32 }

fn main() {
    let p = P { x: 1 };
    let P { y, .. } = p;       // ✗
}
```

---

## گروه ۶: نام‌ها و ماژول‌ها

### E0425: نام پیدا نشد

`error[E0425]: cannot find value 'undefined_name' in this scope`

```rust,compile_fail,E0425
fn main() {
    println!("{}", undefined_name);    // ✗
}
```

دلایل: غلط املایی، خارج از scope (متغیر داخل بلوک دیگر)، یا فراموشی `self.` در متدها.

### E0433: مسیر یا نوع پیدا نشد

`error[E0433]: cannot find type 'HashMap' in this scope`

```rust,compile_fail,E0433
fn main() {
    let m = HashMap::new();            // ✗ use فراموش شده
    let _: &std::collections::HashMap<i32, i32> = &m;
}
```

```rust
use std::collections::HashMap;

fn main() {
    let m: HashMap<i32, i32> = HashMap::new();
    println!("{}", m.len());
}
```

### E0432: import نامعتبر

`error[E0432]: unresolved import 'std::collections::HashMapp'`

```rust,compile_fail,E0432
use std::collections::HashMapp;        // ✗ غلط املایی

fn main() {}
```

اگر نام درست است ولی از crate خارجی می‌آید، `cargo add` را فراموش کرده‌اید. در edition 2018 به بعد برای crate های داخلی از `crate::` شروع کنید.

### E0603: آیتم خصوصی است

`error[E0603]: function 'private' is private`

```rust,compile_fail,E0603
mod m {
    fn private() {}
}

fn main() {
    m::private();                      // ✗
}
```

راه‌حل: `pub fn private()` (فصل [ماژول‌ها](13-modules.md)). فیلدهای struct هم جداگانه `pub` می‌خواهند.

---

## جدول مرجع سریع

| کد | معنی کوتاه | اولین کاری که امتحان کنید |
|---|---|---|
| E0382 | مقدار move شده | `&` بگیرید یا `.clone()` |
| E0499 | دو `&mut` همزمان | بازه‌ها را جدا کنید، `split_at_mut` |
| E0502 | `&mut` با `&` همزمان | مقدار را کپی کنید، ترتیب را عوض کنید |
| E0505 | move وقتی قرض هست | استفاده از قرض را زودتر تمام کنید |
| E0506 | انتساب وقتی قرض هست | همان |
| E0507 | move از پشت reference | `&`، `clone`، `take()` |
| E0596 | تغییر بدون `mut` | `let mut` یا پارامتر `&mut` |
| E0597 / E0716 | مقدار زود می‌میرد | مالک را در scope بالاتر نگه دارید |
| E0515 | برگرداندن reference محلی | مالکیت را برگردانید |
| E0373 / E0521 | closure/thread بیشتر از داده عمر می‌کند | `move`، `Arc`، `thread::scope` |
| E0106 | lifetime مبهم | `<'a>` یا `String` |
| E0308 | نوع‌ها نمی‌خوانند | `parse`/`into`/`as`، `;` اضافه |
| E0277 | trait لازم نیست | `derive`، قید generic |
| E0282 | نوع مشخص نیست | annotation یا turbofish |
| E0599 | متد نیست | `use` trait، قید generic |
| E0004 | match ناقص | حالت‌های جا‌مانده را بنویسید |
| E0425 / E0433 / E0432 | نام پیدا نشد | `use`، املا، `cargo add` |
| E0603 | خصوصی | `pub` |

## ترفندهای عمومی برای گیر کردن

۱. **پیغام را تا آخر بخوانید**، به‌خصوص `help:` و `note:`.
۲. `rustc --explain E0xxx` یا جستجوی همان کد در مستندات رسمی؛ هر کد مثال دارد.
۳. `cargo clippy` اغلب راه اصطلاحی‌تر را نشان می‌دهد.
۴. برای شکافتن پیچیدگی: عبارت بلند را به چند `let` با **نوع صریح** تقسیم کنید تا کامپایلر دقیق بگوید کجا نوع عوض می‌شود.
۵. وقتی با borrow checker دعوا دارید: اول `clone` بگذارید که کار کند، بعد با دانش فصل [Ownership](07-ownership.md) برگردید و حذفش کنید.
۶. برای تغییر ساختار به‌جای مبارزه: indices به‌جای reference، `Rc<RefCell<T>>` (فقط در صورت نیاز واقعی)، یا جدا کردن struct بزرگ به دو بخش مستقل.

## تمرین

1. برنامه‌ای بنویسید که عمداً سه خطای E0382، E0502 و E0597 بدهد؛ پیغام کامل هر کدام را بخوانید و برچسب‌های زیر کد را به زبان خودتان توضیح دهید.
2. هر کدام را با دو راه متفاوت درست کنید (مثلاً یکی با `clone` و یکی با تغییر ساختار).
3. تابعی بنویسید که اولین کلمه‌ی یک متن را برگرداند و ابتدا بدون lifetime کامپایل کنید تا E0106 را ببینید؛ بعد درستش کنید.
4. با `rustc --explain E0499` توضیح رسمی را بخوانید و مثال آن را اجرا کنید.
