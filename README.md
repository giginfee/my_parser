XML parser
===

XML parser, which can be used to parse XML to pairs of tokens 


### USAGE
To bring this crate into your repository, either add `my_xml_parser` to your
`Cargo.toml`, or run `cargo add my_xml_parser`.

### EXAMPLE
```rust
 use my_xml_parser::*;
  

pub fn main() -> anyhow::Result <()>{

    let successful_parse = parse_xml("<r><a></a></r>")?;
    println!("{:?}", successful_parse);
    Ok(())
  }
  ```