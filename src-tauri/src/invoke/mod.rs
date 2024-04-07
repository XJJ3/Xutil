use self::common::CommandTrait;

mod common;
mod custom_cmd;
mod util;

// 存储所有命令的容器
pub struct CommandContainer;

impl CommandContainer {
    // 根据名称获取并执行命令
    fn execute_command(name: &str, args: &serde_json::Value) -> Result<serde_json::Value, String> {
        match name {
            "get_all_commands" => custom_cmd::GetAllCommands::execute(args),
            "add_command_group" => custom_cmd::AddCommandGroup::execute(args),
            "add_command" => custom_cmd::AddCommand::execute(args),
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
