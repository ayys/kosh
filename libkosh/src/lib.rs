use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the grammatical category of a word or definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrammarCategory {
    /// संज्ञा (Noun)
    Noun,
    /// क्रिया (Verb)
    Verb,
    /// विशेषण (Adjective)
    Adjective,
    /// क्रियाविशेषण (Adverb)
    Adverb,
    /// सर्वनाम (Pronoun)
    Pronoun,
    /// निपात (Particle)
    Particle,
    /// पूर्वसर्ग (Prefix)
    Prefix,
    /// विस्मयादिबोधक (Interjection/Exclamation)
    Interjection,
    /// अन्य (Other/Unknown)
    Other(String),
}

impl GrammarCategory {
    /// Parse grammar category from the abbreviated form used in the dictionary
    pub fn from_abbreviation(abbr: &str) -> Self {
        match abbr.trim() {
            "ना." => GrammarCategory::Noun,
            "क्रि." => GrammarCategory::Verb,
            "क्रिवि." => GrammarCategory::Adverb,
            "वि." => GrammarCategory::Adjective,
            "सर्व." => GrammarCategory::Pronoun,
            "नि." => GrammarCategory::Particle,
            "पूस." => GrammarCategory::Prefix,
            "विबो." => GrammarCategory::Interjection,
            other => GrammarCategory::Other(other.to_string()),
        }
    }

    /// Convert to the abbreviated form used in the dictionary
    pub fn to_abbreviation(&self) -> &'static str {
        match self {
            GrammarCategory::Noun => "ना.",
            GrammarCategory::Verb => "क्रि.",
            GrammarCategory::Adjective => "वि.",
            GrammarCategory::Adverb => "क्रिवि.",
            GrammarCategory::Pronoun => "सर्व.",
            GrammarCategory::Particle => "नि.",
            GrammarCategory::Prefix => "पूस.",
            GrammarCategory::Interjection => "विबो.",
            GrammarCategory::Other(_) => "अन्य",
        }
    }
}

/// Represents the etymology information for a word
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Etymology {
    /// The source language or origin
    pub source: String,
    /// The original form in the source language
    pub original_form: Option<String>,
    /// Additional etymology notes
    pub notes: Option<String>,
}

impl Etymology {
    /// Parse etymology from the bracket notation used in the dictionary
    /// e.g., "[सं.]", "[प्रा. आम< सं. आम्]"
    pub fn from_bracket_notation(notation: &str) -> Self {
        let cleaned = notation.trim_matches(|c| c == '[' || c == ']');
        
        if let Some(_arrow_pos) = cleaned.find('<') {
            let parts: Vec<&str> = cleaned.split('<').collect();
            if parts.len() == 2 {
                let source = parts[0].trim();
                let original = parts[1].trim();
                Etymology {
                    source: source.to_string(),
                    original_form: Some(original.to_string()),
                    notes: None,
                }
            } else {
                Etymology {
                    source: cleaned.to_string(),
                    original_form: None,
                    notes: None,
                }
            }
        } else {
            Etymology {
                source: cleaned.to_string(),
                original_form: None,
                notes: None,
            }
        }
    }

    /// Convert to the bracket notation used in the dictionary
    pub fn to_bracket_notation(&self) -> String {
        match &self.original_form {
            Some(original) => format!("[{}< {}]", self.source, original),
            None => format!("[{}]", self.source),
        }
    }
}

/// Represents a single definition of a word with its grammatical information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Definition {
    /// The grammatical category of this definition
    pub grammar: GrammarCategory,
    /// Etymology information (optional)
    pub etymology: Option<Etymology>,
    /// List of senses/meanings for this definition
    pub senses: Vec<String>,
}

/// Represents a complete dictionary entry for a word
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictionaryEntry {
    /// The word being defined
    pub word: String,
    /// List of definitions for this word
    pub definitions: Vec<Definition>,
}

/// Represents the complete Nepali dictionary
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NepaliDictionary {
    /// All dictionary entries
    pub entries: Vec<DictionaryEntry>,
    /// Index for fast word lookup
    word_index: HashMap<String, usize>,
}

impl NepaliDictionary {
    /// Create a new empty dictionary
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            word_index: HashMap::new(),
        }
    }

    /// Load dictionary from JSON data
    pub fn from_json(json_data: &str) -> Result<Self, serde_json::Error> {
        let raw_entries: Vec<RawDictionaryEntry> = serde_json::from_str(json_data)?;
        let mut dictionary = Self::new();
        
        for raw_entry in raw_entries {
            let entry = raw_entry.into();
            dictionary.add_entry(entry);
        }
        
        Ok(dictionary)
    }

    /// Add a dictionary entry
    pub fn add_entry(&mut self, entry: DictionaryEntry) {
        let index = self.entries.len();
        self.word_index.insert(entry.word.clone(), index);
        self.entries.push(entry);
    }

    /// Find a word in the dictionary
    pub fn find_word(&self, word: &str) -> Option<&DictionaryEntry> {
        self.word_index.get(word).and_then(|&index| self.entries.get(index))
    }

    /// Search for words that start with the given prefix
    pub fn search_prefix(&self, prefix: &str) -> Vec<&DictionaryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.word.starts_with(prefix))
            .collect()
    }

    /// Search for words containing the given substring
    pub fn search_contains(&self, substring: &str) -> Vec<&DictionaryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.word.contains(substring))
            .collect()
    }

    /// Get the total number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all entries as a slice
    pub fn entries(&self) -> &[DictionaryEntry] {
        &self.entries
    }
}

impl Default for NepaliDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Raw dictionary entry structure for JSON deserialization
/// This matches the exact structure in the JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawDictionaryEntry {
    word: String,
    definitions: Vec<RawDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawDefinition {
    grammar: String,
    etymology: Option<String>,
    senses: Vec<String>,
}

impl From<RawDictionaryEntry> for DictionaryEntry {
    fn from(raw: RawDictionaryEntry) -> Self {
        let definitions = raw.definitions
            .into_iter()
            .map(|raw_def| Definition {
                grammar: GrammarCategory::from_abbreviation(&raw_def.grammar),
                etymology: raw_def.etymology.as_deref().map(Etymology::from_bracket_notation),
                senses: raw_def.senses,
            })
            .collect();

        DictionaryEntry {
            word: raw.word,
            definitions,
        }
    }
}

/// Utility functions for working with the dictionary
pub mod utils {
    use super::*;

    /// Extract all unique grammar categories from a dictionary
    pub fn extract_grammar_categories(dictionary: &NepaliDictionary) -> Vec<GrammarCategory> {
        let mut categories = std::collections::HashSet::new();
        
        for entry in &dictionary.entries {
            for definition in &entry.definitions {
                categories.insert(definition.grammar.clone());
            }
        }
        
        categories.into_iter().collect()
    }

    /// Count entries by grammar category
    pub fn count_by_grammar_category(dictionary: &NepaliDictionary) -> HashMap<GrammarCategory, usize> {
        let mut counts = HashMap::new();
        
        for entry in &dictionary.entries {
            for definition in &entry.definitions {
                *counts.entry(definition.grammar.clone()).or_insert(0) += 1;
            }
        }
        
        counts
    }

    /// Find words with etymology information
    pub fn find_words_with_etymology(dictionary: &NepaliDictionary) -> Vec<&DictionaryEntry> {
        dictionary.entries
            .iter()
            .filter(|entry| {
                entry.definitions.iter().any(|def| def.etymology.is_some())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_category_parsing() {
        assert_eq!(GrammarCategory::from_abbreviation("ना."), GrammarCategory::Noun);
        assert_eq!(GrammarCategory::from_abbreviation("क्रि."), GrammarCategory::Verb);
        assert_eq!(GrammarCategory::from_abbreviation("नि."), GrammarCategory::Particle);
    }

    #[test]
    fn test_etymology_parsing() {
        let etymology = Etymology::from_bracket_notation("[सं.]");
        assert_eq!(etymology.source, "सं.");
        assert_eq!(etymology.original_form, None);

        let etymology = Etymology::from_bracket_notation("[प्रा. आम< सं. आम्]");
        assert_eq!(etymology.source, "प्रा. आम");
        assert_eq!(etymology.original_form, Some("सं. आम्".to_string()));
    }

    #[test]
    fn test_dictionary_creation() {
        let mut dict = NepaliDictionary::new();
        
        let entry = DictionaryEntry {
            word: "अ".to_string(),
            definitions: vec![Definition {
                grammar: GrammarCategory::Noun,
                etymology: None,
                senses: vec!["देवनागरी वर्णमालाको पहिलो स्वर".to_string()],
            }],
        };
        
        dict.add_entry(entry);
        assert_eq!(dict.len(), 1);
        assert!(dict.find_word("अ").is_some());
    }
}
