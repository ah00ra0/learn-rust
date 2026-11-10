# Trait های مهم std و تبدیل انواع

در [فصل Trait](16-traits.md) با مفهوم trait آشنا شدید. کتابخانه‌ی استاندارد Rust ده‌ها trait دارد که «زبان مشترک» همه‌ی کدهای Rust هستند: وقتی نوع شما آن‌ها را پیاده کند، یکپارچه با `println!`، `HashMap`، `sort`، `collect`، `?` و بقیه‌ی اکوسیستم کار می‌کند. این فصل مهم‌ترین‌ها را با مثال و قاعده‌ی انتخاب مرور می‌کند.

## Clone و Copy

- **`Clone`**: کپی صریح با `.clone()`؛ ممکن است گران باشد (کپی heap).
- **`Copy`**: کپی ضمنی و ارزان (bitwise). نوع `Copy` باید `Clone` هم باشد و **نباید** `Drop` یا فیلد غیرـCopy داشته باشد.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Named {
    name: String, // String، Copy نیست، پس Named هم Copy نمی‌شود
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a; // کپی؛ a هنوز معتبر
    println!("{a:?} {b:?}");

    let n1 = Named { name: "علی".to_string() };
    let n2 = n1.clone(); // کپی عمیق
    println!("{n1:?} {n2:?}");
}
```

> **قاعده:** اگر نوعی کوچک و فقط از داده‌ی ساده ساخته شده، `Copy` بدهید. بعداً نمی‌توانید بدون شکستن API آن را حذف کنید.

## Default

مقدار پیش‌فرض یک نوع:

```rust
#[derive(Debug, Default)]
struct Config {
    name: String,   // ""
    retries: u32,   // 0
    verbose: bool,  // false
    ratio: f64,     // 0.0
}

#[derive(Debug)]
struct Server {
    port: u16,
    host: String,
}

impl Default for Server {
    fn default() -> Self {
        Server { port: 8080, host: "localhost".to_string() }
    }
}

fn main() {
    let c = Config::default();
    println!("{c:?}");

    // الگوی رایج: فقط چند فیلد را عوض کنید
    let s = Server { port: 9000, ..Default::default() };
    println!("{s:?}");

    let n: i32 = Default::default();
    let v: Vec<u8> = Default::default();
    println!("{n} {v:?}");
}
```

`#[derive(Default)]` روی enum هم ممکن است، با علامت `#[default]` روی یک variant:

```rust
#[derive(Debug, Default)]
enum Level {
    Low,
    #[default]
    Medium,
    High,
}

fn main() {
    println!("{:?}", Level::default());
    let _ = (Level::Low, Level::High);
}
```

## From و Into

`From<T>` تبدیل **بی‌خطا** از T به نوع شما را تعریف می‌کند. `Into` خودکار از `From` به‌دست می‌آید (هرگز مستقیم `Into` را پیاده نکنید، مگر دلیل خاص داشته باشید).

```rust
#[derive(Debug)]
struct Meters(f64);

#[derive(Debug)]
struct Kilometers(f64);

impl From<Kilometers> for Meters {
    fn from(k: Kilometers) -> Self {
        Meters(k.0 * 1000.0)
    }
}

// پارامتر generic که هر چیزی که به String تبدیل شود را می‌پذیرد
fn greet(name: impl Into<String>) -> String {
    let name: String = name.into();
    format!("سلام {name}")
}

fn main() {
    let m: Meters = Kilometers(1.5).into();
    println!("{m:?} {:?}", Meters::from(Kilometers(2.0)));

    println!("{}", greet("علی"));                 // &str
    println!("{}", greet(String::from("سارا")));  // String
}
```

نمونه‌های std که باید بشناسید: `String::from("..")`، `Vec::from([1,2,3])`، `i64::from(5_i32)`، `f64::from(1_u8)`، `Box::<dyn Error>::from("msg")`، `PathBuf::from("a/b")`.

### From و عملگر ?

عملگر `?` هنگام انتشار خطا، خودکار `From::from` را روی خطا صدا می‌زند. به همین دلیل می‌توانید چند نوع خطا را به یک enum واحد تبدیل کنید (فصل [مدیریت خطا](14-errors.md)):

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Negative(i64),
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_positive(s: &str) -> Result<i64, AppError> {
    let n: i64 = s.trim().parse()?; // ParseIntError → AppError
    if n < 0 {
        return Err(AppError::Negative(n));
    }
    Ok(n)
}

fn main() {
    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive("-3"));
    println!("{:?}", parse_positive("x"));
}
```

## TryFrom و TryInto

وقتی تبدیل **ممکن است شکست بخورد**:

```rust
use std::convert::TryFrom;

#[derive(Debug)]
struct Percent(u8);

impl TryFrom<i32> for Percent {
    type Error = String;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&v) {
            Ok(Percent(v as u8))
        } else {
            Err(format!("{v} بین ۰ و ۱۰۰ نیست"))
        }
    }
}

fn main() {
    println!("{:?}", Percent::try_from(50));
    println!("{:?}", Percent::try_from(150));

    let big: i64 = 300;
    let r: Result<u8, _> = u8::try_from(big);
    println!("{r:?}");

    let ok: u8 = 200_i32.try_into().unwrap();
    println!("{ok}");
}
```

> **چرا `as` خطرناک است؟** `300_i32 as u8` بی‌صدا برابر 44 می‌شود. برای تبدیل‌های ممکن‌ازدست‌رفتن، `try_from` را ترجیح دهید.

## FromStr و parse

`str::parse::<T>()` همان `T::from_str` را صدا می‌زند:

```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

impl FromStr for Rgb {
    type Err = String;

    // فرمت: "#rrggbb"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').ok_or("باید با # شروع شود")?;
        if hex.len() != 6 {
            return Err("طول باید ۶ باشد".to_string());
        }
        let part = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string());
        Ok(Rgb(part(0)?, part(2)?, part(4)?))
    }
}

fn main() {
    let c: Rgb = "#ff8000".parse().unwrap();
    println!("{c:?}");
    println!("{:?}", "ff8000".parse::<Rgb>());
    println!("{:?}", Rgb::from_str("#12"));
}
```

## AsRef و AsMut

برای توابعی که «هر چیزی که شبیه X است» را بپذیرند، بدون مصرف مالکیت:

```rust
use std::path::Path;

fn print_len(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

fn exists_in_name(p: impl AsRef<Path>) -> bool {
    p.as_ref().file_name().is_some()
}

fn main() {
    println!("{}", print_len("abc"));
    println!("{}", print_len(String::from("abcd")));
    println!("{}", exists_in_name("dir/file.txt"));
    println!("{}", exists_in_name(std::path::PathBuf::from("x/y")));
}
```

`AsRef<Path>` الگوی استاندارد توابع فایل‌سیستم است: هم `&str`، هم `String`، هم `PathBuf` می‌پذیرد.

## Borrow، ToOwned و Cow

- `Borrow<B>`: شبیه `AsRef` ولی با تضمین اینکه `Eq/Ord/Hash` مقدار قرض‌گرفته با اصلی یکی باشد. به همین دلیل می‌توانید `HashMap<String, V>` را با `&str` جستجو کنید.
- `ToOwned`: تعمیم `Clone` برای نوع‌های unsized (`&str → String`, `&[T] → Vec<T>`).
- `Cow<'a, B>` (Clone-on-Write): یا قرض‌گرفته یا دارای مالک؛ فقط وقتی لازم شد کپی می‌کند.

```rust
use std::borrow::Cow;
use std::collections::HashMap;

fn normalize(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_")) // فقط در صورت نیاز تخصیص
    } else {
        Cow::Borrowed(input)                // بدون تخصیص
    }
}

fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("کلید".to_string(), 1);
    println!("{:?}", m.get("کلید")); // جستجو با &str (به لطف Borrow)

    let a = normalize("no_spaces");
    let b = normalize("has some spaces");
    println!("{a} {b}");
    println!("{} {}", matches!(a, Cow::Borrowed(_)), matches!(b, Cow::Owned(_)));
}
```

## مقایسه: PartialEq، Eq، PartialOrd، Ord

| Trait | عملگر | قاعده |
|---|---|---|
| `PartialEq` | `==`, `!=` | ممکن است `a == a` درست نباشد (مثل `NaN`) |
| `Eq` | | تضمین بازتابی بودن؛ لازم برای کلید `HashMap` |
| `PartialOrd` | `<`, `<=`, `>`, `>=` | ترتیب جزئی؛ `partial_cmp` → `Option<Ordering>` |
| `Ord` | | ترتیب کامل؛ لازم برای `sort()`, `BTreeMap`, `max()` |

```rust
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u32, u32, u32); // ترتیب فیلدها = ترتیب مقایسه

#[derive(Debug, PartialEq, Eq)]
struct Task {
    priority: u32,
    name: String,
}

// ترتیب دستی: اولویت بالاتر اول
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority).then_with(|| self.name.cmp(&other.name))
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("{}", Version(1, 2, 3) < Version(1, 10, 0));

    let mut tasks = vec![
        Task { priority: 1, name: "ب".into() },
        Task { priority: 5, name: "الف".into() },
        Task { priority: 5, name: "الف۲".into() },
    ];
    tasks.sort();
    println!("{:?}", tasks.iter().map(|t| &t.name).collect::<Vec<_>>());
}
```

### چرا f64 ، Ord نیست؟

چون `NaN` با هیچ‌چیز (حتی خودش) قابل مقایسه نیست. پس `v.sort()` روی `Vec<f64>` کامپایل نمی‌شود:

```rust,compile_fail,E0277
fn main() {
    let mut v = vec![2.5, 1.0];
    v.sort(); // ✗ f64: Ord نیست
}
```

راه‌حل‌ها:

```rust
fn main() {
    let mut v = vec![2.5, 1.0, 3.2];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap()); // اگر مطمئنید NaN نیست
    println!("{v:?}");

    let mut w = vec![2.5, f64::NAN, 1.0];
    w.sort_by(|a, b| a.total_cmp(b));            // ترتیب کامل شامل NaN
    println!("{w:?}");
}
```

## Hash

برای کلید `HashMap/HashSet` نوع باید `Eq + Hash` باشد، و **قرارداد مهم** این است: اگر `a == b` آنگاه `hash(a) == hash(b)`. `derive` همیشه هر دو را سازگار می‌سازد؛ اگر `PartialEq` را دستی نوشتید، `Hash` را هم دستی و سازگار بنویسید.

```rust
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Id(u32);

fn main() {
    let mut s = HashSet::new();
    s.insert(Id(1));
    s.insert(Id(1));
    s.insert(Id(2));
    println!("{}", s.len());
}
```

## Display و Debug

| | `Debug` | `Display` |
|---|---|---|
| مخاطب | برنامه‌نویس | کاربر نهایی |
| قالب | `{:?}` / `{:#?}` | `{}` |
| derive | بله | **خیر**، دستی |
| `to_string()` | | خودکار با `Display` |

```rust
use std::fmt;

struct Money {
    cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

fn main() {
    let m = Money { cents: 1999 };
    println!("{m}");
    let s: String = m.to_string();
    println!("{s:>10}|"); // توجه: padding روی String کار می‌کند
}
```

> برای اینکه `{:>10}` روی نوع خودتان هم رعایت شود، در `fmt` از `f.pad(...)` استفاده کنید. جزئیات در فصل [قالب‌بندی](formatting-and-io.md).

## Deref و DerefMut

با `Deref` یک wrapper می‌تواند «شفاف» رفتار کند (فصل [Smart Pointer](20-smart-pointers.md)). **هشدار:** از `Deref` برای شبیه‌سازی وراثت استفاده نکنید؛ فقط برای انواع شبیه pointer.

## IntoIterator، FromIterator، Extend

سه trait که تعیین می‌کنند نوع شما در حلقه‌ی `for` و `collect()` چطور رفتار کند:

```rust
#[derive(Debug)]
struct Bag(Vec<i32>);

// for x in bag  (مصرف)
impl IntoIterator for Bag {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// for x in &bag  (قرض)
impl<'a> IntoIterator for &'a Bag {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// iter.collect::<Bag>()
impl FromIterator<i32> for Bag {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Bag(iter.into_iter().collect())
    }
}

// bag.extend(...)
impl Extend<i32> for Bag {
    fn extend<I: IntoIterator<Item = i32>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

fn main() {
    let mut bag: Bag = (1..=3).collect();
    bag.extend([10, 20]);

    for x in &bag {
        print!("{x} ");
    }
    println!();

    let total: i32 = bag.into_iter().sum();
    println!("{total}");
}
```

## سربارگذاری عملگرها

همه در `std::ops` هستند:

| عملگر | Trait | عملگر | Trait |
|---|---|---|---|
| `+` `-` `*` `/` `%` | `Add Sub Mul Div Rem` | `+=` `-=` ... | `AddAssign SubAssign ...` |
| `-x` | `Neg` | `!x` | `Not` |
| `&` `\|` `^` `<<` `>>` | `BitAnd BitOr BitXor Shl Shr` | `a[i]` | `Index IndexMut` |
| `*x` | `Deref DerefMut` | `f(x)` | `Fn FnMut FnOnce` (غیرپایدار برای پیاده‌سازی دستی) |

```rust
use std::ops::{Add, AddAssign, Index, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct V2 {
    x: f64,
    y: f64,
}

impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        V2 { x: self.x + o.x, y: self.y + o.y }
    }
}
impl AddAssign for V2 {
    fn add_assign(&mut self, o: V2) {
        self.x += o.x;
        self.y += o.y;
    }
}
impl Mul<f64> for V2 {
    type Output = V2;
    fn mul(self, k: f64) -> V2 {
        V2 { x: self.x * k, y: self.y * k }
    }
}
impl Mul<V2> for f64 {
    type Output = V2;
    fn mul(self, v: V2) -> V2 {
        v * self
    }
}
impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        V2 { x: -self.x, y: -self.y }
    }
}
impl Index<usize> for V2 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("اندیس {i} نامعتبر است"),
        }
    }
}

fn main() {
    let mut a = V2 { x: 1.0, y: 2.0 };
    let b = V2 { x: 0.5, y: 0.5 };
    a += b;
    println!("{:?} {:?} {:?} {}", a + b, 2.0 * a, -a, a[1]);
}
```

> **توصیه:** عملگرها را فقط وقتی سربارگذاری کنید که معنای ریاضی/شهودی روشن دارند. `+` برای الحاق دو رشته در std هم یک استثنا و بحث‌برانگیز است.

## Marker Trait ها

traitهایی بدون متد که فقط یک ویژگی را اعلام می‌کنند:

| Trait | معنی |
|---|---|
| `Send` | مالکیت مقدار را می‌توان به thread دیگر برد |
| `Sync` | `&T` را می‌توان بین thread ها به اشتراک گذاشت |
| `Sized` | اندازه در زمان کامپایل معلوم است (همه‌ی generic ها پیش‌فرض `T: Sized` دارند؛ برای `str` و `[T]` باید `T: ?Sized` بنویسید) |
| `Unpin` | مقدار را می‌توان پس از pin شدن هم جابه‌جا کرد (کاربرد در async) |
| `Copy` | کپی bitwise ارزان |

```rust
use std::fmt::Display;

// ?Sized اجازه می‌دهد T مثل str باشد (که Sized نیست)
fn show<T: Display + ?Sized>(x: &T) {
    println!("{x}");
}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    show("متن");          // T = str
    show(&42);            // T = i32
    assert_send::<Vec<i32>>();
    assert_sync::<String>();
}
```

## قاعده‌ی Orphan و الگوی Newtype

می‌توانید trait را فقط وقتی برای نوعی پیاده کنید که **یکی از آن‌ها متعلق به crate شما** باشد. برای پیاده‌کردن trait خارجی روی نوع خارجی، آن را داخل struct تک‌فیلدی (newtype) بپیچید:

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec!["الف".into(), "ب".into()]);
    println!("{w}");
}
```

newtype هزینه‌ی اجرایی ندارد و برای افزودن **ایمنی نوع** هم عالی است (مثلاً `UserId(u64)` و `OrderId(u64)` را نمی‌توان اشتباهی جابه‌جا کرد).

## راهنمای: کدام trait را derive کنم؟

یک struct داده‌ی ساده معمولاً این مجموعه را می‌خواهد:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Tag {
    name: String,
}

fn main() {
    println!("{:?}", Tag::default());
}
```

- همیشه `Debug` (برای خطایابی و `assert_eq!`).
- `Clone` اگر منطقی است کپی گرفته شود؛ `Copy` فقط برای انواع کوچک ساده.
- `PartialEq/Eq` برای مقایسه و کلید map؛ `Hash` همراه `Eq`.
- `PartialOrd/Ord` فقط وقتی ترتیب طبیعی دارند.
- `Default` وقتی «مقدار خالی» معنا دارد.
- برای نوع‌های عمومی کتابخانه، رویه‌ی خوب این است که تا جای ممکن این trait ها را پیاده کنید تا کاربران محدود نشوند (قاعده‌ی «eagerly implement common traits» در Rust API Guidelines).

## تمرین

1. struct `Temperature(f64)` بسازید و `From<Celsius>` و `From<Fahrenheit>` را برایش پیاده کنید.
2. `TryFrom<&str>` برای enum `Color` بنویسید که `"red"` و `"green"` را بپذیرد و بقیه را خطا بدهد.
3. تابعی بنویسید که با `impl AsRef<str>` هم `String` و هم `&str` و هم `Cow<str>` بپذیرد.
4. نوع `Matrix2([[f64; 2]; 2])` بسازید و `Add`, `Mul` و `Index<(usize, usize)>` را پیاده کنید.
5. نوعی بسازید که `FromIterator<char>` را پیاده کند و رشته‌ی حروف بزرگ را جمع کند.
