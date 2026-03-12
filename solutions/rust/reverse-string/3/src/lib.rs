pub fn reverse(input: &str) -> String {
    let mut reverse_string: String = String::new();
    
    //1. turn string into a list of chars
    let mut chars: Vec<char> = input.chars().collect();
    
    //2. access list of chars from right to left

    //before compiler suggestion
    // while !chars.is_empty() {
    //     let current_char: char = chars.pop().unwrap();
    //     reverse_string.push( current_char );
    // }

    //after compiler suggestion
    //this says while chars.pop() returns a Some optional not None, then continue processing
    //note, current_char is already unwrapped
    //cleaner way to write a while and match (to check if the optional is a some or none)
    while let Some( current_char ) = chars.pop() {
        reverse_string.push ( current_char );
    }
    
    println!("{}", reverse_string);
    
    reverse_string
}
