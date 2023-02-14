use std::process::Command;
use forensic_rs::prelude::ForensicResult;
use windows::Win32::{Networking::WinSock::ADDRESS_FAMILY, NetworkManagement::IpHelper::{MIB_IPNET_TABLE2, GetIpNetTable, GetIpNetTable2, MIB_IPNETTABLE}};
use windows::Win32::Networking::WinSock::inet_ntoa;
use std::net::Ipv4Addr;


#[derive(Debug, Default)]
pub struct ArpCacheEntry{
    if_index: String,
    ip_address: String,
    mac_address: String
}

#[derive(Debug, Default)]
pub struct ArpCacheReturn{
    entries: Vec<ArpCacheEntry>
}

pub struct ArpCache{
}

impl ArpCache{

    pub fn acquire_cache(){
            unsafe{
                let mut size: u32 = 0;
                let pointer_to_size = &mut size as *mut u32;
                let mut argument: MIB_IPNETTABLE = MIB_IPNETTABLE::default();
                let mut pointer_to_argument = &mut argument as *mut MIB_IPNETTABLE;
                let option: Option<*mut MIB_IPNETTABLE> = Some(pointer_to_argument);
                GetIpNetTable(option, pointer_to_size, false);
                println!("{}", *pointer_to_size);
                GetIpNetTable(option, pointer_to_size, false);
                let n_elements = (*pointer_to_argument).dwNumEntries;
                println!("{}", n_elements);
                for _ in 0..n_elements {
                    let ip_address = Ipv4Addr::from((*pointer_to_argument).table[0].dwAddr);
                    println!("{}", ip_address);
                    pointer_to_argument = pointer_to_argument.add(1);
                }
                
            }
    }

    pub fn get_artifact() -> ForensicResult<ArpCacheReturn> {
        let output = Command::new("powershell").current_dir("/").args(["Get-NetNeighbor"]).output()?;
        let output = String::from_utf8_lossy(&output.stdout).to_string();

        let mut ret = ArpCacheReturn::default();

        for line in output.lines().skip(3){
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() < 5 {
                continue;
            }
            let mut temp = ArpCacheEntry::default();
            temp.if_index = String::from(words[0]);
            temp.ip_address = String::from(words[1]);
            temp.mac_address = String::from(words[2]);
            ret.entries.push(temp);
        }
        
        Ok(ret)
    }
}
