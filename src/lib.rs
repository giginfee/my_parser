use pest::Parser;
use pest_derive::Parser;

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

#[test]
fn field_test() -> anyhow::Result< ()>{
    let field = parse_field("<>");
    assert!(field.is_err());

    let field = parse_field("field");
    assert_eq!(field.is_err(), false);

    let field = parse_field("<field></field>");
    assert_eq!(field.is_err(), false);

    let field = parse_field("<>");
    assert!(field.is_err());   

   Ok(())
}
