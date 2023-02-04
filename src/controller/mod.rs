mod dns_cache;
mod arp_cache;
mod services;
mod traits;
mod browsing_history;
use std::env;
use self::{ dns_cache::DnsCache};
use self::{ arp_cache::ArpCache};
use self::{ services::Services};
use crate::controller::traits::Artifact;

pub struct Controller {
    available_artifacts: Vec<Box<dyn Artifact>>
}

impl Controller {
    pub fn new() -> Self {
        let mut artifacts: Vec<Box<dyn Artifact>> = Vec::new();
        let dns_cache = DnsCache{};
        let arp_cache = ArpCache{};
        let services = Services{};
        artifacts.push(Box::new(dns_cache));
        artifacts.push(Box::new(arp_cache));
        artifacts.push(Box::new(services));
        Self {
            available_artifacts: artifacts
        }
    }

    pub fn acquire(&mut self) -> Result<(), String> {
        match self.get_win_version() {
            Ok(v) => {
                match self.get_artifacts(&v) {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e)
                }
            }
            Err(e) => return Err(e),
        }
    }

    fn get_artifacts(&self, version: &str) -> Result<(), String> {
        println!("{}", version);
        for artifact in &self.available_artifacts {
            match artifact.get_artifact(){
                Ok(output) => println!("{}", output),
                Err(e) => return Err(e)
            }
        }
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
