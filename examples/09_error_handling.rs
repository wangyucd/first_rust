// ============================================================
// Rust 基础教程 09: 错误处理 (Error Handling)
// 运行: cargo run --example 09_error_handling
// ============================================================

use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    // ==================== panic! 不可恢复错误 ====================
    // panic! 会导致程序崩溃
    // panic!("这是一个崩溃"); // 取消注释会 panic

    // 数组越界也会 panic
    // let v = vec![1, 2, 3];
    // v[99]; // panic: index out of bounds

    // ==================== Result<T, E> 可恢复错误 ====================
    // Result 是一个枚举: Ok(T) 或 Err(E)

    let result = divide(10.0, 3.0);
    println!("10 / 3 = {:?}", result);

    let result = divide(10.0, 0.0);
    println!("10 / 0 = {:?}", result);

    // ==================== match 处理 Result ====================
    let filename = "test.txt";
    match read_file(filename) {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取失败: {}", e),
    }

    // ==================== unwrap 和 expect ====================
    // unwrap: 成功返回值，失败 panic
    let good: Result<i32, &str> = Ok(42);
    println!("unwrap: {}", good.unwrap());

    // expect: 类似 unwrap，但可以自定义错误消息
    // let bad: Result<i32, &str> = Err("错误");
    // bad.expect("期望成功"); // panic: 期望成功: "错误"

    // ==================== ? 运算符 ====================
    // ? 运算符用于传播错误，等价于 match + return Err
    match parse_and_double("21") {
        Ok(n) => println!("解析并翻倍: {}", n),
        Err(e) => println!("错误: {}", e),
    }

    match parse_and_double("abc") {
        Ok(n) => println!("解析并翻倍: {}", n),
        Err(e) => println!("错误: {}", e),
    }

    // ==================== 自定义错误类型 ====================
    match read_config("config.txt") {
        Ok(config) => println!("配置: {}", config),
        Err(e) => println!("配置错误: {}", e),
    }

    // ==================== Option<T> 处理 ====================
    // Option 用于可能不存在的值
    let numbers = vec![1, 2, 3, 4, 5];

    // find 返回 Option
    let found = numbers.iter().find(|&&x| x > 3);
    println!("find > 3: {:?}", found);

    // first 返回 Option
    let first = numbers.first();
    println!("first: {:?}", first);

    let empty: Vec<i32> = vec![];
    let first = empty.first();
    println!("empty first: {:?}", first);

    // map 转换 Option
    let doubled = found.map(|x| x * 2);
    println!("map: {:?}", doubled);

    // and_then 链式操作
    let result = Some(3)
        .and_then(|x| if x > 0 { Some(x * 2) } else { None })
        .and_then(|x| Some(x + 1));
    println!("and_then 链: {:?}", result);

    // unwrap_or 提供默认值
    let none: Option<i32> = None;
    println!("unwrap_or: {}", none.unwrap_or(0));
    println!("unwrap_or_else: {}", none.unwrap_or_else(|| 42));

    // ==================== 组合使用 ====================
    // 处理多个可能失败的操作
    match process_data("42", "58") {
        Ok(result) => println!("处理结果: {}", result),
        Err(e) => println!("处理失败: {}", e),
    }

    match process_data("42", "abc") {
        Ok(result) => println!("处理结果: {}", result),
        Err(e) => println!("处理失败: {}", e),
    }

    // ==================== 收集 Result ====================
    // collect 可以将 Result 的迭代器收集为 Vec 或错误
    let strings = vec!["1", "2", "3", "4", "5"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("全部解析成功: {:?}", numbers);

    let strings = vec!["1", "2", "abc", "4"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("解析失败: {:?}", numbers);

    // partition 分离成功和失败
    let strings = vec!["1", "2", "abc", "4", "xyz"];
    let (successes, failures): (Vec<_>, Vec<_>) = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();
    let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();
    println!("成功: {:?}", successes);
    println!("失败: {:?}", failures);

    // ==================== 实用示例: 链式错误处理 ====================
    match parse_csv_line("10,20,30") {
        Ok(sum) => println!("CSV 求和: {}", sum),
        Err(e) => println!("CSV 错误: {}", e),
    }

    match parse_csv_line("10,abc,30") {
        Ok(sum) => println!("CSV 求和: {}", sum),
        Err(e) => println!("CSV 错误: {}", e),
    }

    println!("\n✅ 错误处理教程完成！");
}

// ==================== 返回 Result 的函数 ====================

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

// ==================== ? 运算符 ====================
// ? 运算符: 如果是 Ok，取出值继续；如果是 Err，提前返回错误
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?; // 如果解析失败，直接返回 Err
    Ok(n * 2)
}

// ==================== 自定义错误 ====================

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO 错误: {}", e),
            ConfigError::ParseError(msg) => write!(f, "解析错误: {}", msg),
        }
    }
}

// 实现 From trait 以便使用 ? 运算符自动转换
impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

fn read_config(filename: &str) -> Result<String, ConfigError> {
    let content = fs::read_to_string(filename)?; // io::Error 自动转换为 ConfigError
    if content.is_empty() {
        return Err(ConfigError::ParseError("配置文件为空".to_string()));
    }
    Ok(content)
}

// ==================== 组合使用 ====================

fn process_data(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

// ==================== 实用示例 ====================

fn parse_csv_line(line: &str) -> Result<i32, ParseIntError> {
    let sum: Result<i32, _> = line
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .try_fold(0, |acc, r| r.map(|n| acc + n));
    sum
}