mod commands;
mod git;
mod logger;
mod paths;
mod skills;
mod store;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化目录
    paths::init_app_dirs();

    // 加载配置
    store::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // dev 模式自动打开 DevTools
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 仓库管理
            commands::repo_list,
            commands::repo_add,
            commands::repo_delete,
            commands::repo_pull,
            // 技能
            commands::skills_scan,
            commands::skills_install,
            commands::skills_toggle_app,
            commands::skills_uninstall,
            commands::skills_check_updates,
            commands::skills_apply_update,
            // 已安装
            commands::installed_list,
            // 工具
            commands::tools_list,
            commands::tools_check_installed,
            // 设置
            commands::settings_get,
            commands::settings_set,
            // 日志
            commands::logs_read,
            commands::logs_clear,
            // 目录
            commands::dir_open,
            commands::dir_list,
            commands::dir_size,
            // 导入导出
            commands::config_export,
            commands::config_import,
            // skills.sh
            commands::skills_sh_search,
            commands::skills_sh_trending,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
