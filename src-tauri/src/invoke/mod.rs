use self::common::CommandTrait;

mod common;
mod custom_cmd;
mod util;

// 存储所有命令的容器
pub struct CommandContainer;

impl CommandContainer {
    // 根据名称获取并执行命令
    fn execute_command(name: &str, args: &serde_json::Value) -> Result<serde_json::Value, String> {
        // 这里只是一个示例，你需要根据实际情况来实现查找和执行逻辑
        match name {
            "add_command" => custom_cmd::AddCommand::execute(args),
            "command_two" => custom_cmd::CommandTwo::execute(args),
            // ...其他命令
            _ => Err(format!("Unknown command: {}", name)),
        }
    }
}

#[tauri::command]
pub fn dispatch_command(
    name: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    CommandContainer::execute_command(&name, &args)
}
