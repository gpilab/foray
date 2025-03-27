use std::{fs::read_to_string, path::PathBuf};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

/// User data that should persist across sessions, but isn't explicitly configured by the user
/// e.g. recent files, recently used nodes, etc.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserData {
    most_recent_network_file: Option<PathBuf>,
}

impl UserData {
    fn user_data_dir() -> PathBuf {
        let binding = directories::ProjectDirs::from("", "", "gpi")
            .expect("application configuration folder is accessible");
        binding.cache_dir().to_path_buf()
    }
    fn user_data_file() -> PathBuf {
        Self::user_data_dir().join("user_data.ron")
    }
    pub fn read_user_data() -> Self {
        let user_data_file = Self::user_data_file();
        match read_to_string(&user_data_file).map(|s| ron::from_str::<UserData>(&s)) {
            Ok(Ok(c)) => {
                info!("Loaded UserData: {user_data_file:?}");
                c
            }
            Ok(Err(e)) => {
                error!("Error reading user data {user_data_file:?}, using default. \n{e}");
                UserData::default()
            }
            Err(e) => {
                warn!("Could not read user data file {user_data_file:?}, using default. \n{e}");
                UserData::default()
            }
        }
    }

    pub fn set_recent_network_file(&mut self, file: PathBuf) {
        self.most_recent_network_file = Some(file);
        self.write();
    }
    pub fn get_recent_network_file(&self) -> &Option<PathBuf> {
        &self.most_recent_network_file
    }
    pub fn network_search_dir(&self) -> PathBuf {
        if let Some(file) = &self.most_recent_network_file {
            if let Some(parent) = file.parent() {
                return parent.to_path_buf();
            }
        }

        let binding = directories::UserDirs::new().expect("user directory should be  accessible");
        binding.home_dir().to_path_buf()
    }

    fn write(&self) {
        let user_data_file = Self::user_data_file();
        let _ = std::fs::create_dir_all(Self::user_data_dir());
        std::fs::write(
            &user_data_file,
            ron::to_string(&self)
                .unwrap_or_else(|e| panic!("Could not parse user_data {user_data_file:?}\n{e}")),
        )
        .unwrap_or_else(|e| error!("Could not write user data file {user_data_file:?}\n{e}"));
    }
}
