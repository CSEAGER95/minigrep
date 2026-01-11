pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//can change these methods to match the style used in the new search method.
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

pub fn search_exact_word<'a> (query: &str, contents: &'a str,) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        // Split line into words (by whitespace and punctuation)
        let words: Vec<&str> = line.split(|c: char| !c.is_alphanumeric()).collect();
        
        // Check if any word matches the query exactly
        if words.iter().any(|word| word == &query) {
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
    #[test]
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

    // #[test]
    // fn case_word_count() {
    //     let query = "safe";
    //     let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

    //     assert_eq!(
    //         vec!["1"],
    //         search_case_word_count(query, contents)
    //     );
    // }
}

