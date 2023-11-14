use pest::Parser;
use pest_derive::Parser;
use anyhow::*; 


#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct XMLParser;
pub fn parse_xml(input:&str) -> anyhow::Result <pest::iterators::Pairs<'_, Rule>>{

    let successful_parse = XMLParser::parse(Rule::xml, input)?;
    Ok(successful_parse)

}
pub fn parse_field(input:&str) -> anyhow::Result <pest::iterators::Pairs<'_, Rule>>{

  let parse = XMLParser::parse(Rule::field, input)?;
  Ok(parse)
  
}
pub fn parse_text(input:&str) -> anyhow::Result <pest::iterators::Pairs<'_, Rule>>{
  // let parse = XMLParser::parse(Rule::field, input)?;


  let parse = XMLParser::parse(Rule::text, input)?;
  Ok(parse)
  
}
#[cfg(test)]
mod tests {
  use super::*;
#[test]
fn field_test() -> anyhow::Result< ()>{
  let str ="<>";
  let field = parse_field(str)?.next().ok_or_else( || anyhow!( "no pair" ) )?;
   assert_eq!(field.as_str(),"");

  let str ="field";
  let field = parse_field(str)?.next().ok_or_else( || anyhow!( "no pair" ) );
  assert_eq!(field?.as_span().end(), str.len());


  let str ="<field></field>";
  let field = parse_field(str)?.next().ok_or_else( || anyhow!( "no pair" ) );
  assert_eq!(field.is_err(), false);
  assert_eq!(field?.as_span().end(), str.len());
  

  let str ="<field>fff</field>";
  let field = parse_field(str)?.next().ok_or_else( || anyhow!( "no pair" ) );
  assert_eq!(field?.as_span().end(), str.len());


  let str ="<field><f></field>";
  let field = parse_field(str)?.next().ok_or_else( || anyhow!( "no pair" ) );
  assert_eq!(field?.as_str(),"");

 Ok(())
}

#[test]
fn text_without_brackets_test() -> anyhow::Result< ()>{
  let str ="<>";
  let field = parse_text(str);
  assert!(field.is_err());

  let str: &str ="field";
  let field = parse_text(str)?.next().ok_or_else( || anyhow!( "no pair" ) );
  assert!(field.is_ok());

  let str ="<field>";
  let field = parse_text(str);
  assert!(field.is_err());

  let str ="";
  let field = parse_text(str);
  assert!(field.is_err());

 Ok(())
}

#[test]
fn xml_test() -> anyhow::Result< ()>{
  let str ="<r>
     <a>    
     </a>
     <b>
     </b>
  </r>";
  let field = parse_xml(str);
  assert!(field.is_ok());
  assert_eq!(field?.as_str(),str);

  let str ="";
  let field = parse_xml(str);
  assert!(field.is_ok());
  assert_eq!(field?.as_str(),str);

  let str ="<a></b>";
  let field = parse_xml(str);
  assert!(field.is_err());

  let str ="<a></a><a></a>";
  let field = parse_xml(str);
  assert!(field.is_err());

  let str ="<r><a>text</a><a></a></r>";
  let field = parse_xml(str);
  assert!(field.is_ok());
  assert_eq!(field?.as_str(),str);

  let str ="<r>text<a></a><a></a></r>";
  let field = parse_xml(str);
  assert!(field.is_ok());
  assert_eq!(field?.as_str(),str);




 Ok(())
}
}
