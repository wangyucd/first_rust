// ============================================================
// Rust 基础教程 06: 结构体 (Struct)
// 运行: cargo run --example 06_structs
// ============================================================

fn main() {
    // ==================== 定义和实例化 ====================
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("rustacean"),
        active: true,
        sign_in_count: 1,
    };
    println!("用户: {} ({})", user.username, user.email);

    // 可变实例 - 整个实例必须是可变的
    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("new_email@example.com");
    println!("修改邮箱: {}", user2.email);

    // ==================== 结构体更新语法 ====================
    // 使用 .. 从另一个实例复制剩余字段
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user2 // 其余字段从 user2 复制 (active, sign_in_count)
    };
    println!("user3: {} active={} sign_in={}", user3.username, user3.active, user3.sign_in_count);

    // ==================== 元组结构体 ====================
    // 没有字段名，只有类型
    struct Color(i32, i32, i32);
    struct Point(f64, f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);
    println!("颜色: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点: ({}, {}, {})", origin.0, origin.1, origin.2);

    // ==================== 单元结构体 ====================
    // 没有任何字段，常用于实现 trait
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // ==================== 方法 ====================
    let rect = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    println!("矩形: {}×{}", rect.width, rect.height);
    println!("面积: {}", rect.area());
    println!("周长: {}", rect.perimeter());
    println!("是正方形? {}", rect.is_square());

    // 方法可以修改实例 (&mut self)
    let mut rect2 = Rectangle::new(10.0, 10.0);
    rect2.scale(2.0);
    println!("缩放后: {}×{}", rect2.width, rect2.height);

    // 关联函数 (类似静态方法)
    let square = Rectangle::square(5.0);
    println!("正方形: {}×{}, 面积={}", square.width, square.height, square.area());

    // ==================== 多个 impl 块 ====================
    // 一个结构体可以有多个 impl 块
    let p = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    println!("点: {}", p);
    println!("到原点距离: {:.2}", p.distance_from_origin());

    // ==================== 结构体与所有权 ====================
    // 结构体可以持有引用，但需要生命周期标注 (见生命周期教程)
    // 这里使用拥有所有权的 String 而非 &str
    let user = User {
        email: String::from("owned@example.com"),
        username: String::from("owned_user"),
        active: true,
        sign_in_count: 0,
    };
    // user 的 email 和 username 是 String 类型，拥有所有权
    // 当 user 离开作用域时，这些 String 也会被释放

    // ==================== 打印结构体 ====================
    // 使用 #[derive(Debug)] 可以用 {:?} 打印
    let rect = Rectangle::new(10.0, 20.0);
    println!("Debug: {:?}", rect);
    println!("Pretty: {:#?}", rect); // 美化输出

    // ==================== 实用示例: 链表 ====================
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("链表: {}", list);
    println!("链表长度: {}", list.len());

    println!("\n✅ 结构体教程完成！");
}

// ==================== 结构体定义 ====================

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// ==================== 带方法的结构体 ====================

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 关联函数 - 没有 self 参数，用 :: 调用
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    fn square(size: f64) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 方法 - &self: 不可变借用
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    // 方法 - &mut self: 可变借用
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // 方法 - self: 获取所有权 (消耗实例)
    fn into_square(self) -> Self {
        let size = self.width.max(self.height);
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// ==================== 多个 impl 块 ====================

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// ==================== 实用示例: 简单链表 ====================

struct LinkedList {
    head: Option<Box<Node>>,
    length: usize,
}

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // take() 取出 Option 中的值并留下 None
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        let mut values = Vec::new();
        while let Some(node) = current {
            values.push(node.value.to_string());
            current = &node.next;
        }
        write!(f, "[{}]", values.join(" -> "))
    }
}