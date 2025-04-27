// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32::consts::PI;  // 精确导入PI常量

fn main() {
    let radius = 5.00f32;
    let area = PI * radius.powi(2);  // 直接使用PI并优化方法调用

    println!(
        "The area of a circle with radius {radius:.2} is {area:.5}!"
    );  // 内联格式化参数并添加分号
}
