#[derive(Clone, Debug)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub release: i32,
}

impl std::default::Default for Version {
    fn default() -> Self {
        Version {
            major: 7,
            minor: 22,
            release: 0,
        }
    }
}
