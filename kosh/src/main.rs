use libkosh::{NepaliDictionary, DictionaryEntry, Definition, GrammarCategory, Etymology};

fn main() {
    // Example usage of the Nepali dictionary data model
    let mut dictionary = NepaliDictionary::new();
    
    // Create a sample dictionary entry
    let entry = DictionaryEntry {
        word: "अ".to_string(),
        definitions: vec![
            Definition {
                grammar: GrammarCategory::Noun,
                etymology: None,
                senses: vec![
                    "देवनागरी वर्णमालाको स्वर वर्णमध्ये पहिलो स्वर वर्ण".to_string(),
                    "लेखाइका क्रममा विषयको विभाजन उपविभाजनका निम्ति स्वर वर्णको प्रयोग गरिँदा दिइने क्रमबोधक पहिलो चिह्न".to_string(),
                ],
            },
            Definition {
                grammar: GrammarCategory::Particle,
                etymology: Some(Etymology::from_bracket_notation("[सं.]")),
                senses: vec![
                    "संस्कृत एकाक्षरी कोशअनुसार मूलतः विष्णुलाई जनाउने मङ्गलवाची शब्द".to_string(),
                ],
            },
        ],
    };
    
    dictionary.add_entry(entry);
    
    // Demonstrate dictionary functionality
    println!("Nepali Dictionary Data Model Demo");
    println!("=================================");
    println!("Total entries: {}", dictionary.len());
    
    if let Some(found_entry) = dictionary.find_word("अ") {
        println!("\nFound word: {}", found_entry.word);
        for (i, definition) in found_entry.definitions.iter().enumerate() {
            println!("  Definition {}: {} ({})", 
                i + 1, 
                definition.senses.join("; "),
                definition.grammar.to_abbreviation()
            );
            if let Some(etymology) = &definition.etymology {
                println!("    Etymology: {}", etymology.to_bracket_notation());
            }
        }
    }
    
    // Demonstrate search functionality
    let prefix_results = dictionary.search_prefix("अ");
    println!("\nWords starting with 'अ': {}", prefix_results.len());
    
    // Demonstrate utility functions
    let categories = libkosh::utils::extract_grammar_categories(&dictionary);
    println!("Grammar categories found: {:?}", categories);
}
