use std::process::Command;


pub struct DnsCache{
}

impl DnsCache{
    pub fn new() -> Self {
        Self { }
    }
}

impl DnsCache{
    
    ///Execute powershell command and save
    fn get_artifact(&self) -> Result<String, String> {
        let output = match Command::new("powershell").current_dir("/").args(["Get-DnsClientCache"]).output(){
            Ok(output) => output,
            Err(_) => return Err(String::from("Error al extraer caché DNS"))
        };
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

} 
