use std::io::Write;

pub fn query_input_or_default(question: &str, default: &str) -> String {
    print!("{}({}): ", question, default);
    std::io::stdout().flush().unwrap();

    let mut output = String::new();

    match std::io::stdin().read_line(&mut output) {
        Ok(_) => output.trim().to_string(),
        Err(_) => default.to_string(),
    }
}

pub fn query_required(question: &str) -> String {
    loop {
        print!("{}: ", question);
        std::io::stdout().flush().unwrap();

        let mut output = String::new();

        match std::io::stdin().read_line(&mut output) {
            Ok(_) => {
                if output == "\n" || output.is_empty() {
                    eprintln!("Warning: Cannot leave field empty!");
                } else {
                    return output.trim().to_string();
                }
                eprintln!("Warning: Cannot leave field empty!");
            }
            Err(e) => eprintln!("Error: failed retrieving input: {}", e),
        };
    }
}

pub fn query_valid_path(path_query: &str, is_a_directory: bool) -> String {
    loop {
        let path_string = query_required(path_query);
        let path = std::path::Path::new(path_string.trim());

        if is_a_directory {
            if path.is_dir() {
                return path_string.trim().to_string();
            }

            eprintln!("Warning: Given path is not a directory!");
        } else {
            if path.is_file() {
                return path_string.trim().to_string();
            }

            eprintln!("Warning: Given path is not a file!");
        }
    }
}
