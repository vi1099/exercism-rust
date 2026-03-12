pub fn reverse(input: &str) -> String {
    let mut reverseString: String = String::new();
    
    //1. turn string into a list of chars
    let mut chars: Vec<char> = input.chars().collect();
    
    //2. access list of chars from right to left
    while chars.len() > 0 {
        let currentChar: char = chars.pop().unwrap();
        reverseString.push( currentChar );
    }

    println!("{}", reverseString);
    return reverseString;
}
