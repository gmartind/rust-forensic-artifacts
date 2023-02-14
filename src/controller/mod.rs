mod dns_cache;
mod arp_cache;
mod services;
mod traits;
mod browsing_history;
mod network_drive;

use std::env;
use windows::Win32::Foundation::DNS_ERROR_DATABASE_BASE;
use self::network_drive::NetworkDriveArtifact;
use self::browsing_history::BrowsingHistoryArtifact;
use self::dns_cache::DnsCache;
use self::arp_cache::ArpCache;
use self::services::ServicesArtifact;
use crate::controller::network_drive::NetworkDriveReturn;


pub struct Controller {
    //available_artifacts: Vec<Box<dyn Artifact>>
}

impl Controller {
    pub fn new() -> Self {
        Self {

        }
    }


    ///If we are in a windows system, get forensic artifacts
    pub fn acquire(&mut self) -> Result<(), String> {
        let version= self.get_win_version()?;
        self.get_artifacts(&version)
    }

    ///This function checks if it is running in a Windows System and returns the version
    fn get_win_version(&mut self) -> Result<String, String> {
        if env::consts::OS != "windows" {
            return Err("This tool just works for Windows devices".to_string());
        }
        let info = os_info::get().to_string();
        Ok(info)
    }

    ///Iterate the implemented artifacts and acquire 
    fn get_artifacts(&self, version: &str) -> Result<(), String> {
        println!("{}", version);
        let service_ret = match ServicesArtifact::acquire(){
            Ok(r) => r,
            Err(_) => return Err(String::from("No se pudieron extraer los servicios"))
        };
        for s in service_ret.get_services() {
            println!("{}    {}", s.get_name(), s.get_path());
        }
        //let browsing_history_ret = match BrowsingHistoryArtifact::acquire(){
        //    Ok(r) => r,
        //    Err(_) => return Err(String::from("No se pudieron extraer los historiales de navegación"))
        //};

        //for (user, history) in browsing_history_ret.get_user_history(){
        //    println!("Historial de navegación para usuario {:?}:{:?}", user.sid, user.image_path);
        //    println!("  Historial de Google Chrome:");
        //    for entry in &history.chrome{
        //        println!("  {}: {}", entry.time, entry.path);
        //    }
        //}

        //let ret = DnsCache::get_artifact().unwrap();
        //println!("{:?}", ret);

    
        let ret = ArpCache::get_artifact().unwrap();
        println!("{:?}", ret);
        /*
        let ret = match NetworkDriveArtifact::acquire(){
            Ok(r) => r,
            Err(_) => NetworkDriveReturn::default()
        };
        println!("{:?}", ret);

        let ret = ServicesArtifact::acquire().unwrap();
        for service in ret.get_services() {
            println!("{} {}", service.get_name(), service.get_path());
        }*/
        Ok(())
    }
}
