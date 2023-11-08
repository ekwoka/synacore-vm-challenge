# Synacore Challenge

I recently stumbled upon the Synacore challenge, from the creator of Advent of Code.

It is an interesting different way to do some coding challenges. Instead of large word problems, you're given a Binary and some basic instructions for how the Binary is encoded. You then have to implement a VM that can run the binary, to be able to gather keys from within it's contents.

Seems pretty interesting!

## So, obviously, I chose Rust

While I could tackle this easily in TypeScript, I wanted to use a more appropriate Systems-level language, since the goal is implementing a VM, which is a systems level problem.

## The Process

### Humble Beginnings.

The first 2 keys are pretty simple. One is actually, just in the instructions document as a gimme for reading. The second can actually also just be found by opening the binary in vscode, and reading the start of the file. To get it properly, you implement 3 of the operations (noop, print to stdout, and exit). Then the program can print out the characters, that are mostly just the same as in the binary when read as UTF-8 for the first steps, before the program exits.

To get to this point, I mostly ignored the instructions about setting up memory, registers, or the stack. None of those 3 operations (at least at this point) involved the stack before the program terminates, so it wasn't necessary to implement it (yet).

### Some Problems

Knowing that this initial task would only be running code in sequence until termination, ignoring unimplemented operations, I settled on just streaming the bytes from the file and handling the operations as I went.

I did get stuck for a bit too long, not realizing that the bytes with little-endian (ie, the first byte in the code is actually the second byte in the u16) which did waste a bit of time.

Similarly, I did get a bit stuck on how to handle streaming the bytes into u16s without involving collecting the bytes into a Vec. I think I'll still want to do streaming the loading of the program in later versions, so I fought through it and managed to get it working.

```rs
fn read_binary() -> Box<dyn Iterator<Item = u16>> {
    let challenge_bin = File::open("challenge.bin").unwrap();

    Box::new(
        challenge_bin
            .bytes()
            .map(|b| b.unwrap_or(0))
            .into_chunks::<2>()
            .map(u16::from_le_bytes),
    )
}
```

Not exactly earth shattering, but it works. I don't get to use nice iterators in TypeScript, so I wanted to do a lot here. Probably shouldn't have but it was fun! I couldn't figure out how to implement `into_chunks` to provide arrays like the unstable `array_chunks` does, so I used Vec originally. While it worked, it was less than ideal, so I looked more into how the source for `array_chunks` worked, and implemented a simplified version for my use case.

In the process, I also realized my code for getting the u16 from the little endian u8 was already implemented on `u16` and just used the function core provides.

Oh well! Learning to do it myself is still fun, and then going to using the real thing.

This did enough to get me a second key, and give me some experience for moving forward to the next steps.
