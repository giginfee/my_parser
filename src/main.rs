 use my_xml_parser::*;
  

pub fn main() -> anyhow::Result <()>{

    let successful_parse = parse_xml("<r><a></a></r>")?;
    println!("{:?}", successful_parse);
    Ok(())
  }