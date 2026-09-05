use libkosh::{NepaliDictionary, DictionaryEntry, Definition, GrammarCategory, Etymology};
use libkosh::dict_server::{DictServer, ServerInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sample dictionary with some entries for testing
    let mut dictionary = NepaliDictionary::new();
    
    // Create sample dictionary entries
    let entry1 = DictionaryEntry {
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

    let entry2 = DictionaryEntry {
        word: "आ".to_string(),
        definitions: vec![
            Definition {
                grammar: GrammarCategory::Noun,
                etymology: None,
                senses: vec![
                    "देवनागरी वर्णमालाको दोस्रो स्वर वर्ण".to_string(),
                ],
            },
        ],
    };

    let entry3 = DictionaryEntry {
        word: "कोश".to_string(),
        definitions: vec![
            Definition {
                grammar: GrammarCategory::Noun,
                etymology: Some(Etymology::from_bracket_notation("[सं. कोश]")),
                senses: vec![
                    "शब्दसङ्ग्रह; शब्दकोश".to_string(),
                    "खजाना, भण्डार".to_string(),
                ],
            },
        ],
    };

    dictionary.add_entry(entry1);
    dictionary.add_entry(entry2);
    dictionary.add_entry(entry3);

    // Create server info
    let server_info = ServerInfo {
        name: "kosh".to_string(),
        version: "0.1.0".to_string(),
        description: "Kosh - an extendable DICT server written in Rust".to_string(),
    };

    // Create and start the DICT server
    let server = DictServer::with_info(dictionary, server_info);
    
    println!("Starting Kosh DICT server...");
    println!("Listening on 127.0.0.1:2628");
    println!("Press Ctrl+C to stop the server");
    
    // Start server on standard DICT port
    server.start("127.0.0.1:2628").await?;
    
    Ok(())
}
