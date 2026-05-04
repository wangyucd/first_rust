// ============================================================
// Rust 基础教程 01: 变量和数据类型
// 运行: cargo run --example 01_variables_and_types
// ============================================================

fn main() {
    // ==================== 变量绑定 ====================
    // Rust 中使用 let 声明变量，默认不可变(immutable)
    let x = 5;
    println!("x = {}", x);

    // 使用 mut 关键字使变量可变
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y (修改后) = {}", y);

    // 变量遮蔽 (Shadowing) - 可以重新 let 同名变量，甚至改变类型
    let z = 5;
    let z = z + 1;       // 遮蔽，z = 6
    let z = z * 2;       // 遮蔽，z = 12
    let z = "hello";     // 遮蔽，类型从 i32 变为 &str
    println!("z = {}", z);

    // 常量 - 必须标注类型，编译时确定值，不可用 mut
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // ==================== 基本数据类型 ====================

    // --- 整数类型 ---
    let a: i8 = -128;       // 8位有符号
    let b: u8 = 255;        // 8位无符号
    let c: i32 = -1000;     // 32位有符号 (默认整数类型)
    let d: u64 = 18_446_744_073_709_551_615; // 64位无符号
    let e: isize = -100;    // 指针大小的有符号整数
    let f: usize = 100;     // 指针大小的无符号整数
    println!("整数: i8={}, u8={}, i32={}, u64={}, isize={}, usize={}", a, b, c, d, e, f);

    // 不同进制表示
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // 仅限 u8
    println!("进制: 十进制={}, 十六进制={}, 八进制={}, 二进制={}, 字节={}", decimal, hex, octal, binary, byte);

    // --- 浮点数类型 ---
    let f1: f32 = 3.14;     // 单精度
    let f2: f64 = 2.718281828; // 双精度 (默认浮点类型)
    println!("浮点数: f32={}, f64={}", f1, f2);

    // --- 布尔类型 ---
    let t = true;
    let f: bool = false;
    println!("布尔: t={}, f={}", t, f);

    // --- 字符类型 ---
    // Rust 的 char 是 4 字节 Unicode 标量值
    let c1 = 'A';
    let c2 = '中';
    let c3 = '🦀';
    println!("字符: c1={}, c2={}, c3={}", c1, c2, c3);

    // ==================== 复合数据类型 ====================

    // --- 元组 (Tuple) ---
    // 固定长度，可以包含不同类型
    let tup: (i32, f64, &str) = (500, 6.4, "hello");
    let (a, b, c) = tup; // 解构
    println!("元组解构: a={}, b={}, c={}", a, b, c);
    println!("元组索引: tup.0={}, tup.1={}, tup.2={}", tup.0, tup.1, tup.2);

    // 单元类型 () - 空元组，表示"无值"
    let unit: () = ();
    println!("单元类型: {:?}", unit);

    // --- 数组 (Array) ---
    // 固定长度，所有元素类型相同，存储在栈上
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("数组: arr={:?}, 第一个={}, 第二个={}", arr, first, second);

    // 带类型标注的数组: [类型; 长度]
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    // 初始化为相同值的数组: [值; 长度]
    let arr3 = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("数组2: {:?}", arr2);
    println!("数组3: {:?}", arr3);

    // ==================== 类型转换 ====================
    let x: i32 = 42;
    let y: f64 = x as f64; // 使用 as 进行类型转换
    println!("类型转换: i32 {} -> f64 {}", x, y);

    let s: &str = "42";
    let n: i32 = s.parse().expect("解析失败"); // 字符串解析为数字
    println!("字符串解析: \"{}\" -> i32 {}", s, n);

    // ==================== 数值溢出 ====================
    // 在 debug 模式下溢出会 panic，release 模式下会回绕(wrapping)
    // 使用 wrapping_*、checked_*、saturating_* 方法显式处理
    let a: u8 = 255;
    let b: u8 = a.wrapping_add(1); // 回绕: 0
    let c: Option<u8> = a.checked_add(1); // 溢出返回 None
    let d: u8 = a.saturating_add(1); // 饱和: 保持 255
    println!("溢出处理: wrapping={}, checked={:?}, saturating={}", b, c, d);

    println!("\n✅ 变量和数据类型教程完成！");
}