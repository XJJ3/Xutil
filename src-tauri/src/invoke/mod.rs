use self::common::{AsyncCommandTrait, CommandTrait};

mod common;
mod custom_cmd;
mod translate;
mod util;

// 存储所有命令的容器
pub struct CommandContainer;

impl CommandContainer {
    // 根据名称获取并执行命令
    async fn execute_command(
        name: &str,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        match name {
            "get_all_commands" => custom_cmd::GetAllCommands::execute(args),
            "add_command_group" => custom_cmd::AddCommandGroup::execute(args),
            "del_command_group" => custom_cmd::DelCommandGroup::execute(args),
            "add_command" => custom_cmd::AddCommand::execute(args),
            "del_command" => custom_cmd::DelCommand::execute(args),
            "execute_cmd" => custom_cmd::ExecuteCmd::execute(args),
            "translate" => translate::TranslateText::execute(args).await,
            _ => return Err(format!("Unknown command: {}", name)),
        }
    }
}

#[tauri::command]
pub async fn dispatch_command(
    name: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    CommandContainer::execute_command(&name, &args).await
}
