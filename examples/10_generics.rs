// ============================================================
// Rust 基础教程 10: 泛型 (Generics)
// 运行: cargo run --example 10_generics
// ============================================================

fn main() {
    // ==================== 泛型函数 ====================
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("最大的数字: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("最大的字符: {}", result);

    // ==================== 泛型结构体 ====================
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: {:?}", integer_point);
    println!("浮点点: {:?}", float_point);

    // 混合类型泛型
    let mixed = Point2 { x: 5, y: 4.0 };
    println!("混合点: {:?}", mixed);

    // ==================== 泛型方法 ====================
    let p = Point { x: 3, y: 4 };
    println!("x = {}", p.x());

    let p = Point { x: 3.0, y: 4.0 };
    println!("距离原点: {}", p.distance_from_origin());

    // 不同类型的 impl 块
    let p1 = Point2 { x: 5, y: 10 };
    let p2 = Point2 { x: 1.0, y: 4.0 };
    let result = p1.mixup(p2);
    println!("混合点: x={}, y={}", result.x, result.y);

    // ==================== 泛型枚举 ====================
    // Option<T> 和 Result<T, E> 就是泛型枚举
    let some_int: Option<i32> = Some(5);
    let some_str: Option<&str> = Some("hello");
    let none: Option<f64> = None;
    println!("Option: {:?}, {:?}, {:?}", some_int, some_str, none);

    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("错误");
    println!("Result: {:?}, {:?}", ok, err);

    // ==================== 泛型与多个类型参数 ====================
    let pair = Pair::new(1, 2);
    println!("Pair: {}", pair);

    let pair = Pair::new("hello", "world");
    println!("Pair: {}", pair);

    // ==================== 泛型约束 (使用 trait bound) ====================
    // 见 traits 教程，这里简单演示
    let p = Point { x: 3, y: 4 };
    println!("显示: {}", p);

    // ==================== const 泛型 ====================
    // Rust 支持 const 泛型，用常量值作为泛型参数
    let arr = [1, 2, 3, 4, 5];
    println!("数组长度: {}", array_length(&arr));

    let arr = [1, 2, 3];
    println!("数组长度: {}", array_length(&arr));

    // const 泛型结构体
    let matrix = Matrix::new([[1, 2], [3, 4]]);
    println!("矩阵: {:?}", matrix);
    println!("转置: {:?}", matrix.transpose());

    // ==================== 泛型与迭代器 ====================
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = map_vec(&numbers, |x| x * 2);
    println!("map: {:?}", doubled);

    let strings = map_vec(&numbers, |x| format!("#{}", x));
    println!("map: {:?}", strings);

    // ==================== 实用示例: 泛型栈 ====================
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("栈: {:?}", stack);
    println!("弹出: {:?}", stack.pop());
    println!("栈顶: {:?}", stack.peek());
    println!("栈: {:?}", stack);

    // 字符串栈
    let mut str_stack = Stack::new();
    str_stack.push(String::from("hello"));
    str_stack.push(String::from("world"));
    println!("字符串栈: {:?}", str_stack);

    println!("\n✅ 泛型教程完成！");
}

// ==================== 泛型函数 ====================
// T: PartialOrd 是 trait bound，要求 T 实现了 PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ==================== 泛型结构体 ====================

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 不同类型的泛型
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// ==================== 泛型方法 ====================

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只为特定类型实现方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 使用不同类型参数的方法
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// 为实现了 Display 的类型实现额外方法
impl<T: std::fmt::Display> Point<T> {
    fn to_string_custom(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ==================== 泛型与多个类型参数 ====================

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

// ==================== const 泛型 ====================

fn array_length<T, const N: usize>(arr: &[T; N]) -> usize {
    N
}

#[derive(Debug)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new(data: [[T; COLS]; ROWS]) -> Self {
        Matrix { data }
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut result = Matrix {
            data: [[T::default(); ROWS]; COLS],
        };
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

// ==================== 泛型与迭代器 ====================

fn map_vec<T, U, F>(vec: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    vec.iter().map(f).collect()
}

// ==================== 实用示例: 泛型栈 ====================

#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}