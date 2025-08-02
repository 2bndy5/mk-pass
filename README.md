# Comprehensive Password Generator

This cross-platform compatible software will generate a password comprehensively.

## Features

Admittedly, the word "comprehensive" is not a scientific term.
In this software, the term "comprehensive" boasts the following features
when generating a password:

1. No characters are repeated.
2. Ensure at least one of each type of character is present:

   - uppercase letters
   - lowercase letters
   - numbers (if permitted)
   - special characters (if permitted)
3. Ensure the first character is a letter (if enabled).
   When enabled, the first character will be either a uppercase or
   lowercase alphabetical letter.

### What is a "special" character?

This software uses the following set characters to generate special characters in a password:

> ``- . / \ : ` + & , @ $ ! _ # % ~``

The space character is not actually considered a special character,
but spaces are used to make the above set more readable.

Obviously, this is not an exhaustive list of all printable, non-alphanumeric characters.
However, these are special characters that are widely accepted by most sign-on services.

## Command Line Interface

While this software can be used as a library, a binary executable is also provided for each release.

The following will print the available options and their default values.

```shell
mk-pass -h
```
