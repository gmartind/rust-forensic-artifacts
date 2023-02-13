use forensic_rs::prelude::RegistryReader;


const DRIVES_ROUTE: &str = "Network";

#[derive(Debug, Default)]
pub struct NetworkDrive{
    pub local_path: String,
    pub remote_path: String,
    pub provider: String
}

#[derive(Debug, Default)]
pub struct NetworkDriveReturn{
    pub drives: Vec<NetworkDrive>
}

pub struct NetworkDriveArtifact{

}

impl NetworkDriveArtifact{
    pub fn acquire() -> Result<NetworkDriveReturn, String>{
        //1.Abrimos registro
        //2.Recorremos claves
        //2.1 Para cada clave volvemos a abrir registro
        //3.Extraemos información


        //1. Abrimos registro
        let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
        let registry_key = registry.open_key(forensic_rs::prelude::RegHiveKey::
            HkeyCurrentUser, DRIVES_ROUTE);
        let values = match registry.enumerate_keys(registry_key.unwrap()){
            Ok(v) => v,
            Err(_) => return Ok(NetworkDriveReturn::default())
        };

        //2. Recorremos claves
        let mut ret = NetworkDriveReturn::default();
        for v in &values{
            let mut temp: NetworkDrive = NetworkDrive::default();
            temp.local_path = v.to_string().to_uppercase();
            temp.local_path.push(':');
            let mut route = String::from(DRIVES_ROUTE);
            route.push_str(r"\");
            route.push_str(&v[..]);

            //2.1 Para cada clave, volvemos a abrir registro
            let registry_key = registry.open_key(forensic_rs::prelude::RegHiveKey::
                HkeyCurrentUser, &route).unwrap();

            //3. Extraemos información
            temp.remote_path = match registry.read_value(registry_key, "RemotePath") {
                Ok(route) => String::from(TryInto::<String>::try_into(route).unwrap()),
                Err(_) => String::new()
            };
            temp.provider = match registry.read_value(registry_key, "ProviderName") {
                Ok(route) => String::from(TryInto::<String>::try_into(route).unwrap()),
                Err(_) => String::new()
            };
            ret.drives.push(temp);
            }
            
        Ok(ret)
    }


}