use nom_sql::parser::parse_query;

// get ast from query string
fn parse() {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_queryset(queries: Vec<String>) -> (i32, i32) {
        let mut parsed_ok = Vec::new();
        let mut parsed_err = 0;
        for query in queries.iter() {
            println!("Trying to parse '{}': ", &query);
            match parse_query(&query) {
                Ok(_) => {
                    println!("ok");
                    parsed_ok.push(query);
                }
                Err(_) => {
                    println!("failed");
                    parsed_err += 1;
                }
            }
        }

        println!("\nParsing failed: {} queries", parsed_err);
        println!("Parsed successfully: {} queries", parsed_ok.len());
        println!("\nSuccessfully parsed queries:");
        for q in parsed_ok.iter() {
            println!("{:?}", q);
        }

        (parsed_ok.len() as i32, parsed_err)
    }

    #[test]
    fn parse_simple_select() {
        let (ok, fail) = parse_queryset(vec!["SELECT a+b FROM c;".to_string()]);
        assert_eq!(ok, 1);
        assert_eq!(fail, 0);
    }
}
