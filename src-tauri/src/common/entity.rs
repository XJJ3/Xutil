use serde::{Deserialize, Serialize};

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
pub struct TransResult {
    pub src: String,
    pub dst: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponseBody {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<TransResult>,
    pub error_code: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchedulerData {
    pub scheduler_id: String,
    pub scheduler: String,
    pub notice_title: String,
    pub title_position: String,
    pub font_size: u16,
    pub title_color: String,
    pub background_color: String,
    pub is_run: bool,
}
