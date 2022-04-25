# Serde-Toml-JSON

With a concrete structure/organization of a TOML file, we can use a known technique shown in this project, toml_static, to serialize and deserialize data
### To run it
Clone the toml_static repo and type in the terminal:
 ```sh
 cargo run ./data/config.toml
 ```
 The program will only print the following line:

```sh
[postgresql].database: Rust2018
```
### Storing and Retrieving Data
This project uses two crates:
1. serde: This enables the use of the basic serialization/deserialization operations.
2. serde_json: Like XML, it allows to encode structured data in a text format that can be easily read by humans. Its simple syntax and native compatibility with JavaScript have made it a widely used format.

serde is the standard serialization/deserialization library. Serialization is the process of converting data structures of the program into a string (or a stream). Deserialization is the reverse process; it is the process of converting a string (or a stream) into some data structures of the program.