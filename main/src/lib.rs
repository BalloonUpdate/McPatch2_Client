pub mod global_config;
pub mod utility;
pub mod error;

use std::path::Path;
use std::path::PathBuf;

use crate::global_config::GlobalConfig;
use crate::utility::is_running_under_cargo;

pub struct AppContext {
    pub working_dir: PathBuf,
    pub workspace_dir: PathBuf,
    pub public_dir: PathBuf,
    pub index_file: PathBuf,
    pub config: GlobalConfig
}

pub struct StartupParameter {
    pub graphic_mode: bool,
    pub standalone_progress: bool,
    pub disable_log_file: bool,
    // pub external_config_file: String,
}

pub async fn run(_parameters: StartupParameter) {
    let working_dir = get_working_dir().await;
    let executable_dir = get_executable_dir().await;
    let global_config = GlobalConfig::load(&executable_dir.join("mcpatch.yml"));

    
}

/// 获取更新起始目录
async fn get_base_dir(global_config: &GlobalConfig) -> Result<PathBuf, String> {
    let working_dir = get_working_dir().await;

    if is_running_under_cargo() {
        return Ok(working_dir);
    }

    // 智能搜索.minecraft文件夹
    if global_config.base_path.is_empty() {
        let mut current = &working_dir as &Path;

        for _ in 0..7 {
            let detect = tokio::fs::try_exists(current.join(".minecraft")).await;

            match detect {
                Ok(found) => {
                    if found {
                        return Ok(current.to_owned());
                    }

                    current = match current.parent() {
                        Some(parent) => parent,
                        None => break,
                    }
                },
                Err(_) => break,
            }
        }

        return Err(".minecraft not found".into());
    }

    let base_dir = working_dir.join(&global_config.base_path);
    tokio::fs::create_dir_all(&base_dir).await.unwrap();
    Ok(base_dir)
}

/// 获取可执行文件所在目录
async fn get_executable_dir() -> PathBuf {
    if is_running_under_cargo() {
        get_working_dir().await
    } else {
        let exe = std::env::args().next().unwrap();
        PathBuf::from(exe).parent().unwrap().to_owned()
    }
}

/// 获取工作目录
async fn get_working_dir() -> PathBuf {
    let mut working_dir = std::env::current_dir().unwrap();
        
    if is_running_under_cargo() {
        working_dir = working_dir.join("test");
    }

    tokio::fs::create_dir_all(&working_dir).await.unwrap();
    working_dir
}