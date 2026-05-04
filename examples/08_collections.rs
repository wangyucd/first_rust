// ============================================================
// Rust 基础教程 08: 集合类型 (Collections)
// 运行: cargo run --example 08_collections
// ============================================================

use std::collections::HashMap;

fn main() {
    // ==================== Vec<T> 动态数组 ====================
    println!("===== Vec<T> =====");

    // 创建方式
    let mut v1: Vec<i32> = Vec::new();       // 空 Vec
    let v2 = vec![1, 2, 3, 4, 5];           // 用宏创建
    let v3 = vec![0; 10];                    // 10 个 0

    // 添加元素
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v3 = {:?}", v3);

    // 访问元素
    let third = v2[2]; // 索引访问，越界会 panic
    println!("v2[2] = {}", third);

    let tenth = v2.get(9); // get 访问，越界返回 None
    println!("v2.get(9) = {:?}", tenth);

    // 修改元素
    let mut v = vec![1, 2, 3];
    v[0] = 10;
    println!("修改后: {:?}", v);

    // 删除元素
    let last = v.pop(); // 移除最后一个
    println!("pop: {:?}, 剩余: {:?}", last, v);
    v.remove(0); // 移除指定索引
    println!("remove(0): {:?}", v);

    // 遍历
    let v = vec![10, 20, 30, 40, 50];
    print!("不可变遍历: ");
    for item in &v {
        print!("{} ", item);
    }
    println!();

    // 可变遍历
    let mut v = vec![1, 2, 3, 4, 5];
    for item in &mut v {
        *item *= 2;
    }
    println!("可变遍历翻倍: {:?}", v);

    // 常用方法
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("长度: {}", v.len());
    println!("是否为空: {}", v.is_empty());
    println!("包含 5: {}", v.contains(&5));

    // 排序
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort();
    println!("排序: {:?}", v);
    v.reverse();
    println!("反转: {:?}", v);

    // 迭代器方法
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    let max = v.iter().max();
    let min = v.iter().min();
    println!("sum={}, max={:?}, min={:?}", sum, max, min);

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map 翻倍: {:?}", doubled);

    let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter 偶数: {:?}", evens);

    // 使用枚举在 Vec 中存储不同类型
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Cell::Int(42),
        Cell::Float(3.14),
        Cell::Text(String::from("hello")),
    ];
    println!("混合类型: {:?}", row);

    // ==================== String 字符串 ====================
    println!("\n===== String =====");

    // 创建方式
    let s1 = String::new();                    // 空字符串
    let s2 = String::from("hello");            // 从字面量创建
    let s3 = "hello".to_string();              // to_string()
    let s4 = "hello".to_owned();               // to_owned()

    // 拼接
    let mut s = String::from("hello");
    s.push_str(", world");  // 追加字符串切片
    s.push('!');            // 追加单个字符
    println!("push: {}", s);

    // + 运算符 (注意: 左边被移动，右边是引用)
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 被移动，不能再使用
    println!("+: {}", s3);

    // format! 宏 (不会移动任何值)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);

    // 字符串不能通过索引访问
    let hello = String::from("hello");
    // let h = hello[0]; // ❌ 编译错误！
    // 因为 UTF-8 编码中每个字符可能占多个字节

    // 遍历字符
    print!("字符遍历: ");
    for c in "你好世界🦀".chars() {
        print!("{} ", c);
    }
    println!();

    // 遍历字节
    print!("字节遍历: ");
    for b in "hello".bytes() {
        print!("{} ", b);
    }
    println!();

    // 常用方法
    let s = String::from("  Hello, World!  ");
    println!("trim: '{}'", s.trim());
    println!("len (字节): {}", s.len());
    println!("chars count: {}", s.chars().count());
    println!("is_empty: {}", s.is_empty());
    println!("starts_with: {}", s.trim().starts_with("Hello"));
    println!("contains: {}", s.contains("World"));
    println!("replace: {}", s.replace("World", "Rust"));

    // split
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("split: {:?}", fruits);

    // join
    let joined = fruits.join(" | ");
    println!("join: {}", joined);

    // ==================== HashMap<K, V> ====================
    println!("\n===== HashMap =====");

    // 创建
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("scores: {:?}", scores);

    // 从元组集合创建
    let teams = vec![String::from("Blue"), String::from("Red")];
    let points = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(points.into_iter()).collect();
    println!("zip: {:?}", scores);

    // 访问
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);
    let blue_score = scores.get("Blue");
    println!("Blue: {:?}", blue_score);

    // 遍历
    print!("遍历: ");
    for (key, value) in &scores {
        print!("{}={} ", key, value);
    }
    println!();

    // 更新: 覆盖
    scores.insert("Blue", 25);
    println!("覆盖后: {:?}", scores);

    // 更新: 不存在时才插入 (entry + or_insert)
    scores.entry("Yellow").or_insert(30);
    scores.entry("Blue").or_insert(100); // Blue 已存在，不会更新
    println!("or_insert: {:?}", scores);

    // 更新: 基于旧值计算
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("词频统计: {:?}", word_count);

    // 常用方法
    println!("长度: {}", scores.len());
    println!("包含 Red: {}", scores.contains_key("Red"));
    println!("移除 Red: {:?}", scores.remove("Red"));
    println!("最终: {:?}", scores);

    // ==================== 综合示例 ====================
    println!("\n===== 综合示例 =====");

    // 学生成绩管理
    let mut students: HashMap<&str, Vec<f64>> = HashMap::new();
    students.entry("Alice").or_insert_with(Vec::new).extend([90.0, 85.0, 92.0]);
    students.entry("Bob").or_insert_with(Vec::new).extend([78.0, 82.0, 88.0]);
    students.entry("Charlie").or_insert_with(Vec::new).extend([95.0, 91.0, 87.0]);

    for (name, grades) in &students {
        let avg: f64 = grades.iter().sum::<f64>() / grades.len() as f64;
        println!("{}: 平均分 {:.1}, 成绩 {:?}", name, avg, grades);
    }

    println!("\n✅ 集合类型教程完成！");
}