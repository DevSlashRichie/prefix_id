<h1 align="center">prefix_id</h1>

<div align="center">
  <p>
    <strong>All the magic from [nanoid][] but with a prefix</strong>
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
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> 
</sup>

<br>

[nanoid]: https://github.com/nikolay-govorov/nanoid
