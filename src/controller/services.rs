use forensic_rs::prelude::{RegistryReader, RegValue};
use crate::controller::traits::Artifact;

const KEY_ROUTE: &str = "SYSTEM\\CurrentControlSet\\Services";

pub struct Services {

}


fn get_image_path(registry: &mut frnsc_liveregistry_rs::LiveRegistryReader, key: &str) -> String{
    let mut registry_key_route = String::from(KEY_ROUTE);
    registry_key_route.push_str("\\");
    registry_key_route.push_str(&key[..]);
    let registry_key = match registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, 
        &registry_key_route[..]){
            Ok(key) => key,
            Err(_) => return "".to_string()
        };
    let image_path: RegValue = match registry.read_value(registry_key, "ImagePath") {
            Ok(route) => route,
            Err(_) => RegValue::ExpandSZ(String::from(" "))
        };
    let ret = String::from(match TryInto::<String>::try_into(image_path){
        Ok(s) => s,
        Err(_) => return "".to_string()
    });
    ret
}


impl Artifact for Services {
    fn get_artifact(&self) -> Result<String, String>{
        let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
        let registry_key = match registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, 
            KEY_ROUTE){
                Ok(key) => key,
                Err(_) => return Err("Unable to open registry.".to_string())
            };

        let keys = match registry.enumerate_keys(registry_key){
            Ok(keys) => keys,
            Err(_) => return Err("No keys found".to_string())
        };

        let mut ret: String = String::new();
        for key in keys{
            ret.push_str(&key[..]);
            ret.push_str("          ");
            let image_path = get_image_path(&mut registry, &key);
            ret.push_str(&format!("  {}", image_path));
            ret.push('\n');
        }
        Ok(ret)
    }
}

