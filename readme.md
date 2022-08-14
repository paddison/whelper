# WHELPER 
   
A small helper to solve wordle puzzles. The application may be started with commandline arguments listed below under `Usage`.


## General tipps

It is always good to start with the word 'yeast' for reasons Jannika explained to me, which I have since long forgotten.

## Installation

Make sure you have cargo installed on your system. See https://www.rust-lang.org/tools/install for installing cargo and rustup.

There are several ways to install whelper:
```
git clone https://github.com/paddison/whelper.git
cd whelper
cargo run -- [options]
```
or
```
git clone https://github.com/paddison/whelper.git
cargo install --path whelper --root /path/to/dir
/path/to/dir/bin/whelper [options]
```
or 
```
cargo install --git https://github.com/paddison/whelper.git --root /path/to/dir
/path/to/dir/bin/whelper [options]
```

## Description

After starting the Application, the user is asked to provide certain inputs in order to filter the possible words. 
Inputs can be `letters` and `positions`.
A `letter` is a single alphabetic ascii character. Several letters can be provided by separating them with spaces (`' '`).
A `position` is a number between 1 and 5 (both inclusive) which indicates a certain position in a word.
Some options allow the user to specify letters at certain position. A single letter position pair is separated with a space, e.g. '`1 b`'. This would indicate that the word should start with the letter b.
Note that positions are not zero indexed, but start from one. Several position letter tuples can be entered 

The user is asked to enter inputs in the following order:

1. Enter new letters that are in the word:
    The user should enter letters that are known to be in the word, with an unknown position. 
    Example: If the user knows that the letters 'b' and 'c' are in the word but not where, he can provide an input like `b c`.
2. Enter letters and known positions:
    The user should enter position letter pairs, which are known to be at a certain position.
    Example: If the user knows that the letter c is the third letter of the word, and h the fourth, he can provide and input like `3 c 4 h` 
3. Enter new letters that are not in the word:
    The user should enter letters that are known to not be in the word, with unknown positions. This works similar to (1).
4. Enter letters and positions, which are known to be false:
    The user should enter position letter pairs, which are known to be NOT at a certain position.
    Example: If the user knows that the letter i is not the fourth letter in the word, he can provid an input like `4 i`.

## Command line options
```
Usage:
    whelper [OPTIONS]

OPTIONS:
    -c (letter )*           List of letters separated by one space that are known to be in the word (e.g. -c a b c)
    -C (letter )*           List of letters separated by one space that are known to NOT be in the word (e.g. -C x y z)   
    -p (pos letter)*        List a pair of position and letters for which the position in the word are known (e.g. -p 1 a 5 b)
    -P (pos letter)*        List a pair of position and letters for which are known not to be at the specified position (e.g. -P 2 x 4 y)
    -h                      Print this help text";
```
