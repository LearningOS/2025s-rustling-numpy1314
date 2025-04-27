// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 修复第一个警告：for循环改为if let
    if let Some(x) = option {
        res += x;
    }
    
    // 修复第二个警告：格式化字符串直接使用变量
    println!("{res}");  // 分号不能省略
}