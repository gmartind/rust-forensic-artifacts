use std::env;

enum Artifact {
    BrowsingHistory,
}

pub struct Controller {
    available_artifacts: Vec<Artifact>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            available_artifacts: vec![Artifact::BrowsingHistory],
        }
    }

    pub fn acquire(&mut self) -> Result<(), String> {
        match self.get_win_version() {
            Ok(v) => {
                self.get_artifacts(&v);
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn get_artifacts(&mut self, version: &str) -> Result<(), String> {
        Ok(())
    }

    fn get_win_version(&mut self) -> Result<String, String> {
        if env::consts::OS != "windows" {
            return Err("This tool just works for Windows devices".to_string());
        }
        let info = os_info::get().to_string();
        Ok(info)
    }
}
