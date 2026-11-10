# Trait Object و برنامه‌نویسی شی‌گرا

آیا Rust شی‌گراست؟ پاسخ: **بخشی از ویژگی‌های OOP را دارد، به شکل خودش.**

| ویژگی OOP | در Rust |
|---|---|
| شیء (داده + رفتار) | `struct` / `enum` + بلوک `impl` |
| کپسوله‌سازی | `pub` و فیلدهای خصوصی |
| وراثت (Inheritance) | **ندارد.** به‌جایش trait و composition |
| چندریختی (Polymorphism) | generics با trait bound (ایستا) و trait object (پویا) |

Rust عمداً وراثت ندارد، چون به coupling زیاد و اشتراک ناخواسته‌ی کد منجر می‌شود. برای **اشتراک کد** از پیاده‌سازی پیش‌فرض trait یا composition استفاده کنید.

## کپسوله‌سازی

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self { list: vec![], average: 0.0 }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut c = AveragedCollection::new();
    c.add(10);
    c.add(20);
    println!("{}", c.average());
}
```

فیلدها خصوصی‌اند، پس نمی‌توان `average` را بدون هماهنگی با `list` تغییر داد.

## Trait Object و چندریختی پویا

مسئله: لیستی از اشیای **مختلف** که همه یک trait را پیاده می‌کنند.

با generics این ممکن نیست، چون `Vec<T>` فقط یک نوع T می‌تواند داشته باشد. راه‌حل: **`dyn Trait`**:

```rust
trait Draw {
    fn draw(&self) -> String;
}

struct Button {
    label: String,
}

struct Slider {
    max: u32,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("[ {} ]", self.label)
    }
}

impl Draw for Slider {
    fn draw(&self) -> String {
        format!("|----o---| (0..{})", self.max)
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn render(&self) {
        for c in &self.components {
            println!("{}", c.draw());
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button { label: String::from("تایید") }),
            Box::new(Slider { max: 100 }),
        ],
    };
    screen.render();
}
```

`Box<dyn Draw>` یعنی "pointer به هر نوعی که `Draw` را پیاده کرده". فراخوانی متد از طریق **vtable** (جدول توابع مجازی) در زمان اجرا تعیین می‌شود.

> نوع قرض‌گرفته هم ممکن است: `&dyn Draw`.

## Generic در برابر dyn

| | Generic (`T: Trait`) | Trait Object (`dyn Trait`) |
|---|---|---|
| زمان تعیین نوع | کامپایل (static dispatch) | اجرا (dynamic dispatch) |
| سرعت | سریع‌تر (inline ممکن است) | کمی کندتر (جدول مجازی) |
| اندازه‌ی باینری | بزرگ‌تر (monomorphization) | کوچک‌تر |
| مجموعه‌ی ناهمگن | ممکن نیست | ممکن است |

قاعده‌ی سرانگشتی: اگر مجموعه‌ی نوع‌ها را **از قبل می‌دانید** و همگن است، generic. اگر کاربر/کتابخانه‌ی دیگر می‌تواند نوع‌های جدید اضافه کند یا مجموعه‌ی ناهمگن لازم دارید، `dyn`.

وقتی مجموعه‌ی نوع‌ها **بسته و معلوم** است، **enum** اغلب بهترین انتخاب است:

```rust
enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14159 * r * r,
            Shape::Rect(w, h) => w * h,
        }
    }
}

fn main() {
    let shapes = vec![Shape::Circle(1.0), Shape::Rect(2.0, 3.0)];
    let total: f64 = shapes.iter().map(Shape::area).sum();
    println!("{total:.2}");
}
```

## Object Safety (dyn-compatible)

همه‌ی trait ها را نمی‌توان `dyn` کرد. trait فقط وقتی قابل استفاده به‌صورت `dyn` است که:

- متدهایش نوع `Self` را برنگردانند.
- متدهایش generic نباشند.

```rust,compile_fail
trait Cloner {
    fn duplicate(&self) -> Self;   // برگرداندن Self
}

fn takes(_x: &dyn Cloner) {}      // ✗ خطا: این trait به‌صورت dyn قابل استفاده نیست
```

(دلیل: وقتی نوع واقعی پنهان است، کامپایلر اندازه‌ی `Self` را نمی‌داند.)

## الگوی State با trait object

مثال کلاسیک: یک پست وبلاگ که حالت‌هایی دارد (پیش‌نویس، در انتظار بازبینی، منتشرشده):

```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;
struct Pending;
struct Published;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> { Box::new(Pending) }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
}
impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { Box::new(Published) }
}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post { state: Some(Box::new(Draft)), content: String::new() }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("امروز راست یاد گرفتم");
    println!("[{}]", post.content());   // خالی
    post.request_review();
    println!("[{}]", post.content());   // خالی
    post.approve();
    println!("[{}]", post.content());   // متن
}
```

## الگوی Type State (روش Rust-محور)

به‌جای حالت در زمان اجرا، حالت را در **نوع** کدگذاری کنید تا حالت‌های غیرمجاز اصلاً **کامپایل نشوند**:

```rust
pub struct DraftPost {
    content: String,
}

pub struct PendingPost {
    content: String,
}

pub struct PublishedPost {
    content: String,
}

impl DraftPost {
    pub fn new() -> Self {
        DraftPost { content: String::new() }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingPost {
        PendingPost { content: self.content }
    }
}

impl PendingPost {
    pub fn approve(self) -> PublishedPost {
        PublishedPost { content: self.content }
    }
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut draft = DraftPost::new();
    draft.add_text("متن پست");
    let pending = draft.request_review();
    let post = pending.approve();
    println!("{}", post.content());
    // draft.content()  ← اصلاً وجود ندارد؛ خوانده شدن پیش‌نویس در زمان کامپایل ممنوع است
}
```

این سبک در Rust بسیار رایج است: **حالت‌های نامعتبر را غیرقابل‌نمایش کنید.**

## تمرین

1. trait ی به نام `Plugin` با متد `run(&self) -> String` تعریف کنید و `Vec<Box<dyn Plugin>>` بسازید.
2. همان مسئله را یک بار با generic و یک بار با enum حل کنید و تفاوت را توضیح بدهید.
3. با الگوی type state، اتصال شبکه‌ای مدل کنید که حالت‌های `Disconnected` و `Connected` دارد و `send` فقط روی `Connected` تعریف شده باشد.

فصل بعد: [Macro](25-macros.md).
