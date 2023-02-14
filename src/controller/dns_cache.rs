use std::process::Command;


#[derive(Debug, Default)]
pub struct DnsCacheEntry{
    entry: String,
    record_type: String,
    status: String,
    ttl: String,
    data: String
}

#[derive(Debug, Default)]
pub struct DnsCacheReturn{
    entries: Vec<DnsCacheEntry>
}

pub struct DnsCache{
}

impl DnsCache{
    
    pub fn get_artifact() -> Result<DnsCacheReturn, String> {
        let output = match Command::new("powershell").current_dir("/").args(["Get-DnsClientCache"]).output(){
            Ok(output) => output,
            Err(_) => return Err(String::from("Error al extraer caché DNS"))
        };
        let mut ret = DnsCacheReturn::default();
        let output = String::from_utf8_lossy(&output.stdout).to_string();
        for line in output.lines().skip(4){
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() < 8 {
                continue;
            }
            let mut temp: DnsCacheEntry = DnsCacheEntry::default();
            temp.entry = String::from(words[0]);
            temp.record_type = String::from(words[2]);
            temp.status= String::from(words[3]);
            temp.ttl = String::from(words[6]);
            temp.data = String::from(words[7]);
            ret.entries.push(temp);
        }
        Ok(ret)
    }

} 
