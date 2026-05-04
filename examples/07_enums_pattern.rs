// ============================================================
// Rust 基础教程 07: 枚举与模式匹配 (Enum & Pattern Matching)
// 运行: cargo run --example 07_enums_pattern
// ============================================================

fn main() {
    // ==================== 基本枚举 ====================
    let four = Direction::North;
    let home = Direction::South;
    println!("方向: {:?}, {:?}", four, home);

    // match 匹配枚举
    let dir = Direction::East;
    match dir {
        Direction::North => println!("向北"),
        Direction::South => println!("向南"),
        Direction::East => println!("向东"),
        Direction::West => println!("向西"),
    }

    // ==================== 带数据的枚举 ====================
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 128, 0);

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);

    // ==================== Option<T> ====================
    // Rust 没有 null，用 Option 表示值可能存在或不存在
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // 必须处理 None 的情况才能使用内部值
    match some_number {
        Some(n) => println!("有值: {}", n),
        None => println!("无值"),
    }

    // unwrap: 直接取出值，如果是 None 会 panic
    println!("unwrap: {}", some_number.unwrap());

    // unwrap_or: 提供默认值
    println!("unwrap_or: {}", no_number.unwrap_or(0));

    // map: 对 Some 中的值进行转换
    let doubled = some_number.map(|x| x * 2);
    println!("map: {:?}", doubled);

    // and_then: 链式操作
    let result = some_number.and_then(|x| {
        if x > 0 { Some(x * 10) } else { None }
    });
    println!("and_then: {:?}", result);

    // if let 简洁匹配
    if let Some(n) = some_number {
        println!("if let: {}", n);
    }

    // ==================== 枚举方法 ====================
    let ip1 = IpAddr::V4(127, 0, 0, 1);
    let ip2 = IpAddr::V6(String::from("::1"));
    println!("IP: {}, {}", ip1, ip2);
    println!("是 IPv4? {} {}", ip1.is_v4(), ip2.is_v4());

    // ==================== 带方法的枚举 ====================
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(&coin);
    println!("硬币价值: {} 美分", value);

    // ==================== 复杂模式匹配 ====================

    // 匹配守卫
    let num = Some(42);
    match num {
        Some(x) if x < 0 => println!("负数"),
        Some(x) if x == 0 => println!("零"),
        Some(x) if x > 100 => println!("大数: {}", x),
        Some(x) => println!("普通数: {}", x),
        None => println!("无值"),
    }

    // 匹配多个模式
    let x = 1;
    match x {
        1 | 2 => println!("一或二"),
        3..=9 => println!("三到九"),
        _ => println!("其他"),
    }

    // 解构结构体
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x, y: 0 } => println!("在 x 轴上: x={}", x),
        Point { x: 0, y } => println!("在 y 轴上: y={}", y),
        Point { x, y } => println!("({}, {})", x, y),
    }

    // 解构嵌套枚举
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: ({}, {}, {})", r, g, b);
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: ({}, {}, {})", h, s, v);
        }
        _ => {}
    }

    // 忽略值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("忽略: 第一个={}, 第三个={}, 第五个={}", first, third, fifth);
        }
    }

    // .. 忽略剩余
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x = {}", x),
    }

    // ==================== 枚举与迭代 ====================
    for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
        print!("{:?} ", direction);
    }
    println!();

    // ==================== 实用示例: 简单计算器 ====================
    let expr = Expr::Add(
        Box::new(Expr::Number(3.0)),
        Box::new(Expr::Mul(
            Box::new(Expr::Number(4.0)),
            Box::new(Expr::Number(5.0)),
        )),
    );
    println!("3 + 4 * 5 = {}", eval(&expr));

    println!("\n✅ 枚举与模式匹配教程完成！");
}

// ==================== 基本枚举 ====================

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ==================== 带数据的枚举 ====================

// 枚举变体可以持有不同类型和数量的数据
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },   // 命名字段 (类似结构体)
    Write(String),              // 包含一个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
    }
}

// ==================== 枚举方法 ====================

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn is_v4(&self) -> bool {
        match self {
            IpAddr::V4(..) => true,
            IpAddr::V6(..) => false,
        }
    }
}

impl std::fmt::Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => write!(f, "{}", s),
        }
    }
}

// ==================== 枚举与模式匹配示例 ====================

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("州: {:?}", state);
            25
        }
    }
}

// ==================== 嵌套枚举 ====================

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    ChangeColor(Color),
}

// ==================== 实用示例: 表达式求值 ====================

enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(a) => -eval(a),
    }
}