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

## Day 4

Pretty chill, I failed last night to solve part 2, and after refactoring this morning, I've tried again a value I thought I've already checked yesterday just in case, and I validated the day.

Once again, when I say I suck sometimes at making a solution for a problem: I originally started to store the copies of each tickets in an hashmap instead of a vec. Why? What's the feature I was needing in that case? None, and a vec was just the right tool for this.

Also, I should test my stuff more frequently. I'm glad I wrote a small unit test here because the feedback of the test gave me instantly some confidence. I'll try to write more testing cases for the following days too.

## Day 5

First part was trivial, second part made my laptop crashed. I was not expecting high volumes of computation to happen already! The mistake is I store data for every seeds, which becomes soon out of control.

Hell yeah. After struggling so much (my brain just stops working when it comes to [sparse signals](https://github.com/usnistgov/ActEV_Scorer/blob/master/lib/sparse_signal.py)), I managed to solve today's challenge by working with ranges instead of numbers. I was afraid/excited [it would take ages to compute](https://www.reddit.com/r/adventofcode/comments/18b8r5x/2023_day_5_part_2rust_when_you_have_32_cores/), because I then may have to use [rayon](https://docs.rs/rayon/latest/rayon/) for the first time, but it ended up yielding instantly the answer. Looking forward for tomorrow!

## Day 6

Surprised by the size of the input! Let's see what surprises it hides...

Seems today was just an easy and fun day: and we won the race! I've learnt about kerning.

## Day 7

First day I'm late. I should stop doing AoC in the morning because it fucks the rest of my day if I fail.

My main mistake for day 1 was implementing features that where not required (I always think I understand the problem but nope). Eventually, I also noticed my model was bad so I rewrote it (twice and still unhappy). For step 2 I was too lazy to do it well and I broke step 1.

The good thing about day 7 is, I wrote a lot of tests because nothing was working properly.

Time to move on day 8, and I'm scared...