# sha1_cracker
## Basic sha1_cracker
This was made after I followed a chapter in the book ["Black Hat Rust"](https://kerkour.com/black-hat-rust) and then decided to add some extra functionality to the example program. The original program only allowed you to process the wordlist line-by-line but now you have parameters that you can input to either maintain original functionality (line) or to alternatively load the wordlist entirely into memory or to read the wordlist line-by-line but with multiple threads. 

### Usage
sha_1cracker: `<wordlist.txt> <sha1_hash> <mode>`

`<wordlist.txt>` is path to your wordlist

#### Modes
mem - Load the entire list into memory. (Not recommended for large lists as it can exceed the available RAM and result in an Out-Of-Memory error) 

line - Read list line by line. (Slow as it is single-threaded and reads the list only line-by-line)

threads - Uses multiple threads. Chunks of the wordlist are read into memory and are processed in parallel with multiple threads, done for each chunk until completed. (Recommended Mode)
