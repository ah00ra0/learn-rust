# مجموعه‌ها: Vec، String و HashMap

کتابخانه‌ی استاندارد Rust چند ساختار داده‌ی مفید دارد که داده‌هایشان روی **heap** ذخیره می‌شود و می‌توانند در زمان اجرا بزرگ و کوچک شوند. سه مورد پرکاربرد آن‌ها: `Vec`، `String` و `HashMap`.

## Vec: آرایه‌ی پویا

### ساخت

```rust
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];     // ماکرو vec!
    let v3 = vec![0; 5];        // [0, 0, 0, 0, 0]
    println!("{v1:?} {v2:?} {v3:?}");
}
```

### اضافه و حذف

```rust
fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let last = v.pop();          // Some(7)
    v.insert(0, 100);            // درج در اندیس ۰
    v.remove(1);                 // حذف عنصر اندیس ۱
    println!("{v:?} {last:?}");
}
```

### خواندن عناصر

دو روش دارید:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];           // panic اگر اندیس نامعتبر باشد
    println!("{third}");

    match v.get(2) {                    // Option برمی‌گرداند و panic نمی‌کند
        Some(x) => println!("{x}"),
        None => println!("وجود ندارد"),
    }

    println!("{:?}", v.get(100));       // None
}
```

### Vec و قوانین borrowing

```rust,compile_fail
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];  // قرض غیرقابل‌تغییر
    v.push(6);          // ✗ خطا: push ممکن است حافظه را جابه‌جا کند
    println!("{first}");
}
```

چرا؟ چون `push` ممکن است Vec را در جای دیگری از حافظه بازتخصیص دهد و `first` به حافظه‌ی نامعتبر اشاره کند. Rust از این باگ در زمان کامپایل جلوگیری می‌کند.

### پیمایش

```rust
fn main() {
    let mut v = vec![100, 32, 57];

    for i in &v {            // فقط خواندن
        println!("{i}");
    }

    for i in &mut v {        // تغییر مقادیر
        *i += 50;
    }
    println!("{v:?}");
}
```

### ذخیره‌ی انواع مختلف با enum

```rust
#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("سلام")),
        Cell::Float(10.12),
    ];
    println!("{row:?}");
}
```

### متدهای مفید

```rust
fn main() {
    let mut v = vec![5, 3, 8, 1];

    v.sort();
    println!("{v:?}");                      // [1, 3, 5, 8]
    println!("{}", v.contains(&3));         // true
    println!("{:?}", v.iter().max());       // Some(8)
    println!("{}", v.len());
    v.reverse();
    v.retain(|&x| x > 2);                   // فقط عناصر بزرگ‌تر از ۲ بمانند
    println!("{v:?}");
    v.extend([10, 20]);
    v.truncate(3);
    println!("{v:?} {}", v.is_empty());
}
```

## String

### String در برابر &str

- `str`: داده‌ی متنی (معمولاً به‌صورت `&str` دیده می‌شود)، تغییرناپذیر.
- `String`: متن **قابل رشد** و **دارای مالک** روی heap. در واقع یک `Vec<u8>` است که UTF-8 بودنش تضمین شده.

### ساخت و تغییر

```rust
fn main() {
    let mut s = String::new();
    s.push_str("سلام");
    s.push(' ');
    s += "دنیا";               // عملگر +=
    println!("{s}");

    let a = String::from("Hello, ");
    let b = String::from("world!");
    let c = a + &b;            // a به اینجا move می‌شود
    println!("{c}");

    let x = String::from("tic");
    let y = String::from("tac");
    let z = String::from("toe");
    let joined = format!("{x}-{y}-{z}");   // هیچ کدام move نمی‌شوند
    println!("{joined} {x}");
}
```

### اندیس‌گذاری ممنوع است

```rust,compile_fail
fn main() {
    let s = String::from("hello");
    let h = s[0]; // ✗ خطا: String را نمی‌توان اندیس‌گذاری کرد
}
```

دلیل: رشته‌ها UTF-8 هستند و کاراکتر ممکن است بیش از یک بایت باشد. اندیس `s[0]` معنی مشخصی ندارد (بایت؟ کاراکتر؟ grapheme؟). به‌جایش:

```rust
fn main() {
    let s = "नमस्ते";
    println!("{}", s.len());              // بایت‌ها
    println!("{}", s.chars().count());    // کاراکترها

    let t = "درود";
    println!("{:?}", t.chars().nth(1));   // Some('ر')
    println!("{:?}", t.char_indices().next());
}
```

### متدهای پرکاربرد

```rust
fn main() {
    let s = String::from("  Hello, Rust World  ");

    println!("[{}]", s.trim());
    println!("{}", s.to_uppercase());
    println!("{}", s.replace("Rust", "راست"));
    println!("{}", s.contains("Rust"));
    println!("{}", s.starts_with("  He"));

    for word in s.split_whitespace() {
        println!("{word}");
    }

    let parts: Vec<&str> = "a,b,c".split(',').collect();
    println!("{parts:?}");

    let n: i32 = "42".parse().unwrap();
    println!("{}", n + 1);
}
```

## HashMap

نگهداری جفت **کلید-مقدار** با جستجوی سریع. باید `use` شود:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("آبی"), 10);
    scores.insert(String::from("قرمز"), 50);

    let team = String::from("آبی");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
```

### مالکیت در HashMap

مقادیر از نوع Copy کپی می‌شوند؛ مقادیر دارای مالک (مثل `String`) به map **منتقل** می‌شوند:

```rust
use std::collections::HashMap;

fn main() {
    let key = String::from("رنگ");
    let value = String::from("آبی");

    let mut map = HashMap::new();
    map.insert(key, value);
    // println!("{key}"); // ✗ key به map move شد
    println!("{map:?}");
}
```

### بازنویسی، افزودن در صورت نبود، به‌روزرسانی

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("a", 10);
    scores.insert("a", 25);                 // بازنویسی

    scores.entry("b").or_insert(50);        // فقط اگر وجود نداشته باشد
    scores.entry("b").or_insert(99);        // تغییری نمی‌کند

    println!("{scores:?}");
}
```

الگوی کلاسیک **شمارش کلمات**:

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut entries: Vec<_> = map.iter().collect();
    entries.sort();
    println!("{entries:?}");
}
```

> **نکته:** ترتیب پیمایش `HashMap` تضمین‌شده نیست. اگر ترتیب مرتب لازم دارید از `BTreeMap` استفاده کنید.

## مجموعه‌های دیگر

| نوع | کاربرد |
|---|---|
| `VecDeque<T>` | صف دوطرفه |
| `HashSet<T>` | مجموعه‌ی عناصر یکتا |
| `BTreeMap<K, V>` / `BTreeSet<T>` | نسخه‌ی مرتب |
| `BinaryHeap<T>` | صف اولویت |
| `LinkedList<T>` | لیست پیوندی (به‌ندرت لازم است) |

```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2);
    println!("{}", set.len()); // 2
}
```

## تمرین

1. با یک `Vec` از اعداد، میانگین و میانه را حساب کنید.
2. رشته‌ای را به Pig Latin تبدیل کنید (کلمه‌ای که با حرف بی‌صدا شروع می‌شود، حرف اول به انتها می‌رود و "ay" اضافه می‌شود).
3. با `HashMap` و `Vec` سیستم ساده‌ای بسازید که کارمندان را به بخش‌ها اضافه کند و لیست مرتب‌شده‌ی هر بخش را چاپ کند.
4. برنامه‌ای بنویسید که تعداد تکرار هر حرف در یک رشته را بشمارد.

فصل بعد: [Package، Crate و Module](13-modules.md).
