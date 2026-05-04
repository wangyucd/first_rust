// ============================================================
// Rust 基础教程 02: 函数
// 运行: cargo run --example 02_functions
// ============================================================

fn main() {
    // ==================== 基本函数调用 ====================
    greet();
    greet_name("Rust 学习者");

    // ==================== 带参数的函数 ====================
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    // ==================== 返回值 ====================
    let result = multiply(4, 6);
    println!("4 × 6 = {}", result);

    // ==================== 表达式 vs 语句 ====================
    // Rust 中函数体由一系列语句组成，最后一个表达式是返回值
    let val = expression_demo();
    println!("表达式演示: {}", val);

    // ==================== 提前返回 ====================
    let max = max_of_two(10, 20);
    println!("max(10, 20) = {}", max);

    // ==================== 多返回值 (元组) ====================
    let (quotient, remainder) = div_rem(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);

    // ==================== 函数作为参数 ====================
    let result = apply(5, double);
    println!("apply(5, double) = {}", result);

    let result = apply(5, triple);
    println!("apply(5, triple) = {}", result);

    // ==================== 闭包 (匿名函数) ====================
    // 闭包可以捕获环境中的变量
    let offset = 10;
    let add_offset = |x| x + offset; // 闭包捕获了 offset
    println!("add_offset(5) = {}", add_offset(5));

    // 闭包作为参数
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("map 翻倍: {:?}", doubled);

    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter 偶数: {:?}", evens);

    // ==================== 高阶函数 ====================
    let sum = (1..=100).fold(0, |acc, x| acc + x);
    println!("1 到 100 的和 = {}", sum);

    // ==================== 永不返回的函数 ====================
    // fn diverge() -> ! { panic!("永不返回") }
    // -> ! 表示发散函数，永远不会返回

    // ==================== 方法 (关联函数) ====================
    // 使用 impl 为类型定义方法，见结构体教程
    let rect = Rectangle::new(10.0, 5.0);
    println!("矩形面积 = {}", rect.area());
    println!("矩形周长 = {}", rect.perimeter());

    println!("\n✅ 函数教程完成！");
}

// ==================== 函数定义 ====================

// 无参数无返回值
fn greet() {
    println!("你好，世界！");
}

// 带参数的函数 - 参数必须标注类型
fn greet_name(name: &str) {
    println!("你好，{}！", name);
}

// 带返回值的函数 - 使用 -> 标注返回类型
fn add(a: i32, b: i32) -> i32 {
    a + b // 注意：没有分号，这是一个表达式，作为返回值
}

// 等价写法：使用 return 关键字
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

// 表达式演示
fn expression_demo() -> i32 {
    let x = 5;
    // 代码块 {} 也是表达式，其值是最后一个表达式的值
    let y = {
        let a = 3;
        let b = 4;
        a + b // 没有分号 -> 表达式，值为 7
    };
    x + y // 返回 12
}

// 提前返回
fn max_of_two(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

// 返回多个值 (元组)
fn div_rem(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// ==================== 函数指针 ====================

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

// 接受函数指针作为参数
fn apply(x: i32, f: fn(i32) -> i32) -> i32 {
    f(x)
}

// ==================== 结构体方法 ====================

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 关联函数 (类似其他语言的静态方法/构造函数)
    // 没有 self 参数，用 :: 调用
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // 方法 - &self 表示不可变借用
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}