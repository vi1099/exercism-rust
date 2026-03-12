pub fn reverse(input: &str) -> String {
    let mut reverse_string: String = String::new();
    
    //1. turn string into a list of chars
    let mut chars: Vec<char> = input.chars().collect();
    
    //2. access list of chars from right to left
    while !chars.is_empty() {
        let current_char: char = chars.pop().unwrap();
        reverse_string.push( current_char );
    }

    println!("{}", reverse_string);
    
    reverse_string
}
