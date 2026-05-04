// ============================================================
// Rust 基础教程 05: 引用与借用 (References & Borrowing)
// 运行: cargo run --example 05_references_borrowing
// ============================================================

fn main() {
    // ==================== 不可变引用 (&T) ====================
    // 引用允许你使用值但不获取所有权
    let s = String::from("hello");
    let len = calculate_length(&s); // &s 创建一个指向 s 的引用
    println!("\"{}\" 的长度是 {}", s, len); // ✅ s 仍然有效

    // 可以同时有多个不可变引用
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("多个不可变引用: r1={}, r2={}, r3={}", r1, r2, r3);

    // ==================== 可变引用 (&mut T) ====================
    // 同一时刻只能有一个可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("可变引用修改后: {}", s);

    // 可变引用的限制: 同一时刻只能有一个
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // ❌ 不能同时有两个可变引用
    println!("可变引用: {}", r1);

    // ==================== 引用的作用域 (NLL) ====================
    // Non-Lexical Lifetimes (NLL): 引用的作用域在最后一次使用时结束
    let mut s = String::from("hello");
    let r1 = &s;     // r1 的作用域开始
    let r2 = &s;     // r2 的作用域开始
    println!("r1={}, r2={}", r1, r2); // r1 和 r2 最后一次使用
    // r1 和 r2 的作用域在这里结束 (NLL)

    let r3 = &mut s; // ✅ 现在可以创建可变引用了
    println!("r3={}", r3);

    // ==================== 不可变引用与可变引用不能共存 ====================
    let mut s = String::from("hello");
    let r1 = &s;       // 不可变引用
    let r2 = &s;       // 不可变引用
    // let r3 = &mut s; // ❌ 不能在不可变引用存在时创建可变引用
    println!("r1={}, r2={}", r1, r2);
    // r1, r2 在此之后不再使用
    let r3 = &mut s;   // ✅ 现在可以了
    println!("r3={}", r3);

    // ==================== 悬垂引用 (Dangling Reference) ====================
    // Rust 编译器保证引用永远不会悬垂
    // let reference_to_nothing = dangle(); // ❌ 编译错误
    let valid = no_dangle();
    println!("有效引用: {}", valid);

    // ==================== 切片 (Slice) ====================
    // 切片是对集合中连续元素序列的引用

    // --- 字符串切片 (&str) ---
    let s = String::from("hello world");
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    println!("切片: hello={}, world={}", hello, world);

    // 简写语法
    let s = String::from("hello");
    let slice1 = &s[..2];   // 从头开始，等价于 &s[0..2]
    let slice2 = &s[2..];   // 到末尾
    let slice3 = &s[..];    // 整个字符串
    println!("简写: [..2]={}, [2..]={}, [..]={}", slice1, slice2, slice3);

    // 字符串字面量就是切片
    let s: &str = "hello world"; // 类型是 &str，指向程序二进制中的某个位置
    println!("字面量切片: {}", s);

    // --- 数组切片 (&[T]) ---
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // [2, 3]
    println!("数组切片: {:?}", slice);

    // ==================== 切片作为函数参数 ====================
    // 使用切片作为参数更灵活，可以接受 String 引用、&str、切片等
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("第一个单词: {}", first);

    // 字符串字面量也可以传入
    let first = first_word("hello world");
    println!("字面量第一个单词: {}", first);

    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    println!("数组和: {}", sum_slice(&arr[..]));
    println!("部分和: {}", sum_slice(&arr[1..4]));

    // ==================== 可变切片 ====================
    let mut arr = [1, 2, 3, 4, 5];
    double_slice(&mut arr[..]);
    println!("翻倍后: {:?}", arr);

    // ==================== 实用示例 ====================
    // 在字符串中查找子串
    let text = String::from("the quick brown fox jumps over the lazy dog");
    if let Some(pos) = find_substring(&text, "fox") {
        println!("\"fox\" 在位置 {} 被找到", pos);
    }

    println!("\n✅ 引用与借用教程完成！");
}

// 接收不可变引用
fn calculate_length(s: &String) -> usize {
    s.len()
    // s 是借用的，不会获取所有权
}

// 接收可变引用
fn change(s: &mut String) {
    s.push_str(", world");
}

// ❌ 悬垂引用示例 (编译不通过)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // s 在函数结束时被释放，引用指向无效内存
// }

// ✅ 正确做法: 返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 所有权转移给调用者
}

// 使用字符串切片作为参数 - 更通用
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// 数组切片作为参数
fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in slice {
        sum += item;
    }
    sum
}

// 可变切片
fn double_slice(slice: &mut [i32]) {
    for item in slice.iter_mut() {
        *item *= 2;
    }
}

// 在字符串中查找子串
fn find_substring(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}