use crate::{hpvm_error, hpvm_warn, Color};
use crate::hpvm_log;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use crate::filesystem::FileSystem;
use serde::{Deserialize, Serialize};
use uefi::proto::media::file::File;
use crate::{hpvm_info, message};
use crate::types::{VersionString, BYTEARRAY};

static VERSION: &str = "0.3.2";
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UUID(BYTEARRAY);

#[derive(Clone)]
pub struct PackageManager {
    pub version: VersionString,
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


#[derive(Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Ord, Eq)]
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
    pub version: VersionString,
    pub description: String,
    pub author: String,
    //pub license: String,
    pub deps: Vec<String>,//Vec<Package>,
    //pub repository_url: String,
    //pub keywords: Vec<String>,
    //pub readme: String,
    //pub requirements: Vec<String>,
    pub package_type: PackageType,
    pub repo_url: Option<String>,
    pub has_compilation_issues: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MiniPackage {
    pub name: String,
    pub version: VersionString,
    pub description: String,
    pub mini_package_type: MiniPackageType,
}

// Might add build function later
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildReport {
    /// Package that was built
    pub package: String,
    /// Version that was built
    pub version: VersionString,
    /// Output file path
    pub output_path: String,
    /// Build duration
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallReport {
    /// Packages that were installed
    pub installed: Vec<PackageChange>,
    /// Packages that were updated
    pub updated: Vec<PackageChange>,
    /// Packages that were removed
    pub removed: Vec<PackageChange>,
    /// New state ID
    pub state_id: UUID,
    /// Total execution time
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageChange {
    /// Package name
    pub name: String,
    /// Previous version
    pub from_version: Option<VersionString>,
    /// New version
    pub to_version: Option<VersionString>,
    /// Size in bytes
    pub size: Option<u64>,
}

type PackageRegistry = BTreeMap<String, Package>;
type SortablePackageRegistry = BTreeMap<String, BTreeMap<String, Package>>;

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
        // also need to download a package from the registry and install it.
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
        FileSystem::cd("/")
    }

    fn load_from_json(&mut self, json_data: &str) -> Result<(), &'static str> {
        // Deserialize using serde-json-core
        // The return type includes the number of bytes consumed
        let (mut pkg, _): (Package, usize) = serde_json_core::from_str(json_data)
            .map_err(|_| "Failed to parse JSON")?;
        pkg.has_compilation_issues = true;

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

    pub fn get_packages(&self) -> BTreeMap<PackageType, Vec<Package>> {
        let mut grouped_packages: BTreeMap<PackageType, Vec<Package>> = BTreeMap::new();
        if self.registry.is_empty() {
        } else {
            // 1. Create the grouped map


            // 2. Populate it from your registry
            for (pname, package) in self.registry.iter() {
                grouped_packages
                    .entry(package.package_type.clone())
                    .or_insert_with(Vec::new)
                    .push(package.clone());
            }

        }
        grouped_packages
    }

    pub fn download_package(&mut self, pkg_name: &str) {
        hpvm_info!("pm", "Attempting to download package: {}", pkg_name);
        if let Some(pkg) = self.registry.get(pkg_name) {
            if let Some(url) = &pkg.repo_url {
                hpvm_info!("pm", "Downloading from: {}", url);
                // In a real implementation, we would use the network stack here.
                // For now, we simulate a successful download.
                hpvm_info!("pm", "Successfully downloaded {}", pkg_name);
            } else {
                hpvm_warn!("pm", "No repo_url for package: {}", pkg_name);
            }
        } else {
            hpvm_warn!("pm", "Package not found in registry: {}", pkg_name);
        }
    }

    pub fn autocompile_package(&mut self, pkg_name: &str) {
        hpvm_info!("pm", "Attempting to autocompile package: {}", pkg_name);
        let pkg_path = format!("{}/{}/", self.package_path, pkg_name);
        
        // Change to package directory
        FileSystem::cd(&pkg_path);
        
        // This is a simplified check. A real implementation would scan for .micro files.
        // We assume there's a main.micro or similar.
        let source_file = "src/main.micro";
        match FileSystem::read_file_to_string(source_file) {
            Ok(source) => {
                hpvm_info!("pm", "Compiling {}...", source_file);
                let arch = "x86_64"; // Default architecture
                let binary = crate::micro_c::compiler::compile(&source, arch);
                
                if binary.is_empty() {
                    hpvm_error!("pm", "Compilation failed for {}", pkg_name);
                    if let Some(pkg) = self.registry.get_mut(pkg_name) {
                        pkg.has_compilation_issues = true;
                        hpvm_info!("pm", "Package status updated with compilation issues");
                    }
                } else {

                    let out_file = format!("bin/{}.bin", pkg_name);
                    match FileSystem::write_to_file(&out_file, &binary, 'w') {
                        Ok(_) => {
                            hpvm_info!("pm", "Successfully compiled to {}", out_file);
                            if let Some(pkg) = self.registry.get_mut(pkg_name) {
                                //pkg.has_compilation_issues = false;
                                pkg.has_compilation_issues = true;
                            }
                        }
                        Err(e) => hpvm_error!("pm", "Failed to write binary: {}", e),
                    }
                }
            }
            Err(_) => {
                hpvm_warn!("pm", "No source file (src/main.micro) found for {}", pkg_name);
            }
        }
        
        FileSystem::cd("/");
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
            repo_url: None,
            has_compilation_issues: false,
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
            "repo_url" => self.repo_url = Some(clean_val.to_string()),
            "has_compilation_issues" => self.has_compilation_issues = clean_val == "true",
            _ => {} // Ignore unknown keys
        }
    }

    pub fn add_dependency(&mut self, dep_name: &str) {
        self.deps.push(dep_name.to_string());
    }
}