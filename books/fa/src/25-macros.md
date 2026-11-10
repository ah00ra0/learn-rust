# Macro

**Macro** کدی است که **کد دیگری تولید می‌کند** (metaprogramming). تا اینجا بارها استفاده‌شان کرده‌اید: `println!`، `vec!`، `format!`، `assert_eq!`، `matches!`. علامت `!` نشان‌دهنده‌ی macro است.

## تفاوت macro و تابع

- تابع تعداد پارامتر ثابت دارد؛ macro می‌تواند **تعداد دلخواه** بگیرد (`println!("a")` یا `println!("{} {}", a, b)`).
- macro قبل از کامپایل **گسترش** می‌یابد، پس می‌تواند trait پیاده کند یا نوع بسازد.
- نوشتنشان سخت‌تر و دیباگشان دشوارتر است.

Rust دو نوع macro دارد:

1. **Declarative** با `macro_rules!`
2. **Procedural**: سه شکل derive، attribute-like و function-like

## macro_rules!

ساخت یک نسخه‌ی ساده از `vec!`:

```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = Vec::new();
            $( temp.push($x); )*
            temp
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    println!("{v:?}");
}
```

اجزا:

- `( ... ) => { ... }`: الگو و کد خروجی (مثل `match`).
- `$x:expr`: ضبط یک **expression** و نام‌گذاری آن `$x`.
- `$( ... ),*`: تکرار صفر یا چند بار با جداکننده‌ی `,`.
- `$( temp.push($x); )*` در خروجی، به‌ازای هر تکرار کد را تولید می‌کند.

### انواع fragment

| نوع | مثال |
|---|---|
| `expr` | `2 + 3` |
| `ident` | نام متغیر یا تابع |
| `ty` | نوع مثل `i32` |
| `tt` | یک token tree (انعطاف‌پذیرترین) |
| `pat` | الگو |
| `stmt` / `block` / `item` / `literal` | ... |

### چند شاخه

```rust
macro_rules! maximum {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+) => {
        {
            let a = $x;
            let b = maximum!($($rest),+);
            if a > b { a } else { b }
        }
    };
}

fn main() {
    println!("{}", maximum!(3));
    println!("{}", maximum!(3, 9, 4));
}
```

### تولید تابع و struct

```rust
macro_rules! make_getter {
    ($name:ident, $field:ident, $ty:ty) => {
        fn $name(&self) -> &$ty {
            &self.$field
        }
    };
}

struct User {
    name: String,
    age: u32,
}

impl User {
    make_getter!(get_name, name, String);
    make_getter!(get_age, age, u32);
}

fn main() {
    let u = User { name: "علی".into(), age: 30 };
    println!("{} {}", u.get_name(), u.get_age());
}
```

### macro برای HashMap literal

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

fn main() {
    let ages = hashmap! {
        "علی" => 30,
        "سارا" => 22,
    };
    println!("{}", ages["سارا"]);
}
```

## Procedural Macro

این macro ها یک تابع Rust هستند که **جریان token ها** را می‌گیرند و جریان جدید برمی‌گردانند. باید در crate جداگانه با `proc-macro = true` نوشته شوند (و معمولاً از crate های `syn` و `quote` کمک می‌گیرند).

### ۱. Derive Macro

همان `#[derive(Debug)]` که می‌شناسید. crate ی مثل `serde` با `#[derive(Serialize, Deserialize)]` کد تبدیل به JSON را تولید می‌کند:

```rust,ignore
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}
```

### ۲. Attribute-like

```rust,ignore
#[route(GET, "/")]
fn index() {}
```

### ۳. Function-like

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id = 1);
```

اسکلت یک derive macro (کد به‌خاطر نیاز به crate مستقل فقط برای مشاهده است):

```rust,ignore
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("سلام! من {} هستم", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

## کی macro بنویسیم؟

- وقتی تابع و generic کافی نیست: تعداد آرگومان متغیر، تولید کد تکراری، DSL.
- ابتدا تابع یا generic را امتحان کنید. macro آخرین گزینه است.
- ابزار `cargo expand` (باید نصب شود) خروجی گسترش‌یافته‌ی macro را نشان می‌دهد و برای یادگیری و دیباگ عالی است.

## تمرین

1. macro ی `square!(x)` بنویسید که مربع یک expression را حساب کند.
2. macro ی `min!` مانند `maximum!` بنویسید.
3. macro ی بنویسید که برای یک struct با لیست فیلدها، تابع `new` تولید کند.

فصل بعد: [Async و await](26-async.md).
