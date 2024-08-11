use std::collections::HashSet;

#[derive(Debug)]
pub struct Rule {
    birth: HashSet<u8>,
    survival: HashSet<u8>
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