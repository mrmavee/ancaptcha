//! Identifier obfuscation and mapping logic.

use std::collections::HashMap;

/// Bidirectional mapper for semantic-to-obfuscated name conversion.
pub struct NameMapper {
    mapping: HashMap<String, String>,
    reverse_mapping: HashMap<String, String>,
    counter: usize,
    seed: u64,
}

impl NameMapper {
    /// Creates a new mapper initialized with the provided seed.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            mapping: HashMap::new(),
            reverse_mapping: HashMap::new(),
            counter: 0,
            seed,
        }
    }

    /// Returns the obfuscated equivalent of a semantic name, creating it if necessary.
    pub fn get_or_create(&mut self, semantic_name: &str) -> String {
        if let Some(obfuscated) = self.mapping.get(semantic_name) {
            return obfuscated.clone();
        }

        let obfuscated = self.generate_short_name();
        self.mapping
            .insert(semantic_name.to_string(), obfuscated.clone());
        self.reverse_mapping
            .insert(obfuscated.clone(), semantic_name.to_string());
        obfuscated
    }

    /// Attempts to retrieve the original semantic name from an obfuscated string.
    #[must_use]
    pub fn get_semantic(&self, obfuscated_name: &str) -> Option<String> {
        self.reverse_mapping.get(obfuscated_name).cloned()
    }

    fn generate_short_name(&mut self) -> String {
        let mut n = (self.seed.wrapping_add(self.counter as u64)) % 17576;
        let mut suffix = String::new();

        loop {
            let rem = n % 26;
            suffix.push((b'a' + rem as u8) as char);
            n /= 26;
            if n == 0 {
                break;
            }
            n -= 1;
        }

        self.counter += 1;
        let rev_suffix: String = suffix.chars().rev().collect();
        format!("anC_{rev_suffix}")
    }

    /// Pre-populates the mapper with identifiers required for specific captcha types.
    pub fn warm_up(&mut self, steps: u8, captcha_type: &str) {
        let core = [
            "container",
            "checkbox",
            "completed",
            "label",
            "error",
            "h",
            "token",
        ];
        for s in core {
            let _ = self.get_or_create(s);
        }

        match captcha_type {
            crate::common::CAPTCHA_TYPE_ROTATE => {
                let _ = self.get_or_create("modal");
                let _ = self.get_or_create("btn");
                let _ = self.get_or_create("stage");
                let _ = self.get_or_create("ctrl");
                let _ = self.get_or_create("ir-w");
                let _ = self.get_or_create("st");
                for i in 0..steps {
                    let _ = self.get_or_create(&format!("st{i}"));
                    let _ = self.get_or_create(&format!("st_v{i}"));
                }
                for i in 0..steps {
                    let _ = self.get_or_create(&format!("s{i}"));
                    for j in 0..8 {
                        let _ = self.get_or_create(&format!("r{i}_{j}"));
                        let _ = self.get_or_create(&format!("v{j}"));
                    }
                }
                for i in 0..steps {
                    let _ = self.get_or_create(&format!("ir{i}"));
                    let _ = self.get_or_create(&format!("img{i}"));
                }
                let _ = self.get_or_create("rc");
                for i in 0..steps {
                    for j in 0..8 {
                        let _ = self.get_or_create(&format!("l{i}_{j}"));
                    }
                }
                let _ = self.get_or_create("nc");
            }
            crate::common::CAPTCHA_TYPE_SLIDER => {
                let _ = self.get_or_create("modal");
                let _ = self.get_or_create("btn");
                let _ = self.get_or_create("stage");
                let _ = self.get_or_create("track");
                let _ = self.get_or_create("st");
                for i in 0..steps {
                    let _ = self.get_or_create(&format!("st{i}"));
                    let _ = self.get_or_create(&format!("st_v{i}"));
                }
                for i in 0..steps {
                    let _ = self.get_or_create(&format!("s{i}"));
                    for j in 0..20 {
                        let _ = self.get_or_create(&format!("s{i}_{j}"));
                        let _ = self.get_or_create(&format!("p{j}"));
                    }
                }
                let _ = self.get_or_create("tg");
                let _ = self.get_or_create("nc");
            }
            crate::common::CAPTCHA_TYPE_PAIR => {
                let _ = self.get_or_create("modal");
                let _ = self.get_or_create("grid");
                let _ = self.get_or_create("btn");
                let _ = self.get_or_create("stage");
                let _ = self.get_or_create("pst");
                for i in 0..steps {
                    let _ = self.get_or_create(&format!("pst{i}"));
                    let _ = self.get_or_create(&format!("pst_v{i}"));
                }
                for i in 0..steps {
                    for j in 0..9 {
                        let _ = self.get_or_create(&format!("g{i}_{j}"));
                        let _ = self.get_or_create(&format!("s{i}_{j}"));
                        let _ = self.get_or_create(&format!("c{j}"));
                    }
                }
            }
            _ => {}
        }
    }
}

impl Default for NameMapper {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistency() {
        let mut mapper = NameMapper::new(12345);
        let name1 = mapper.get_or_create("container");
        let name2 = mapper.get_or_create("container");
        assert_eq!(name1, name2);
    }

    #[test]
    fn uniqueness() {
        let mut mapper = NameMapper::new(54321);
        let name1 = mapper.get_or_create("container");
        let name2 = mapper.get_or_create("button");
        assert_ne!(name1, name2);
    }

    #[test]
    fn prefix_check() {
        let mut mapper = NameMapper::new(42);
        for i in 0..20 {
            let name = mapper.get_or_create(&format!("test_{i}"));
            assert!(name.starts_with("anC_"));
        }
    }

    #[test]
    fn seeded_determinism() {
        let mut mapper1 = NameMapper::new(1337);
        let mut mapper2 = NameMapper::new(1337);
        let mut mapper3 = NameMapper::new(7331);

        let name1 = mapper1.get_or_create("test");
        let name2 = mapper2.get_or_create("test");
        let name3 = mapper3.get_or_create("test");

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }
}
