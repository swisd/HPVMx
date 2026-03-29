use crate::{hpvm_error, hpvm_warn, Color};
use crate::hpvm_log;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use crate::filesystem::FileSystem;
use serde::{Deserialize, Serialize};
use uefi::proto::media::file::File;
use crate::{hpvm_info, message};

static VERSION: &str = "0.1.0";

#[derive(Clone)]
pub struct PackageManager {
    pub version: String,
    pub package_path: String,
    pub registry: BTreeMap<String, Package>,
    pub buffer: Vec<u8>,
    pub state: StateManager,
    pub config: Config,
}

#[derive(Clone)]
pub struct StateManager {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub general: GeneralConfig,
    pub paths: PathConfig,
    pub verification: VerificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub color: u32,
    pub parallel_downloads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    pub store_path: Option<String>,
    pub state_path: Option<String>,
    pub build_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub enabled: bool,
    pub level: String, // "quick", "standard", or "full"
    pub discrepancy_handling: DiscrepancyHandling,
    pub user_file_policy: UserFilePolicy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DiscrepancyHandling {
    /// Fail the operation when discrepancies are found
    FailFast,
    /// Report discrepancies but continue operation
    ReportOnly,
    /// Automatically heal discrepancies when possible
    AutoHeal,
    /// Auto-heal but fail if healing is not possible
    AutoHealOrFail,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserFilePolicy {
    /// Preserve user-created files
    Preserve,
    /// Remove user-created files
    Remove,
    /// Backup user-created files before removal
    Backup,
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum PackageType {
    Library,
    Executable,
    Extension,
    ResourcePack,
    Driver,
    PShader,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MiniPackageType {
    Library,
    Executable,
    Driver,
    PShader,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    //pub license: String,
    pub deps: Vec<String>,//Vec<Package>,
    //pub repository_url: String,
    //pub keywords: Vec<String>,
    //pub readme: String,
    //pub requirements: Vec<String>,
    pub package_type: PackageType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MiniPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub mini_package_type: MiniPackageType,
}

impl PackageManager {
    pub fn new() -> Self {
        PackageManager {
            version: VERSION.to_string(),
            package_path: "/pm/package/".to_string(),
            registry: BTreeMap::new(),
            buffer: Vec::new(),
            state: StateManager {},
            config: Config {
                general: GeneralConfig {
                    color: 0xFFFFFF,
                    parallel_downloads: 0
                },
                paths: PathConfig {
                    store_path: None,
                    state_path: None,
                    build_path: None,
                },
                verification: VerificationConfig {
                    enabled: false,
                    level: "".to_string(),
                    discrepancy_handling: DiscrepancyHandling::FailFast,
                    user_file_policy: UserFilePolicy::Preserve,
                }
            },
        }
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    pub fn clone(&self) -> Self {
        PackageManager {
            version: self.version.clone(),
            package_path: self.package_path.clone(),
            registry: self.registry.clone(),
            buffer: self.buffer.clone(),
            state: self.state.clone(),
            config: self.config.clone(),
        }
    }
    // Adds a package to the internal registry
    pub fn install(&mut self, pkg: Package) {
        self.registry.insert(pkg.name.clone(), pkg);
    }

    // Checks if all dependencies for a given package are present
    pub fn verify_dependencies(&self, pkg_name: &str) -> () {
        let mut missing = Vec::new();

        if let Some(pkg) = self.registry.get(pkg_name) {
            for dep in &pkg.deps {
                if !self.registry.contains_key(dep) {
                    missing.push(dep.clone());
                }
            }
        } else {
            hpvm_warn!("pm", "Package not found: {}", pkg_name);
        }

        if missing.is_empty() {
            hpvm_info!("pm", "no missing packages for: {}", pkg_name);
        } else {
            hpvm_warn!("pm", "Packages not found: {:?}", missing);
        }
    }

    pub fn load_registry(&mut self) {
        FileSystem::cd(&*self.package_path);
        FileSystem::get_cwd();
        hpvm_info!("pm", "loading packages from {}registry.prg", self.package_path);
        let reg = FileSystem::read_file_to_string("registry.prg").expect("could not open registry");
        let pkgs = reg.replace("\r", "");
        let packages = pkgs.split("\n").collect::<Vec<&str>>();
        for package in packages {
            hpvm_info!("pm", "found package '{}'", package.to_string());
            let pack_path = self.package_path.clone() + "/" + package + "/";
            FileSystem::cd(&*pack_path);
            let json = FileSystem::read_file_to_string("package.json").unwrap();
            let clean_json = json.trim_matches(char::from(0)).trim();
            self.load_from_json_no_alloc(clean_json);
        }
    }

    fn load_from_json(&mut self, json_data: &str) -> Result<(), &'static str> {
        // Deserialize using serde-json-core
        // The return type includes the number of bytes consumed
        let (pkg, _): (Package, usize) = serde_json_core::from_str(json_data)
            .map_err(|_| "Failed to parse JSON")?;

        self.registry.insert(pkg.name.clone(), pkg);
        Ok(())
    }

    pub fn load_from_json_no_alloc(&mut self, json: &str) -> () {
        let mut pkg = Package::new();

        let mut current_key = String::new();
        let mut buffer = String::new();

        let mut in_quotes = false;
        let mut in_deps_array = false;

        let chars: Vec<char> = json.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    if !in_quotes {
                        // Just closed a quote. Check if this was a KEY or a VALUE.
                        // We look ahead to see if the next non-whitespace char is a colon.
                        let mut lookahead = i + 1;
                        while lookahead < chars.len() && chars[lookahead].is_whitespace() {
                            lookahead += 1;
                        }

                        if !in_deps_array && chars.get(lookahead) == Some(&':') {
                            // It's a KEY
                            current_key = buffer.trim().to_string();
                            i = lookahead; // Skip to the colon
                        } else if in_deps_array {
                            // It's an entry in the ["dep1", "dep2"] list
                            let val = buffer.trim().to_string();
                            if !val.is_empty() {
                                pkg.deps.push(val);
                            }
                        } else {
                            // It's a normal VALUE
                            let val = buffer.trim().to_string();
                            pkg.assign_value(&current_key, &val);
                        }
                        buffer.clear();
                    }
                }
                '[' => {
                    if !in_quotes { in_deps_array = true; }
                    else { buffer.push(c); }
                }
                ']' => {
                    if !in_quotes { in_deps_array = false; }
                    else { buffer.push(c); }
                }
                // Structural characters: ignore them if they are outside of quotes
                '{' | '}' | ':' | ',' | '\r' | '\n' | '\t' | ' ' => {
                    if in_quotes {
                        buffer.push(c);
                    }
                }
                _ => {
                    // Always buffer actual content
                    buffer.push(c);
                }
            }
            i += 1;
        }

        if pkg.name.is_empty() {
            return hpvm_warn!("pm", "Invalid Package: Package File Invalid");
        }
        self.registry.insert(pkg.name.clone(), pkg);
        hpvm_info!("pm", "Registry size now: {}", self.registry.len());
    }

    pub fn list_packages(&self) -> () {
        if self.registry.is_empty() {
            message!("", "no packages");
            return;
        } else {
            for (pname, package) in self.registry.iter() {
                message!("", "package: {}; type: {:?}; version: {:?}; author: {:?};",
                    pname, package.package_type, package.version, package.author);
            }
        }
    }

}

impl Package {
    pub fn new() -> Self {
        Package {
            name: "".to_string(),
            version: "".to_string(),
            description: "".to_string(),
            author: "".to_string(),
            //license: "".to_string(),
            deps: vec![],
            //repository_url: "".to_string(),
            //keywords: vec![],
            //readme: "".to_string(),
            //requirements: vec![],
            package_type: PackageType::Library,
        }
    }

    fn assign_value(&mut self, key: &str, value: &str) {
        // Clean the key one last time to be safe
        let clean_key = key.trim_matches(|c: char| c.is_whitespace() || c == '"');
        let clean_val = value.trim_matches(|c: char| c.is_whitespace() || c == '"');

        match clean_key {
            "name" => self.name = clean_val.to_string(),
            "version" => self.version = clean_val.to_string(),
            "description" => self.description = clean_val.to_string(),
            "author" => self.author = clean_val.to_string(),
            "package_type" => {
                self.package_type = match clean_val {
                    "Library" => PackageType::Library,
                    "Executable" => PackageType::Executable,
                    "PShader" => PackageType::PShader,
                    "ResourcePack" => PackageType::ResourcePack,
                    "Extension" => PackageType::Extension,
                    "Driver" => PackageType::Driver,
                    _ => PackageType::Library,
                };
            }
            _ => {} // Ignore unknown keys
        }
    }

    pub fn add_dependency(&mut self, dep_name: &str) {
        self.deps.push(dep_name.to_string());
    }
}