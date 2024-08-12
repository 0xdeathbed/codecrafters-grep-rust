[![progress-banner](https://backend.codecrafters.io/progress/grep/8d517af6-4764-4532-b7b1-74876f7f4646)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own grep" Challenge](https://app.codecrafters.io/courses/grep/overview).

[Regular expressions](https://en.wikipedia.org/wiki/Regular_expression)
(Regexes, for short) are patterns used to match character combinations in
strings. [`grep`](https://en.wikipedia.org/wiki/Grep) is a CLI tool for
searching using Regexes.

In this challenge you'll build your own implementation of `grep`. Along the way
we'll learn about Regex syntax, how parsers/lexers work, and how regular
expressions are evaluated.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Usage

`echo "<INPUT>" | ./your_program.sh -E "<pattern>"`

```bash
echo -n "cat" | ./your_program.sh -E "a" # to check if literal is present or not
echo -n "cat_2" | ./your_program.sh -E "\w" # to check if input have alphanumeric
echo -n "cat1" | ./your_program.sh -E "\d" # to check if input have digit
echo -n "sscat" | ./your_program.sh -E "^st" # to check if input starts with given pattern `st`
echo -n "sscataa" | ./your_program.sh -E "aa$" # to check if input ends with given pattern `aa`
echo -n "sscat" | ./your_program.sh -E "s+cat" # to check have occurence of one or more
echo -n "cat" | ./your_program.sh -E "s?cat" # to check have occurence of zero or more
echo -n "cat" | ./your_program.sh -E "(dog|cat)" # to check have multiple patterns
echo -n "cat" | ./your_program.sh -E "c." # to check with wildcard

```


Program sets exit code for pattern found. It will be set to 0 if found or to 1 if not found or have any error. we can get exit code after running above command
```bash
echo $?

```
