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

/// DICT Protocol Server Implementation
pub mod dict_server {
    use crate::NepaliDictionary;
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::sync::RwLock;

    /// DICT protocol response codes
    pub mod response_codes {
        pub const BANNER: &str = "220";
        pub const CLOSING_CONNECTION: &str = "221";
        pub const OK: &str = "250";
        pub const DATABASES_FOLLOW: &str = "110";
        pub const STRATEGIES_FOLLOW: &str = "111";
        pub const HELP_FOLLOWS: &str = "113";
        pub const DEFINITIONS_RETRIEVED: &str = "150";
        pub const DEFINITION_FOLLOWS: &str = "151";
        pub const MATCHES_FOLLOW: &str = "152";
        pub const STATUS_INFO: &str = "210";
        pub const SYNTAX_ERROR: &str = "500";
        pub const ILLEGAL_PARAMS: &str = "501";
        pub const COMMAND_NOT_IMPLEMENTED: &str = "502";
        pub const NO_MATCH: &str = "552";
        pub const NO_DATABASES: &str = "554";
    }

    /// Represents a DICT server instance
    pub struct DictServer {
        dictionary: Arc<RwLock<NepaliDictionary>>,
        server_info: ServerInfo,
    }

    /// Server information for identification
    #[derive(Debug, Clone)]
    pub struct ServerInfo {
        pub name: String,
        pub version: String,
        pub description: String,
    }

    impl Default for ServerInfo {
        fn default() -> Self {
            Self {
                name: "kosh".to_string(),
                version: "0.1.0".to_string(),
                description: "Kosh - an extendable DICT server written in Rust".to_string(),
            }
        }
    }

    impl DictServer {
        /// Create a new DICT server with the given dictionary
        pub fn new(dictionary: NepaliDictionary) -> Self {
            Self {
                dictionary: Arc::new(RwLock::new(dictionary)),
                server_info: ServerInfo::default(),
            }
        }

        /// Create a new DICT server with custom server info
        pub fn with_info(dictionary: NepaliDictionary, server_info: ServerInfo) -> Self {
            Self {
                dictionary: Arc::new(RwLock::new(dictionary)),
                server_info,
            }
        }

        /// Start the DICT server on the specified address
        pub async fn start(&self, addr: &str) -> tokio::io::Result<()> {
            let listener = TcpListener::bind(addr).await?;
            println!("DICT server listening on {}", addr);

            loop {
                let (stream, peer_addr) = listener.accept().await?;
                println!("New connection from: {}", peer_addr);

                let dictionary = Arc::clone(&self.dictionary);
                let server_info = self.server_info.clone();

                tokio::spawn(async move {
                    if let Err(e) = Self::handle_client(stream, dictionary, server_info).await {
                        eprintln!("Error handling client {}: {}", peer_addr, e);
                    }
                });
            }
        }

        /// Handle a single client connection
        async fn handle_client(
            mut stream: TcpStream,
            dictionary: Arc<RwLock<NepaliDictionary>>,
            server_info: ServerInfo,
        ) -> tokio::io::Result<()> {
            // Send initial banner
            let banner = format!(
                "{} {} {} <kosh.{}.{}>\r\n",
                response_codes::BANNER,
                server_info.name,
                server_info.description,
                server_info.version,
                chrono::Utc::now().timestamp()
            );
            stream.write_all(banner.as_bytes()).await?;

            // Split the stream for reading and writing
            let (reader, mut writer) = stream.into_split();
            let reader = BufReader::new(reader);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await? {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let response = Self::process_command(&line, &dictionary, &server_info).await;
                writer.write_all(response.as_bytes()).await?;

                // Check if client sent QUIT
                if line.to_uppercase().starts_with("QUIT") {
                    break;
                }
            }

            Ok(())
        }

        /// Process a DICT command and return the appropriate response
        pub async fn process_command(
            command: &str,
            dictionary: &Arc<RwLock<NepaliDictionary>>,
            server_info: &ServerInfo,
        ) -> String {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.is_empty() {
                return format!("{} Syntax error, command not recognized\r\n", response_codes::SYNTAX_ERROR);
            }

            let cmd = parts[0].to_uppercase();

            match cmd.as_str() {
                "HELP" => Self::handle_help().await,
                "QUIT" => Self::handle_quit().await,
                "DEFINE" => Self::handle_define(&parts[1..], dictionary).await,
                "SHOW" => Self::handle_show(&parts[1..], dictionary, server_info).await,
                "STATUS" => Self::handle_status(server_info).await,
                "CLIENT" => Self::handle_client_info(&parts[1..]).await,
                "MATCH" => Self::handle_match(&parts[1..], dictionary).await,
                _ => format!("{} Syntax error, command not recognized\r\n", response_codes::SYNTAX_ERROR),
            }
        }

        /// Handle HELP command
        async fn handle_help() -> String {
            let mut response = format!("{} Help text follows\r\n", response_codes::HELP_FOLLOWS);
            response.push_str("DEFINE database word            look up word in database\r\n");
            response.push_str("MATCH database strategy word    match word in database using strategy\r\n");
            response.push_str("SHOW DB                         list available databases\r\n");
            response.push_str("SHOW STRAT                      list available strategies\r\n");
            response.push_str("SHOW INFO                       provide information about the server\r\n");
            response.push_str("SHOW SERVER                     provide site-specific information\r\n");
            response.push_str("CLIENT info                     provide information about client\r\n");
            response.push_str("STATUS                          display server status\r\n");
            response.push_str("HELP                            display this help\r\n");
            response.push_str("QUIT                            close connection\r\n");
            response.push_str(".\r\n");
            response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
            response
        }

        /// Handle QUIT command
        async fn handle_quit() -> String {
            format!("{} Closing Connection\r\n", response_codes::CLOSING_CONNECTION)
        }

        /// Handle STATUS command
        async fn handle_status(server_info: &ServerInfo) -> String {
            format!(
                "{} {} {} uptime: {}s\r\n",
                response_codes::STATUS_INFO,
                server_info.name,
                server_info.version,
                chrono::Utc::now().timestamp() % 3600 // Simple uptime simulation
            )
        }

        /// Handle CLIENT command
        async fn handle_client_info(_args: &[&str]) -> String {
            format!("{} ok\r\n", response_codes::OK)
        }

        /// Handle DEFINE command
        async fn handle_define(args: &[&str], dictionary: &Arc<RwLock<NepaliDictionary>>) -> String {
            if args.len() < 2 {
                return format!("{} Syntax error, illegal parameters\r\n", response_codes::ILLEGAL_PARAMS);
            }

            let _database = args[0];
            let word = args[1];

            // For now, ignore database selection and search in our dictionary
            let dict = dictionary.read().await;
            
            if let Some(entry) = dict.find_word(word) {
                let mut response = format!("{} 1 definitions found: list follows\r\n", response_codes::DEFINITIONS_RETRIEVED);
                response.push_str(&format!(
                    "{} \"{}\" nepali \"Nepali Dictionary\" : definition follows\r\n",
                    response_codes::DEFINITION_FOLLOWS,
                    word
                ));
                
                // Format definitions
                for (i, def) in entry.definitions.iter().enumerate() {
                    response.push_str(&format!("{}. {} ({})\r\n", i + 1, def.senses.join("; "), def.grammar.to_abbreviation()));
                    if let Some(etymology) = &def.etymology {
                        response.push_str(&format!("   Etymology: {}\r\n", etymology.to_bracket_notation()));
                    }
                }
                
                response.push_str(".\r\n");
                response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
                response
            } else {
                format!("{} No match\r\n", response_codes::NO_MATCH)
            }
        }

        /// Handle MATCH command (basic implementation)
        async fn handle_match(args: &[&str], dictionary: &Arc<RwLock<NepaliDictionary>>) -> String {
            if args.len() < 3 {
                return format!("{} Syntax error, illegal parameters\r\n", response_codes::ILLEGAL_PARAMS);
            }

            let _database = args[0];
            let strategy = args[1];
            let word = args[2];

            let dict = dictionary.read().await;
            
            let matches = match strategy {
                "prefix" => dict.search_prefix(word),
                "substring" => dict.search_contains(word),
                "exact" => {
                    if let Some(_) = dict.find_word(word) {
                        vec![dict.find_word(word).unwrap()]
                    } else {
                        vec![]
                    }
                },
                _ => vec![], // Unknown strategy
            };

            if matches.is_empty() {
                format!("{} No match\r\n", response_codes::NO_MATCH)
            } else {
                let mut response = format!("{} {} matches found: list follows\r\n", response_codes::MATCHES_FOLLOW, matches.len());
                for entry in matches {
                    response.push_str(&format!("nepali \"{}\"\r\n", entry.word));
                }
                response.push_str(".\r\n");
                response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
                response
            }
        }

        /// Handle SHOW commands
        async fn handle_show(
            args: &[&str],
            dictionary: &Arc<RwLock<NepaliDictionary>>,
            server_info: &ServerInfo,
        ) -> String {
            if args.is_empty() {
                return format!("{} Syntax error, illegal parameters\r\n", response_codes::ILLEGAL_PARAMS);
            }

            match args[0].to_uppercase().as_str() {
                "DB" => {
                    let dict = dictionary.read().await;
                    let count = dict.len();
                    let mut response = format!("{} 1 database present: list follows\r\n", response_codes::DATABASES_FOLLOW);
                    response.push_str(&format!("nepali \"Nepali Dictionary ({} entries)\"\r\n", count));
                    response.push_str(".\r\n");
                    response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
                    response
                }
                "STRAT" => {
                    let mut response = format!("{} 3 strategies available: list follows\r\n", response_codes::STRATEGIES_FOLLOW);
                    response.push_str("exact \"Exact match\"\r\n");
                    response.push_str("prefix \"Match prefixes\"\r\n");
                    response.push_str("substring \"Match substrings\"\r\n");
                    response.push_str(".\r\n");
                    response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
                    response
                }
                "INFO" => {
                    let mut response = format!("{} server information follows\r\n", response_codes::DATABASES_FOLLOW);
                    response.push_str(&format!("Server: {}\r\n", server_info.description));
                    response.push_str(&format!("Version: {}\r\n", server_info.version));
                    response.push_str("Protocol: DICT 1.0 (RFC 2229)\r\n");
                    response.push_str(".\r\n");
                    response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
                    response
                }
                "SERVER" => {
                    let mut response = format!("{} server information follows\r\n", response_codes::DATABASES_FOLLOW);
                    response.push_str("This is a Kosh DICT server implementation\r\n");
                    response.push_str("Built in Rust for serving Nepali dictionary data\r\n");
                    response.push_str(".\r\n");
                    response.push_str(&format!("{} Command complete\r\n", response_codes::OK));
                    response
                }
                _ => format!("{} Syntax error, illegal parameters\r\n", response_codes::ILLEGAL_PARAMS),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict_server::{DictServer, ServerInfo};

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

    #[tokio::test]
    async fn test_dict_server_help_command() {
        let dict = NepaliDictionary::new();
        let server_info = ServerInfo::default();
        
        let response = DictServer::process_command("HELP", &std::sync::Arc::new(tokio::sync::RwLock::new(dict)), &server_info).await;
        
        assert!(response.contains("113 Help text follows"));
        assert!(response.contains("DEFINE database word"));
        assert!(response.contains("MATCH database strategy word"));
        assert!(response.contains("250 Command complete"));
    }

    #[tokio::test]
    async fn test_dict_server_quit_command() {
        let dict = NepaliDictionary::new();
        let server_info = ServerInfo::default();
        
        let response = DictServer::process_command("QUIT", &std::sync::Arc::new(tokio::sync::RwLock::new(dict)), &server_info).await;
        
        assert!(response.contains("221 Closing Connection"));
    }

    #[tokio::test]
    async fn test_dict_server_define_command() {
        let mut dict = NepaliDictionary::new();
        let entry = DictionaryEntry {
            word: "test".to_string(),
            definitions: vec![Definition {
                grammar: GrammarCategory::Noun,
                etymology: None,
                senses: vec!["a test word".to_string()],
            }],
        };
        dict.add_entry(entry);
        
        let server_info = ServerInfo::default();
        let response = DictServer::process_command("DEFINE nepali test", &std::sync::Arc::new(tokio::sync::RwLock::new(dict)), &server_info).await;
        
        assert!(response.contains("150 1 definitions found"));
        assert!(response.contains("151 \"test\" nepali"));
        assert!(response.contains("a test word"));
    }

    #[tokio::test]
    async fn test_dict_server_show_db_command() {
        let mut dict = NepaliDictionary::new();
        let entry = DictionaryEntry {
            word: "test".to_string(),
            definitions: vec![Definition {
                grammar: GrammarCategory::Noun,
                etymology: None,
                senses: vec!["a test word".to_string()],
            }],
        };
        dict.add_entry(entry);
        
        let server_info = ServerInfo::default();
        let response = DictServer::process_command("SHOW DB", &std::sync::Arc::new(tokio::sync::RwLock::new(dict)), &server_info).await;
        
        assert!(response.contains("110 1 database present"));
        assert!(response.contains("nepali \"Nepali Dictionary (1 entries)\""));
        assert!(response.contains("250 Command complete"));
    }
}
