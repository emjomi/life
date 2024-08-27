use std::collections::HashSet;

#[derive(Debug)]
pub struct Rule {
    birth: HashSet<u8>,
    survival: HashSet<u8>
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
}

impl TryFrom<&str> for Rule {
    type Error = ParseError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('/').collect();
        
        if parts.len() != 2 || !parts[0].starts_with('B') || !parts[1].starts_with('S') {
            return Err(ParseError::InvalidFormat);
        }

        let birth = parts[0].chars().skip(1).map(|c| {
                    c.to_digit(10)
                        .ok_or(ParseError::InvalidNumber)
                        .map(|n| n as u8)
                }).collect::<Result<HashSet<u8>, Self::Error>>()?;
        let survival = parts[1].chars().skip(1).map(|c| {
                    c.to_digit(10)
                        .ok_or(ParseError::InvalidNumber)
                        .map(|n| n as u8)
                }).collect::<Result<HashSet<u8>, Self::Error>>()?;
        
        Ok(Rule {birth, survival})
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            birth: [3].into_iter().collect(),
            survival: [2, 3].into_iter().collect()
        }
    }
}

impl Rule {
    pub fn new(birth: HashSet<u8>, survival: HashSet<u8>) -> Self {
        Self {
            birth,
            survival
        }
    }
    
    pub fn is_born(&self, neighbors: u8) -> bool {
        self.birth.contains(&neighbors)
    }
    
    pub fn is_survivor(&self, neighbors: u8) -> bool {
        self.survival.contains(&neighbors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_rule() {
        let rule = Rule::try_from("B3/S23").unwrap();
        
        assert_eq!(rule.birth, [3].into_iter().collect());
        assert_eq!(rule.survival, [2, 3].into_iter().collect());
    }
    
    #[test]
    fn empty_birth_and_survival() {
        let rule = Rule::try_from("B/S").unwrap();

        assert_eq!(rule.birth, HashSet::new());
        assert_eq!(rule.survival, HashSet::new());
    }

    #[test]
    fn invalid_format_no_slash() {
        let result = Rule::try_from("B3S23");
        
        assert!(matches!(result, Err(ParseError::InvalidFormat)));
    }

    #[test]
    fn invalid_format_wrong_prefix() {
        let result = Rule::try_from("A3/S23");
        
        assert!(matches!(result, Err(ParseError::InvalidFormat)));
    }

    #[test]
    fn invalid_number_in_birth() {
        let result = Rule::try_from("B3x/S23");
        
        assert!(matches!(result, Err(ParseError::InvalidNumber)));
    }

    #[test]
    fn invalid_number_in_survival() {
        let result = Rule::try_from("B3/Sx3");
        
        assert!(matches!(result, Err(ParseError::InvalidNumber)));
    }
}