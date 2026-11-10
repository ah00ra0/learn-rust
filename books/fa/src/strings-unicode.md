# رشته‌ها و یونیکد در عمق

در [فصل Slice](09-slices.md) و [مجموعه‌ها](12-collections.md) با `String` و `&str` آشنا شدید. این فصل همان موضوع را عمیق‌تر می‌شکافد: Rust دقیقاً چه چیزی را در حافظه ذخیره می‌کند، چرا نمی‌توان با اندیس به رشته دسترسی داشت، و مهم‌تر برای فارسی‌زبانان، **چرا متن فارسی و عربی چالش‌های ویژه‌ای دارد** (نیم‌فاصله، دو شکل «ی» و «ک»، ارقام فارسی و ...).

> همه‌ی خروجی‌های این فصل از اجرای واقعی کد گرفته شده‌اند. کتابخانه‌ی کمکی با تست در [`projects/strings-unicode`](https://github.com/ah00ra0/learn-rust/tree/main/projects/strings-unicode) موجود است.

## مدل ذهنی: سه سطح «طول»

یک متن را می‌توان در سه سطح شمرد:

| سطح | تعریف | در Rust |
|---|---|---|
| **بایت** | واحد ذخیره‌سازی در UTF-8 | `s.len()` |
| **Scalar Value** (که Rust به آن `char` می‌گوید) | یک «نقطه‌ی کد» یونیکد | `s.chars().count()` |
| **Grapheme Cluster** | آنچه کاربر «یک حرف» می‌بیند | crate ی `unicode-segmentation` |

```rust
fn main() {
    let latin = "abc";
    let persian = "سلام";
    assert_eq!(latin.len(), 3);
    assert_eq!(latin.chars().count(), 3);

    assert_eq!(persian.len(), 8);            // هر حرف فارسی ۲ بایت
    assert_eq!(persian.chars().count(), 4);  // ۴ حرف

    // "é" به دو شکل: یک کاراکتر یا e + accent ترکیبی
    let composed = "\u{e9}";
    let decomposed = "e\u{301}";
    assert_eq!(composed.chars().count(), 1);
    assert_eq!(decomposed.chars().count(), 2); // ولی برای کاربر یک حرف است
    assert_ne!(composed, decomposed);          // و از نظر == متفاوت‌اند!
}
```

نتیجه: «طول یک رشته» بدون مشخص کردن سطح بی‌معناست. اگر می‌خواهید بدانید «چند حرف» روی صفحه است، به grapheme نیاز دارید.

## نمایش درونی: UTF-8

`String` و `&str` همیشه **UTF-8 معتبر** هستند (Rust این را تضمین می‌کند). UTF-8 هر کاراکتر را با ۱ تا ۴ بایت کد می‌کند:

| بازه‌ی کد | بایت‌ها | نمونه |
|---|---|---|
| U+0000 – U+007F | ۱ | `a`, `5`, فاصله |
| U+0080 – U+07FF | ۲ | `é`, حروف فارسی و عربی، یونانی |
| U+0800 – U+FFFF | ۳ | بیشتر حروف چینی، نیم‌فاصله `U+200C` |
| U+10000 – U+10FFFF | ۴ | بیشتر emoji ها |

```rust
fn main() {
    for c in ['a', 'ی', '\u{200c}', '😀'] {
        println!("{c:?} → {} بایت، U+{:04X}", c.len_utf8(), c as u32);
    }
    assert_eq!('a'.len_utf8(), 1);
    assert_eq!('ی'.len_utf8(), 2);
    assert_eq!('\u{200c}'.len_utf8(), 3);
    assert_eq!('😀'.len_utf8(), 4);
}
```

`String` در واقع یک `Vec<u8>` است که UTF-8 بودنش حفظ می‌شود.

### چرا s[0] ممنوع است؟

```rust,compile_fail,E0277
fn main() {
    let s = String::from("سلام");
    let c = s[0];   // ✗ the type `str` cannot be indexed by `{integer}`
}
```

چون سه معنی ممکن دارد (بایت اول؟ کاراکتر اول؟ حرف اول؟) و دسترسی با اندیس کاراکتر در UTF-8 از مرتبه‌ی O(n) است، نه O(1). Rust عمداً آن را ممنوع کرده تا ناخواسته کند و غلط نشود.

### slice با اندیس بایت

```rust
fn main() {
    let s = "علی";
    assert_eq!(&s[0..2], "ع");       // بایت‌های ۰ و ۱ = یک حرف کامل
    assert_eq!(&s[2..], "لی");
    assert!(s.is_char_boundary(2));
    assert!(!s.is_char_boundary(1)); // وسط یک حرف
    // &s[0..1] اینجا panic می‌کرد
    assert_eq!(s.get(0..1), None);   // نسخه‌ی ایمن: None برمی‌گرداند
    assert_eq!(s.get(0..2), Some("ع"));
}
```

> **قاعده:** با `&s[a..b]` فقط وقتی کار کنید که `a` و `b` را از خود Rust گرفته‌اید (مثلاً از `find` یا `char_indices`). برای ورودی کاربر از `get()` استفاده کنید.

## پیمایش: chars، char_indices، bytes

```rust
fn main() {
    let s = "علی";

    let chars: Vec<char> = s.chars().collect();
    assert_eq!(chars, ['ع', 'ل', 'ی']);

    // موقعیت بایت هر کاراکتر
    let idx: Vec<(usize, char)> = s.char_indices().collect();
    assert_eq!(idx, [(0, 'ع'), (2, 'ل'), (4, 'ی')]);

    assert_eq!(s.bytes().count(), 6);
    assert_eq!(s.as_bytes()[0], 0xD8); // بایت اول UTF-8 دو‌بایتی («ع»)

    // n-امین کاراکتر: O(n)
    assert_eq!(s.chars().nth(1), Some('ل'));
}
```

### معکوس کردن متن

```rust
fn main() {
    let s = "علی";
    let rev: String = s.chars().rev().collect();
    assert_eq!(rev, "یلع");
}
```

این کار روی scalar value ها معکوس می‌کند. برای متن با علامت‌های ترکیبی یا emoji های مرکب، نتیجه ممکن است خراب شود (اول accent و بعد حرف). برای نمایش به کاربر باید روی grapheme معکوس کرد.

## نیم‌فاصله و ZWNJ

نیم‌فاصله (U+200C، Zero Width Non-Joiner) در فارسی حیاتی است («می‌خواهم»، «کتاب‌ها»). ویژگی‌های مهم از نظر برنامه‌نویسی:

```rust
fn main() {
    let zwnj = '\u{200c}';

    assert!(!zwnj.is_whitespace());          // فضای خالی حساب نمی‌شود!
    assert_eq!("  x\u{200c} ".trim(), "x\u{200c}"); // trim آن را برنمی‌دارد

    let w = "می\u{200c}خواهم";
    assert_eq!(w.chars().count(), 8);        // یک کاراکتر مستقل است
    let parts: Vec<&str> = w.split(zwnj).collect();
    assert_eq!(parts, ["می", "خواهم"]);      // ولی می‌توان روی آن split کرد
    assert_eq!(zwnj.len_utf8(), 3);          // ۳ بایت
}
```

پیامدها:

- `trim()` و `split_whitespace()` نیم‌فاصله را فاصله نمی‌دانند؛ این معمولاً همان چیزی است که می‌خواهید.
- هنگام **جستجو** ممکن است بخواهید «میخواهم» و «می‌خواهم» یکی حساب شوند. باید قبل از مقایسه نیم‌فاصله را بردارید یا یکسان‌سازی کنید.
- هنگام شمردن حروف نمایشی، نیم‌فاصله به حرف قبل می‌چسبد (در سطح grapheme، پایین‌تر).

## دو «ی» و دو «ک»: یکسان‌سازی حروف

متن عربی از `ي` (U+064A) و `ك` (U+0643) استفاده می‌کند؛ فارسی از `ی` (U+06CC) و `ک` (U+06A9). صفحه‌کلیدهای اشتباه یا داده‌های قدیمی گاهی عربی را وارد می‌کنند. این دو **شبیه‌اند ولی برابر نیستند**:

```rust
fn main() {
    let arabic = "علي";   // ي عربی
    let persian = "علی";  // ی فارسی
    assert_ne!(arabic, persian);

    let fixed: String = arabic
        .chars()
        .map(|c| match c {
            '\u{064A}' | '\u{0649}' => '\u{06CC}', // ي ى → ی
            '\u{0643}' => '\u{06A9}',              // ك → ک
            other => other,
        })
        .collect();
    assert_eq!(fixed, persian);
}
```

> **توصیه:** برای جستجو و ذخیره‌ی شناسه‌ها (نام کاربری، ایمیل)، متن را **قبل از ذخیره و مقایسه** یکسان‌سازی کنید.

## ارقام فارسی و عربی

```rust
fn main() {
    assert!('۵'.is_numeric());          // عدد یونیکد است
    assert!(!'۵'.is_ascii_digit());     // ولی رقم ASCII نیست
    assert_eq!('۵'.to_digit(10), None); // to_digit فقط 0-9 ASCII را می‌شناسد
    assert!("۱۲۳".parse::<i32>().is_err()); // parse هم نمی‌پذیرد!
    assert_eq!("123".parse::<i32>(), Ok(123));

    // تبدیل دستی ارقام فارسی (۰-۹) و عربی-هندی (٠-٩) به ASCII
    let to_ascii = |s: &str| -> String {
        s.chars()
            .map(|c| match c {
                '\u{06F0}'..='\u{06F9}' => char::from(b'0' + (c as u32 - 0x06F0) as u8),
                '\u{0660}'..='\u{0669}' => char::from(b'0' + (c as u32 - 0x0660) as u8),
                other => other,
            })
            .collect()
    };
    assert_eq!(to_ascii("۱۲۳ و ٤٥٦"), "123 و 456");
    assert_eq!(to_ascii("۱۲۳").parse::<i32>(), Ok(123));
}
```

اگر برنامه‌ی شما از کاربر ایرانی عدد می‌گیرد (مبلغ، شماره‌ی تلفن)، **قبل از `parse` ارقام را به ASCII تبدیل کنید**، وگرنه `Err` می‌گیرید.

## تغییر حروف بزرگ و کوچک (case)

`to_lowercase` و `to_uppercase` قواعد کامل یونیکد را رعایت می‌کنند، و نتیجه ممکن است **طول متفاوت** داشته باشد:

```rust
fn main() {
    assert_eq!("ß".to_uppercase(), "SS");        // ۱ کاراکتر → ۲ کاراکتر
    assert_eq!("İ".to_lowercase().chars().count(), 2);
    assert_eq!("ÅΣ".to_lowercase(), "åς");       // سیگمای پایانی طبق قاعده‌ی زبان یونانی
    assert!('ی'.is_alphabetic());
    assert_eq!("سلام".to_uppercase(), "سلام");   // فارسی حرف بزرگ و کوچک ندارد
}
```

برای مقایسه‌ی بدون توجه به حروف:

- متن ASCII: `a.eq_ignore_ascii_case(b)`
- متن عمومی: هر دو را `to_lowercase()` کنید (ولی این **case folding** کامل نیست؛ برای دقت بیشتر از crate های ICU یا `caseless` استفاده کنید).

## Normalization (نرمال‌سازی یونیکد)

یک «حرف» می‌تواند به چند شکل کد شود (مثل `é` در بالا). یونیکد چهار صورت نرمال دارد: **NFC, NFD, NFKC, NFKD**. برای مقایسه و جستجو، هر دو طرف را به یک صورت (معمولاً NFC) ببرید. کتابخانه‌ی استاندارد این کار را ندارد؛ از `unicode-normalization` استفاده می‌شود:

```rust,ignore
use unicode_normalization::UnicodeNormalization;

fn main() {
    let decomposed = "e\u{301}";
    let nfc: String = decomposed.nfc().collect();
    assert_eq!(nfc, "\u{e9}");
}
```

(کد کامل و تست‌شده در پروژه‌ی `strings-unicode`، تابع `normalize_for_search`.)

## Grapheme Cluster

استاندارد یونیکد (UAX #29) قواعد «یک حرف از دید کاربر» را مشخص می‌کند. برای آن از `unicode-segmentation` استفاده کنید:

```rust,ignore
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // «می‌خواهم» = ۸ کاراکتر ولی ۷ grapheme (نیم‌فاصله به «ی» می‌چسبد)
    let w = "می\u{200c}خواهم";
    assert_eq!(w.chars().count(), 8);
    assert_eq!(w.graphemes(true).count(), 7);

    // خانواده‌ی emoji: ۵ کاراکتر (با ZWJ) ولی ۱ grapheme
    let family = "👨‍👩‍👧";
    assert_eq!(family.len(), 18);
    assert_eq!(family.chars().count(), 5);
    assert_eq!(family.graphemes(true).count(), 1);

    // کلمات (با قواعد یونیکد، نه فقط فاصله)
    let words: Vec<&str> = "سلام دنیا، خوبی؟ hello world".unicode_words().collect();
    assert_eq!(words, ["سلام", "دنیا", "خوبی", "hello", "world"]);
}
```

برش امن برای نمایش («n حرف اول»):

```rust,ignore
// از پروژه‌ی strings-unicode
pub fn take_graphemes(s: &str, n: usize) -> &str {
    match s.grapheme_indices(true).nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}
```

این تابع هرگز یک emoji، accent یا نیم‌فاصله را از وسط نمی‌شکند.

## متن راست‌به‌چپ (RTL)

نکته‌ی مهم: **Rust و رشته‌ها ترتیب منطقی (logical order) را ذخیره می‌کنند**، یعنی همان ترتیبی که تایپ می‌شود. «راست‌به‌چپ بودن» فقط در لایه‌ی **نمایش** (ترمینال، مرورگر، UI) اعمال می‌شود (الگوریتم Bidi). پس:

- اندیس ۰ در `"سلام"` همیشه «س» است، حتی اگر روی صفحه سمت راست دیده شود.
- `chars().rev()` ترتیب **منطقی** را برعکس می‌کند، نه ترتیب بصری.
- ترکیب متن فارسی با اعداد یا انگلیسی در ترمینال ممکن است به‌هم‌ریخته نمایش داده شود؛ این مشکل ترمینال است، نه داده‌ی شما.
- در ترمینال، هم‌ترازی ستون‌ها با `{:<10}` برای متن فارسی دقیق نیست چون عرض نمایشی کاراکترها یکسان نیست. برای ابزار جدی از `unicode-width` استفاده کنید.

برای نمایش مطمئن در UI، تا حد ممکن از کتابخانه‌ی UI استفاده کنید که Bidi را خودش انجام می‌دهد.

## بایت‌های نامعتبر: from_utf8 و lossy

داده‌ی ورودی (فایل، شبکه) همیشه UTF-8 معتبر نیست:

```rust
fn main() {
    let bad = vec![b'a', 0xFF, 0xFE];

    // ۱. با بررسی: خطا می‌دهد
    assert!(String::from_utf8(bad.clone()).is_err());
    assert!(std::str::from_utf8(&bad).is_err());

    // ۲. بدون خطا: بایت‌های نامعتبر با U+FFFD (�) جایگزین می‌شوند
    let s = String::from_utf8_lossy(&bad);
    assert_eq!(s, "a\u{FFFD}\u{FFFD}");

    // ۳. فقط بخش معتبر ابتدا
    let err = std::str::from_utf8(&bad).unwrap_err();
    assert_eq!(err.valid_up_to(), 1);
}
```

برای فایل‌هایی با کدگذاری قدیمی فارسی (Windows-1256، ISIRI) باید از crate ی `encoding_rs` برای تبدیل استفاده کنید؛ Rust فقط UTF-8 را مستقیم می‌شناسد.

## ساخت رشته‌ی کارآمد

```rust
use std::fmt::Write;

fn main() {
    // ۱. ظرفیت را از قبل بدهید تا بازتخصیص کم شود
    let mut s = String::with_capacity(64);
    assert!(s.capacity() >= 64);

    // ۲. push_str و push
    s.push_str("سلام");
    s.push(' ');

    // ۳. write! مستقیم در String (بدون String موقتی)
    write!(s, "{}+{}={}", 1, 2, 1 + 2).unwrap();
    assert_eq!(s, "سلام 1+2=3");

    // ۴. الحاق یک مجموعه: join و concat
    let parts = ["الف", "ب", "پ"];
    assert_eq!(parts.join("-"), "الف-ب-پ");
    assert_eq!(parts.concat(), "الفبپ");

    // ۵. repeat
    assert_eq!("=".repeat(5), "=====");

    // ۶. جمع با collect
    let joined: String = ["a", "b"].iter().copied().collect();
    assert_eq!(joined, "ab");
}
```

> **ضدالگو:** `s = s + &other` در یک حلقه برای رشته‌های بزرگ؛ `s.push_str(&other)` بهتر است. `format!` هم هر بار یک `String` جدید می‌سازد، پس در حلقه‌ی داغ `write!` به یک بافر مشترک بهتر است.

## جستجو و تغییر

```rust
fn main() {
    let text = "Rust is fast, Rust is safe";

    assert!(text.contains("fast"));
    assert!(text.starts_with("Rust"));
    assert!(text.ends_with("safe"));

    assert_eq!(text.find("Rust"), Some(0));       // اندیس بایت!
    assert_eq!(text.rfind("Rust"), Some(14));
    assert_eq!(text.matches("Rust").count(), 2);

    assert_eq!(text.replace("Rust", "Go"), "Go is fast, Go is safe");
    assert_eq!(text.replacen("Rust", "Go", 1), "Go is fast, Rust is safe");

    assert_eq!(text.split(", ").collect::<Vec<_>>(), ["Rust is fast", "Rust is safe"]);
    assert_eq!(text.split_once(", "), Some(("Rust is fast", "Rust is safe")));
    assert_eq!("k=v=w".splitn(2, '=').collect::<Vec<_>>(), ["k", "v=w"]);
    assert_eq!("a1b2c3".split(|c: char| c.is_ascii_digit()).collect::<Vec<_>>(), ["a", "b", "c", ""]);

    assert_eq!("  x  ".trim(), "x");
    assert_eq!("  x  ".trim_start(), "x  ");
    assert_eq!("xxhixx".trim_matches('x'), "hi");
    assert_eq!("prefix-body".strip_prefix("prefix-"), Some("body"));
    assert_eq!("body.rs".strip_suffix(".rs"), Some("body"));

    // lines() هم \n و هم \r\n را می‌شناسد
    assert_eq!("a\r\nb\nc".lines().collect::<Vec<_>>(), ["a", "b", "c"]);
}
```

## Escape و رشته‌ی خام

```rust
fn main() {
    let a = "خط اول\nخط دوم\tتب \"نقل‌قول\" و \\";
    let b = r"C:\path\no\escape";           // raw string
    let c = r#"او گفت: "سلام""#;            // raw با #
    let d = "یک رشته‌ی \
             چندخطی که خط جدید را حذف می‌کند";
    let e = b"bytes";                        // &[u8; 5]
    let f = "\u{1F600} \x41";                // یونیکد و hex

    assert_eq!(b, "C:\\path\\no\\escape");
    assert_eq!(c, "او گفت: \"سلام\"");
    assert_eq!(d, "یک رشته‌ی چندخطی که خط جدید را حذف می‌کند");
    assert_eq!(e.len(), 5);
    assert_eq!(f, "😀 A");
    assert!(a.contains('\n'));
}
```

برای چاپ امن یک رشته‌ی ناشناخته (مثلاً در لاگ): `s.escape_debug()` و `s.escape_default()`:

```rust
fn main() {
    // escape_debug کاراکترهای نامرئی مثل نیم‌فاصله را آشکار می‌کند
    assert_eq!("a\nb\u{200c}".escape_debug().to_string(), "a\\nb\\u{200c}");
    assert_eq!("a\nb".escape_default().to_string(), "a\\nb");

    // {:?} هم همین کار را می‌کند: برای خطایابی متن فارسی بسیار مفید است
    assert_eq!(format!("{:?}", "a\u{200c}b"), "\"a\\u{200c}b\"");
}
```

## تبدیل: parse و FromStr

```rust
fn main() {
    assert_eq!("42".parse::<i32>(), Ok(42));
    assert_eq!("3.5".parse::<f64>(), Ok(3.5));
    assert_eq!("true".parse::<bool>(), Ok(true));
    assert_eq!("x".parse::<char>(), Ok('x'));
    assert!("4 2".parse::<i32>().is_err());
    assert!(" 42".parse::<i32>().is_err()); // فاصله مجاز نیست؛ اول trim کنید
    assert_eq!("+7".parse::<i32>(), Ok(7));
    assert_eq!(i32::from_str_radix("ff", 16), Ok(255));
}
```

برای نوع خودتان `FromStr` را پیاده کنید ([فصل Trait های std](std-traits-conversions.md)).

## OsString، OsStr و Path

نام فایل در سیستم‌عامل‌ها **لزوماً UTF-8 نیست** (یونیکس: بایت دلخواه؛ ویندوز: UTF-16 بالقوه نامعتبر). پس std نوع جدا دارد:

| نوع | همتای |
|---|---|
| `OsStr` / `OsString` | `str` / `String` برای ارتباط با سیستم‌عامل |
| `Path` / `PathBuf` | روی `OsStr` ساخته شده |
| `CStr` / `CString` | رشته‌های C با پایان `\0` (فصل [FFI](ffi-wasm-embedded.md)) |

```rust
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let p = Path::new("پوشه/فایل.txt");
    let name: &OsStr = p.file_name().unwrap();

    assert_eq!(name.to_str(), Some("فایل.txt"));        // None اگر UTF-8 نباشد
    assert_eq!(name.to_string_lossy(), "فایل.txt");     // Cow<str>، با جایگزینی
    assert_eq!(p.extension(), Some(OsStr::new("txt")));
}
```

## Cow: کپی فقط در صورت نیاز

وقتی تابع «گاهی» رشته را تغییر می‌دهد و بیشتر وقت‌ها نه، `Cow<str>` از تخصیص بی‌مورد جلوگیری می‌کند:

```rust
use std::borrow::Cow;

fn remove_zwnj(s: &str) -> Cow<'_, str> {
    if s.contains('\u{200c}') {
        Cow::Owned(s.replace('\u{200c}', ""))
    } else {
        Cow::Borrowed(s)
    }
}

fn main() {
    let a = remove_zwnj("سلام");
    let b = remove_zwnj("می\u{200c}خواهم");
    assert!(matches!(a, Cow::Borrowed(_)));   // کپی نشد
    assert!(matches!(b, Cow::Owned(_)));      // فقط اینجا ساخته شد
    assert_eq!(b, "میخواهم");
}
```

## کدام نوع رشته را انتخاب کنم؟

| موقعیت | انتخاب |
|---|---|
| پارامتر ورودی تابع که فقط می‌خواند | `&str` |
| متن را ذخیره یا تغییر می‌دهید | `String` |
| ورودی که «چیزی شبیه رشته» است | `impl AsRef<str>` یا `impl Into<String>` |
| گاهی تغییر می‌دهید، اغلب نه | `Cow<'a, str>` |
| رشته‌ی ثابت در برنامه | `&'static str` |
| داده‌ی باینری / ممکن است UTF-8 نباشد | `&[u8]` / `Vec<u8>` |
| نام فایل | `&Path` / `PathBuf` |
| رشته‌ی بسیار فراوان با تکرار زیاد | بررسی interning (مثلاً crate های `string-interner`) |

## چک‌لیست متن فارسی در برنامه‌ی واقعی

۱. ورودی کاربر: `NFC` + یکسان‌سازی «ی/ک» + تبدیل ارقام به ASCII (برای ذخیره و جستجو).
۲. نیم‌فاصله را در داده نگه دارید (برای نمایش لازم است) ولی در **کلید جستجو** حذفش کنید.
۳. هرگز با اندیس بایت برش ندهید؛ `get()` یا grapheme.
۴. برای مرتب‌سازی الفبایی فارسی، `sort()` ساده بر اساس نقطه‌ی کد است و ترتیب الفبایی درست نمی‌دهد؛ برای مرتب‌سازی زبانی (collation) از ICU (`icu_collator`) استفاده کنید.
۵. برای حروف بزرگ و کوچک لاتین کنار فارسی، `to_lowercase` را روی کل رشته صدا بزنید؛ حروف فارسی بدون تغییر می‌مانند.

## تمرین

1. تابعی بنویسید که تعداد بایت، کاراکتر و grapheme یک رشته را به‌صورت tuple برگرداند، و آن را روی «سلام»، «می‌خواهم» و یک emoji مرکب امتحان کنید.
2. تابع `slugify` بنویسید که متن فارسی را به آدرس URL تبدیل کند (حروف مجاز: حرف، رقم؛ نیم‌فاصله و فاصله → `-`).
3. تابعی بنویسید که ارقام فارسی داخل یک متن را به ASCII تبدیل کند و سپس همه‌ی اعداد متن را جمع بزند.
4. با `char_indices` تابعی بنویسید که رشته را از وسط یک کاراکتر **نمی‌شکند** و حداکثر n بایت اول را برمی‌گرداند.
5. توضیح دهید چرا `"é" == "e\u{301}"` نادرست است و راه‌حلی برای مقایسه‌ی آن‌ها ارائه دهید.
