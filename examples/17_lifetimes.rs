// ============================================================
// Rust 基础教程 17: 生命周期 (Lifetimes)
// 运行: cargo run --example 17_lifetimes
// ============================================================

fn main() {
    // ==================== 为什么需要生命周期 ====================
    // 生命周期确保引用始终有效，防止悬垂引用

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("较长的字符串: {}", result);
    }
    // 注意: 如果把 println! 移到大括号外，会编译失败
    // 因为 string2 的生命周期更短

    // ==================== 生命周期标注语法 ====================
    // 'a 是生命周期参数
    // &i32        - 引用
    // &'a i32     - 带有生命周期的引用
    // &'a mut i32 - 带有生命周期的可变引用

    let s1 = String::from("hello world");
    let s2 = "hi";
    let result = longest_with_announcement(s1.as_str(), s2, "比较字符串");
    println!("结果: {}", result);

    // ==================== 结构体中的生命周期 ====================
    // 当结构体持有引用时，必须标注生命周期

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = Excerpt {
            part: &novel[..i],
        };
        println!("摘录: {}", first_sentence.part);
        println!("摘录长度: {}", first_sentence.level());
    }

    // ==================== 生命周期省略规则 ====================
    // 编译器可以自动推断生命周期的三种情况:
    // 1. 每个引用参数都有自己的生命周期
    // 2. 只有一个输入生命周期时，它被赋给所有输出
    // 3. 方法中 &self 的生命周期被赋给所有输出

    let s = String::from("hello");
    let len = first_word_len(&s);
    println!("第一个单词长度: {}", len);

    // ==================== 静态生命周期 ====================
    // 'static 生命周期存活于整个程序期间
    let s: &'static str = "我有静态生命周期";
    println!("{}", s);

    // ==================== 泛型 + Trait Bound + 生命周期 ====================
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let result = p1.distance_to(&p2);
    println!("距离: {}", result);

    // ==================== 实用示例 ====================
    let text = String::from("hello world rust programming");
    let word = first_word(&text);
    println!("第一个单词: {}", word);

    let config = Config {
        name: "my_app",
        version: "1.0",
    };
    println!("配置: {} v{}", config.name, config.version);

    println!("\n✅ 生命周期教程完成！");
}

// ==================== 生命周期标注 ====================
// 告诉编译器: 返回值的生命周期等于两个参数中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a>(x: &'a str, y: &'a str, ann: &str) -> &'a str {
    println!("公告: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ==================== 结构体中的生命周期 ====================

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // 方法中 &self 的生命周期自动赋给返回值
    fn level(&self) -> usize {
        self.part.len()
    }

    // 显式标注
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

// ==================== 生命周期省略 ====================
// 编译器可以推断，不需要手动标注
fn first_word_len(s: &str) -> usize {
    s.find(' ').unwrap_or(s.len())
}

// ==================== 泛型 + Trait Bound + 生命周期 ====================

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> Point<T> {
    fn distance_to(&self, other: &Point<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

// ==================== 实用示例 ====================

fn first_word<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

struct Config<'a> {
    name: &'a str,
    version: &'a str,
}

// ==================== 生命周期与 Trait ====================

trait Summary<'a> {
    fn summarize(&self) -> &'a str;
}

struct Article<'a> {
    title: &'a str,
    content: &'a str,
}

impl<'a> Summary<'a> for Article<'a> {
    fn summarize(&self) -> &'a str {
        &self.content[..20.min(self.content.len())]
    }
}