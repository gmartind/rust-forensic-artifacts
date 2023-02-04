use std::process::Command;

use crate::controller::traits::Artifact;


pub struct ArpCache{

}

impl Artifact for ArpCache{
    fn get_artifact(&self) -> Result<String, String> {
        let output = match Command::new("powershell").current_dir("/").args(["Get-NetNeighbor"]).output(){
            Ok(output) => output,
            Err(_) => return Err(String::from("Error al extraer caché ARP."))
        };
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
