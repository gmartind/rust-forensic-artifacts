use forensic_rs::prelude::RegistryReader;

use forensic_rs::traits::registry;
use sqlite::State;
use std::fs;
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;

#[derive(Debug)]
enum BrowsersBrands {
    Chrome,
    MSEdge,
    IExplorer,
    Firefox,
    Opera,
    Unknown,
}

fn chrome_history(){
    fs::copy("C:\\Users\\gonza\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\History", "chrome_history.sqlite").unwrap();
    let connection = sqlite::open("chrome_history.sqlite").unwrap();
    let query = "SELECT urls.url AS url, datetime(visits.visit_time / 1000000 + (strftime('%s', '1601-01-01')), 'unixepoch', 'localtime') AS date FROM urls JOIN visits ON visits.url   = urls.id ORDER BY visits.visit_time ASC";
    let mut statement = connection.prepare(query).unwrap();
    let mut output = File::create("chrome_history.txt").expect("unable to create file");
    while let Ok(State::Row) = statement.next(){
        writeln!(output, "{} : {}", statement.read::<String, _>("date").unwrap(), statement.read::<String, _>("url").unwrap()).unwrap();
    }
}

fn mozilla_history() -> Result<Vec<(i64, String)>, &'static str>{
    /*let prueba = get_users_sids();
    for i in &prueba {

        let mut registry = frnsc_liveregistry_rs::LiveRegistryReader{};
        let mut key_name = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\".to_owned();
        key_name.push_str(i);
        let registry_key = registry.open_key(forensic_rs::prelude::RegHiveKey::HkeyLocalMachine, &key_name);
        let values = registry.enumerate_keys(registry_key.unwrap()).unwrap();
        
    }*/
    fs::copy("C:\\Users\\gonza\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\pfaexsjz.default-release\\places.sqlite", "mozilla_places.sqlite").unwrap();
    let connection = sqlite::open("mozilla_places.sqlite").unwrap();
    let query = "SELECT moz_places.url AS url, moz_historyvisits.visit_date AS date FROM moz_historyvisits JOIN moz_places ON moz_historyvisits.place_id = moz_places.id ORDER BY moz_historyvisits.visit_date ASC";
    let mut statement = connection.prepare(query).unwrap();
    let mut output = File::create("mozilla_history.txt").expect("unable to create file");
    while let Ok(State::Row) = statement.next(){
        let time = statement.read::<i64, _>("date").unwrap() / 1000;
        let nt = NaiveDateTime::from_timestamp_millis(time).unwrap();
        let dt = DateTime::<Utc>::from_utc(nt,Utc);
        let res = dt.format("%d-%m-%Y %H:%M:%S");
        writeln!(output, "{} : {}", res, statement.read::<String, _>("url").unwrap()).unwrap();
    }
    Ok(Vec::new())
}

fn browsers() -> Option<Vec<BrowsersBrands>>{
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
        else{
            normalized_values.push(BrowsersBrands::Unknown);
        }
    }
    //Return:
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

fn is_windows() -> bool {
    let os = std::env::consts::OS;
    if os.eq("windows"){ true }
    else {false}
} 

fn main() {
    //1. Comprobar sistema operativo
    //  -> Si es Windows:
    //     2. Identificar navegadores
    //     3. Escribir en un fichero el historial de cada navegador
    //          Podemos añadir tambien la ruta a cada navegador
    if is_windows() {       //Comprobar sistema operativo
        let mut browsers = browsers().expect("No browsers!");
        while let Some(top) = browsers.pop() {
            match top{
                BrowsersBrands::Firefox => mozilla_history(),
                BrowsersBrands::Chrome => chrome_history(),
                _ => println!("Not implemented {:?}", top),
            }
        }
    }
}

#[test]
fn test_chrome_parser(){
    
}