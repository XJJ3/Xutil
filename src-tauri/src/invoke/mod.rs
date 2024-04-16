use self::common::CommandTrait;

mod common;
mod custom_cmd;
mod util;

// 存储所有命令的容器
pub struct CommandContainer;

impl CommandContainer {
    // 根据名称获取并执行命令
    async fn execute_command(
        name: &str,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let r#fn = match name {
            "get_all_commands" => custom_cmd::GetAllCommands::execute,
            "add_command_group" => custom_cmd::AddCommandGroup::execute,
            "add_command" => custom_cmd::AddCommand::execute,
            "del_command" => custom_cmd::DelCommand::execute,
            "execute_cmd" => custom_cmd::ExecuteCmd::execute,
            _ => return Err(format!("Unknown command: {}", name)),
        };
        r#fn(args)
    }
}

#[tauri::command]
pub async fn dispatch_command(
    name: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    CommandContainer::execute_command(&name, &args).await
}
