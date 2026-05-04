// ============================================================
// Rust 基础教程 12: 闭包与迭代器 (Closures & Iterators)
// 运行: cargo run --example 12_closures_iterators
// ============================================================

fn main() {
    // ==================== 闭包基础 ====================
    // 闭包是可以捕获环境变量的匿名函数
    // 语法: |参数| 表达式 或 |参数| { 语句块 }

    // 简单闭包
    let add = |a, b| a + b;
    println!("闭包加法: {}", add(3, 5));

    // 带类型标注的闭包
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("闭包乘法: {}", multiply(4, 6));

    // 多行闭包
    let calculate = |x: i32| {
        let doubled = x * 2;
        let result = doubled + 10;
        result
    };
    println!("闭包计算: {}", calculate(5));

    // ==================== 闭包捕获环境 ====================
    let name = String::from("Rust");
    let greeting = || println!("Hello, {}!", name);
    greeting(); // 闭包捕获了 name 的引用

    // 不可变借用捕获
    let value = 42;
    let print_value = || println!("值: {}", value);
    print_value();
    println!("原始值仍然可用: {}", value);

    // 可变借用捕获
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("计数: {}", counter);
    };
    increment();
    increment();
    increment();
    println!("最终计数: {}", counter);

    // 所有权转移 (move)
    let data = vec![1, 2, 3];
    let owns_data = move || {
        println!("拥有数据: {:?}", data);
    };
    owns_data();
    // println!("{:?}", data); // ❌ data 已被移动到闭包中

    // ==================== 闭包作为参数 ====================
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用闭包过滤
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // 使用闭包转换
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("翻倍: {:?}", doubled);

    // 传递闭包给函数
    let result = apply_to_vec(&numbers, |x| x * x);
    println!("平方: {:?}", result);

    // ==================== 闭包 Trait ====================
    // Fn: 不可变借用捕获的变量
    // FnMut: 可变借用捕获的变量
    // FnOnce: 获取捕获变量的所有权（只能调用一次）

    // Fn 示例
    let multiplier = 3;
    let multiply_by = |x| x * multiplier;
    println!("Fn: {}", apply_fn(&multiply_by, 5));
    println!("Fn: {}", apply_fn(&multiply_by, 10));

    // FnMut 示例
    let mut log = Vec::new();
    let mut logger = |msg: &str| {
        log.push(msg.to_string());
    };
    call_fn_mut(&mut logger, "第一条");
    call_fn_mut(&mut logger, "第二条");
    println!("日志: {:?}", log);

    // FnOnce 示例
    let data = String::from("重要数据");
    let consume = || {
        println!("消费: {}", data);
        drop(data); // 消耗 data
    };
    call_fn_once(consume);
    // consume(); // ❌ 不能再调用

    // ==================== 迭代器基础 ====================
    // 迭代器是惰性的，只有在被消费时才会产生值

    let v = vec![1, 2, 3, 4, 5];

    // 创建迭代器
    let mut iter = v.iter();

    // next() 方法
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());

    // 遍历迭代器
    for item in v.iter() {
        print!("{} ", item);
    }
    println!();

    // ==================== 迭代器适配器 ====================
    // 适配器是惰性的，不会立即执行

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: 转换每个元素
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("map: {:?}", doubled);

    // filter: 过滤元素
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter: {:?}", evens);

    // filter_map: 过滤并转换
    let parsed: Vec<i32> = vec!["1", "abc", "3", "def", "5"]
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("filter_map: {:?}", parsed);

    // enumerate: 添加索引
    for (i, val) in numbers.iter().enumerate() {
        if i < 3 {
            print!("[{}]={} ", i, val);
        }
    }
    println!("...");

    // zip: 合并两个迭代器
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![90, 85, 92];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("zip: {:?}", pairs);

    // chain: 连接迭代器
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chain: {:?}", chained);

    // take 和 skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("take(3): {:?}", first_three);
    println!("skip(3): {:?}", skip_three);

    // ==================== 迭代器消费者 ====================
    // 消费者会消耗迭代器，产生最终结果

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // collect: 收集为集合
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("collect: {:?}", doubled);

    // sum: 求和
    let total: i32 = numbers.iter().sum();
    println!("sum: {}", total);

    // count: 计数
    let count = numbers.iter().count();
    println!("count: {}", count);

    // max 和 min
    println!("max: {:?}", numbers.iter().max());
    println!("min: {:?}", numbers.iter().min());

    // any 和 all
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("any even: {}", has_even);
    println!("all positive: {}", all_positive);

    // find: 查找第一个满足条件的
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("find first even: {:?}", first_even);

    // position: 查找位置
    let pos = numbers.iter().position(|&x| x == 5);
    println!("position of 5: {:?}", pos);

    // fold: 折叠（类似 reduce）
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold sum: {}", sum);

    let product = numbers.iter().take(5).fold(1, |acc, &x| acc * x);
    println!("fold product (1-5): {}", product);

    // ==================== 自定义迭代器 ====================
    // 实现 Iterator trait

    let counter = Counter::new(5);
    println!("自定义迭代器: {:?}", counter.collect::<Vec<usize>>());

    // 链式使用
    let sum: usize = Counter::new(10).filter(|&x| x % 2 == 0).sum();
    println!("偶数和: {}", sum);

    // ==================== 迭代器 vs 循环 ====================
    // 迭代器通常更简洁，且编译器可以优化（零成本抽象）

    let words = vec!["hello", "world", "rust", "is", "awesome"];

    // 命令式风格
    let mut result = Vec::new();
    for word in &words {
        if word.len() > 3 {
            result.push(word.to_uppercase());
        }
    }
    println!("命令式: {:?}", result);

    // 函数式风格（迭代器）
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect();
    println!("函数式: {:?}", result);

    // ==================== 实用示例 ====================

    // 单词频率统计
    let text = "hello world hello rust world hello";
    let word_count = count_words(text);
    println!("词频: {:?}", word_count);

    // 链式数据处理
    let data = vec![
        ("Alice", 90),
        ("Bob", 75),
        ("Charlie", 85),
        ("Diana", 95),
        ("Eve", 60),
    ];

    let honor_roll: Vec<&str> = data
        .iter()
        .filter(|(_, score)| *score >= 80)
        .map(|(name, _)| *name)
        .collect();
    println!("荣誉榜: {:?}", honor_roll);

    let average: f64 = data.iter().map(|(_, score)| *score as f64).sum::<f64>() / data.len() as f64;
    println!("平均分: {:.1}", average);

    println!("\n✅ 闭包与迭代器教程完成！");
}

// ==================== 闭包作为函数参数 ====================

fn apply_to_vec(vec: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    vec.iter().map(|&x| f(x)).collect()
}

// Fn trait
fn apply_fn(f: &impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// FnMut trait
fn call_fn_mut(f: &mut impl FnMut(&str), msg: &str) {
    f(msg);
}

// FnOnce trait
fn call_fn_once(f: impl FnOnce()) {
    f();
}

// ==================== 自定义迭代器 ====================

struct Counter {
    count: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// ==================== 实用示例 ====================

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}