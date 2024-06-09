<h1 align="center">prefix_id</h1>

<div align="center">
  <p>
    <strong>All the magic from nanoid but with a prefix</strong>
  </p>
  <p>Inspired by <a href="https://github.com/nikolay-govorov/nanoid">nanoid</a></p>
</div>

<div align="center">
  <!-- Docs.rs docs -->
  <a href="https://docs.rs/prefix_id">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="Docs.rs docs" /></a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/prefix_id">
    <img src="https://img.shields.io/crates/v/prefix_id.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/prefix_id">
    <img src="https://img.shields.io/crates/d/prefix_id.svg?style=flat-square"
      alt="Download" /></a>
</div>

## Installation

```shell
cargo add prefix_id
```

## Usage

```rust
create_id!(MyEntityId, "id");

let id = MyEntityId::new(); // => "id_1234567890avjfwdnfvqdp"

// then you can use the impl methods

id.as_str(); // => "id_1234567890avjfwdnfvqdp"
let my_string: String = id.into();

// you can also validate/convert it from string

let str = "id_1234567890avjfwdnfvqdp";
let id = MyEntityId::from_str(str).unwrap();

// you can optionally set the feature `serde` flag to serialize/deserialize

let serialized = serde_json::to_string(&id).unwrap();
let deserialized = serde_json::from_str::<MyEntityId>(&serialized).unwrap();

```

## License

<sup>
Licensed under <a href="LICENSE-APACHE">Apache License, Version
2.0</a> 
</sup>

<br>
