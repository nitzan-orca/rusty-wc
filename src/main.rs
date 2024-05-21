mod testing_resources;

use clap::Parser;
use std::fs;

/// wc impl in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print the number of lines in each input file
    #[arg(short = 'l')]
    should_lines: bool,

    /// Print the number of bytes in each input file
    #[arg(short = 'c')]
    should_characters: bool,

    /// Print the number of words in each input file
    #[arg(short = 'w')]
    should_words: bool,

        /// Print the number of words in each input file
    #[arg(short = 'f')]
    should_frequency: bool,

    /// Paths to input files we want to `wc`. If more than one input file is
    /// specified, a line of cumulative counts for all the files is displayed
    /// on a separate line after the output for the last file.
    paths: Vec<String>,
}

fn main() {
    let parsed_args = Args::parse();
    let should_words: bool;
    let should_lines: bool;
    let should_characters: bool;
    let should_frequency: bool;
    let mut should_exit_with_err: bool = false;
    if !parsed_args.should_characters && !parsed_args.should_lines && !parsed_args.should_words && !parsed_args.should_frequency {
        // Compat with wc behavior, no flags passed means all these should be on.
        // -f should be mutual exclusive with other flags
        should_characters = true;
        should_lines = true;
        should_words = true;
        should_frequency = false;
    } else if parsed_args.should_frequency {
        should_characters = false;
        should_lines = false;
        should_words = false;
        should_frequency = true;
    }
    else {
        should_characters = parsed_args.should_characters;
        should_lines = parsed_args.should_lines;
        should_words = parsed_args.should_words;
        should_frequency = false;
    }

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    let mut total_frequency_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for path in parsed_args.paths.iter() {
        let file_contents = match fs::read_to_string(path.clone()) {
            Ok(x) => x,
            Err(e) => {
                eprint!("wc: {}: {}", path, e.to_string());
                should_exit_with_err = true;
                continue;
            }
        };
        if should_lines {
            let lines_in_this_content = count_lines_in_content(&file_contents);
            total_lines += lines_in_this_content;
            print!("{:>8}", lines_in_this_content);
        }
        if should_words {
            let words_in_this_content = count_words_in_content(&file_contents);
            total_words += words_in_this_content;
            print!("{:>8}", words_in_this_content);
        }
        if should_characters {
            let characters_in_this_content = count_characters_in_content(&file_contents);
            total_characters += characters_in_this_content;
            print!("{:>8}", characters_in_this_content);
        }
        if should_frequency {
            let frequency_map_this_content:std::collections::HashMap<String, usize> = calculate_frequency(&file_contents, &mut total_frequency_map);
            print!("{:?}", frequency_map_this_content);
        }
        println!(" {}", path)
    }
    if should_frequency {
        let top_words: std::collections::HashMap<String, usize> = calculate_top_words_mut(&mut total_frequency_map);
        println!("{:?}", top_words);
    }
    // Now if more than 1 path, print total
    if parsed_args.paths.len() > 1 {
        if should_lines {
            print!("{:>8}", total_lines);
        }
        if should_words {
            print!("{:>8}", total_words);
        }
        if should_characters {
            print!("{:>8}", total_characters);
        }
        println!(" total")
    }
    if should_exit_with_err {
        std::process::exit(0x00000001);
    }
}

fn calculate_frequency(content: &str, total_frequency_map: &mut std::collections::HashMap<String, usize>) -> std::collections::HashMap<String, usize> {
    let mut frequency_map_this_content: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for word in content.split_ascii_whitespace() {
        *frequency_map_this_content.entry(word.to_owned()).or_default() += 1;
        *total_frequency_map.entry(word.to_owned()).or_default() += 1;
    }
    calculate_top_words(frequency_map_this_content)
}


fn calculate_top_words(frequency_map: std::collections::HashMap<String, usize>) -> std::collections::HashMap<String, usize>{
    let mut top_words: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (word, count) in frequency_map.iter() {
        if top_words.len() < 10 {
            top_words.insert(word.to_owned(), *count);
        } else {
            let mut min_count = std::usize::MAX;
            let mut min_word = String::new();
            for (top_word, top_count) in top_words.iter() {
                if *top_count < min_count {
                    min_count = *top_count;
                    min_word = top_word.to_owned();
                }
            }
            if *count > min_count {
                top_words.remove(&min_word);
                top_words.insert(word.to_owned(), *count);
            }
        }
    }
    top_words
}
fn calculate_top_words_mut(total_frequency_map: &mut std::collections::HashMap<String, usize>) -> std::collections::HashMap<String, usize>{
    let mut top_words: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (word, count) in total_frequency_map.iter() {
        if top_words.len() < 10 {
            top_words.insert(word.to_owned(), *count);
        } else {
            let mut min_count = std::usize::MAX;
            let mut min_word = String::new();
            for (top_word, top_count) in top_words.iter() {
                if *top_count < min_count {
                    min_count = *top_count;
                    min_word = top_word.to_owned();
                }
            }
            if *count > min_count {
                top_words.remove(&min_word);
                top_words.insert(word.to_owned(), *count);
            }
        }
    }
    top_words
}


fn count_lines_in_content(content: &str) -> usize {
    // My initial implementation
    // content.split('\n').fold(0, |lines: u64, _x| lines + 1)
    // Easier way, still wrong
    // content.split('\n').count()
    // Apparently, wc counts `\n` in content, not lines
    content.match_indices('\n').count()
}

fn count_characters_in_content(content: &str) -> usize {
    content.chars().count()
}

fn count_words_in_content(content: &str) -> usize {
    content.split_ascii_whitespace().count()
}

#[cfg(test)]
mod tests {
    use crate::testing_resources::EXAMPLE_CONTENT_EMPTY;
    use crate::testing_resources::EXAMPLE_CONTENT_FIVE_WORDS;
    use crate::testing_resources::EXAMPLE_CONTENT_TEN_CHARS;
    use crate::testing_resources::EXAMPLE_CONTENT_WITH_FOUR_LINES;

    use super::*;

    #[test]
    fn test_count_lines_in_content() {
        assert_eq!(4, count_lines_in_content(EXAMPLE_CONTENT_WITH_FOUR_LINES));
        assert_eq!(0, count_lines_in_content(EXAMPLE_CONTENT_EMPTY));
    }

    #[test]
    fn test_count_words_in_content() {
        assert_eq!(5, count_words_in_content(EXAMPLE_CONTENT_FIVE_WORDS));
        assert_eq!(0, count_words_in_content(EXAMPLE_CONTENT_EMPTY));
    }

    #[test]
    fn test_count_characters_in_content() {
        assert_eq!(10, count_characters_in_content(EXAMPLE_CONTENT_TEN_CHARS));
        assert_eq!(0, count_characters_in_content(EXAMPLE_CONTENT_EMPTY));
    }
}
