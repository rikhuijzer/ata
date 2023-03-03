use std::convert::Infallible;
use std::ffi::OsString;
use std::fmt::{self, Display};

use std::path::{Path, PathBuf};
use std::str::FromStr;

use bevy_reflect::{Reflect, Struct as _};
use bevy_utils::HashMap;
use directories::ProjectDirs;
use os_str_bytes::OsStrBytes as _;
use os_str_bytes::OsStringBytes as _;
use serde::{Deserialize, Serialize};
use toml::de::Error as TomlError;

lazy_static! {
    static ref DEFAULT_CONFIG_FILENAME: PathBuf = "ata.toml".into();
}

// For definitions, see https://platform.openai.com/docs/api-reference/completions/create
#[repr(C)]
#[derive(Clone, Deserialize, Debug, Serialize, Reflect)]
#[serde(default)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub max_tokens: i64,
    pub temperature: f64,
    pub suffix: Option<String>,
    pub top_p: f64,
    pub n: u64,
    pub stream: bool,
    pub logprobs: u8,
    pub echo: bool,
    pub stop: Vec<String>,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
    pub best_of: u64,
    pub logit_bias: HashMap<String, f64>,
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err(String::from("API key is missing"));
        }

        if self.model.is_empty() {
            return Err(String::from("Model ID is missing"));
        }

        if self.max_tokens < 1 || self.max_tokens > 2048 {
            return Err(String::from("Max tokens must be between 1 and 2048"));
        }

        if self.temperature < 0.0 || self.temperature > 1.0 {
            return Err(String::from("Temperature must be between 0.0 and 1.0"));
        }

        if let Some(suffix) = &self.suffix {
            if suffix.is_empty() {
                return Err(String::from("Suffix cannot be an empty string"));
            }
        }

        if self.top_p < 0.0 || self.top_p > 1.0 {
            return Err(String::from("Top-p must be between 0.0 and 1.0"));
        }

        if self.n < 1 || self.n > 10 {
            return Err(String::from("n must be between 1 and 10"));
        }

        if self.logprobs > 2 {
            return Err(String::from("logprobs must be 0, 1, or 2"));
        }

        if self.stop.iter().any(|stop| stop.is_empty()) || self.stop.len() > 4 {
            return Err(String::from("Stop phrases cannot contain empties"));
        }

        if self.presence_penalty < 0.0 || self.presence_penalty > 1.0 {
            return Err(String::from("Presence penalty must be between 0.0 and 1.0"));
        }

        if self.frequency_penalty < 0.0 || self.frequency_penalty > 1.0 {
            return Err(String::from(
                "Frequency penalty must be between 0.0 and 1.0",
            ));
        }

        if self.best_of < 1 || self.best_of > 5 {
            return Err(String::from("best_of must be between 1 and 5"));
        }

        for (key, value) in &self.logit_bias {
            if value < &-2.0 || value > &2.0 {
                return Err(format!(
                    "logit_bias for {} must be between -2.0 and 2.0",
                    key
                ));
            }
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "text-davinci-003".into(),
            max_tokens: 16,
            temperature: 0.5,
            suffix: None,
            top_p: 1.0,
            n: 1,
            stream: false,
            logprobs: 0,
            echo: false,
            stop: vec![],
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            best_of: 1,
            logit_bias: HashMap::new(),
            api_key: String::default(),
        }
    }
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
        Ok(if !s.contains(".") && s.len() > 0 {
            Self::Named(s.into())
        } else if s.trim().len() > 0 {
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

fn get_config_dir() -> PathBuf {
    ProjectDirs::from(
        "ata",
        "Ask the Terminal Anything (ATA) Project Authors",
        "ata",
    )
    .unwrap()
    .config_dir()
    .into()
}

pub fn default_path(name: Option<&Path>) -> PathBuf {
    let mut config_file = get_config_dir().to_path_buf();
    let file: Vec<_> = if let Some(name) = name {
        let mut name = name.to_path_buf();
        name.set_extension("toml");
        name.as_os_str()
            .to_raw_bytes()
            .into_iter()
            .map(|i| *i)
            .collect()
    } else {
        let name = DEFAULT_CONFIG_FILENAME.to_string_lossy();
        name.bytes().collect()
    };
    let file = OsString::assert_from_raw_vec(file);
    config_file.push(&file);
    config_file
}

impl ConfigLocation {
    pub fn location(&self) -> PathBuf {
        match self {
            ConfigLocation::Auto => {
                let config_dir = get_config_dir().to_path_buf();
                if DEFAULT_CONFIG_FILENAME.exists() {
                    warn!(
                        "{} found in working directory BUT UNSPECIFIED. \
                          This behavior is DEPRECATED. \
                          Please move it to {}.",
                        DEFAULT_CONFIG_FILENAME.display(),
                        config_dir.display()
                    );
                    return DEFAULT_CONFIG_FILENAME.clone();
                }
                default_path(None)
            }
            ConfigLocation::Path(pb) => pb.clone(),
            ConfigLocation::Named(name) => default_path(Some(name)),
        }
    }
}

impl FromStr for Config {
    type Err = TomlError;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        toml::from_str(&contents)
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

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut ok = Ok(());
        for (i, value) in self.iter_fields().enumerate() {
            if !ok.is_ok() {
                break;
            }
            let value: &dyn Reflect = value;
            let key = self.name_at(i).unwrap();
            if key == "api_key" {
                continue
            }
            ok = writeln!(f, "{key}: {:#?}", value);
        }
        ok
    }
}
