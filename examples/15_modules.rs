// ============================================================
// Rust 基础教程 15: 模块系统 (Module System)
// 运行: cargo run --example 15_modules
// ============================================================

// ==================== 内联模块 ====================
// 可以在文件内定义模块

mod math {
    // pub 使函数公开
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // 私有函数
    fn validate(n: i32) -> bool {
        n >= 0
    }

    // 公开函数可以调用私有函数
    pub fn abs(n: i32) -> i32 {
        if validate(n) {
            n
        } else {
            -n
        }
    }

    // 嵌套模块
    pub mod advanced {
        pub fn power(base: i32, exp: u32) -> i32 {
            (0..exp).fold(1, |acc, _| acc * base)
        }

        pub fn factorial(n: u64) -> u64 {
            (1..=n).product()
        }
    }
}

// ==================== 结构体可见性 ====================
mod shapes {
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Rectangle {
        // 关联函数（构造器）
        pub fn new(width: f64, height: f64) -> Self {
            Rectangle { width, height }
        }

        pub fn area(&self) -> f64 {
            self.width * self.height
        }

        pub fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
    }

    // 私有结构体
    struct Triangle {
        base: f64,
        height: f64,
    }

    // 通过公开函数暴露私有结构体的功能
    pub fn triangle_area(base: f64, height: f64) -> f64 {
        let t = Triangle { base, height };
        0.5 * t.base * t.height
    }
}

// ==================== 枚举可见性 ====================
// 枚举的变体自动继承枚举的可见性
mod directions {
    #[derive(Debug)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Direction {
        pub fn opposite(&self) -> Direction {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }
    }
}

// ==================== use 关键字 ====================
// 引入模块到当前作用域

use directions::Direction;
use math::advanced::factorial;
use math::advanced::power;
use math::{add, subtract};
use shapes::Rectangle;

// 重命名引入
use std::collections::HashMap as Map;

// glob 引入（谨慎使用）
// use std::collections::*;

fn main() {
    // ==================== 使用模块函数 ====================
    println!("===== 模块函数 =====");
    println!("3 + 5 = {}", add(3, 5));
    println!("10 - 4 = {}", subtract(10, 4));
    println!("|-5| = {}", math::abs(-5));

    // 使用嵌套模块
    println!("2^10 = {}", power(2, 10));
    println!("10! = {}", factorial(10));

    // ==================== 使用结构体 ====================
    println!("\n===== 结构体模块 =====");
    let rect = Rectangle::new(10.0, 5.0);
    println!("矩形面积: {}", rect.area());
    println!("矩形周长: {}", rect.perimeter());
    println!("三角形面积: {}", shapes::triangle_area(10.0, 5.0));

    // ==================== 使用枚举 ====================
    println!("\n===== 枚举模块 =====");
    let dir = Direction::North;
    println!("方向: {:?}", dir);
    println!("反方向: {:?}", dir.opposite());

    // ==================== 使用重命名 ====================
    println!("\n===== 重命名引入 =====");
    let mut scores: Map<String, i32> = Map::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    println!("成绩: {:?}", scores);

    // ==================== 模块可见性规则 ====================
    println!("\n===== 可见性规则 =====");
    println!("1. 默认所有项都是私有的");
    println!("2. pub 关键字使项公开");
    println!("3. 父模块不能访问子模块的私有项");
    println!("4. 子模块可以访问祖先模块的所有项");
    println!("5. pub(crate) 使项在当前 crate 内公开");
    println!("6. pub(super) 使项在父模块中公开");

    // ==================== 模块最佳实践 ====================
    println!("\n===== 最佳实践 =====");
    println!("1. 按功能组织模块，而不是按类型");
    println!("2. 使用 pub(crate) 限制内部可见性");
    println!("3. 保持模块接口简洁");
    println!("4. 使用 mod.rs 或文件名.rs 组织子模块");

    println!("\n✅ 模块系统教程完成！");
}