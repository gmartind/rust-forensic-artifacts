use std::process::Command;
use windows::Win32::{Networking::WinSock::ADDRESS_FAMILY, NetworkManagement::IpHelper::{MIB_IPNET_TABLE2, GetIpNetTable, GetIpNetTable2, MIB_IPNETTABLE}};
use windows::Win32::Networking::WinSock::inet_ntoa;
use std::net::Ipv4Addr;

pub struct ArpCache{
}

impl ArpCache{
    pub fn acquire(){
        unsafe{
            let address_family: ADDRESS_FAMILY = ADDRESS_FAMILY{0: 2};                                      //IPV4
            let mut table: MIB_IPNET_TABLE2 = MIB_IPNET_TABLE2::default();
            let mut argument = &mut table as *mut MIB_IPNET_TABLE2;
            let argument2 = &mut argument as *mut *mut MIB_IPNET_TABLE2;
            GetIpNetTable2(address_family, argument2).unwrap();
            {
                let table = (*argument).Table;
                for element in &table{
                    let ip_address = inet_ntoa(element.Address.Ipv4.sin_addr).to_string().unwrap();
                    println!("{}", ip_address);
                }
            };
            let n_elements = (*argument).NumEntries;
            println!("Nº elements: {}", n_elements);
            
            let mut subtable = argument;
            
            for _ in 0..n_elements {
                let ip_address = inet_ntoa((*subtable).Table[0].Address.Ipv4.sin_addr).to_string().unwrap();
                subtable = subtable.add(1);
                println!("{}", ip_address);
            }
        }

        
        
    }
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
    pub fn get_artifact() -> Result<String, String> {
        let output = match Command::new("powershell").current_dir("/").args(["Get-NetNeighbor"]).output(){
            Ok(output) => output,
            Err(_) => return Err(String::from("Error al extraer caché ARP."))
        };
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
