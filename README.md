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

### The Real Work Begins

Of course, the prior code was not able to really expand to the future needs. What is needed to get to the next keys is having functional memory space and implementing all of the Operations the binary may indicate.

This took quite a LOT of writing, deleting, rewriting, refactoring, rewriting, deleting, and so on. Doing it wrong, then doing it worse, then doing it okay.

One part that gave some issues is that Rust doesn't have simple mutatable static variables that you can use. I tried a few methods to attempt to keep it as simple as possible, and while they compiled they ended up later not working properly and I went back and forth, using once locks, and functions to retrieve them, and so on. Eventually through many iterations to get it working, then simplifying later, I ended up with a pretty simple little struct to use for Memory and Registers.

```rs
pub struct Memory<const N: usize> {
    memory: RwLock<[u16; N]>,
}

impl<const N: usize> Memory<N> {
    const fn new() -> Memory<N> {
        Self {
            memory: RwLock::new([0; N]),
        }
    }
    pub fn write(&self, address: u16, value: u16) {
        self.memory.write().unwrap()[address as usize] = value
    }
    pub fn read(&self, address: u16) -> u16 {
        self.memory.read().unwrap()[address as usize]
    }
}
```

This can then be used in 15bit and 8 bit variations to the Memory space and Registers respectively. This does also provide a nice thread-safe abstraction. My original intent was not to be concerned with thread-safety, as I think there are other concerns in the architecture spec that would make actual multithreading virtually impossible anyway, but it seemed to near impossible to implement this in a manner that the compiler was okay with that wasn't also thread-safe on a technical level.

The `RwLock` (Read-Write Lock) allows internal mutability, and allows multiple readers but only one writer, as opposed to `Mutex` that is single owner. All this allows the Memory to be instantiated as a static immutable variable, that exposes methods that can mutate inner state.

The Stack was handled similarly, just with a `Vec` instead of an array, and `push` / `pop` methods.

#### 15-bit numbers

Another thing that did cause some issues is the fact that the numbers and math used in the spec of the machine are u15, and even worse, they are special u15, where the highest 8 allowed values are not numbers, but addresses to the registers (and if used in math should yield the value stored in that address, not its own literal value). Combine this with the fact the values should overflow, and it makes for some interesting math.

Rust doesn't have a `u15` struct, but also it wouldn't properly account for the last 8 values thing either, so I'd need to handle this myself. I went back and forth on if I should provide my own special struct to handle these, or using helper functions that could operate on `u16` following the above rules.

I ended up with the former, as it did more to take advantage of how Rust works, with implementing Math traits for my special number struct, and even worked to allow `impl From` to make actually calling operation methods much simpler.

```rs
impl Add<SynacoreValue> for SynacoreValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % 32768)
    }
}
```

This just allows me to do simply `lhs + rhs` when those values are my custom number struct.

#### The Magic of `impl From`

Traits are quite magical, and I think `From` encapsulates some of the coolest behaviors imaginable. At it's simplest, it's just providing a method that can convert from one type to another. So you can do things like `MyNumber::from(5)` and the method takes over to actually do whatever is necessary. But where it becomes most magical, is that when working with a type, and it is being put somewhere that wants another type, you can simply do `5.into()` and Rust uses the type system to understand what it needs to convert it to and what methods to run to do it.

Assuming `From` or `Into` is implemented for that transition, of course.

In my case, what I really wanted to do was be able to have the different Operation functions define the types they accept, and then have the function that handles actually calling the operation simply provide a consistent piece of information (in this case the location of the opcode in memory) and not be concerned with what the function actually accepted.

```rs
/*
add: 9 a b c
  assign into <a> the sum of <b> and <c> (modulo 32768)
 */
fn add(TripleArg(destination, lhs, rhs): TripleArg<Address, SynacoreValue>, position: &mut u16) {
    destination.write((lhs + rhs).into());
    *position += 4;
}
```

Above, the `add` function defines it's arguments as `TripleArg<Address, SynacoreValue>`. This basically says "I want 3 arguments of these types (the final argument types is reused for any unspecified)". That's pretty simple, and not special. The magic comes in that we have the following

```rs
impl<F, S, T> From<&mut u16> for TripleArg<F, S, T>
where
    F: From<u16>,
    S: From<u16>,
    T: From<u16>,
{
    fn from(position: &mut u16) -> Self {
        let mem = &MEMORY;
        Self(
            mem.read(*position + 1).into(),
            mem.read(*position + 2).into(),
            mem.read(*position + 3).into(),
        )
    }
}
```

So here, we define that any TripleArg where all of it's containing types are `From<u16>` can be created from a `u16`. Specifically, if the caller passes in something like `position.into()`, then this logic automatically grabs the 3 values from memory and converts them into the types that the function expects. Now the caller doesn't need to handle this logic itself, or be adjusted to be aware of the number of arguments a function takes, it can just pass in the `position` and the trait handles the rest.

```rs
match self {
    OpCode::Add => add(position.into(), position),
}
```

I had seen this used in Bevy to allow the user to define systems and basically query the world for the data they need. In something like TypeScript, you would need to instead accept the object, or destructure it, and possibly further process it inside the function body, since you can't modify the caller. But in Rust, you can just define the types you want, `impl From`, and the system uses `into` to provide exactly what you want. It's a good kind of magic.

So, I implemented all the OpCodes like that.

This actually results in the main `loop` being sparse.

```rs
let mut position: u16 = 0;

loop {
    OpCode::from(position).execute(&mut position);
}
```

The provided binary actually runs a self check to validate all the OpCodes are implemented properly, and yields the next key (3/8 if you're counting).

It also starts running, and we land ourselves in

### The Foothills
