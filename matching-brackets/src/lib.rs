pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut open_brackets = Vec::new();

    for char in string.chars() {
        match char {
            '(' | '{' | '[' => open_brackets.push(char),
            ')' | '}' | ']' => {
                if let Some(bracket) = open_brackets.pop() {
                    match (bracket, char) {
                        ('(', ')') | ('{', '}') | ('[', ']') => continue,
                        _ => (),
                    }
                }

                return false;
            }
            _ => (),
        }
    }

    open_brackets.is_empty()
}
