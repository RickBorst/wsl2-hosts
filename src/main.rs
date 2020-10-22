extern crate regex;

use regex::Regex;

use std::process::Command;
use std::fs;
use std::path::PathBuf;

const CONFIG: &str = "config.txt";

fn get_ifconfig () -> String {
    let output = Command::new("wsl")
        .args(&["--", "ifconfig"])
        .output()
        .expect("Failed to fetch WSL2 ifconfig.");

    let ifconfig = output.stdout;
    
    String::from_utf8(ifconfig)
        .expect("Found invalid UTF-8")
}

fn get_ip_from_ifconfig (ifconfig: String) -> String {
    let ip_matcher = Regex::new(r"eth0:(?:[\S .]+\n)+ *inet ([0-9\.]+)").unwrap();
    let captures = ip_matcher.captures(&ifconfig).unwrap();
    let ip_match = captures.get(1);
    ip_match
        .map_or("", |ip| ip.as_str())
        .to_string()
}

fn get_ip () -> String {
    let ifconfig = get_ifconfig();
    let ip = get_ip_from_ifconfig(ifconfig);
    println!("WSL2 ip found: {}", ip);
    ip
}

fn get_hosts () -> String {
    fs::read_to_string("C:/Windows/System32/drivers/etc/hosts")
        .expect("Failed to read hosts file")
}

fn get_current_path () -> PathBuf {
    std::env::current_dir()
        .expect("Failed to get current path")
}

fn get_config () -> String {
    let mut path = get_current_path();
    path.push(CONFIG);
    fs::read_to_string(path).unwrap()
}

fn get_wsl2_snippet (ip: String) -> String {
    let config = get_config();
    let lines: Vec<_> = config.lines()
        .map(|entry| format!("{}            {}\n", ip, entry))
        .collect();

    format!("# WSL2(begin)\n{}# WSL2(end)", lines.concat())
}

fn write_ip_to_hosts (ip: String) {
    let mut hosts = get_hosts();
    
    if hosts.contains(&ip) {
        println!("Ip {} was already present in hosts file:\n\n{}", ip, hosts);
        println!("Done!");
        std::process::exit(0)
    } else if hosts.contains("# WSL2(begin)") {
        let excerpt = Regex::new(r"(\n?# WSL2\(begin\)\n(?:.+\n)+# WSL2\(end\)\n?)").unwrap();
        hosts = excerpt.replace(&hosts, "").into_owned();
    }

    let wsl2_snippet = get_wsl2_snippet(ip);
    hosts = format!("{}\n{}", &hosts, wsl2_snippet);
    fs::write("C:/Windows/System32/drivers/etc/hosts", hosts)
        .expect("Failed to write hosts file");
}

fn windows_only () {
    let ip = get_ip();
    write_ip_to_hosts(ip);
    println!("Done!");
}

fn non_windows () {
    println!("This is a windows only script.");
    std::process::exit(0)
}

fn main() {
    if cfg!(target_os = "windows") {
        windows_only();
    } else {
        non_windows();
    };
}
