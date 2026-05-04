// ============================================================
// Rust 基础教程 11: 特征 (Trait)
// 运行: cargo run --example 11_traits
// ============================================================

use std::fmt;

fn main() {
    // ==================== 定义和实现 Trait ====================
    let article = NewsArticle {
        title: String::from("Rust 1.80 发布"),
        author: String::from("Rust Team"),
        content: String::from("Rust 1.80 带来了许多新特性..."),
    };
    println!("文章摘要: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("刚学了 Rust 的 Trait，太强大了！"),
        reply: false,
        retweet: false,
    };
    println!("推文摘要: {}", tweet.summarize());

    // ==================== 默认实现 ====================
    // Summary trait 有默认实现
    let article = NewsArticle {
        title: String::from("标题"),
        author: String::from("作者"),
        content: String::from("内容"),
    };
    println!("默认摘要: {}", article.default_summary());

    // ==================== Trait 作为参数 ====================
    notify(&article);
    notify(&tweet);

    // ==================== Trait Bound 语法 ====================
    // impl Trait 语法 (语法糖)
    notify(&article);

    // 完整的泛型 + Trait Bound 语法
    notify_explicit(&article);

    // 多个 Trait Bound
    notify_display(&article);

    // where 子句 (更清晰的写法)
    notify_where(&article);

    // ==================== 返回 impl Trait ====================
    let summary = create_summary();
    println!("返回 impl Trait: {}", summary.summarize());

    // ==================== Trait 对象 (动态分发) ====================
    // 当需要在运行时确定具体类型时，使用 trait 对象
    let articles: Vec<Box<dyn Summary>> = vec![
        Box::new(NewsArticle {
            title: String::from("文章1"),
            author: String::from("作者1"),
            content: String::from("内容1"),
        }),
        Box::new(Tweet {
            username: String::from("user1"),
            content: String::from("推文1"),
            reply: false,
            retweet: false,
        }),
    ];

    for item in &articles {
        println!("动态分发: {}", item.summarize());
    }

    // ==================== 常用标准库 Trait ====================

    // Display trait
    let p = Point { x: 3.0, y: 4.0 };
    println!("Display: {}", p);

    // Debug trait
    println!("Debug: {:?}", p);

    // Clone 和 Copy
    let p1 = Point3D { x: 1, y: 2, z: 3 };
    let p2 = p1; // Copy，p1 仍然有效
    println!("Copy: p1={:?}, p2={:?}", p1, p2);

    // PartialEq
    println!("相等: {}", p1 == p2);
    println!("不等: {}", p1 != Point3D { x: 1, y: 2, z: 4 });

    // Default
    let p: Point3D = Default::default();
    println!("Default: {:?}", p);

    // From 和 Into
    let p = Point3D::from((10, 20, 30));
    println!("From tuple: {:?}", p);

    // ==================== Trait 继承 ====================
    let dog = Dog { name: String::from("旺财") };
    dog.speak();
    dog.fetch();

    let cat = Cat { name: String::from("咪咪") };
    cat.speak();
    cat.purr();

    // ==================== 实用示例: 可打印的集合 ====================
    let list = PrintableList {
        items: vec![1, 2, 3, 4, 5],
    };
    println!("{}", list);

    let list = PrintableList {
        items: vec!["hello", "world", "rust"],
    };
    println!("{}", list);

    println!("\n✅ Trait 教程完成！");
}

// ==================== 定义 Trait ====================

trait Summary {
    // 必须实现的方法
    fn summarize(&self) -> String;

    // 默认实现
    fn default_summary(&self) -> String {
        String::from("(阅读更多...)")
    }
}

// ==================== 为类型实现 Trait ====================

struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, 作者: {}", self.title, self.author)
    }
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summarize())
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// ==================== Trait 作为参数 ====================

// impl Trait 语法
fn notify(item: &impl Summary) {
    println!("通知: {}", item.summarize());
}

// Trait Bound 语法 (等价写法)
fn notify_explicit<T: Summary>(item: &T) {
    println!("通知: {}", item.summarize());
}

// 多个 Trait Bound
fn notify_display(item: &(impl Summary + fmt::Display)) {
    println!("通知: {}", item.summarize());
}

// where 子句
fn notify_where<T>(item: &T)
where
    T: Summary + fmt::Display,
{
    println!("通知: {}", item.summarize());
}

// ==================== 返回 impl Trait ====================

fn create_summary() -> impl Summary {
    Tweet {
        username: String::from("bot"),
        content: String::from("自动生成的摘要"),
        reply: false,
        retweet: false,
    }
}

// ==================== 常用标准库 Trait ====================

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Point3D {
    fn default() -> Self {
        Point3D { x: 0, y: 0, z: 0 }
    }
}

impl From<(i32, i32, i32)> for Point3D {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Point3D {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ==================== Trait 继承 ====================

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) {
        println!("{} 发出了声音", self.name());
    }
}

trait Pet: Animal {
    fn fetch(&self) {
        println!("{} 去捡球了", self.name());
    }
}

trait CatLike: Animal {
    fn purr(&self) {
        println!("{}: 呼噜呼噜~", self.name());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    fn speak(&self) {
        println!("{}: 汪汪!", self.name);
    }
}

impl Pet for Dog {}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn speak(&self) {
        println!("{}: 喵喵!", self.name);
    }
}

impl CatLike for Cat {}

// ==================== 实用示例 ====================

struct PrintableList<T: fmt::Display> {
    items: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for PrintableList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.items.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", items.join(", "))
    }
}