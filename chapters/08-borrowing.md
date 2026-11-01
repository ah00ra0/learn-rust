# Reference و Borrowing (قرض‌گیری)

در فصل قبل دیدیم که پس دادن مالکیت پس از هر فراخوانی خسته‌کننده است. راه‌حل: به‌جای **انتقال** مالکیت، مقدار را **قرض بدهیم**. این کار با **Reference** (مرجع) و علامت `&` انجام می‌شود.

## Reference غیرقابل‌تغییر (&T)

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // قرض دادن s1
    println!("طول '{s1}' برابر است با {len}"); // s1 هنوز معتبر است
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s از scope خارج می‌شود اما چون مالک نیست، چیزی drop نمی‌شود
```

یک reference همیشه به یک مقدار **معتبر** اشاره می‌کند. به عمل ساختن reference می‌گویند **borrowing**.

با reference غیرقابل‌تغییر نمی‌توان مقدار را عوض کرد:

```rust,compile_fail
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // ✗ خطا: نمی‌توان مقدار قرض‌گرفته را تغییر داد
}
```

## Reference قابل‌تغییر (&mut T)

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

دقت کنید: متغیر باید `mut` باشد و هنگام فراخوانی هم `&mut` بنویسید.

## قوانین Borrowing

در هر لحظه، شما می‌توانید **یکی** از این دو حالت را داشته باشید:

- **هر تعداد** reference غیرقابل‌تغییر (`&T`)
- **دقیقاً یک** reference قابل‌تغییر (`&mut T`)

و همیشه: reference ها باید **معتبر** باشند (هرگز dangling نباشند).

### چرا فقط یک &mut؟

برای جلوگیری از **data race**:

```rust,compile_fail
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // ✗ خطا: بیش از یک قرض قابل‌تغییر

    println!("{r1}, {r2}");
}
```

### ترکیب &T و &mut T

```rust,compile_fail
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // مشکلی نیست
    let r2 = &s;     // مشکلی نیست
    let r3 = &mut s; // ✗ خطا: وقتی r1 و r2 زنده‌اند نمی‌شود قرض قابل‌تغییر گرفت

    println!("{r1}, {r2}, {r3}");
}
```

منطقش این است: کسی که فقط می‌خواند، انتظار ندارد مقدار زیر دستش عوض شود.

### NLL: عمر reference تا آخرین استفاده است

Reference از جایی که ساخته می‌شود تا **آخرین جایی که استفاده می‌شود** زنده است (نه تا انتهای بلوک). این قابلیت را **Non-Lexical Lifetimes (NLL)** می‌گویند:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} و {r2}"); // آخرین استفاده از r1 و r2

    let r3 = &mut s; // ✓ حالا مشکلی نیست
    r3.push_str("!");
    println!("{r3}");
}
```

## Dangling Reference

در زبان‌هایی مثل C ممکن است اشاره‌گری به حافظه‌ی آزادشده بسازید. در Rust کامپایلر جلوی آن را می‌گیرد:

```rust,compile_fail
fn main() {
    let r = dangle();
}

fn dangle() -> &String { // ✗ خطا: مقداری برای قرض دادن وجود ندارد
    let s = String::from("hello");
    &s
} // s اینجا drop می‌شود و reference به حافظه‌ی نامعتبر اشاره می‌کند
```

راه‌حل: مالکیت را برگردانید.

```rust
fn main() {
    let s = no_dangle();
    println!("{s}");
}

fn no_dangle() -> String {
    String::from("hello")
}
```

## تفاوت reference با pointer

- reference ها در Rust همیشه **غیر null** و **معتبر** هستند.
- با عملگر `*` می‌توانید به مقدار اشاره‌شده دسترسی بگیرید (dereference):

```rust
fn main() {
    let mut x = 10;
    let r = &mut x;
    *r += 5;
    println!("{x}");
}
```

در بیشتر مواقع (مثل فراخوانی متد) Rust خودکار dereference می‌کند و نیازی به `*` نیست.

## خلاصه

| نوع | نوشتار | تعداد مجاز همزمان |
|---|---|---|
| قرض فقط‌خواندنی | `&T` | هر تعداد |
| قرض قابل‌تغییر | `&mut T` | فقط یکی (و بدون `&T` همزمان) |

## تمرین

1. تابعی بنویسید که با `&Vec<i32>` مجموع عناصر را حساب کند.
2. تابعی بنویسید که با `&mut Vec<i32>` عنصری به انتها اضافه کند.
3. کدی بنویسید که دو `&mut` همزمان بگیرد و خطای کامپایلر را بخوانید.
4. تابعی بنویسید که بزرگ‌ترین عدد یک آرایه را به‌صورت `&i32` برگرداند (راهنمایی: به lifetime ها در فصل ۱۷ نیاز پیدا می‌کنید؛ فعلاً با `&[i32]` به‌عنوان ورودی امتحان کنید).

فصل بعد: [Slice](09-slices.md).
