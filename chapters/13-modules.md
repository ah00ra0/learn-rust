# Package، Crate و Module

وقتی برنامه بزرگ‌تر می‌شود، باید کد را سازمان‌دهی کنیم. سیستم ماژول Rust چند سطح دارد:

| مفهوم | توضیح |
|---|---|
| **Package** | یک یا چند crate که با Cargo ساخته، تست و منتشر می‌شوند (دارای `Cargo.toml`) |
| **Crate** | یک واحد کامپایل: یا فایل اجرایی (binary) یا کتابخانه (library) |
| **Module** | سازمان‌دهی داخل crate و کنترل **خصوصی/عمومی** بودن |
| **Path** | نام‌گذاری یک آیتم (تابع، struct و ...) |

## Crate Root

- برای پروژه‌ی اجرایی: فایل `src/main.rs`
- برای کتابخانه: فایل `src/lib.rs`

یک package می‌تواند یک `lib.rs` و چند binary داشته باشد (در پوشه‌ی `src/bin/`).

## تعریف ماژول

```rust
mod restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("به لیست انتظار اضافه شد");
        }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
    }
}

fn main() {
    restaurant::hosting::add_to_waitlist();
}
```

## Path (مسیر)

- **مسیر مطلق**: از ریشه‌ی crate با `crate::`
- **مسیر نسبی**: از ماژول فعلی، یا با `self::` و `super::`

```rust
mod restaurant {
    pub mod hosting {
        pub fn seat() {}
    }

    pub fn eat_at_restaurant() {
        crate::restaurant::hosting::seat(); // مطلق
        hosting::seat();                    // نسبی
    }

    fn deliver_order() {}

    pub mod back_of_house {
        pub fn fix_order() {
            cook_order();
            super::deliver_order(); // یک ماژول بالاتر
        }
        fn cook_order() {}
    }
}

fn main() {
    restaurant::eat_at_restaurant();
    restaurant::back_of_house::fix_order();
}
```

## خصوصی و عمومی (pub)

قاعده: همه‌چیز به‌طور پیش‌فرض **خصوصی** است.

- کد **فرزند** می‌تواند آیتم‌های خصوصی **اجدادش** را ببیند.
- کد **پدر** نمی‌تواند آیتم‌های خصوصی **فرزند** را ببیند.

```rust,compile_fail
mod a {
    mod b {
        pub fn f() {}
    }
}

fn main() {
    a::b::f(); // ✗ خطا: ماژول b خصوصی است
}
```

### pub در struct و enum

- در **struct**، هر فیلد جداگانه باید `pub` شود.
- در **enum**، اگر enum عمومی باشد همه‌ی حالت‌هایش عمومی‌اند.

```rust
mod shop {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,   // خصوصی
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("هلو"),
            }
        }
        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let mut meal = shop::Breakfast::summer("چاودار");
    meal.toast = String::from("گندم");
    println!("{} {}", meal.toast, meal.fruit());
    let _order = shop::Appetizer::Soup;
    let _ = shop::Appetizer::Salad;
}
```

## کلمه‌ی use

برای کوتاه کردن مسیرها:

```rust
mod front {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

قراردادها:

- برای **توابع**، ماژول والد را import کنید (`use front::hosting;`) تا مشخص باشد تابع محلی نیست.
- برای **struct و enum**، مسیر کامل را import کنید (`use std::collections::HashMap;`).

### نام مستعار با as

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let _a: Option<Result> = None;
    let _b: Option<IoResult<()>> = None;
}
```

### use های تو در تو و glob

```rust
use std::{cmp::Ordering, collections::HashMap, io};
use std::collections::*;   // همه‌چیز؛ به‌جز در تست‌ها و prelude توصیه نمی‌شود

fn main() {
    let _ = (Ordering::Less, HashMap::<i32, i32>::new(), io::stdout());
    let _ = BTreeMap::<i32, i32>::new();
}
```

### re-export با pub use

```rust
mod internal {
    pub mod deep {
        pub fn helper() {}
    }
}

pub use internal::deep::helper;

fn main() {
    helper();
}
```

این الگو برای ساخت API تمیز در کتابخانه‌ها رایج است.

## جداسازی ماژول‌ها در فایل‌ها

وقتی ماژول بزرگ شد، آن را به فایل جدا ببرید. در `src/main.rs`:

```rust,ignore
mod restaurant;      // به‌دنبال src/restaurant.rs یا src/restaurant/mod.rs می‌گردد

fn main() {
    restaurant::hosting::add_to_waitlist();
}
```

فایل `src/restaurant.rs`:

```rust,ignore
pub mod hosting;     // به‌دنبال src/restaurant/hosting.rs می‌گردد
```

فایل `src/restaurant/hosting.rs`:

```rust,ignore
pub fn add_to_waitlist() {}
```

ساختار نهایی:

```text
src/
├── main.rs
├── restaurant.rs
└── restaurant/
    └── hosting.rs
```

> کلمه‌ی `mod` یعنی "این ماژول را از فایل بخوان و به درخت ماژول اضافه کن". یک بار `mod` کافی است؛ نباید آن را مثل `include` در C همه‌جا تکرار کنید.

## استفاده از Crate های خارجی

```bash
cargo add rand
```

```rust,ignore
use rand::Rng;

fn main() {
    let n: u32 = rand::thread_rng().gen_range(1..=100);
    println!("{n}");
}
```

جستجوی کتابخانه‌ها: [crates.io](https://crates.io) و مستندات هر crate در [docs.rs](https://docs.rs).

## تمرین

1. ماژولی به نام `math` با توابع `add` و `multiply` بسازید و از `main` صدایشان بزنید.
2. ماژول را به فایل `math.rs` منتقل کنید.
3. struct عمومی با فیلد خصوصی و تابع سازنده‌ی عمومی بسازید.
4. یک crate از crates.io اضافه کنید و از آن استفاده کنید.

فصل بعد: [مدیریت خطا](14-errors.md).
