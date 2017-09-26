use std::string::String;

/// Take a string and a desired column width, split the string into lines where
/// each line is at maximum width.
pub fn text_fill_column(text: &str, column_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut line = String::new();
    let mut width = 0;
    for word in text.split(' ') {
        if width + word.len() > column_width {
            width = 0;
            result.push(line.clone());
            line.clear();
        }
        width += word.len();
        line.push_str(word);
        line.push(' ');
    }
    result.push(line.clone());
    result
}

/// Take a string and a desired column width, fill it with space to the desired
/// `column_width`.
pub fn _code_fill_column(code: &str, column_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut filled_line = String::with_capacity(column_width);
    for line in code.lines() {
        let remaining = column_width - line.len();
        filled_line.push_str(line);
        for _i in 0..remaining {
            filled_line.push(' ');
        }
        result.push(filled_line.clone());
        filled_line.clear();
    }
    result
}
