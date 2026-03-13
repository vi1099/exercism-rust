use std::collections::HashSet;
use std::collections::HashMap;

//Referenced 'the book' for hashmaps and counting occurences, hashsets, lifetimes (&'a) and referencing/dereferencing(*)
// and .chars() & .len()
//Needed to change original possible_anagrams signature to take a referenced string
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    //RULE: if the possible anagram is the word itself, it is not an anagram

    //Initial notes/thoughts:
    //First check the length of word
    //if len == len then check if it is an ananagram
    
    //before checking an anagram, i need to keep a occurency map of the input word. example: hello -> h: 1, e: 1, l: 2, o: 1
    //need to build a frequency map for each possible word

    //----------------------------SOLUTION-------------------------------
    //1. Normalize the original word since something like "L" and "l" are not the same value
    let normalized_word = word.to_lowercase();

    //2. Valid anagram set that we will return
    let mut anagrams = HashSet::new();

    //3. We need to build a frequency map of the original word
    let mut word_hashmap = HashMap::new();
    for letter in normalized_word.chars() {
        let count = word_hashmap.entry(letter).or_insert(0);
        //using * to access the value of the reference not the reference itself (letter -> value)
        *count += 1;
    }

    //4. Then we need to subsequently loop through each word in the possible anagrams list
    //and build a frequency map for each one
    //There are few places we can shortcircuit the loop. 
    for possible_word in possible_anagrams {
        let normalized_possible_word = possible_word.to_lowercase();

        //If the possible anagram is the same as the original then it's not an anagram
        if normalized_possible_word == normalized_word {
            continue;
        }

        //If the length is not the same as the length of the orignal word then skip loop
        if normalized_possible_word.len() != normalized_word.len() {
            continue;
        }

        let mut current_word_occurences = HashMap::new();
        for letter in normalized_possible_word.chars() {
            let c = current_word_occurences.entry(letter).or_insert(0);
            *c += 1;
        }
        if current_word_occurences == word_hashmap {
            //We must dereference the possible anagram word because when we add it to the anagrams set, 
            //we are adding a reference to what already is a reference, making it a double reference
            anagrams.insert( *possible_word );
        }
    }

    anagrams
}
