use forensic_rs::prelude::{RegistryReader, RegValue};

const KEY_ROUTE: &str = r"SYSTEM\CurrentControlSet\Services";




pub struct ServiceReturn{
    services: Vec<Service>
}

impl ServiceReturn{
    pub fn new() -> Self{
        Self { services: Vec::new() }
    }

    pub fn get_services(&self) -> &Vec<Service> {
        &self.services
    }
}

pub struct Service{
    pub name: String,
    pub image_path: String
}

impl Service{
    pub fn get_name(&self) -> &String{
        &self.name
    }

    pub fn get_path(&self) -> &String{
        &self.image_path
    }
}


pub struct ServicesArtifact {
    
}

impl ServicesArtifact{

    pub fn acquire() -> Result<ServiceReturn, String> {
        let mut ret = ServiceReturn::new();
        let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
        let registry_key = match registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, 
            KEY_ROUTE){
                Ok(key) => key,
                Err(_) => return Err("Unable to open registry.".to_string())
        };

        let keys = match registry.enumerate_keys(registry_key){
            Ok(keys) => keys,
            Err(_) => Vec::new()
        };


        for key in keys{
            let image_path = get_image_path(&mut registry, &key);
            ret.services.push(Service { name: key, image_path: image_path });
        }
        Ok(ret)
    }
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



