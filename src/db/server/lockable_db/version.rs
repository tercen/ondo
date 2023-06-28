// version.rs
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub commit: String,
    pub date: String,
    pub features: String,
}

impl Version {
    pub fn new() -> Self {
        let ver = match semver::Version::parse(option_env!("VERSION").unwrap_or("0.0.0")) {
            Ok(ver) => ver,
            Err(_) => semver::Version::parse("0.0.0").unwrap(),
        };

        Version {
            major: ver.major,
            minor: ver.minor,
            patch: ver.patch,
            commit: option_env!("COMMIT_NUMBER")
                .map(|env| env.to_string())
                .unwrap_or("".to_string()),
            date: option_env!("BUILD_DATE")
                .map(|env| env.to_string())
                .unwrap_or("".to_string()),
            features: "".to_string(),
        }
    }
}
