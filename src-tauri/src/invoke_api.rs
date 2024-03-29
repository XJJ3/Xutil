use std::{process::Command, thread, time::Duration};

#[tauri::command]
pub fn open_in_vscode(path: &str) {
    println!("路径=> {}", path);
    // 假设你的 shell 脚本位于 "/path/to/your/script.sh"
    let script_path = "/Users/xujunjie/Documents/unitreeProjects/go2_app/deploy-dev.sh";

    // 使用 Command 来执行脚本
    let status = Command::new("sh")
        .arg(script_path) // 添加脚本路径作为参数
        .current_dir("/Users/xujunjie/Documents/unitreeProjects/go2_app/") // 设置工作目录为脚本所在的目录
        .status() // 执行命令并获取退出状态码
        .expect("Failed to execute command");

    // 检查命令是否成功执行
    if status.success() {
        println!("Script executed successfully!");
    } else {
        println!("Script failed to execute: {}", status);
    }
}

#[tauri::command]
pub fn greet() -> String {
    // format!("Hello, {}!", name)
    let output = Command::new("ls").output().expect("执行异常，提示");
    let ls_list = String::from_utf8(output.stdout);
    match ls_list {
        Ok(value) => value,
        Err(e) => '1'.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::invoke_api::greet;

    #[test]
    fn it_greet() {
        assert_eq!("", greet());
    }
}
