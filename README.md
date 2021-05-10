

### Background

I needed some crate for debugging a `cargo` issue.



### Features

This crate promises (*in a non-legal sense) to be as side-effect-free as crate-possible:

- should compile with Rust 1.0.0,
- no dependencies,
- `#![no_std]`,
- doesn't set any app-global state,
- only a handful of functions exported.

Some non-default feature flags might be added later to opt into additional behavior.



### Usage

**Cargo.toml**:

```
[dependencies]
some_crate = "0.1.0"
```

**main.rs**:

```
pub fn main() {
    some_crate::f();
}
```


### License

[Do What The Fuck You Want To Public License](https://choosealicense.com/licenses/wtfpl/)
