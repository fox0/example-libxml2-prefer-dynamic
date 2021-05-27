use libxml::parser::Parser;
use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;

fn main() {
    let mut xsdparser = SchemaParserContext::from_file("tests/test.xsd");
    let mut xsd = match SchemaValidationContext::from_parser(&mut xsdparser) {
        Ok(v) => v,
        Err(errors) => {
            for e in &errors {
                eprintln!("error: {}", e.message());
            }
            ::std::process::exit(1);
        }
    };

    let xml = match Parser::default().parse_file("tests/test.xml") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {}", e);
            ::std::process::exit(2);
        }
    };

    if let Err(errors) = xsd.validate_document(&xml) {
        for e in &errors {
            eprintln!("error: {}", e.message());
        }
        ::std::process::exit(3);
    }

    ::std::process::exit(0);
}
