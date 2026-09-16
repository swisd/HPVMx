use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub type OSVersion = u32;

pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    unstable: bool,
    pre: Option<EarlyVersion>,
    metadata: Option<String>,

}

pub struct EarlyVersion {
    ver_type: String,
    major: Option<u32>,
    minor: Option<u32>,
    patch: Option<u32>,
}

pub struct BuildName {
    build_number: String,
    abbreviated_name: String,
}

pub struct VersionGroup {
    version: Version,
    build_name: BuildName,
}

impl EarlyVersion {
    pub fn new(ver_typ: &str, major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Self {
        Self {
            ver_type: ver_typ.to_string(),
            major,
            minor,
            patch,
        }
    }

    pub fn from_string(s: String) -> Self {
        // parse type, major,minor, patch from string, i.e.  alpha.1.2.0 or alpha-1.2.0
        let normalized = s.replace('-', ".");
        let partial: Vec<&str> = normalized.split(".").collect();

        let major = partial[1].parse().unwrap_or("".to_string());
        let minor = partial[2].parse().unwrap_or("".to_string());
        let patch = partial[3].parse().unwrap_or("".to_string());

        EarlyVersion::new(
            partial[0],
            if major != "" { Some(u32::from_string(major)) } else { None } ,
            if minor != "" { Some(u32::from_string(minor)) } else { None } ,
            if patch != "" { Some(u32::from_string(patch)) } else { None } ,
        )

    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32, unstable: bool, pre: Option<EarlyVersion>, metadata: Option<String>) -> Self {
        Version {
            major,
            minor,
            patch,
            unstable,
            pre,
            metadata,
        }
    }

    pub fn from_string(s: String) -> Self {
        let normalized = s.replace('-', ".");
        let parts: Vec<&str> = normalized.split('.').collect();

        let major = parts.get(0).and_then(|v| v.parse().ok()).unwrap_or(0);
        let minor = parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        let patch = parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(0);

        let pre = if parts.len() > 3 {
            let pre_string = parts[3..].join(".");
            Some(EarlyVersion::from_string(pre_string))
        } else {
            None
        };

        Version::new(major, minor, patch, pre.is_some(), pre, None)
    }

    pub fn from_env() -> Self {
        let version_string = String::from(env!("CARGO_PKG_VERSION"));
        Version::from_string(version_string)
    }

    pub fn format_to_string(&self) -> String {
        let mut version_str = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre) = &self.pre {
            version_str.push_str(&format!("-{}", pre.ver_type));
            if let Some(major) = pre.major {
                version_str.push_str(&format!(".{}", major));
            }
            if let Some(minor) = pre.minor {
                version_str.push_str(&format!(".{}", minor));
            }
            if let Some(patch) = pre.patch {
                version_str.push_str(&format!(".{}", patch));
            }
        }
        if let Some(metadata) = &self.metadata {
            version_str.push_str(&format!("+{}", metadata));
        }
        version_str
    }
}

impl BuildName {
    pub fn new(build_number: String, abbreviated_name: String) -> Self {
        Self {
            build_number,
            abbreviated_name,
        }
    }
    pub fn from_env() -> Self {
        let build_number = String::from(env!("BUILD_NUMBER"));
        let abbreviated_name = String::from(""); // Placeholder for abbreviated name
        BuildName::new(build_number, abbreviated_name)
    }

    pub fn format_to_string(&self) -> String {
        format!("{} [{}]", self.build_number, self.abbreviated_name)
    }
}

impl VersionGroup {
    pub fn new(version: Version, build_name: BuildName) -> Self {
        Self {
            version,
            build_name,
        }
    }

    pub fn from_env() -> Self {
        let version = Version::from_env();
        let build_name = BuildName::from_env();
        VersionGroup::new(version, build_name)
    }

    pub fn format_to_string(&self) -> String {
        format!("{} build {}", self.version.format_to_string(), self.build_name.format_to_string())
    }
}

pub trait Stringable {
    fn as_string(&self) -> String;
    fn as_self(&self) -> Self;
    fn from_string(s: String) -> Self;
    fn from_char(c: char) -> Self;
}

macro_rules! impl_stringable {
    ($($t:ty)*) => ($(impl Stringable for $t {
        fn as_string(&self) -> String {
            self.to_string()
        }
        fn as_self(&self) -> Self {
            *self
        }
        fn from_string(s: String) -> Self {
            s.parse().unwrap()
        }
        fn from_char(c: char) -> Self {
            c.to_digit(10).unwrap() as Self
        }
    } )*);
}

impl_stringable! { u8 u16 u32 i8 i16 i32 }






/// Combine parts of a version into an [`OSVersion`].
///
/// The size of the parts are inherently limited by Mach-O's `LC_BUILD_VERSION`.
#[inline]
const fn pack_os_version(major: u16, minor: u8, patch: u8) -> OSVersion {
    let (major, minor, patch) = (major as u32, minor as u32, patch as u32);
    (major << 16) | (minor << 8) | patch
}

/// [`pack_os_version`], but takes `i32` and saturates.
///
/// Instead of using e.g. `major as u16`, which truncates.
#[inline]
fn pack_i32_os_version(major: i32, minor: i32, patch: i32) -> OSVersion {
    let major: u16 = major.try_into().unwrap_or(u16::MAX);
    let minor: u8 = minor.try_into().unwrap_or(u8::MAX);
    let patch: u8 = patch.try_into().unwrap_or(u8::MAX);
    pack_os_version(major, minor, patch)
}