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

## Day 8

Step 1 was fun and easy, and I'm glad I was able to correctly use lifetimes for once. Making progress!

Step 2 seems not that bad... And I was wrong again. After implementing my first solution, I understood it's one of these problems where you have to be smart, because the number of iterations is way to high. The problem statement was also suggesting it (`It's going to take significantly more steps to escape!`)...

I was pretty much out of ideas so I went on the subreddit to check if my guts were right, and according to the memes[[1]](https://www.reddit.com/r/adventofcode/comments/18di483/2023_day_8/)[[2]](https://www.reddit.com/r/adventofcode/comments/18dhks8/2023_day_8_part_2_me_reading_part_2/)[[3]](https://www.reddit.com/r/adventofcode/comments/18dg2v6/2023_day_8_if_i_learned_anything/) it was. I was spoiled about the solution (computing the least common denominator), so I implemented it. I was not very convinced it would work because I did not see why the paths patterns would be cyclic.

Been reading [this (a guy asking the same question)](https://www.reddit.com/r/adventofcode/comments/18dg1hw/2023_day_8_part_2_about_the_correctness_of_a/), and while looking at [this guy's visualization](https://www.reddit.com/r/adventofcode/comments/18did3d/2023_day_8_part_1_my_input_maze_plotted_using/), I figured out I completely missed the point here. I guess the challenge was about solving the puzzle, that means finding what algorithm to use to complete the challenge. And for that, inspecting the data would be the starting point, which I completely skipped because [I was late](#day-7).

Tomorrow is Saturday, I have no excuse.

## Day 9

I may be over-excited about this but I'm glad I was able to understand how things work for step 2.
By reversing the order of the sequence and running the same algorithm, we can compute the antecedent of the sequence.

Because it's Saturday I was afraid the challenge would be super hard, and it was very accessible. It also feels good, it makes me start my day happy.

I find it easier and easier to write tests (the examples help a lot, ngl), which I'm happy about. `step1()` and `step2()` could be documented just a
little, because the code can be hard to understand at first glance, but I think it's fine like that.

Looking forward for tomorrow!

## Day 10

Tonight I'm devastated. I don't know if it is because of the exhaustment or the bottomless brunch (in
any case I should not drink that much) but day 10 was PAINFULL.

The way I modeled the grid sucks: a cell doesn't know its own position - so it does not know shit about its neighbors -, I've added useful enums discribing the kind of the pipes (I'm 100% I can refactor and get rid of `Direction` too), you name it. I'm just pissed I took so long to get something working. I tried to write nice code, following the Rust way, it ended up a nightmare.

On the bright side, I will from now work my model on paper before going straight to coding, in hopes that I can spot misconceptions before writing them. I like writing tests for small units of code so I'll keep doing it. And that is also ten days of AoC in a row.

I wish I will redo this exercise in the future, well rested, and see if I can make what I had in mind at first.
