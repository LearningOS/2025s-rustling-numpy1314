//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 设置环境变量TEST_FOO为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // tests8的部分暂时留空或用正确指令
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
