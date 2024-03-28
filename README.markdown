# DOLLSCRIPT :dolls: :ribbon: :gear:

***A statically-typed, interpreted programming language for dramatic dolls. :dolls: :ribbon: :gear:***

## STATUS :warning.

***THIS PROJECT IS STILL HEAVILY WORK IN PROGRESS!***

## TODO :gear:

- [ ] Implement a lexer.
- [ ] Immplement a parser.
- [ ] Implement a byte-code generator.
- [ ] Implement a multi-threaded VM to run byte-code.
- [ ] Implement a CLI.

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
    let range_end: cash = limit + 1;
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
- Licensed under the GNU GPL v3.
