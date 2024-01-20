Task Details

create a word counter program in Rust.

Steps:


1- Create a struct named WordCounter.

In the WordCounter struct, have the following field:

text (text to count words from)


2- Implement the following functions for WordCounter:

new(text: &str) -> WordCounter: This function should take a text and create an instance of the WordCounter struct.

count_words() -> usize: This function should count the number of words in the text and return the result as an integer. A word is defined as a string separated by whitespace characters.

In the main function, prompt the user to enter a text.

In the count_words function, check if the text is empty. If it is empty, return an error message. 


3- Create an instance of WordCounter using the text entered by the user.

Call the count_words function on the created WordCounter instance.

Print the obtained word count to the screen.


4- Compile and run the program to test its functionality.


Checklist:

1- Define a struct named WordCounter.

2- In the WordCounter struct, define a field named text.

3- Implement the new function.

4- Implement the count_words function.

5- Prompt the user for text input.

6- Create a WordCounter instance using the input text.

7- Call the count_words function and print the word count to the screen.

8- Compile and run the program to test its functionality.


