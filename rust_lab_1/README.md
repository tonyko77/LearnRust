# Lab 1: Hide and Seek, but with explosions!

## Story ##
&nbsp;&nbsp;&nbsp; You are playing hide and seek with your family in a forest. But you and your family have super powers. So it's not normal hide and seek, its hide and seek with explosions!
It's your turn to hide, and your little brother's turn to see. In order to seek you he shoots random explosions at you until he finds you and hits you 3 times. Your brother goes and stands on a cliff overlooking the forest to get a better view and the games begin!


## Problem ##
&nbsp;&nbsp;&nbsp; 
Your brother can see the entire forest, a 5 x 5 grid, and knows that you are hiding somewhere in there. He decides to fire off explosions at random spots in the forest. Maybe he'll hit you, maybe he won't. Either way, he has a limited amount of energy can only fire off 36 explosions before his turn is over. But, if your brother hits you 3 times then he will win and get to choose what to eat for dinner.

&nbsp;&nbsp;&nbsp; You must simulate this duel, and print the result of who wins at the end.

## Other Rules ##
- You will hide in the same spot until he is hit. If you are hit you will jump to a new random spot in the forest.
- Every turn your brother must choose a new random spot to launch an attack on.
- Your sister is also hiding in a random spot in the forest. If your brother hits her, your brother becomes frozen and loses three explosions. your sister will never move.
- You and your sister cannot be in the same spot.
- At the beginning of each round output a visual of the forest. `"_"` marks a tree 'Y' marks yourself, 'S' marks your sister.
- After each round (your brother's explosion) output whether he hit, who he it or if he missed, as well as the location of the attack.
- Also after each round output your remaining stamina, and your brother's remaining explosions.

&nbsp;&nbsp;&nbsp; Once either you are out of stamina or your brother is out of explosions the contest is over. Print the winner followed by 'Time for dinner!'

## Advice:
- This exercise can be completed with sections Chapters 1-9 of [The Rust book](https://doc.rust-lang.org/book/). Though you could just use math to calculate all the results of this it's much more fun to over-engineer this! Try using structs, enums and options to represent the state of the game. For example, you could represent the forest as a struct, or a vector of a vector of enums. Feel free to really go crazy with this!
- Here is the documentation for the rand crate you need to use: [docs.rs/rand](https://docs.rs/rand/latest/rand/).
- The "rules" are not "rules" so much as "guidelines". If you encounter a case that is ambiguous feel free to make up your own rule.
- Ask other people for help and feedback
- Think you're done? Have `clippy` tell you what you can do to improve your code. Just run `cargo clippy`.

## Challenge mode:
- Use [Clap](https://github.com/clap-rs/clap) to generate a cli for this simulation that allows the user to change the initial parameters of the simulation, such as grid size, your brothers's stamina, how many sisters you have, etc.
- Wanna work ahead of where we are in the book? Create a `Forest` object to represent the game's current state and implement the `Display` trait.
- Add some unit tests! I get that unit testing is kind of odd in programs that use RNG but if you design your functions cleverly you can get around that. Note that you can test private functions in rust.

## Sample output:
```
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ S _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ Y _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
Your  is attacking (1, 5)!
Your brother hit you!
Your health: 2
Brother's blasts left: 2

_ _ _ _ _ _ Y _ _ _
_ _ _ _ _ _ S _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
Your Brother is attacking (8, 6)!
Your Brother missed completely!
Your health: 2
Brother's blasts left: 1

_ _ _ _ _ _ Y _ _ _
_ _ _ _ _ _ S _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _
Your Brother is attacking (4, 6)!
Your Brother missed completely!
Your health: 2
Brother's blasts left: 0
You are the winner!
```
