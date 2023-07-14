# sha1_cracker
This utility was inspired by a chapter in the book ["Black Hat Rust"](https://kerkour.com/black-hat-rust), and I have expanded upon the original program. The original program allowed you to process the wordlist line-by-line. Now, my version gives you parameters that let you choose to maintain the original functionality (line), load the entire wordlist into memory (mem), or process the wordlist line-by-line with multiple threads (threads). In addition, I have introduced a "rule" functionality that allows you to modify the words in the list according to specific rules, such as appending or prepending a digit, replacing 'a' with '@', and replacing 'i' with '1'. 

## Building
```cargo build --release```

The binary will be located in the target/release folder.

## Usage
sha1_cracker: `<sha1_hash> <wordlist.txt> --mode <mode> [--rule <rule1> <rule2> ...]`

`<wordlist.txt>` is the path to your wordlist.

### Modes
**mem** - Loads the entire list into memory. *(Not recommended for large lists as it can exceed the available RAM and result in an Out-Of-Memory error)* 

**line** - Reads the list line by line. *(Slow as it is single-threaded and reads the list only line-by-line)*

**threads** - Uses multiple threads. *Chunks of the wordlist are read into memory and are processed in parallel with multiple threads, done for each chunk until completed. (Recommended Mode)*

### Rules
**append** - Appends a digit to the word.

**prepend** - Prepends a digit to the word.

**replace_a** - Replaces 'a' in the word with '@'.

**replace_i** - Replaces 'i' in the word with '1'.

**all** - Applies all the above rules.
