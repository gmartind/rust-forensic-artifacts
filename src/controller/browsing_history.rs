use forensic_rs::prelude::RegistryReader;

use crate::controller::traits::Artifact;

enum BrowsersBrands {
    Chrome,
    MSEdge,
    IExplorer,
    Firefox,
    Opera
}

pub struct BrowsingHistory {

}

impl Artifact for BrowsingHistory{
    fn get_artifact(&self) -> Result<String, String>{
        let browsers = match get_browsers(){
            Some(b) => b,
            None => return Ok(String::from("No browsers"))
        };
        Ok(String::from("ok"))
    }
}

fn get_browsers() -> Option<Vec<BrowsersBrands>>{
    let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
    let registry_key = registry.open_key(forensic_rs::prelude::RegHiveKey::
        HkeyLocalMachine, "Software\\Clients\\StartMenuInternet");
    let mut values = registry.enumerate_keys(registry_key.unwrap()).unwrap();
    let mut normalized_values: Vec<BrowsersBrands> = Vec::new();
    while let Some(top) = values.pop() {
        if top.to_lowercase().contains("chrome"){
            normalized_values.push(BrowsersBrands::Chrome);
        }
        else if top.to_lowercase().contains("edge"){
            normalized_values.push(BrowsersBrands::MSEdge);
        }
        else if top.to_lowercase().contains("iexplore"){
            normalized_values.push(BrowsersBrands::IExplorer);
        }
        else if top.to_lowercase().contains("firefox"){
            normalized_values.push(BrowsersBrands::Firefox);
        }
        else if top.to_lowercase().contains("opera"){
            normalized_values.push(BrowsersBrands::Opera);
        }
    }
    if normalized_values.len() == 0 {
        return None
    }
    Some(normalized_values)
}

fn get_users_sids() -> Vec<String> {
    let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
    let registry_key = registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList");
    let values = registry.enumerate_keys(registry_key.unwrap()).unwrap();
    let mut ret = Vec::new();
    for i in values{
        if i.contains("S-1-5-21") {
            ret.push(i);
        }
    }
    ret
}