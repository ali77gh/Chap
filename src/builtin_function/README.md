# Builtin Functions

## /math

| Names             | Input params      | output  | description                                                |
| ----------------- | ----------------- | ------- | ---------------------------------------------------------- |
| add               | num, num          | num     | adds two numbers 1 + 2 = 3 or 1.5 + 1 = 2.5                |
| add_many, add_all | num, num, num,... | num     | adds many numbers 1 + 2 + 3 = 6                            |
| minus             | num, num          | num     | minus two numbers 3 - 2 = 1                                |
| multiply          | num, num          | num     | minus two numbers 3 \* 2 = 6                               |
| divide            | num, num          | num     | divide two numbers 3 / 2 = 1.5                             |
| modulus, mod      | num, num          | num     | divide remaining 3 / 2 = 1                                 |
| power, pow        | num, num          | num     | power 3 \*\* 2 = 9                                         |
| square_root, sqrt | num               | num     | square root 9 -> sqrt -> 3                                 |
| increase, inc     | $num              | nothing | adds one to variable short form of: $a,1 -> add -> $a      |
| decrease, dec     | $num              | nothing | minus one from variable short form of: $a,1 -> minus -> $a |

## /std_io

| Names                 | Input params      | output  | description                  |
| --------------------- | ----------------- | ------- | ---------------------------- |
| std_out, print, show  | any, any, any,... | any     | prints params to console     |
| std_in, input         | nothing           | string  | read user input from console |
| exit, quit, kill, end | nothing           | nothing | ends execution               |

## /bools

| Names                 | Input params | output | description                                             |
| --------------------- | ------------ | ------ | ------------------------------------------------------- |
| assign                | any          | any    | put a value or variable in other variable 1 -> $a       |
| equal, eq             | any, any     | bool   | true if 1st and 2nd are equal and false if they are not |
| not_equal, neq        | any, any     | bool   | true if 1st and 2nd are not equal and false if they are |
| and                   | bool, bool   | bool   | and logical gate                                        |
| or                    | bool, bool   | bool   | or logical gate                                         |
| xor                   | bool, bool   | bool   | xor logical gate                                        |
| not                   | bool         | bool   | not logical gate                                        |
| greater_than, gt      | num, num     | bool   | true if 1st param is bigger than 2nd param 3,2 -> true  |
| less_than, lt         | num, num     | bool   | true if 1st param is less than 2nd param 3,2 -> false   |
| greaterThanEqual, gte | num, num     | bool   | grater than equal operator >=                           |
| lessThanEqual, lte    | num, num     | bool   | less than equal operator <=                             |

## /strings

| Names              | Input params  | output | description                                                      |
| ------------------ | ------------- | ------ | ---------------------------------------------------------------- |
| concat, cat        | any, any      | string | convert inputs to string and concat them "al","i" -> "ali"       |
| repeat             | any, int      | string | convert inputs to string and repeat "a",3 -> "aaa"               |
| length, len        | any           | int    | convert input to string and returns length 456 -> 3              |
| contains, has      | any           | bool   | convert inputs to string and returns 1st contains 2nd 11,1->true |
| slice, sub_string  | any, int, int | string | "hello", 1, 3 -> "el"                                            |
| charAt             | string, int   | string | "ali",2 -> char_at -> "l"                                        |
| toUpper, upperCase | string        | string | "ali" -> upper case -> "ALI"                                     |
| toLower, lowerCase | string        | string | "ALI" -> upper case -> "ali"                                     |
| trim               | string        | string | removes around spaces " ali " -> trim -> "ali"                   |

## /type_conversion

| Names     | Input params | output | description                                      |
| --------- | ------------ | ------ | ------------------------------------------------ |
| to_string | any          | string | convert input to string 1 -> "1"                 |
| to_float  | string       | float  | convert input to float "1.5" -> 1.5 ; "a"->error |
| to_int    | string       | int    | convert input to int "1" -> 1 ; "a"->error       |
| to_json   | any          | string |                                                  |
| from_json | string       | map    |                                                  |

## /control_flow

| Names                   | Input params   | output  | description                                              |
| ----------------------- | -------------- | ------- | -------------------------------------------------------- |
| jump                    | @tag           | nothing | moves executor curser to closest tag with specified name |
| jump_if                 | @tag, bool     | nothing | jumps to tag if 1st param is true                        |
| jump_if_not             | @tag, bool     | nothing | jumps to tag if 1st param is false                       |
| jump_if_equal, jeq      | @tag, any, any | nothing | jumps to tag if 2th and 3th params are equal             |
| jump_if_not_equal, jneq | @tag, any, any | nothing | jumps to tag if 2th and 3th params are not equal         |
| new_tag                 | @tag           | nothing | creates tag (you can call this just by writing tag name) |

## /collection

Same function names for lists and maps

| Names  | Input params | output  | description                                        |
| ------ | ------------ | ------- | -------------------------------------------------- |
| get    | list, int    | any     | get nth item of list second param is index of item |
| has    | list, any    | bool    | check if an item exist in a list                   |
| insert | list, any    | nothing | insert an item to list                             |
| remove | list, any    | nothing | removes a given item from list                     |
| get    | map, string  | any     | gets key string and returns value                  |
| has    | map, any     | bool    | check if an key exist in a map                     |
| insert | map, map     | nothing | adds second map entries to first one               |
| remove | map, string  | nothing | removes entry from map with key of second param    |

## /list

| Names     | Input params | output  | description                                    |
| --------- | ------------ | ------- | ---------------------------------------------- |
| pop       | list         | any     | remove last item of list and returns it        |
| last      | list         | any     | return lsat item of list (without removing it) |
| remove_at | list, int    | nothing | removes item at index of second param          |
| index_of  | list, any    | int     | search for an item on list and returns index   |

## /data_time

| Names                  | Input params | output  | description                             |
| ---------------------- | ------------ | ------- | --------------------------------------- |
| now_sec, now, unixTime | nothing      | float   | unix time standard in seconds           |
| wait_mil, wait_millis  | int          | nothing | delay code execution for n milliseconds |
| wait_sec, wait_sec     | int          | nothing | delay code execution for n seconds      |
| wait_min, wait_minute  | int          | nothing | delay code execution for n minutes      |
| wait_hour              | int          | nothing | delay code execution for n hours        |

## /net (Not on WASM)

| Names    | Input params | output | description        |
| -------- | ------------ | ------ | ------------------ |
| http_get | nothing      | bool   | sends http request |

## /random (Not on WASM)

| Names         | Input params | output | description                                          |
| ------------- | ------------ | ------ | ---------------------------------------------------- |
| random_bool   | nothing      | bool   | true or false                                        |
| random_choice | any, any...  | any    | takes bunch of params and return on on them randomly |
| random_number | int, int     | int    | takes min and max as inputs                          |
| random_string | string,int   | bool   | param1: alphabet, param2: length                     |

## other

| Names             | Input params | output  | description                                       |
| ----------------- | ------------ | ------- | ------------------------------------------------- |
| assign            | any          | any     | put a value or variable in other variable 1 -> $a |
| dump, dump_memory | nothing      | nothing | prints all variables with values                  |
| type_of, type     | any          | str     | prints type of param 1 -> int; "s" -> string      |
