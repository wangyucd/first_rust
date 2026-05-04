// ============================================================
// Rust 基础教程 13: 智能指针 (Smart Pointers)
// 运行: cargo run --example 13_smart_pointers
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // ==================== Box<T> 堆分配 ====================
    // Box 将数据存储在堆上，栈上只保留指针

    let b = Box::new(5);
    println!("Box 值: {}", b);

    // 递归类型必须使用 Box
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("链表: {:?}", list);

    // 大数据移到堆上
    let big_array = Box::new([0u8; 1024]);
    println!("大数组长度: {}", big_array.len());

    // ==================== Rc<T> 引用计数 ====================
    // Rc 允许多个所有者共享同一数据（只读）

    let shared = Rc::new(String::from("共享数据"));
    println!("引用计数: {}", Rc::strong_count(&shared));

    let clone1 = Rc::clone(&shared);
    println!("clone1: {}", clone1);
    println!("引用计数: {}", Rc::strong_count(&shared));

    {
        let clone2 = Rc::clone(&shared);
        println!("clone2: {}", clone2);
        println!("引用计数: {}", Rc::strong_count(&shared));
    } // clone2 离开作用域

    println!("离开作用域后引用计数: {}", Rc::strong_count(&shared));

    // Rc 用于共享链表
    let a = Rc::new(List::Cons(5, Box::new(List::Nil)));
    let b = List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))));
    // 注意：这里为了演示简单，实际共享需要用 Rc 包裹 List 本身

    // ==================== RefCell<T> 内部可变性 ====================
    // RefCell 在运行时检查借用规则

    let data = RefCell::new(vec![1, 2, 3]);

    // 不可变借用
    {
        let borrowed = data.borrow();
        println!("不可变借用: {:?}", borrowed);
    }

    // 可变借用
    {
        let mut borrowed_mut = data.borrow_mut();
        borrowed_mut.push(4);
        println!("可变借用: {:?}", borrowed_mut);
    }

    println!("修改后: {:?}", data.borrow());

    // ==================== Rc<RefCell<T>> 组合 ====================
    // 共享可变数据

    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);

    clone1.borrow_mut().push(4);
    clone2.borrow_mut().push(5);

    println!("共享可变数据: {:?}", shared_data.borrow());
    println!("引用计数: {}", Rc::strong_count(&shared_data));

    // ==================== Deref trait ====================
    // 智能指针可以像引用一样使用

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref 解引用

    // Deref 强制转换
    let name = MyBox::new(String::from("Rust"));
    hello(&name); // MyBox<String> -> String -> str

    // ==================== Drop trait ====================
    // 自定义清理逻辑

    {
        let _r1 = CustomSmartPointer {
            data: String::from("第一个"),
        };
        let _r2 = CustomSmartPointer {
            data: String::from("第二个"),
        };
        println!("CustomSmartPointers 已创建");
    } // 离开作用域时自动调用 drop

    // 手动提前释放
    let r = CustomSmartPointer {
        data: String::from("提前释放"),
    };
    println!("创建: {}", r.data);
    drop(r); // 手动调用 drop
    println!("已手动释放");

    // ==================== 实用示例: 共享配置 ====================
    let config = Rc::new(RefCell::new(AppConfig {
        debug: false,
        max_connections: 100,
    }));

    let module_a = Rc::clone(&config);
    let module_b = Rc::clone(&config);

    // 模块 A 修改配置
    module_a.borrow_mut().debug = true;

    // 模块 B 读取配置
    println!("模块 B 看到的配置: {:?}", module_b.borrow());

    // 模块 B 也修改配置
    module_b.borrow_mut().max_connections = 200;

    // 所有模块看到相同配置
    println!("最终配置: {:?}", config.borrow());

    println!("\n✅ 智能指针教程完成！");
}

// ==================== Box 递归类型 ====================

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// ==================== Deref trait ====================

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// ==================== Drop trait ====================

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("释放 CustomSmartPointer: {}", self.data);
    }
}

// ==================== 实用示例 ====================

#[derive(Debug)]
struct AppConfig {
    debug: bool,
    max_connections: usize,
}