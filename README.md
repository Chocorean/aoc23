# Advent of Code 2023

This is going to be my diary for AoC23. Just venting, writing comments and whatnot. Cheers!

## Day 1

Note for tomorrow: even if I'm excited about it, maybe it would be best to be rested before playing.

Surprised by the difficulty of this year's first challenge, and a little bit afraid for the rest, not gonna lie.
This year I'll push myself to do every single puzzle, even if that means getting inspiration from others. I know I'm not the
best at identifying how to solve problems, and I don't want to pressure myself too much since I really want to do all days.

And I'm sick. So I'll go at my own pace, lurk around the subreddit if I need inspiration, and hopefully learn new tricks in Rust and have fun.

Also, I should have trusted myself and go for this kind of implementation straight away instead of toying with regexes again, because that was much easier than I thought.

## Day 2

Surprisingly easier than day 1, I finally used my beloved regexes. Looking at the leaderboard, I would be very curious to see how people send this in 90 seconds.

I'm glad I've started AoC this year already, because I've discovered non-capturing groups (`(?:)` pattern) in a failed attempt for day 1, and I've already used it for day 2. My only wish is I was faster than that (I took 25 mins), but that is still okay I guess.

## Day 3

### Part 1

Tough too, I wasn't too sure by what angle I wanted to do it. My gut told me if I had it right, the part two would be easy. Let's see if I can do this! (I consider myself usually bad at problem solving on-sight)

### Part 2

If you did it yourself, I hope you were not as devastated as myself when we both read the part 2 problem. My intuition was kinda wrong: I approach the problem by identifying the numbers first, then looking for a symbol around. Now, it revolves around the symbol... Let's see if I can find something clever to solve this without rewritting everything.

[...]

Alright, I'm glad I've found something quick to do. Not very clean but it's late already. I've basically used an hashmap and stored all part ids, indexed by the symbol position. I've also used for the first time the [ref](https://doc.rust-lang.org/stable/reference/patterns.html#identifier-patterns) keyword! Glad I'm learning.

By the way, I've just realized while typing this I actually did not follow the wording:

`A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.`

I did not check whether the symbol was a `*`, but still got the correct result. Extremely lucky with the input, or bug in the input generation?