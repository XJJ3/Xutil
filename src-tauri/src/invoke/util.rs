use std::{
    fs::{self, create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use rand::{distributions::Alphanumeric, Rng};
use tauri::api::path::local_data_dir;

use super::common::{USER_CMD_FILE, USER_CMD_ICON_FILE_DIR};
use crate::common::GLOBAL_USER_DATA_DIR;

pub fn get_user_data_directory() -> Option<Box<PathBuf>> {
    let data_dir = match local_data_dir() {
        Some(mut path) => {
            path.push(GLOBAL_USER_DATA_DIR);
            path
        }
        None => return None,
    };

    if !data_dir.exists() {
        match create_dir_all(&data_dir) {
            Ok(_) => Some(Box::new(data_dir)),
            Err(_) => return None,
        }
    } else {
        Some(Box::new(data_dir))
    }
}

pub fn get_command_file_path() -> Option<Box<Path>> {
    let command_data_dir = match get_user_data_directory() {
        Some(mut path) => {
            path.push(USER_CMD_FILE);
            path
        }
        None => return None,
    };

    Some(command_data_dir.as_path().into())
}

pub fn get_command_icons_file_dir() -> Option<Box<Path>> {
    let command_icons_dir = match get_user_data_directory() {
        Some(mut path) => {
            path.push(USER_CMD_ICON_FILE_DIR);
            path
        }
        None => return None,
    };

    if !command_icons_dir.exists() {
        match fs::create_dir(command_icons_dir.as_path()) {
            Ok(_) => Some(command_icons_dir.as_path().into()),
            Err(_) => None,
        }
    } else {
        Some(command_icons_dir.as_path().into())
    }
}

pub fn get_file(file_path: &Path) -> Result<File, String> {
    if !file_path.exists() {
        Err(String::from("路径不存在"))
    } else {
        match OpenOptions::new().read(true).write(true).open(file_path) {
            Err(why) => Err(why.to_string()),
            Ok(file) => Ok(file),
        }
    }
}

pub fn read_user_command_setting_data() -> Result<String, String> {
    let data_dir = match get_command_file_path() {
        Some(path) => path,
        None => return Err(String::from("指令文件路径查询失败")),
    };

    let mut file = match get_file(&data_dir) {
        Err(why) => return Err(why.to_string()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    Ok(contents)
}

pub fn write_user_command_setting_data(content: String) -> Result<String, String> {
    if let Some(path) = get_command_file_path() {
        let mut file = match get_file(&path) {
            Err(_) => match File::create(&path) {
                Err(_) => return Err("路径不存在，尝试创建文件失败！".to_string()),
                Ok(file) => file,
            },
            Ok(file) => file,
        };

        let _ = file.set_len(0); // 清空文件内容

        println!("写入文件内容： {}", content);
        match file.write_all(content.as_bytes()) {
            Err(why) => Err(why.to_string()),
            Ok(_) => Ok(String::from("successfully")),
        }
    } else {
        Err("系统用户数据路径获取失败".to_string())
    }
}

pub fn generate_uuid() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()
}
