# kosh
Kosh - an extendable DICT server written in rust

## Features

- **DICT Protocol Server**: RFC 2229 compliant DICT protocol server
- **Nepali Dictionary Support**: Built-in support for Nepali language dictionaries
- **TCP Server**: Listens on standard DICT port 2628
- **Dictionary Management**: Add, search, and manage dictionary entries
- **Multi-strategy Search**: Support for exact, prefix, and substring matching

## Usage

### Running the Server

```bash
cargo run
```

The server will start listening on `127.0.0.1:2628` (standard DICT port).

### Testing with a Client

You can test the server using `telnet` or `nc`:

```bash
# Using netcat
echo -e "HELP\nQUIT" | nc 127.0.0.1 2628

# Using telnet  
telnet 127.0.0.1 2628
```

### Supported DICT Commands

- `HELP` - Display available commands
- `DEFINE database word` - Look up word definition
- `MATCH database strategy word` - Find matching words
- `SHOW DB` - List available databases
- `SHOW STRAT` - List available search strategies
- `SHOW INFO` - Server information
- `SHOW SERVER` - Site-specific information
- `STATUS` - Server status
- `CLIENT info` - Provide client information
- `QUIT` - Close connection

### Example Session

```
$ nc 127.0.0.1 2628
220 kosh Kosh - an extendable DICT server written in Rust <kosh.0.1.0.1759136616>
HELP
113 Help text follows
DEFINE database word            look up word in database
MATCH database strategy word    match word in database using strategy
[... more help text ...]
.
250 Command complete
DEFINE nepali कोश
150 1 definitions found: list follows
151 "कोश" nepali "Nepali Dictionary" : definition follows
1. शब्दसङ्ग्रह; शब्दकोश; खजाना, भण्डार (ना.)
   Etymology: [सं. कोश]
.
250 Command complete
QUIT
221 Closing Connection
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```
