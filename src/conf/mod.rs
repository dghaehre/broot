use {
    directories,
    std::path::{Path, PathBuf},
};

mod conf;
mod format;
mod verb_conf;

pub use {
    conf::Conf,
    format::*,
    lazy_static::lazy_static,
    verb_conf::VerbConf,
};


/// the content of the conf.hjson file broot creates when there's none.
///
/// It features some default configuration and many sections the user
/// can uncomment then edit.
pub const DEFAULT_CONF_FILE: &str = include_str!("../../resources/default-conf.hjson");

/// return the instance of ProjectDirs holding broot's specific paths
pub fn app_dirs() -> directories::ProjectDirs {
    directories::ProjectDirs::from("org", "dystroy", "broot")
        .expect("Unable to find configuration directories")
}

#[cfg(not(target_os = "macos"))]
fn find_conf_dir() -> PathBuf {
    app_dirs().config_dir().to_path_buf()
}

#[cfg(target_os = "macos")]
fn find_conf_dir() -> PathBuf {
    if let Some(user_dirs) = directories::UserDirs::new() {
        // We first search in ~/.config/broot which should be the prefered solution
        let prefered = user_dirs.home_dir().join(".config/broot");
        if prefered.exists() {
            return prefered;
        }
        // The directories crate has a non usual choice of config directory,
        // especially for a CLI application. We use it only when
        // the prefered directory doesn't exist and this one exists.
        // See https://github.com/Canop/broot/issues/103
        let second_choice = app_dirs().config_dir().to_path_buf();
        if second_choice.exists() {
            // An older version of broot was used to write the
            // config, we don't want to lose it.
            return second_choice;
        }
        // Either the config has been scraped or it's a new installation
        return prefered;
    } else {
        // there's no home. There are probably other problems too but here we
        // are just looking for a place for our config, not for a shelter for all
        // so the default will do
        app_dirs().config_dir().to_path_buf()
    }
}

/// return the path to the config directory
pub fn dir() -> &'static Path {
    lazy_static! {
        static ref CONF_DIR: PathBuf = find_conf_dir();
    }
    &*CONF_DIR
}
