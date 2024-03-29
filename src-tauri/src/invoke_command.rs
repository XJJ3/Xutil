use std::any::Any;
use tauri::command;

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

        println!("获得参数： {:?}", _args);
        Ok(serde_json::json!({ "result": "CommandOne executed" }))
    }
}

#[derive(Debug)]
struct CommandTwo;

impl CommandTrait for CommandTwo {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
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
