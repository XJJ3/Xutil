use serde::{Deserialize, Serialize};

pub static USER_CMD_FILE: &str = "commands.x";
pub static USER_CMD_ICON_FILE_DIR: &str = "cmd_icons";

// 定义一个命令 trait，所有命令都会实现这个 trait
pub trait CommandTrait: Sized + Send + Sync + 'static {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandData {
    pub cmd: String,
    pub args: Vec<String>,
    pub current_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandGroupData {
    pub group_name: String,
    pub group_icon: Option<String>,
    pub commands: Vec<CommandData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommandGroupParams {
    pub group_name: Option<String>,
    pub group_icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommandParams {
    pub group_name: Option<String>,
    pub cmd: String,
    pub args: Vec<String>,
    pub current_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub name: String,
    pub age: u16,
}
