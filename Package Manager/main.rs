use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

struct PackageManager {
    registry: HashMap<String, HashMap<String, String>>,
    installed_packages: HashSet<String>,
}

impl PackageManager {
    fn new() -> Self {
        Self {
            registry: HashMap::new(),
            installed_packages: HashSet::new(),
        }
    }

    fn install_package(&mut self, package_name: &str, version: &str) -> io::Result<()> {
        if self.installed_packages.contains(package_name) {
            println!("Package {} is already installed.", package_name);
            return Ok(());
        }

        let package_versions = self.registry.entry(package_name.to_string()).or_default();

        if !package_versions.contains_key(version) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Package {}@{} not found in the registry.", package_name, version),
            ));
        }

        let package_content = self.download_package(package_name, version)?;
        self.install_package_files(package_name, version, &package_content)?;

        self.installed_packages.insert(package_name.to_string());

        println!("Package {}@{} installed successfully.", package_name, version);

        Ok(())
    }

    fn download_package(&self, package_name: &str, version: &str) -> io::Result<String> {
        let cdn_url = format!("https://cdn.kars.bio/{}/{}", package_name, version);
        let package_content = reqwest::blocking::get(&cdn_url)?.text()?;

        Ok(package_content)
    }

    fn install_package_files(
        &self,
        package_name: &str,
        version: &str,
        package_content: &str,
    ) -> io::Result<()> {
        let install_path = Path::new("packages").join(format!("{}@{}", package_name, version));
        fs::create_dir_all(&install_path)?;

        let package_json_path = install_path.join("package.json");
        let mut file = File::create(&package_json_path)?;
        file.write_all(package_content.as_bytes())?;

        println!("Package files installed in: {}", install_path.display());

        Ok(())
    }
}

fn main() {
    let mut package_manager = PackageManager::new();

    package_manager.registry.insert(
        "example-package".to_string(),
        hashmap! {
            "1.0.0".to_string() => "package content for version 1.0.0".to_string()
        },
    );
    package_manager.registry.insert(
        "another-package".to_string(),
        hashmap! {
            "2.0.0".to_string() => "package content for version 2.0.0".to_string(),
            "2.1.0".to_string() => "package content for version 2.1.0".to_string(),
        },
    );

    if let Err(err) = package_manager.install_package("example-package", "1.0.0") {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = package_manager.install_package("another-package", "2.1.0") {
        eprintln!("Error: {}", err);
    }
}

#[macro_export]
macro_rules! hashmap {
    ($( $key:expr => $val:expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
