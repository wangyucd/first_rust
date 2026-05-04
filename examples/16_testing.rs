// ============================================================
// Rust 基础教程 16: 测试 (Testing)
// 运行: cargo run --example 16_testing
// 注意: 测试代码通常在 #[cfg(test)] 模块中
//       这里用 main 函数演示测试概念
// ============================================================

fn main() {
    println!("===== 测试基础 =====");

    let result = add(2, 3);
    assert!(result == 5, "2 + 3 应该等于 5，实际得到 {}", result);
    println!("✓ assert! 通过");

    assert_eq!(add(2, 3), 5);
    assert_ne!(add(2, 3), 6);
    println!("✓ assert_eq! 和 assert_ne! 通过");

    println!("\n===== 测试函数 =====");
    test_add();
    println!("✓ test_add 通过");
    test_divide();
    println!("✓ test_divide 通过");
    test_greeting();
    println!("✓ test_greeting 通过");
    test_parse();
    println!("✓ test_parse 通过");

    println!("\n===== 测试工具 =====");
    let calc = Calculator::new();
    assert_eq!(calc.add(10, 20), 30);
    assert_eq!(calc.subtract(20, 10), 10);
    assert_eq!(calc.multiply(5, 6), 30);
    assert_eq!(calc.divide(20.0, 5.0), Ok(4.0));
    assert!(calc.divide(10.0, 0.0).is_err());
    println!("✓ Calculator 测试全部通过");

    let mut list = vec![3, 1, 4, 1, 5, 9, 2, 6];
    list.sort();
    assert_eq!(list, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    assert_eq!(list.binary_search(&5), Ok(4));
    println!("✓ 集合操作测试通过");

    println!("\n===== 测试模式 =====");
    let result = find_user("Alice");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "Alice");
    let result = find_user("Unknown");
    assert!(result.is_none());
    println!("✓ Option 测试通过");

    let result = parse_age("25");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 25);
    let result = parse_age("abc");
    assert!(result.is_err());
    println!("✓ Result 测试通过");

    let result = std::panic::catch_unwind(|| panic!("预期的 panic"));
    assert!(result.is_err());
    println!("✓ panic 测试通过");

    println!("\n===== 最佳实践 =====");
    println!("1. 测试函数使用 #[test] 标注");
    println!("2. 测试模块使用 #[cfg(test)] 标注");
    println!("3. 使用 assert!, assert_eq!, assert_ne!");
    println!("4. 测试应该互相独立");
    println!("5. 测试命名要描述行为");
    println!("6. 使用 should_panic 测试预期的 panic");
    println!("7. 使用 Result<T, E> 的测试可以使用 ? 运算符");

    println!("\n===== 运行测试 =====");
    println!("cargo test                    # 运行所有测试");
    println!("cargo test test_name          # 运行匹配名称的测试");
    println!("cargo test -- --show-output   # 显示 println! 输出");
    println!("cargo test -- --test-threads=1 # 单线程运行");

    println!("\n✅ 测试教程完成！");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}

fn test_divide() {
    assert_eq!(divide(10.0, 2.0), Ok(5.0));
    assert_eq!(divide(9.0, 3.0), Ok(3.0));
    assert!(divide(10.0, 0.0).is_err());
}

fn test_greeting() {
    assert_eq!(greeting("Rust"), "Hello, Rust!");
    assert_eq!(greeting(""), "Hello, !");
}

fn test_parse() {
    assert_eq!("42".parse::<i32>(), Ok(42));
    assert!("abc".parse::<i32>().is_err());
}

struct Calculator;

impl Calculator {
    fn new() -> Self {
        Calculator
    }
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }
}

struct User {
    name: String,
}

fn find_user(name: &str) -> Option<User> {
    match name {
        "Alice" => Some(User { name: String::from("Alice") }),
        "Bob" => Some(User { name: String::from("Bob") }),
        _ => None,
    }
}

fn parse_age(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|e| format!("解析错误: {}", e))
}
