# Day 1

[Link to the puzzle for Day 1](https://adventofcode.com/2025/day/1)

Summary of the puzzle:

> ### Part 1:
> You, an Elf, are presented with a safe that has a combination lock. You are given a list of moves (L17, R27 etc).
> 
> These moves don't directly open the safe. The actual combination of the safe is the number of times you land on 0 after performing a move.
>
> The lock starts at position 50.
> 
> ### Part 2:
> The combination also includes the number of times you *pass through* zero


I am currently learning Rust. I haven't programmed anything for about a decade, unless you count bash which I don't.

I feel like I dramatically overcomplicated this!

Since my goal is to learn Rust, I'm going to do some extras that aren't necessary, like pulling the input via HTTPS, simply to learn how these work.

I'm also trying to use match where possible in place of if, since that seems to be good practise although currently I don't 100% understand why.

Here are the interesting bits and what I learned:

```Rust
let client = reqwest::Client::new();
let res = client
    .get(url)
    .header("Cookie", session_token)
    .send()
    .await?;
let body = res.text().await?;
```
There are lots of ways of making get requests, and the reqwest crate seems to be the easiest for simple use cases. I initially wanted to use the blocking method so that I didn't have to deal with an async crate, but I had trouble making it work and it ended up being easier just to use the async example and add Tokio to my project even though I have no idea what Tokio actually does at this point.

Since you need to be logged in, I decided to try just grabbing my session token from my web browser and sending it as a header. That worked. (I was quite surprised it was that simple!)

Extracting the movement direction and number of steps was trivial using substring (which in rust is in its own crate - with Rust I am learning I will have to use a lot of crates to replace functionality that is just present in other languages)

Dealing with the left rotation first

```Rust
match direction {
    "L" => {
        let new_pos = pos - movement_count;
        let corrected_pos = match new_pos {
            x if x < 0 => {
                100 + new_pos
            },
            x if x >= 0 => new_pos,
            _ => panic!()
        };
        pos = corrected_pos;
    },
```

Take the current position and since we're turning counter-clockwise, subtract the number of moves. If this is a positive number, we don't pass zero so we are now at the new position and can continue. If it's a negative number we have gone past 0, so our new position will be 100 minus the new position.

Now the right rotation:

```Rust
"R" => {
    let new_pos = pos + movement_count;
    let corrected_pos = match new_pos {
        x if x <= 99 => new_pos,
        x if x > 100 => {
            new_pos - 100
        },
        100 => 0,
        _ => panic!()
    };
```

Add the number of moves to the position. If it's less than 100, we are now at the new position. If it's equal to 100 we are at zero. If it's more than 100, we need to wrap around, so we just subtract 100 to find our new position.

The final step is just to check if the new position is zero, and if so, increment the password by 1.

I was happy to find that this logic worked first time once I'd wrapped my noodle around it. Except for one gotcha: some of the moves were more than 100 (so the wheel was being turned redundant full revolutions, often a large number).

To deal with that case I did the following before entering my conditional logic:

```Rust
movement_count = movement_count % 100;
```

This divides the number of movements by 100, discards the quotient and keeps the remainder. This has the effect of taking out all the redundant revolutions and just leaving you with the number of movements you need to make.

The code now worked and I moved on to the second part of the puzzle.

Since my code didn't use really any shortcuts to get to the answer, it was pretty simple to count every time the dial rotated through 0 since I had a separate match for this in my conditional logic.

To deal with the number of times it passed through 0 in the redundant turns, I added the following when I dealt with those:

```Rust
password = password + (movement_count / 100);
```

this does the opposite of the operation to remove the redundant rotations. It uses the divisor to count the number of whole rotations, then you just need to add that to the password count.

I still got the wrong result at this point. I wasn't sure what I was doing wrong, so I read the puzzle again, and it suggested if I got stuck I should run my code on the sample inputs. Doing that allowed me to identify when I was incrementing the password incorrectly: there was an edge case where if you START on 0, my code would increment the password even though the rotation did not pass through 0.

All I needed to do was add an `if pos > 0` before incrementing the password. Now my result was correct.