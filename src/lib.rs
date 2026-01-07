pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

//as it stands right now, it doesn't check for words at the front of a new line, or words prepended or appeded with a symbol
//such as: query = "fails"
//a comma connected to the searched word fails, 
//fails when the queried word is at the start of a new line,
//when the query is at the end of the new line it also fails
pub fn search_exact_word<'a> (query: &str, contents: &'a str,) -> Vec<&'a str> {
    let query = format!("{}{}{}", ' ',query, ' ');
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn exact_word() {
        let query = "a";
        let contents = "\
Rust:

safe, fast, productive.
Pick three
Duct a tape.";

            assert_eq!(vec!["Duct a tape."], search_exact_word(query, contents));
    }
        fn exact_word2() {
        let query = "Rust";
        let contents = "\
Rust:

safe, fast, productive.
Pick three
Duct a tape.";

            assert_eq!(vec!["Rust:"], search_exact_word(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_word_count() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["1"],
            search_case_word_count(query, contents)
        );
    }
}

