use chrono::Local;
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::{collections::HashMap, fs, path};
use sysinfo::{Disks, IS_SUPPORTED_SYSTEM, IpNetwork, MacAddr, Networks, System};

#[derive(Serialize, Deserialize)]
pub struct Log {
    name: String,
    creation_time: String,
    compiled_on: String,
    current_platform: String,
    sysinfo: Option<SystemSerDe>,
}

impl Log {
    pub fn new() -> Self {
        let local_time = Local::now();

        Log {
            name: format!(
                "{}_{}_{}",
                local_time.format("%Y-%m-%d"),
                System::host_name().unwrap_or("no hostname".to_string()),
                System::long_os_version().unwrap_or("no os version".to_string())
            ),
            creation_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            compiled_on: COMPILED_ON.to_string(),
            current_platform: CURRENT_PLATFORM.to_string(),
            sysinfo: if IS_SUPPORTED_SYSTEM {
                Some(SystemSerDe::new())
            } else {
                None
            },
        }
    }

    pub fn to_file(&self) {
        let logs_dir = "./logs";
        let log_file_name = format!("{}.json", self.name);
        match path::Path::new(logs_dir).exists() {
            true => {}
            false => match fs::create_dir(logs_dir) {
                Ok(()) => (),
                Err(err) => println!("Directory Creation Error: {}", err),
            },
        }
        fs::write(format!("{}/{}", logs_dir, log_file_name), self.to_json()).unwrap();
    }
    pub fn to_json(&self) -> String {
        to_string_pretty(&self).expect("Something went wrong turning the Log into Json")
    }

    pub fn from_json(json: &str) -> Self {
        from_str(json).unwrap()
    }

    pub fn show(&self) {
        println!("{}", self.to_json());
    }
}

fn readable_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{}B", bytes);
    }
    let mut bytes = bytes as f64;
    let suffix = ["B", "KiB", "MiB", "GiB", "TiB", "EiB"];
    let mut index = 0;
    while bytes >= 1024.0 {
        index += 1;
        bytes /= 1024.0;
    }
    format!("{:.2}{}", bytes, suffix[index])
}

#[derive(Serialize, Deserialize)]
struct SystemSerDe {
    name: String,
    kernel_version: String,
    os_version: String,
    long_os_version: String,
    distribution_id: String,
    host_name: String,
    total_memory: String,
    number_cpus: u64,
    brand_cpu: String,
    networks: HashMap<String, Network>,
    disks: HashMap<String, Disk>,
}

impl SystemSerDe {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let networks = Networks::new_with_refreshed_list();
        let mut network_hash: HashMap<String, Network> = HashMap::new();
        for (interface_name, network) in &networks {
            if interface_name.ne("lo") {
                network_hash.insert(
                    interface_name.into(),
                    Network {
                        mac_address: network.mac_address(),
                        ip_network: network.ip_networks().into(),
                    },
                );
            }
        }

        let mut disk_map: HashMap<String, Disk> = HashMap::new();
        for disk in Disks::new_with_refreshed_list().list() {
            disk_map.insert(
                String::from_utf8_lossy(disk.name().as_encoded_bytes()).to_string(),
                Disk {
                    total_space: readable_bytes(disk.total_space()),
                    avaiable_space: readable_bytes(disk.available_space()),
                    file_system: String::from_utf8_lossy(disk.file_system().as_encoded_bytes())
                        .to_string(),
                    disk_type: disk.kind().to_string(),
                },
            );
        }
        SystemSerDe {
            name: System::name().unwrap(),
            kernel_version: System::kernel_version().unwrap(),
            os_version: System::os_version().unwrap(),
            long_os_version: System::long_os_version().unwrap(),
            distribution_id: System::distribution_id(),
            host_name: System::host_name().unwrap(),
            total_memory: readable_bytes(sys.total_memory()),
            number_cpus: sys.cpus().len() as u64,
            brand_cpu: sys.cpus().get(0).unwrap().brand().to_string(),
            networks: network_hash,
            disks: disk_map,
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Network {
    mac_address: MacAddr,
    ip_network: Vec<IpNetwork>,
}
#[derive(Serialize, Deserialize)]
struct Disk {
    total_space: String,
    avaiable_space: String,
    file_system: String,
    disk_type: String,
}

#[cfg(test)]
mod log_tests {
    use super::readable_bytes;
    #[test]
    fn check_readable_bytes() {
        assert_eq!(String::from("500B"), readable_bytes(500));
        assert_eq!(String::from("1023B"), readable_bytes(1023));
        assert_eq!(String::from("1.00KiB"), readable_bytes(1024));
        assert_eq!(String::from("1.27KiB"), readable_bytes(1300));
        assert_eq!(String::from("2.00KiB"), readable_bytes(2048));
        assert_eq!(String::from("2.50KiB"), readable_bytes(2560));
        assert_eq!(String::from("1.00MiB"), readable_bytes(1048576));
        assert_eq!(String::from("2.00MiB"), readable_bytes(2097152));
        assert_eq!(String::from("1.00GiB"), readable_bytes(1073741824));
    }
}
