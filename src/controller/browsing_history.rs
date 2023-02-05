use std::fs::{self, File};
use sqlite::State;
use std::io::Write;
use forensic_rs::prelude::RegistryReader;

use crate::controller::traits::Artifact;

const PROFILE_LIST_ROUTE: &str = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList";
const CHROME_HISTORY: &str = r"AppData\Local\Google\Chrome\User Data\Default\History";


struct UserInfo{
    sid: String,
    image_path: String
}
#[derive(PartialEq)]
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

        let mut ret = String::new();
        let mut users = get_users_sids();
        while let Some(top) = users.pop(){
            ret.push_str(&browsing_history_for_user(top, &browsers));
        }
        Ok(ret)
    }
}

fn browsing_history_for_user(user: UserInfo, browsers: &Vec<BrowsersBrands>) -> String{
    let mut ret = String::new();
    for browser in browsers {
        if browser == &BrowsersBrands::Firefox{
        }
        else if browser == &BrowsersBrands::Chrome{
            match chrome_history(&user.image_path){
                Ok(r) => ret.push_str(&r),
                Err(_) => panic!()
            };
        }
    }
    ret
}

fn chrome_history(image_path: &str) -> Result<String, String>{
    let mut history_path = String::from(image_path);
    history_path.push_str(r"\");
    history_path.push_str(CHROME_HISTORY);
    println!("{}",history_path);
    fs::copy(history_path, "chrome_history.sqlite").unwrap();
    let connection = sqlite::open("chrome_history.sqlite").unwrap();
    let query = "SELECT urls.url AS url, datetime(visits.visit_time / 1000000 + (strftime('%s', '1601-01-01')), 'unixepoch', 'localtime') AS date FROM urls JOIN visits ON visits.url   = urls.id ORDER BY visits.visit_time ASC";
    let mut statement = connection.prepare(query).unwrap();
    let mut ret = String::new();
    while let Ok(State::Row) = statement.next(){
        ret.push_str(&format!("{} : {}", statement.read::<String, _>("date").unwrap(), statement.read::<String, _>("url").unwrap()));
        ret.push('\n');
    }
    Ok(ret)
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

fn get_users_sids() -> Vec<UserInfo> {
    let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
    let registry_key = match registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, PROFILE_LIST_ROUTE){
        Ok(r) => r,
        Err(_) => panic!()
    };
    let values = registry.enumerate_keys(registry_key).unwrap();
    let mut ret = Vec::new();
    for i in values{
        if i.contains("S-1-5-21") {
            let mut path = String::from(PROFILE_LIST_ROUTE);
            path.push_str("\\");
            path.push_str(&i);
            let registry_key = match registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, &path){
                Ok(r) => r,
                Err(_) => panic!()
            };
            let image_path = match registry.read_value(registry_key, "ProfileImagePath"){
                Ok(reg_value) => match TryInto::<String>::try_into(reg_value){
                    Ok(ret) => ret,
                    Err(_) => panic!()
                },
                Err(_) => panic!()
            };
            let temp: UserInfo = UserInfo { sid: (i), image_path: (image_path) };
            ret.push(temp);
        }
    }
    ret
}