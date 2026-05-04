// ============================================================
// Rust 基础教程 04: 所有权 (Ownership)
// 运行: cargo run --example 04_ownership
// ============================================================

fn main() {
    // ==================== 所有权规则 ====================
    // 1. Rust 中每个值都有一个所有者 (owner)
    // 2. 同一时刻只能有一个所有者
    // 3. 当所有者离开作用域时，值被自动释放 (drop)

    // ==================== 移动 (Move) ====================
    // 对于堆上的数据(如 String)，赋值会转移所有权
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("{}", s1); // ❌ 编译错误！s1 已经无效
    println!("移动后 s2 = {}", s2);

    // 栈上的数据(如 i32)会自动复制 (Copy trait)
    let x = 5;
    let y = x; // x 被复制，不是移动
    println!("复制: x = {}, y = {}", x, y); // ✅ 两个都有效

    // ==================== 克隆 (Clone) ====================
    // 深拷贝堆上的数据
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝，s1 仍然有效
    println!("克隆: s1 = {}, s2 = {}", s1, s2);

    // ==================== 函数与所有权 ====================

    // 传值 - 所有权转移
    let s = String::from("world");
    takes_ownership(s); // s 的所有权移动到函数中
    // println!("{}", s); // ❌ s 已经无效

    // 传基本类型 - 复制
    let x = 42;
    makes_copy(x); // x 被复制
    println!("函数调用后 x = {}", x); // ✅ x 仍然有效

    // ==================== 返回值与所有权 ====================
    let s1 = gives_ownership();        // 函数返回值的所有权转移给 s1
    let s2 = String::from("hello");    // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 移入函数，返回值移给 s3
    println!("s1 = {}, s3 = {}", s1, s3);
    // println!("{}", s2); // ❌ s2 已经移走了

    // ==================== 所有权与集合 ====================
    let v = vec![1, 2, 3, 4, 5];
    // let first = v[0]; // ❌ 对于非 Copy 类型的 Vec 元素不能直接取
    let first = v[0]; // ✅ i32 实现了 Copy，可以复制
    println!("first = {}, v = {:?}", first, v);

    // String 元素需要借用或克隆
    let names = vec![String::from("Alice"), String::from("Bob")];
    // let first_name = names[0]; // ❌ 不能移动 Vec 元素
    let first_name = &names[0]; // ✅ 借用
    println!("first_name = {}", first_name);

    // ==================== Copy trait ====================
    // 以下类型实现了 Copy trait，赋值时会自动复制:
    // - 所有整数类型 (i32, u64, etc.)
    // - 布尔类型 (bool)
    // - 浮点类型 (f32, f64)
    // - 字符类型 (char)
    // - 元组 (如果所有元素都是 Copy 的)
    let a: (i32, f64, bool) = (1, 2.0, true);
    let b = a; // 复制，不是移动
    println!("Copy 元组: a = {:?}, b = {:?}", a, b);

    // ==================== Drop trait ====================
    // 当值离开作用域时，Rust 自动调用 drop 函数
    {
        let s = String::from("我在作用域内");
        println!("{}", s);
    } // s 离开作用域，自动释放内存
    // println!("{}", s); // ❌ s 已经被释放

    // ==================== 所有权转移的常见模式 ====================

    // 模式1: 函数接收所有权后返回
    let mut s = String::from("hello");
    s = append_world(s); // 接收返回的所有权
    println!("追加后: {}", s);

    // 模式2: 使用引用借用 (见下一节)
    let s = String::from("hello");
    let len = calculate_length(&s); // 借用，不转移所有权
    println!("\"{}\" 的长度是 {}", s, len); // ✅ s 仍然有效

    println!("\n✅ 所有权教程完成！");
}

// 接收 String 所有权的函数
fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
} // s 离开作用域，自动释放

// 接收 i32 (Copy 类型) 的函数
fn makes_copy(x: i32) {
    println!("makes_copy: {}", x);
} // x 离开作用域，但因为是 Copy，没什么特别的

// 返回 String，转移所有权给调用者
fn gives_ownership() -> String {
    String::from("hello from gives_ownership")
}

// 接收并返回 String，转移所有权
fn takes_and_gives_back(s: String) -> String {
    s
}

// 接收所有权，修改后返回
fn append_world(mut s: String) -> String {
    s.push_str(", world!");
    s
}

// 借用引用，不获取所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}