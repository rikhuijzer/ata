use directories::ProjectDirs;
use os_str_bytes::OsStrBytes;
use os_str_bytes::OsStringBytes;
use serde::Deserialize;
use std::convert::Infallible;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use toml::de::Error as TomlError;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub max_tokens: i64,
    pub temperature: f64,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub enum ConfigLocation {
    #[default]
    Auto,
    Path(PathBuf),
    Named(PathBuf),
}

impl FromStr for ConfigLocation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if !s.contains('.') && !s.is_empty() {
            Self::Named(s.into())
        } else if !s.is_empty() {
            Self::Path(s.into())
        } else if s.trim().is_empty() {
            Self::Auto
        } else {
            unreachable!()
        })
    }
}

impl<S> From<S> for ConfigLocation
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

fn get_config_dir(old_org: bool) -> PathBuf {
    let organization = if old_org {
        "Ask the Terminal Anything (ATA) Project Authors"
    } else {
        "ask the terminal anything"
    };
    ProjectDirs::from(
        "ata",
        organization,
        "ata",
    )
    .unwrap()
    .config_dir()
    .into()
}

pub fn default_path(name: Option<&Path>, old_org: bool) -> PathBuf {
    let mut config_file = get_config_dir(old_org);
    let file: Vec<_> = if let Some(name) = name {
        let mut name = name.to_path_buf();
        name.set_extension("toml");
        name.as_os_str().to_raw_bytes().iter().copied().collect()
    } else {
        "ata.toml".bytes().collect()
    };
    let file = OsString::assert_from_raw_vec(file);
    config_file.push(&file);
    config_file
}

impl ConfigLocation {
    pub fn location(&self, old_org: bool) -> PathBuf {
        match self {
            ConfigLocation::Auto => {
                let old_org = true;
                if self.location(old_org).exists() {
                    return self.location(old_org);
                }

                let old_org = false;
                if self.location(old_org).exists() {
                    return self.location(old_org);
                }
                default_path(None, old_org)
            }
            ConfigLocation::Path(pb) => pb.clone(),
            ConfigLocation::Named(name) => {
                if name.as_os_str() == "default" {
                    return match Path::new("ata.toml").exists() {
                        true => Path::new(&"ata.toml").into(),
                        false => default_path(None, old_org),
                    };
                }
                default_path(Some(name), old_org)
            }
        }
    }
}

impl FromStr for Config {
    type Err = TomlError;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        toml::from_str(contents)
    }
}

impl<S> From<S> for Config
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        Self::from_str(s.as_ref()).unwrap_or_else(|e| panic!("Config parsing failure!: {:?}", e))
    }
}
