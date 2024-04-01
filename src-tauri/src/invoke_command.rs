use std::{fs::File, io::prelude::*, path::Path};

use tauri::api::path::app_data_dir;

// 定义一个命令 trait，所有命令都会实现这个 trait
pub trait CommandTrait: Sized + Send + Sync + 'static {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String>;
}

// 存储所有命令的容器
pub struct CommandContainer;

impl CommandContainer {
    // 根据名称获取并执行命令
    pub fn execute_command(
        name: &str,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // 这里只是一个示例，你需要根据实际情况来实现查找和执行逻辑
        match name {
            "command_one" => CommandOne::execute(args),
            "command_two" => CommandTwo::execute(args),
            // ...其他命令
            _ => Err(format!("Unknown command: {}", name)),
        }
    }
}

#[derive(Debug)]
struct CommandOne;

impl CommandTrait for CommandOne {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
        // 执行命令逻辑

        let path = Path::new("/Users/xujunjie/Documents/Tauri/lorem_ipsum.x");
        let display = path.display();

        // 以只写模式打开文件，返回 `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {:?}", display, why),
            Ok(file) => file,
        };

        // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
        match file.write_all("LOREM_IPSUM".as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {:?}", display, why)
            }
            Ok(_) => println!("successfully wrote to {}", display),
        }

        Ok(serde_json::json!({ "result": "CommandOne executed" }))
    }
}

#[derive(Debug)]
struct CommandTwo;

impl CommandTrait for CommandTwo {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
        // 指定要读取的文件路径
        let path = "/Users/xujunjie/Documents/Tauri/lorem_ipsum.x";

        // 读取文件内容
        let mut file = match File::open(path) {
            Err(why) => {
                println!("无文件");
                return Ok(serde_json::json!("ssss"));
            }
            Ok(file) => file,
        };

        let mut contents = String::new();

        // 使用 `read_to_string` 方法将文件内容读取到字符串中
        file.read_to_string(&mut contents);

        // 输出文件内容
        println!("File contents: {}", contents);

        // 执行命令逻辑
        Ok(serde_json::json!({ "result": "CommandTwo executed" }))
    }
}

#[tauri::command]
pub fn dispatch_command(
    name: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    CommandContainer::execute_command(&name, &args)
}
