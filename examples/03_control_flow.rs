// ============================================================
// Rust 基础教程 03: 流程控制
// 运行: cargo run --example 03_control_flow
// ============================================================

fn main() {
    // ==================== if 表达式 ====================
    let number = 7;

    // 基本 if
    if number > 5 {
        println!("{} 大于 5", number);
    }

    // if-else
    if number % 2 == 0 {
        println!("{} 是偶数", number);
    } else {
        println!("{} 是奇数", number);
    }

    // if-else if-else
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    println!("成绩 {} → 等级 {}", score, grade);

    // if 是表达式，可以用于赋值
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("if 表达式赋值: {}", value);

    // ==================== loop 循环 ====================
    // loop 创建无限循环，使用 break 返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以返回值
        }
    };
    println!("loop 结果: {}", result); // 20

    // 嵌套循环与标签 (label)
    let mut found = (0, 0);
    'outer: for i in 0..10 {
        for j in 0..10 {
            if i * j > 25 {
                found = (i, j);
                break 'outer; // 跳出外层循环
            }
        }
    }
    println!("嵌套循环找到: {:?}", found);

    // ==================== while 循环 ====================
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("while 循环: 第一个 >= 100 的 2 的幂次 = {}", n);

    // 遍历数组的三种方式
    let arr = [10, 20, 30, 40, 50];

    // 方式1: while + 索引 (不推荐)
    let mut idx = 0;
    while idx < arr.len() {
        print!("{} ", arr[idx]);
        idx += 1;
    }
    println!("← while 遍历");

    // 方式2: for-in 遍历迭代器 (推荐)
    for item in arr.iter() {
        print!("{} ", item);
    }
    println!("← for 遍历");

    // 方式3: for-in + enumerate 获取索引
    for (i, item) in arr.iter().enumerate() {
        print!("[{}]={} ", i, item);
    }
    println!("← enumerate 遍历");

    // ==================== for 循环 ====================

    // 范围遍历
    print!("1..5: ");
    for i in 1..5 {
        print!("{} ", i); // 1 2 3 4 (不包含5)
    }
    println!();

    print!("1..=5: ");
    for i in 1..=5 {
        print!("{} ", i); // 1 2 3 4 5 (包含5)
    }
    println!();

    // 反向遍历
    print!("反向: ");
    for i in (1..=5).rev() {
        print!("{} ", i); // 5 4 3 2 1
    }
    println!();

    // 步长遍历 (使用 step_by)
    print!("步长2: ");
    for i in (0..10).step_by(2) {
        print!("{} ", i); // 0 2 4 6 8
    }
    println!();

    // ==================== match 匹配 ====================
    // match 类似 switch，但更强大
    let number = 13;
    match number {
        1 => println!("一"),
        2..=9 => println!("个位数: {}", number),
        10..=99 => println!("两位数: {}", number),
        _ => println!("更大的数: {}", number), // _ 是通配符
    }

    // match 匹配多个模式
    let x = 5;
    match x {
        1 | 5 | 10 => println!("x 是 1, 5, 或 10"),
        _ => println!("其他值"),
    }

    // match 解构元组
    let point = (3, -5);
    match point {
        (0, 0) => println!("原点"),
        (x, 0) => println!("在 x 轴上, x={}", x),
        (0, y) => println!("在 y 轴上, y={}", y),
        (x, y) => println!("({}, {})", x, y),
    }

    // match 守卫 (match guard)
    let num = Some(4);
    match num {
        Some(x) if x < 0 => println!("负数: {}", x),
        Some(x) if x == 0 => println!("零"),
        Some(x) => println!("正数: {}", x),
        None => println!("无值"),
    }

    // match 绑定变量
    let msg = "Hello, World!";
    match msg {
        s if s.starts_with("Hello") => {
            println!("问候语: {}", s);
        }
        _ => println!("其他消息"),
    }

    // match 作为表达式
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("bool → int: {}", binary);

    // ==================== if let 简洁匹配 ====================
    // 当只关心一种情况时，if let 比 match 更简洁
    let some_value: Option<i32> = Some(42);

    // match 写法
    match some_value {
        Some(v) => println!("match: 值为 {}", v),
        None => {} // 不关心 None 的情况
    }

    // if let 写法 (更简洁)
    if let Some(v) = some_value {
        println!("if let: 值为 {}", v);
    }

    // while let - 当模式匹配时持续循环
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("← while let 弹栈");

    // ==================== 综合示例: FizzBuzz ====================
    println!("\n--- FizzBuzz (1-30) ---");
    for i in 1..=30 {
        match (i % 3, i % 5) {
            (0, 0) => print!("FizzBuzz "),
            (0, _) => print!("Fizz "),
            (_, 0) => print!("Buzz "),
            _ => print!("{} ", i),
        }
    }
    println!();

    println!("\n✅ 流程控制教程完成！");
}