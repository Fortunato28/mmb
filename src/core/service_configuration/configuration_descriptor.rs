use std::hash::Hash;

/// Entity needed to describe a configuration of trading strategy, which helps to determine which strategy the balance change refers.
#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct ConfigurationDescriptor {
    /// Trading strategy name
    pub service_name: String,
    pub service_configuration_key: String,
}

impl ConfigurationDescriptor {
    pub fn new(service_name: String, service_configuration_key: String) -> Self {
        Self {
            service_name,
            service_configuration_key,
        }
    }
}
