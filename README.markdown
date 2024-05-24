# DOLLSCRIPT :dolls: :ribbon: :gear:

![GitHub CI](https://github.com/angeldollface/dollscript/actions/workflows/rust.yml/badge.svg)

***A statically-typed, interpreted programming language for dramatic dolls. :dolls: :ribbon: :gear:***

## STATUS :warning:

***THIS PROJECT IS STILL HEAVILY WORK IN PROGRESS!***

## ABOUT :books:

I've always been very interested in making my own programming language and decided to make something interesting in my favourite programming in the whole world, Rust. The *Dollscript* language was designed by me and is inspired by my absolute favourite girlies in the world, the girls with a passion 4 fashion, the [Bratz dolls](https://bratz.com). This repository contains the source code for the ***Dollscript*** interpreter.

## TODO :gear:

- [ ] Implement a lexer.
- [ ] Immplement a parser.
- [ ] Implement a byte-code generator.
- [ ] Implement a multi-threaded VM to run byte-code.
- [x] Implement a CLI.
- [ ] Implement a shell.
- [ ] Make sure errors error the program.
- [ ] Add unit tests.
- [ ] Make a logo.
- [ ] Write documentation in the wiki.
- [x] Add CI to the project.

## LANGUAGE INFORMATION :book:

***Dollscript*** is a functional, statically-typed, and interpreted language. 
### Datatypes

Since this language is inspired by the [Bratz dolls](https://www.bratz.com/), common datatypes have slightly more glam names.

- Integers aren't declared via the `int` keyword! They are declared via the `cash` keyword because the girls need cash to buy stuff.
- Strings aren't declared via the `string` keyword! They are declared via the `wisdom` keyword because the girls have much wisdom to share.
- Floats aren't declared via the `float` keyword! They are declared via the `smart` keyword because the girls are the smartest around!
- Structures aren't declared via the `struct` keyword! They are declared via the `bag` keyword because every dolly needs a decent, reliable bag!

### Variables

Variables are defined with either the `let` or `law` keyword. The former declares a mutable variable, the latter declares an immutable variable. Variables definitions are structured like this:

```Text
variable_declarator name: type = value;
```

A sample definition could look like this:

```Text
<3 Mutable.
let x: cash = 0;
x = 1;

<3 Immutable.
law x: cash = 0;
x = 1; <3 This will not work, x is immutable.
```

When the dollies decide something is important, it is a law and therefore not changeable at all!

### Functions

Functions are declared via the `shop` keyword because the girls with a passion 4 fashion need to be able to buy new outfits to always look their glammest! This is also the reason why the `return` keyword has been replaced with the `buy` keyword. This is what a function definition for greeting someone could look like:

```Text
shop greet(name: wisdom): wisdom {
    law greeting: wisdom = "Hello, $name!";
    buy greeting;
}
```

Function return types have to be declared explicitly, in this case a string, which is why the `wisdom` keyword is used. When feeding the function arguments, every input-datatype also has to be declared explicitly (`name` of type string). String formatting is done via the dollar sign (I quite like this feature from Bash.). If a function has a return-type of `void`, the return type will be that of `sleep`.

Every program needs to have a main point of entry and this is always done via the `main` function. The `main` function always has a return type of `sleep`.

### Structures

Coming soon!

### Comments

***Dollscript*** knows only one set of comment characters, the "<3" symbol. This can be used for inline and multi-line comments.

### Imports and Exports

Imports and exports are very simple in ***Dollscript***, any type, any function or typefield is declared to be a public entity via the `slayy` keyword because anything that is fit to be public has to slay! To import modules, ***Dollscript*** uses either one of the following options:

- To import a library:

```Text
@inspo "library";
```

- To import a file:

```Text
@inspo "./path/to/file.doll";
```

To export a module no special syntax is needed. All entities one would like to export need to be declared as being public.

To export a set of modules, the following syntax would be used:

```Text
@collection with [
    "module_one.dolls",
    "module_two.dolls"
    <3 etc.
];
```

## USAGE :gear:

### Running a program

To create a ***Dollscript*** program, put your code in a file ending in `.doll`. Run the program using the ***Dollscript*** interpreter like this:

```Bash
dolly do your_program.doll
```

### Linting a program

To lint your program, use the ***Dollscript*** interpreter like this:

```Bash
dolly glam your_program.doll
```

### Interactive shell

To start the interactive interpreter, use the ***Dollscript*** interpreter like this:

```Bash
dolly speak
```

## CODE SAMPLE :hammer_and_pick:

```Text
@inspo "std";

slayy bag FactSummary {
    slayy root: cash,
    slayy res: cash
}

slayy shop new(root: cash, res: cash) for FactSummary: FactSummary {
    buy FactSummary { root: root, res: res };
}

<3 A factorial function
<3 returning an integer.
slayy shop fact(limit: cash): cash {
    let res: cash = 0;
    law range_end: cash = limit + 1;
    for num in range(1, range_end){
        res = res * num;
    }
    buy res;
}

shop main(): sleep  {
    fact(5); <3 should output 120
}
```

## NOTE :scroll:

- *Dollscript :dolls: :ribbon: :gear:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the [DSL v1](https://github.com/angeldollface/doll-software-license).
