# Chap

<img src="./Logo.png" width="130"></img>

[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/) <br>
[![Rust](https://github.com/ali77gh/Chap/actions/workflows/rust.yml/badge.svg?branch=stable)](https://github.com/ali77gh/Chap/actions/workflows/rust.yml)
[![GitHub license](https://badgen.net/github/license/ali77gh/chap)](https://github.com/ali77gh/chap/blob/master/LICENSE)

Chap is an Easy to learn, dynamic, interpretive, isolated, and keyword-less scripting language written in Rust. It is useful when you want your non-programmer users to control some stuff dynamically and perform some custom calculations in a safe sandbox.


Syntax is something between [Lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language)), [Assembly](https://en.wikipedia.org/wiki/Assembly_language) and [Bash](https://www.php.net/).

[ChapApp](https://github.com/ali77gh/chapAPP) is a Chap Editor/Compiler on Browser (Powered by WASM). ChapApp is written in Rust([Dioxus](https://dioxuslabs.com/)) as well.
<a href="https://ali77gh.github.io/ChapApp/" target="_blank">Open ChapApp in new tab</a>.

<br>

# Table of content

1. [Why it named 'Chap'](#name)
1. [Features](#features)
1. [Keywords](#keywords)
1. [Syntax](#syntax)
1. [Operators](#operators)
1. [ControlFlow](#control-flow)
1. [Samples](#samples)
1. [Data Types](#datatypes)
1. [Installation](#installation)
1. [How to use](#how-to-use)
1. [TODOs for version 2.0.0](#todos)
1. [Builtin function](#builtin-functions)

<br>

# Name

Rust or راست in persian means right and Chap or چپ means left.

If you code in rust(right) too much you gradually became capitalist after a while. so you need to write some chap(left) to escape from matrix.

Chap unlocks **Two-Dimensional** Full Stack Development. Front⬆️End, Back⬇️End, Rust➡️End, Chap⬅️End.

<br>

# Features

1. Its so easy to learn, (your grandma can learn it in 15 minutes max).
2. Cross platform (chap runs on Linux, MacOS, Web(WASM) and even Windows!)
3. Slow
4. Useless

<br>

# Keywords

What makes a programming language hard to learn?

```chp
"Keywords"
```

| Language | Keywords  | Difficulty level |
|----------|-----------|------------------|
| C#       | 102       | 5/5              |
| Java     | 48        | 4/5              |
| Python   | 35        | 3/5              |
| Lua      | 22        | 2/5              |
| Chap     | 0         | 0/5              |

<br>

# Syntax

A normal line of code in chap has 3 chunks separated with -> operator:

```chp
chunk1 -> chunk2 -> chunk3
```

| Chunk 1      | Chunk 2        | Chunk 3         |
|--------------|---------------|-----------------|
| input params | function name | output variable |

```chp
param1, param2 -> function_name -> $output_variable
```

For example:

```chp
1, 2 -> add -> $sum
```

1. 1 and 2 separated by "," are input params. <br>
1. These input params are moving to "add" function <br>
1. Finally $sum is a variable that holds the add result in it. <br>

Note: "add" is not a keyword its a builtin function.

## Ok but why?

English language is a <b>"left to right"</b> (aka LTR) language. and programming languages should follows the same rule right?

wrong:

```javascript
// c base languages:
result = send(sqrt(1 + 2).toString());
   ↑       ↑    ↑     ↑       ↑
   5       4    2     1       3
```

```chp
// chap
1,2 -> add -> sqrt -> to_string -> send -> $result
        ↑       ↑         ↑          ↑        ↑
        1       2         3          4        5
```

This is actually left to right like normal english.

Note: "Piping" is added in version 2.0.0

<br>

## Syntax Rules

Make a comment with // and anything you write in right side will be ignored. <br>

```
1, 2 -> add -> $sum // this is a comment
```

You can write many line of code in one line by using ;

```chp
1 -> $a; $a, 2-> sum -> $b; $b -> print -> $_
```

Input params are separated by comma character ",". <br>

Input params can be:

 1. Variable <br>
 1. String with " character around like: "hello world" <br>
 1. Int just a number: 5<br>
 1. Float just a normal form of floating point number 3.14<br>
 1. Bool is a boolean value which is a true or false<br>
 1. Tag starts with @. [more on control flow](#controlflow)

```chp
$a, "hello", 5, 3.14, false -> function_name -> $output_var
```

Function names are not case sensitive. <br>

Function names are not sensitive about anything else:<br>

```chp
// to_string = ToString = TOSTRING = to string = t o s t r_i_n_g
```

variables should start with $ which is known as the most loved feature of PHP.<br>
variable name rules:

```chp
$12 // Ok
$sp a ce // OK
$#^3 //Ok
$a,b // comma not allowed
$really? // question mark at end not allowed
$rea?lly // OK
$some->thing // -> is not allowed 
```

<br>

## Short syntax features

If a function have no output variable you can remove chunk3:

```chp
"hello world" -> print
    ↑              ↑         ↑
   input         function   removed chunk3
```

If a function have no input param you can remove chunk1:

```chp
              input -> $user_input
 ↑              ↑         ↑
nothing      function    output
```

removing chunk2 (function name) means assign:

```chp
1 -> $variable
// its actually short form of:
1 -> assign -> $variable
```

If a function have no input param and output_var you just write function name:

```chp
exit
```

If function has output var but you removed chunk3 the result of function will get printed:

```chp
1, 2 -> add
// its short for of:
1, 2 -> add -> $temp
$temp -> print
```

If you just write some params. chap will print them:

```chp
1, 2
// result: 1, 2

// or
$a
// prints whatever $a is
```

We have worlds smallest hello world:

```chp
"Hello World"
```

I wish I could remove double quotes too :)

# Piping syntax

Sometimes you have a collection of function calls like this:

```chp
1, 2 -> add -> $tmp1
$tmp1 -> sqrt -> $tmp2
$tmp2 -> print
```

As you can see output of a function call is input of the next function call.

In this case you can use piping syntax and write functions next to to each other and get rid of temp variables:

```chp
1, 2 -> add -> sqrt -> print
```

# Parentheses

You can't use Piping while one of functions has more than one param.

```chp
1,2 -> add -> add -> print
               ↑
               This needs two input param
```

In this case you can use Parentheses:

```chp
(1,2 -> add), (3 -> sqrt) -> add -> print
```

This converts two:

```chp
1,2 -> add -> $TMP1
3 -> sqrt -> $TMP2
$TMP1, TMP2 -> add -> print
```


# Operators

There is one operator -> which moves data from left to right and it is language logo.

Why operators are bad? <br> because they behave different with different types.
look at this python example:

```python
number = input("Enter a number:")
result = number * 5 # multiply number to 5
print(number, "* 5 =", result)
```

Following code has bug and result will be:

```python
Enter a number: 3
3 * 5 = 33333
# no runtime error
```

Why? because python use same operator for math.multiply and strings.repeat.

So * operator <b>is not a type safe</b> operator and it will <b>do unexpected things</b> when your forget to pass right type to it and it will happen without throwing runtime errors (which is bad).

Same code in chap:

```chp
input -> $number
$number, 5 -> multiply -> $result 
$result
// error in line 2: multiply function works only with numbers int and float
```

Runtime errors are much better than logical error. and in chap we have repeat function:

```chp
"foo ", 3 -> repeat
// foo foo foo 
```

In many languages "+" operator has same problem:

```python
# python
def add(a, b):
    a + b # concat or add? both?

add("1", "2") # 12
add(1, 2) # 3
```

```chp
// Chap:
"1", "2" -> concat // 12
1, 2 -> concat // 12 // you can concat integers safely
1, 2 -> add // 3
"1", "2" -> add // runtime error
```

<br>

## debugger

You can put a ? end of line to debug that line:

```chp
1 -> $a
2 -> $b
$a, $b -> add? -> $c
// result 1, 2 -> add -> 3
```

Chap also has a function called "dump" which prints every variable you have.

# Control Flow

You can create tag like this:

```chp
@tag_name
```

And you can jump to it:

```chp
@tag_name -> jump
// or
@tag_name, true -> jump_if
// or
@tag_name, 1, 1 -> jump_if_equal
// or
@tag_name, 1, 0 -> jump_if_not_equal
```

## loop

jumping backward makes loops

```chp
@l
    "Hello util your battery dies"
@l -> jump
```

## if

```chp
@i, 1, 1 -> jump_if_equal
    "this will not print"
@i
```

Note: Indention is not necessary

<br>

# Array

Initialize:

```chp
[1 2 3 4] -> $myArray
```

Insert:

```chp
$myArray, 5 -> insert -> $myArray
```

Pop:

```chp
$myArray-> pop -> $last_item
```

Get item by index:

```chp
$myArray, 1 -> get -> $first_item
// arrays index start from 1
```

# Samples

## hello_world.chp

```chp
"Hello world"
```

```sh
Hello world
```

## counter.chp

```chp
0 -> $counter
@l
    $counter -> increase
@l, $counter, 100 -> jump_if_not_equal
$counter
```

```sh
100
```

## number_guess_game.chp

```chp
1,10 -> random_number -> $answer
@loop
    input -> $guess
    $guess -> to_int -> $guess
    @win, $answer, $guess -> jump_if_equal
    "wrong"
@loop -> jump

@win
"you win"
```

```sh
1
wrong
2
wrong
3
you win
```

## christmas_tree.chp

```chp
0 -> $counter
@loop
    $counter -> increase

    (" ", (10, $counter -> minus) -> repeat),
    ("*", ($counter, 2 -> multiply) -> repeat) -> 
    cat
@loop, $counter, 10 -> jump if not equal

```

```sh
         **
        ****
       ******
      ********
     **********
    ************
   **************
  ****************
 ******************
********************
```

<br>

## christmas_tree_with_trunk.chp

```chp

 // Editable
0 -> $counter
@loop
    $counter -> increase

    $counter, 1 -> multiply -> $stars_size
    19, $counter -> minus -> $space_size

    " * ", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat

    "`*-", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat
@loop, $counter, 10 -> jump if not equal

3 -> $c
@loop
    $c-> increase

    $c, 2 -> multiply -> $stars_size
    22, $c-> minus -> $space_size


    "*", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat

@loop, $c, 7 -> jump if not equal

```
```sh

                   * 
                  `*-
                  *  * 
                 `*-`*-
                 *  *  * 
                `*-`*-`*-
                *  *  *  * 
               `*-`*-`*-`*-
               *  *  *  *  * 
              `*-`*-`*-`*-`*-
              *  *  *  *  *  * 
             `*-`*-`*-`*-`*-`*-
             *  *  *  *  *  *  * 
            `*-`*-`*-`*-`*-`*-`*-
            *  *  *  *  *  *  *  * 
           `*-`*-`*-`*-`*-`*-`*-`*-
           *  *  *  *  *  *  *  *  * 
          `*-`*-`*-`*-`*-`*-`*-`*-`*-
          *  *  *  *  *  *  *  *  *  * 
         `*-`*-`*-`*-`*-`*-`*-`*-`*-`*-
                  ********
                 **********
                ************
               **************

```

<br>

# DataTypes

```chp
1 -> type_of
int

3.14 -> type of
float

"ali" -> TypeOf
string

true -> type      
boolean

-> [1 2 3] -> type      
list
```

# Installation

## Download release

[link](https://github.com/ali77gh/Chap/releases)

## Build

```bash
git clone https://github.com/ali77gh/Chap
cargo build --release
sudo cp ./target/release/chap /usr/bin
```

# How To Use

## REPL

[./repl/mod.rs](https://github.com/ali77gh/Chap/blob/master/src/repl/mod.rs)

```bash
❯ chap
-> "hello world"
hello world
-> 
```

## File_executor

[./file_executor/mod.rs](https://github.com/ali77gh/Chap/blob/master/src/file_executor/mod.rs)

```bash
❯ chap number_guess_game.chp 
1
wrong
2
wrong
3
you win answer was: 3
```

## Library

[./lib.rs](https://github.com/ali77gh/Chap/blob/master/src/lib.rs)

## As lib

```bash
cargo build --release --lib
```

## As Wasm module

 TODO

# TODOs

## Release Note version 2.0.0

- [x] Arrays
- [x] fix: 'random' module will not work on WASM
- [x] eval function
- [x] [ChapApp](https://github.com/ali77gh/ChapApp)
- [x] Piping syntax (1, 2 -> add -> toString -> print)
- [x] Parentheses (1, 2 -> add), (2, 3 -> add) -> concat -> $var // 35
- [x] New debugger syntax 1,2 -> add? -> $sum

## Version 3.0.0

- [ ] Fix floating point 0.2, 0.1 -> add -> $a // 0.3
- [ ] Better Closure gen for:
  - [ ] Static analyzer
  - [ ] Performance improvement
- [ ] Chap to C compiler (C has jumps BTW) + (executable or standard lib(dll|so|wasm) with extern)
- [ ] Update [is_prime](https://github.com/ali77gh/language_performance_prime_algorithm) results
- [ ] chap library for other programming lanuages

# Stars

[![Stargazers over time](https://starchart.cc/ali77gh/chap.svg)](https://starchart.cc/ali77gh/chap)

# Builtin Functions

[runtime/builtin_function](https://github.com/ali77gh/Chap/tree/master/src/runtime/builtin_function) <br>
Chap has 46 builtin function(version 1.0.1) (less than javas keywords)

| Names                   | Input params      | output   | description                                                 |
|-------------------------|-------------------|----------|-------------------------------------------------------------|
| assign                  | any               | any      | put a value or variable in other variable  1 -> $a          |
| std_out, print, show    | any, any, any,... | any      | prints params to console                                    |
| std_in, input           | nothing           | string   | read user input from console                                |
| exit, quit, kill, end   | nothing           | nothing  |  ends execution                                             |
| jump                    | @tag              | nothing  | moves executor curser to closest tag with specfied name     |
| jump_if                 | @tag, bool        | nothing  | jumps to tag if 1st param is true                           |
| jump_if_not             | @tag, bool        | nothing  | jumps to tag if 1st param is false                          |
| jump_if_equal, jeq      | @tag, any, any    | nothing  | jumps to tag if 2th and 3th params are equal                |
| jump_if_not_equal, jneq | @tag, any, any    | nothing  | jumps to tag if 2th and 3th params are not equal            |
| new_tag                 | @tag              | nothing  | creates tag (you can call this just by writing tag name     |
| add                     | num, num          | num      | adds two numbers     1 + 2 = 3 or 1.5 + 1 = 2.5             |
| add_many, add_all       | num, num, num,... | num      | adds many numbers    1 + 2 + 3 = 6                          |
| minus                   | num, num          | num      | minus two numbers    3 - 2  = 1                             |
| multiply                | num, num          | num      | minus two numbers    3 * 2  = 6                             |
| divide                  | num, num          | num      | divede two numbers   3 / 2  = 1.5                           |
| modulus, mod            | num, num          | num      | divide remaining     3 / 2  = 1                             |
| power, pow              | num, num          | num      | power                3 ** 2  = 9                            |
| square_root, sqrt       | num               | num      | square root          9 -> sqrt -> 3                         |
| increase, inc           | $num              | nothing  | adds one to variable short form of: $a,1 -> add -> $a       |
| decrease, dec           | $num              | nothing  | minus one from variable short form of: $a,1 -> minus -> $a  |
| equal, eq               | any, any          | bool     | true if 1st and 2nd are equal and false if they are not     |
| not_equal, neq          | any, any          | bool     | true if 1st and 2nd are not equal and false if they are     |
| and                     | bool, bool        | bool     | and logical gate                                            |
| or                      | bool, bool        | bool     | or logical gate                                             |
| not                     | bool              | bool     | not logical gate                                            |
| greater_than, gt        | num, num          | bool     | true if 1st param is bigger than 2nd param 3,2 -> true      |
| less_than, lt           | num, num          | bool     | true if 1st param is less than 2nd param   3,2 -> false     |
| concat, cat             | any, any          | string   | convert inputs to string and concat them "al","i" -> "ali"  |
| repeat                  | any, int          | string   | convert inputs to string and repeat "a",3 -> "aaa"          |
| length, len             | any               | int      | convert input to string and returns length 456 -> 3         |
| contains, has           | any               | bool     | convert inputs to string and returns 1st contains 2nd 11,1->true |
| slice, sub_string       | any, int, int     | string   | "hello", 1, 3 -> "el"                                       |
| to_string               | any               | string   | convert input to string        1 -> "1"                     |
| to_float                | string            | float    | convert input to float  "1.5" -> 1.5 ; "a"->error           |
| to_int                  | string            | int      | convert input to int    "1" -> 1 ; "a"->error               |
| dump, dump_memory       | nothing           | nothing  | prints all variables with values                            |
| type_of, type           | any               | str      | prints type of param   1 -> int; "s" -> string              |
| now_sec, now, unixtime  | nothing           | float    | unix time standard in seconds                               |
| wait_mil, wait_millis   | int               | nothing  | delay code execution for 1st milliseconds                   |
| wait_sec, wait_sec      | int               | nothing  | delay code execution for 1st seconds                        |
| wait_min, wait_minute   | int               | nothing  | delay code execution for 1st minutes                        |
| wait_hour,wait_hour     | int               | nothing  | delay code execution for 1st hours                          |
