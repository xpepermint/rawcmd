/// Structure which holds flag summary.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagSummary {
    name: String,
    alias: Option<String>,
    hint: Option<String>,
    value: Option<String>,
    default_value: Option<String>,
    accepts_value: bool,
    provided: bool,
}

/// Structure implementation.
impl FlagSummary {

    // Returns new instance.
    pub fn with_name(
        name: &str,
        alias: Option<String>,
        hint: Option<String>,
        value: Option<String>,
        default_value: Option<String>,
        accepts_value: bool,
        provided: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias,
            hint,
            value,
            default_value,
            accepts_value,
            provided,
        }
    }
}

/// Structure implementation.
impl FlagSummary {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns alias.
    pub fn alias(&self) -> &Option<String> {
        &self.alias
    }

    /// Returns hint.
    pub fn hint(&self) -> &Option<String> {
        &self.hint
    }

    /// Returns value.
    pub fn value(&self) -> &Option<String> {
        &self.value
    }

    /// Returns default value.
    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }

    /// Returns true if the flag has value.
    pub fn accepts_value(&self) -> bool {
        self.accepts_value
    }

    /// Returns true if the flag has value.
    pub fn provided(&self) -> bool {
        self.provided
    }
}
