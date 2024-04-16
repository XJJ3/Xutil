use crate::invoke::util::{generate_uuid, get_command_icons_file_dir};

use super::{
    common::*,
    util::{read_user_command_setting_data, write_user_command_setting_data},
};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

pub struct GetAllCommands;
impl CommandTrait for GetAllCommands {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let commands_data = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };
        // println!("获取到的内容： {:?}", commands_data);
        Ok(serde_json::json!({ "result": commands_data }))
    }
}

pub struct AddCommandGroup;
impl CommandTrait for AddCommandGroup {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = serde_json::from_str::<AddCommandGroupParams>(&args.to_string()).unwrap();
        println!("准备写入指令数据：{:?}", params);

        let group_name = match params.group_name {
            Some(name) => name,
            None => "default".to_string(),
        };

        let group_icon = match params.group_icon {
            Some(icon) => icon,
            None => "resources/setting.svg".to_string(),
        };
        let group_icon_url = Path::new(&group_icon);

        let path_extension = match group_icon_url.extension() {
            Some(extension) => match extension.to_str() {
                Some(str) => str,
                None => "",
            },
            None => "",
        };

        let icons_dir_path = match get_command_icons_file_dir() {
            Some(path) => path,
            None => return Err("数据路径获取失败".to_string()),
        };

        let new_path = format!(
            "{}/{}.{}",
            icons_dir_path.display(),
            format!("group_{}_{}", group_name, &generate_uuid()),
            path_extension
        );

        // println!("新文件： {}", new_path);

        if let Ok(_file_u64) = fs::copy(group_icon_url, Path::new(&new_path)) {
            println!("拷贝成功");
        } else {
            return Err("图标文件拷贝失败".to_string());
        }

        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_cmd_data: Vec<CommandGroupData> = vec![CommandGroupData {
            group_name: String::from("default"),
            group_icon: None,
            commands: Vec::new(),
        }];

        if let Some(cont) = contents {
            all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&cont).unwrap();
            all_cmd_data.push(CommandGroupData {
                group_name: group_name,
                group_icon: Some(new_path),
                commands: Vec::new(),
            })
        } else {
            all_cmd_data[0].group_name = group_name;
            all_cmd_data[0].group_icon = Some(new_path);
        }

        // println!("现有数据： {:?}", all_cmd_data);

        let json_data = serde_json::to_string(&all_cmd_data).unwrap();
        // println!("添加完之后的数据: {:?}", json_data);

        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": all_cmd_data }))
    }
}

pub struct DelCommandGroup;
impl CommandTrait for DelCommandGroup {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = serde_json::from_str::<DelCommandGroupParams>(&args.to_string()).unwrap();

        let group_name = match params.group_name {
            Some(name) => name,
            None => return Err("请传入需要删除的组合名称".to_string()),
        };

        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_cmd_data: Vec<CommandGroupData> = vec![];
        if let Some(cont) = contents {
            all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&cont).unwrap();
        };

        let res: Vec<CommandGroupData> = all_cmd_data
            .into_iter()
            .filter(|item| item.group_name != group_name)
            .collect();

        let json_data = serde_json::to_string(&res).unwrap();
        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "success" }))
    }
}

pub struct AddCommand;
impl CommandTrait for AddCommand {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = serde_json::from_str::<AddCommandParams>(&args.to_string()).unwrap();
        println!("准备写入指令数据：{:?}", params);

        let group_name = match params.group_name {
            Some(name) => name,
            None => "default".to_string(),
        };

        if params.cmd_name.is_empty() {
            return Err("请输入指令名称".to_string());
        }

        let cmd_icon = match params.cmd_icon {
            Some(icon) => icon,
            None => "".to_string(),
        };

        let mut new_path: Option<String> = None;
        if !cmd_icon.is_empty() {
            let group_icon_url = Path::new(&cmd_icon);
            let path_extension = match group_icon_url.extension() {
                Some(extension) => match extension.to_str() {
                    Some(str) => str,
                    None => "",
                },
                None => "",
            };
            let icons_dir_path = match get_command_icons_file_dir() {
                Some(path) => path,
                None => return Err("数据路径获取失败".to_string()),
            };
            let new_icon_path = format!(
                "{}/{}.{}",
                icons_dir_path.display(),
                format!("group_{}_{}", group_name, &generate_uuid()),
                path_extension
            );
            if let Ok(_file_u64) = fs::copy(group_icon_url, Path::new(&new_icon_path)) {
                println!("拷贝成功");
                new_path = Some(new_icon_path);
            } else {
                return Err("图标文件拷贝失败".to_string());
            }
        };

        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_cmd_data: Vec<CommandGroupData> = vec![CommandGroupData {
            group_name: String::from("default"),
            group_icon: None,
            commands: Vec::new(),
        }];

        if let Some(cont) = contents {
            all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&cont).unwrap();
        }

        // println!("现有数据： {:?}", all_cmd_data);

        for cmd_group in all_cmd_data.iter_mut() {
            if group_name == cmd_group.group_name {
                cmd_group.commands.push(CommandData {
                    cmd: params.cmd.clone(),
                    args: params.args.clone(),
                    current_dir: params.current_dir.clone(),
                    cmd_icon: new_path.clone(),
                    cmd_name: params.cmd_name.clone(),
                });
            }
        }

        let json_data = serde_json::to_string(&all_cmd_data).unwrap();
        println!("添加完之后的数据: {:?}", json_data);

        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "success" }))
    }
}

pub struct DelCommand;
impl CommandTrait for DelCommand {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = serde_json::from_str::<DelCommandParams>(&args.to_string()).unwrap();

        let group_name = params.group_name;
        let cmd_name = params.cmd_name;

        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_cmd_data: Vec<CommandGroupData> = vec![];
        if let Some(cont) = contents {
            all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&cont).unwrap();
        };

        for cmd_group in all_cmd_data.iter_mut() {
            if group_name == cmd_group.group_name {
                let res: Vec<CommandData> = cmd_group
                    .commands
                    .clone()
                    .into_iter()
                    .filter(|cmd| cmd.cmd_name != cmd_name)
                    .collect();
                cmd_group.commands = res;
            }
        }

        println!("最后结果： {:?}", &all_cmd_data);
        let json_data = serde_json::to_string(&all_cmd_data).unwrap();
        println!("最后序列化结果： {:?}", json_data);

        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "success" }))
    }
}

pub struct ExecuteCmd;

impl CommandTrait for ExecuteCmd {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = match serde_json::from_str::<ExecuteCmdData>(&args.to_string()) {
            Ok(arg) => arg,
            Err(_) => return Err("参数解析失败！".to_string()),
        };

        if params.cmd.is_empty() {
            return Err("请传入指令".to_string());
        }
        let mut cmd = Command::new(params.cmd);
        cmd.args(&params.args);

        if let Some(cur_dir) = params.current_dir {
            if !cur_dir.is_empty() {
                cmd.current_dir(cur_dir);
            }
        }

        let handle = thread::spawn(move || {
            // thread::sleep(Duration::from_secs(5));
            let status = cmd.status().expect("Failed to execute command");
            // 检查命令是否成功执行
            if status.success() {
                println!("Script executed successfully!");
            } else {
                println!("Script failed to execute: {}", status);
            }
            status

            // let child = cmd
            //     .stdout(Stdio::piped())
            //     .spawn()
            //     .expect("Failed to execute command");

            // let mut out = BufReader::new(child.stdout.unwrap());
            // let mut line = String::new();
            // while let Ok(_) = out.read_line(&mut line) {
            //     println!("{}", line);
            // }
        });

        let _status = handle.join().unwrap();

        Ok(serde_json::json!({ "result": "执行完毕" }))
    }
}
