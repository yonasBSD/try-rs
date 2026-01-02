use crate::tui::Theme;
use crate::utils::expand_path;
use ratatui::style::Color;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ThemeConfig {
    pub title_try: Option<String>,
    pub title_rs: Option<String>,
    pub search_box: Option<String>,
    pub list_date: Option<String>,
    pub list_highlight_bg: Option<String>,
    pub list_highlight_fg: Option<String>,
    pub help_text: Option<String>,
    pub status_message: Option<String>,
    pub popup_bg: Option<String>,
    pub popup_text: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub tries_path: Option<String>,
    pub colors: Option<ThemeConfig>,
    pub editor: Option<String>,
}

pub fn load_configuration() -> (PathBuf, Theme, Option<String>, bool) {
    // 1. Try to find the default config directory (~/.config)
    let config_dir = dirs::config_dir().unwrap_or_else(|| {
        // Fallback if not found
        dirs::home_dir().expect("Folder not found").join(".config")
    });

    // 2. Build the path: ~/.config/try-rs/config.toml
    let app_config_dir = config_dir.join("try-rs");
    let config_file = app_config_dir
        .join(std::env::var_os("TRY_CONFIG").unwrap_or_else(|| "config".into()))
        .with_extension("toml");

    // 3. Define the old default (fallback)
    let default_path = dirs::home_dir()
        .expect("Folder not found")
        .join("work")
        .join("tries");

    let mut theme = Theme::default();
    let try_path = std::env::var_os("TRY_PATH");
    let try_path_specified = try_path.is_some();
    let mut final_path = try_path.map(PathBuf::from).unwrap_or(default_path);
    let mut editor_cmd = std::env::var("VISUAL")
        .ok()
        .or_else(|| std::env::var("EDITOR").ok());
    let mut is_first_run = false;

    // 4. If the file exists, try to read it
    if config_file.exists() {
        if let Ok(contents) = fs::read_to_string(&config_file)
            && let Ok(config) = toml::from_str::<Config>(&contents)
        {
            if let Some(path_str) = config.tries_path
                && !try_path_specified
            {
                final_path = expand_path(&path_str);
            }
            if let Some(editor) = config.editor {
                editor_cmd = Some(editor);
            }
            if let Some(colors) = config.colors {
                // Helper to parse color string to Color enum
                let parse = |opt: Option<String>, def: Color| -> Color {
                    opt.and_then(|s| Color::from_str(&s).ok()).unwrap_or(def)
                };

                let def = Theme::default();
                theme = Theme {
                    title_try: parse(colors.title_try, def.title_try),
                    title_rs: parse(colors.title_rs, def.title_rs),
                    search_box: parse(colors.search_box, def.search_box),
                    list_date: parse(colors.list_date, def.list_date),
                    list_highlight_bg: parse(colors.list_highlight_bg, def.list_highlight_bg),
                    list_highlight_fg: parse(colors.list_highlight_fg, def.list_highlight_fg),
                    help_text: parse(colors.help_text, def.help_text),
                    status_message: parse(colors.status_message, def.status_message),
                    popup_bg: parse(colors.popup_bg, def.popup_bg),
                    popup_text: parse(colors.popup_text, def.popup_text),
                };
            }
        }
    } else {
        // Create default config if it doesn't exist
        if fs::create_dir_all(&app_config_dir).is_ok() {
            let default_content = format!("tries_path = {final_path:?}");
            let _ = fs::write(&config_file, default_content);
            is_first_run = true;
        }
    }

    // If nothing works or there is no config, return the default
    (final_path, theme, editor_cmd, is_first_run)
}
