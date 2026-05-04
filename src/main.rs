// ============================================================
// 🦀 Rust 基础教程索引
// ============================================================
//
// 本项目包含 17 个循序渐进的 Rust 基础教程示例
// 每个示例都可以独立运行
//
// 运行方式: cargo run --example <示例名>
//
// ============================================================

fn main() {
    println!("🦀 Rust 基础教程");
    println!("================\n");

    let tutorials = [
        ("01_variables_and_types", "变量和数据类型"),
        ("02_functions", "函数"),
        ("03_control_flow", "流程控制"),
        ("04_ownership", "所有权"),
        ("05_references_borrowing", "引用与借用"),
        ("06_structs", "结构体"),
        ("07_enums_pattern", "枚举与模式匹配"),
        ("08_collections", "集合类型"),
        ("09_error_handling", "错误处理"),
        ("10_generics", "泛型"),
        ("11_traits", "特征 (Trait)"),
        ("12_closures_iterators", "闭包与迭代器"),
        ("13_smart_pointers", "智能指针"),
        ("14_concurrency", "并发编程"),
        ("15_modules", "模块系统"),
        ("16_testing", "测试"),
        ("17_lifetimes", "生命周期"),
    ];

    for (i, (name, desc)) in tutorials.iter().enumerate() {
        println!("  {:>2}. {:<30} {}", i + 1, name, desc);
    }

    println!("\n运行示例: cargo run --example <示例名>");
    println!("例如:     cargo run --example 01_variables_and_types");
}