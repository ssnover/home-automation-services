#[derive(serde::Deserialize)]
pub struct Settings {
    pub aurora: AuroraSettings,
    pub ros: RosSettings,
}

#[derive(serde::Deserialize)]
pub struct AuroraSettings {
    pub token: String,
    pub address: String,
}

#[derive(serde::Deserialize)]
pub struct RosSettings {
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}
