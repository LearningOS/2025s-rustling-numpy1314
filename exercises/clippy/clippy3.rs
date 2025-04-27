// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 删除会导致panic的unwrap调用

    let my_arr = &[
        -1, -2, -3,  // 添加逗号分隔数组元素
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");  // 内联格式化参数

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();  // 使用clear替代resize
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);  // 正确交换变量
    println!("value a: {value_a}; value b: {value_b}");
}
