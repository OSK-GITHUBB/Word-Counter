use std::io;

//1- Define a struct named WordCounter
//In the WordCounter struct, define a field named text.

#[derive(Debug)]
struct WordCounter {
    text: String,
}

//2- Implement the following functions for WordCounter:

impl WordCounter {

    //new(text: &str) -> WordCounter: This function should take a text and create an instance of the WordCounter struct.
    fn new(text: &str) -> WordCounter {
        let wordcounter: WordCounter = WordCounter {
            text: text.to_string(),
        };
    
        wordcounter
    }
    
    //count_words() -> usize: This function should count the number of words in the text 
    //and return the result as an integer. 
    //A word is defined as a string separated by whitespace characters.
    //In the count_words function, check if the text is empty. If it is empty, return an error message. 
    
    fn count_words(&self) -> usize {
        if self.text.is_empty() {
            0 // Return 0 for empty text
        } else {
            self.text.split_whitespace().count()
        }
    }
    
}

#[allow(unused_variables)]
fn main() {
    println!("Enter Text:");
    
    let mut input: String = String::new();

    //3- Prompt the user for text input.

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read msg");

    //println!("{}", input);

    //4- Create a WordCounter instance using the input text.

    let wordcounter = WordCounter::new(&input);

    //5- Call the count_words function and print the word count to the screen.
    let words = wordcounter.count_words();

    //println!("{:?}", wordcounter);

    println!("number of words: {}", words);

}
