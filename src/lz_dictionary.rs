pub const EMPTY_PATTERN : u16 = 4095;

pub struct LzDictionary {
    dictionary: [(u16, u16); 4096],
    dictionary_idx: usize
}

impl LzDictionary {
    pub fn new() -> LzDictionary {
		let mut dict =[(0,0); 4096];
		for idx in 0 .. 256 {
			dict[idx] = (EMPTY_PATTERN, idx as u16);
		}
		LzDictionary { dictionary: dict, dictionary_idx: 256 }
    }
    
    pub fn is_pattern_in_dictionary(&self, pattern_to_find : (u16,u16)) -> Option<u16> {
        for idx in 0 .. self.dictionary_idx {
            if pattern_to_find == self.dictionary[idx] {
                return Some(idx as u16);
            }
        }
		None
    }

    pub fn add_pattern_to_dictionary(&mut self, item_to_add : (u16,u16))  {
        self.dictionary[self.dictionary_idx] = item_to_add;
        self.dictionary_idx += 1;
    }

    pub fn get_size(&self) -> usize {
        self.dictionary_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_dictionary() {
        let dict_to_test = LzDictionary::new();

        assert_eq!(256, dict_to_test.get_size());
		for idx in 0 .. 256 {
	        assert_eq!(Some(idx), dict_to_test.is_pattern_in_dictionary((EMPTY_PATTERN, idx as u16)));
		}
    }
	
    #[test]
    fn test_dictionary() {
        let mut dict_to_test = LzDictionary::new();
        dict_to_test.add_pattern_to_dictionary((123,4321));

        assert_eq!(Some(256), dict_to_test.is_pattern_in_dictionary((123,4321)));
        assert_eq!(None, dict_to_test.is_pattern_in_dictionary((321,4321)));
        assert_eq!(257, dict_to_test.get_size());        
    }
}