use libxml::parser::Parser;
use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;

fn main() {
    let mut xsdparser = SchemaParserContext::from_file("tests/test.xsd");
    let mut xsd: SchemaValidationContext = SchemaValidationContext::from_parser(&mut xsdparser)
        .unwrap_or_else(|errors| {
            for e in &errors {
                eprintln!("error: {}", e.message());
            }
            ::std::process::exit(1);
        });

    let xml = Parser::default()
        .parse_file("tests/test.xml")
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            ::std::process::exit(2);
        });

    xsd.validate_document(&xml).unwrap_or_else(|errors| {
        for e in &errors {
            eprintln!("error: {}", e.message());
        }
        ::std::process::exit(3);
    });

    ::std::process::exit(0);
}
