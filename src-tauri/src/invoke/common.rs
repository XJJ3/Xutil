use serde::{Deserialize, Serialize};

pub static USER_CMD_FILE: &str = "commands.x";
pub static USER_CMD_ICON_FILE_DIR: &str = "cmd_icons";

// 定义一个命令 trait，所有命令都会实现这个 trait
pub trait CommandTrait: Sized + Send + Sync + 'static {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String>;
}
pub trait AsyncCommandTrait: Sized + Send + Sync + 'static {
    async fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String>;
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandData {
    pub cmd_name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub current_dir: Option<String>,
    pub cmd_icon: Option<String>,
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
pub struct DelCommandGroupParams {
    pub group_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommandParams {
    pub group_name: Option<String>,
    pub cmd: String,
    pub cmd_name: String,
    pub args: Vec<String>,
    pub current_dir: Option<String>,
    pub cmd_icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelCommandParams {
    pub cmd_name: String,
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteCmdData {
    pub cmd: String,
    pub args: Vec<String>,
    pub current_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateUrlParam {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransResult {
    src: String,
    dst: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct TranslateResponseBody {
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
    error_code: Option<i32>,
}
