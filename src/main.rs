 use my_xml_parser::*;

 use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
  


    if args.len()==1 {
      println!("\nHello! this is XML parser! Try \"cargo run xml \"your xml\" to get parse results. \nTo see more info about about project enter \"cargo run info\". If you need help enter \"cargo run helpXML \n");
      
    }else if args.len()==2 {
    let query = &args[1];


    match query.as_str() {
      "helpXML" => println!("\nXML parser:\n\t -\"cargo run xml \"your xml\" \nMore info about project: \n\t-\"cargo run info.\"\n"),
      "info" => println!("\nThe creator is Bernatska Olha, see more https://crates.io/crates/my_parser.\n"),
      "xml" => println!("\nNo arguments found, try again.\n"),
      _ => println!("\nNo such command. \nSee help \"cargo run helpXML\".\n"),
     }

    }else if args.len()>=3{

 
      let query = &args[1];
      let xml_arg = &args[2];


    match query.as_str() {
      "xml" =>{let xml = parse_xml(xml_arg); 
      match xml{
        Ok(x)=>println!("\nParsed: {:?}\n",x),
        Err(_)=>println!("\nYour input is not XML.\n"),
      
      }
      }
       ,
      _ => println!("\nNo such command. \nSee help \"cargo run helpXML\".\n"),

  }
    }
}

// pub fn main() -> anyhow::Result <()>{

//     let successful_parse = parse_xml("<r>
    
//       <a>    
//         </a>
//         <a>
//         </a>
//     </r>")?;
//     println!("{}", successful_parse);
//     let field = parse_field("<field><f></field>")?.next().ok_or_else( || anyhow!( "no pair" ) )?;

//   let str ="dd";
//     let field = parse_text(str).expect("wh");
//     println!("{:?}", field);
// /*[Pair { rule: field, 
//           span: Span { str: "aa", start: 0, end: 2 }, 
//           inner: [Pair { rule: text, 
//                         span: Span { str: "aa", start: 0, end: 2 }, 
//                         inner: [] }] 
//         }]*/
//     Ok(())
//   }