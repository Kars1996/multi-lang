use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Package {
    name: String,
    version: String,
    dependencies: HashMap<String, String>,
}

impl Package {
    fn new(name: &str, version: &str) -> Self {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            dependencies: HashMap::new(),
        }
    }

    fn save(&self) {
        let path = format!("{}-{}.json", self.name, self.version);
        let serialized = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, serialized).expect("Failed to write package file");
    }

    fn load(name: &str, version: &str) -> Option<Self> {
        let path = format!("{}-{}.json", name, version);
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(package) = serde_json::from_str(&contents) {
                return Some(package);
            }
        }
        None
    }
}

fn install_package(name: &str, version: &str, cdn_url: &str) {
    if let Some(package) = Package::load(name, version) {
        println!("Installing {}@{}...", name, version);
        simulate_installation(&package, cdn_url);
        println!("Package installed successfully!");
    } else {
        println!("Error: Package {}@{} not found.", name, version);
    }
}

fn simulate_installation(package: &Package, cdn_url: &str) {
    for (dep_name, dep_version) in &package.dependencies {
        let dep_url = format!("{}/{}/{}.json", cdn_url, dep_name, dep_version);

        let dep_data = r#"
            {
                "name": "example-dependency",
                "version": "1.0.0"
            }
        "#;

        let dep: Package = serde_json::from_str(dep_data).expect("Failed to parse dependency data");

        dep.save();
    }

    if let Some(script) = package.dependencies.get("scripts") {
        run_script(script);
    }
}

fn run_script(script: &str) {
    println!("Running script: {}", script);
    Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("Failed to run script");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 || args[1] != "install" || args[3].is_empty() {
        println!("Usage: kpm install <package-name>*@<version>");
        std::process::exit(1);
    }

    let package_info: Vec<&str> = args[2].split('*').collect();

    if package_info.len() != 2 || package_info[0].is_empty() || package_info[1].is_empty() {
        println!("Invalid package name or version.");
        std::process::exit(1);
    }

    let package_name = package_info[0];
    let package_version = package_info[1];

    let cdn_url = "https://cdn.kars.bio";

    println!("Fetching package information from CDN...");
    let package_url = format!("{}/{}/{}.json", cdn_url, package_name, package_version);

    let package_data = r#"
        {
            "name": "example-package",
            "version": "1.0.0",
            "dependencies": {
                "dependency-a": "1.2.3",
                "dependency-b": "2.0.0",
                "scripts": "echo 'Hello from package script'"
            }
        }
    "#;

    let parsed_package: Package =
        serde_json::from_str(package_data).expect("Failed to parse package data");

    println!("Package information fetched successfully!");

    parsed_package.save();

    install_package(package_name, package_version, cdn_url);
}
