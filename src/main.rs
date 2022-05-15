use std::env;



fn main() {
    let args = get_command_line_strs();
    let str1 = split_by_words(&args.0);
    let str2 = split_by_words(&args.1);
    let common_regex = get_common_regex(&str1, &str2, 0, 0, 0," ".to_string());
    println!("{}", common_regex)
}



/// Get two string arguments from command line.
fn get_command_line_strs() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    (args[1].to_string(), args[2].to_string())
}



/// Splits &str by words. Returns vector of strings. 
fn split_by_words(string: &str) -> Vec<String> {
    string.to_string()
        .split(" ")
        .map(|word| word.to_string())
        .collect()
}



/// Get the largest regex expression that match two strs given.
fn get_common_regex(str1: &Vec<String>, str2: &Vec<String>, i_str1: usize, i_str2: usize, i_str2_temp: usize, pattern: String) -> String {
    if i_str1 == str1.len() {
        return pattern
    };

    if i_str2 == str2.len() {
        return get_common_regex(str1, str2, i_str1 + 1, i_str2_temp, i_str2_temp, pattern)
    }

    if str1[i_str1] == str2[i_str2] {
        return get_common_regex(str1, str2, i_str1 + 1, i_str2 + 1, i_str2 + 1, pattern + " " + &str1[i_str1])
    } else {
        return get_common_regex(str1, str2, i_str1, i_str2 + 1, i_str2_temp,
            match &pattern[pattern.len()-1..] {
                "*" => pattern,
                _ => pattern + " *"
            }
        )
    }
}
