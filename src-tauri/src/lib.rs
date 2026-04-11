mod commands;
mod models;
mod utils;

use commands::{config, install, repo, skill};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // 配置
            config::load_config,
            config::save_config,
            config::get_tool_path,
            config::set_tool_path,
            config::get_project_paths,
            config::add_project_path,
            config::remove_project_path,
            config::get_default_tool_type,
            config::set_default_tool_type,
            config::clear_project_paths,
            config::clear_tool_paths,
            // 仓库
            repo::add_repo,
            repo::add_local_dir,
            repo::remove_repo,
            repo::update_repo,
            repo::update_all_repos,
            repo::list_repos,
            // 技能
            skill::list_skills,
            skill::search_skills,
            // 安装
            install::install_skill,
            install::uninstall_skill,
            install::check_junction_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
