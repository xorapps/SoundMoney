use core::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Currency {
    identifier: String,
    fractional: u32,
    integral: u32,
    separator: String,
}

impl Currency {
    pub fn new() -> Self {
        Currency {
            identifier: "SOL".to_owned(),
            fractional: u32::default(),
            integral: u32::default(),
            separator: ".".to_owned(),
        }
    }

    pub fn add_identifier(&mut self, identifier: &str) -> &mut Self {
        self.identifier = identifier.to_owned();

        self
    }

    pub fn add_fractional(&mut self, fractional: u32) -> &mut Self {
        self.fractional = fractional;

        self
    }

    pub fn add_integral(&mut self, integral: u32) -> &mut Self {
        self.integral = integral;

        self
    }

    pub fn add_separator(&mut self, separator: &str) -> &mut Self {
        self.separator = separator.to_owned();

        self
    }

    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    pub fn fractional(&self) -> u32 {
        self.fractional
    }

    pub fn integral(&self) -> u32 {
        self.integral
    }

    pub fn separator(&self) -> &str {
        self.separator.as_str()
    }

    pub fn to_string(&self) -> String {
        let mut stringified = String::new();

        stringified.push_str(self.identifier.as_str());
        stringified.push(' ');
        stringified.push_str(self.integral.to_string().as_str());
        stringified.push_str(self.separator.as_str());
        stringified.push_str(self.fractional.to_string().as_str());

        stringified
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.identifier.as_str())?;
        f.write_str(" ")?;
        f.write_str(self.integral.to_string().as_str())?;
        f.write_str(self.separator.as_str())?;
        f.write_str(self.fractional.to_string().as_str())
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::new()
    }
}
