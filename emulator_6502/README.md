## Dev notes

- put in some debugging tools. Both debugger and print statements.
- the flags are all messed up. Looks like they're being set wrong from the beginning. This may be independent of other issues.
- lda zpg op 'a5' is failing to load a properly. Seems like it's not reading the argument correctly. The op is loading a correctly, but with the wrong value.


## Build and Run

Step one: get `cargo` installed on your machine.

Build and run tests: `cargo test`

Run example binary: `cargo run infinite_loop.bin`


For learning purposes, I want to eventually build an assembler, disassembler and a C compiler. The assembler at the very least should be very easy.

Maybe also build out an emulated system similar to a NES, but a lot of this is out of my current ability. I would learn a lot about how the busses work, display, graphics and sound, as well as storage. I'm not sure how the NES storage system worked, probably all RAM volatile, non-volatile, and ROM (I'm guessing).


## Description

This is an emulation of a 6502 CPU and a compatible MMU. The CPU and MMU are implmented/tested for documented specifications, as well as undocumented behavior (to some extent).

This CPU was an 8-bit microprocessor (8 bit data, with 16 bit addresses) used in the Atari, Apple II, Nintendo NES, Commodore 64, and others. It was simmilar to the Zilog Z80 and other 8-bit microcontrollers still used commonly today.

I used these sources.
- http://www.6502.org/tutorials/6502opcodes.html#ASL
- https://github.com/docmarionum1/py65emu
- http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf


I believe that the 6502 CPU is little endian, and so is the z80 and the intel 8080. So to some extent all of these popular microprocessors of the 1970s were similar, and modern desktop and server hardware is derived from a common origin to some extent.


I used this emulator project to learn about CPUs, emulators, and the rust language. I had little experience in any of those topics prior to this project.


## Thoughts and reactions during the project (on the rust language)

### lifetimes, ownership and metaprogramming potential

I feel like I started to understand the lifetimes/ownership system in rust during this project, and why it would be useful. Following typical rust style, ie Constructors pass out ownership, class methods borrow parameters, etc. is very straightforward. None of the problems I was promised actually... On the occasion that I want to make a defensive copy it is easy to derive clone.

That said I love the default behavior with all the derive macros. In fact this one macro seems to make the language VERY high level. Like higher even than python. I know right... I think, and I may be totally wrong on this that the derive capability is inspired by haskell, which is another interesting language which I have only briefly tried, but I am finding that I like rust much more.

 I can tell that the metaprogramming capabilities available in the macros would be very powerful if I needed to build abstractions for some specific reason. And I am pretty ignorant of metaprogramming, but I already feel that the readability of the compiler error messages puts C++ to shame. I have much less concern metaprogramming with confidence with rust over C++, but whether that observation plays out under heavy use remains to be seen. Metaprogramming and macros aside, there are definitely a lot of features that will allow room to grow with rust.


### stricness of the  compiler
I can see that rust's "stricter" rules are useful, for memory saftey, and perhaps for concurrency, though I feel like I still do not fully understand that. Still, I feel like I am making significant progress in my usability of rust when compared with past projects.

Also I do appreciate the strictness of the compiler (relative to some other languages) for general checking of correctness to some extent (things like mutability for example). I'm sure with some static checking analysis C++ could get near to rust's level, but there is something to be said for strictness by default. Also there is something to be said for "the easiest way to get it right" and I don't know, maybe rust is better than alternatives once the language model is fully appreciated.

Its not *just* memory safety. It's also prevents things like having multiple references to the same value while one of them is mutable...... ya its strict.. (now admittedly I think some use of the keyword *restrict* in newer versions of C may offer similar behavior, but this is rarely used in practice.) This basically forces you to localize the places where you do mutation to smaller parts of your code. This gives less chance that things are being mutated in two different places (potentially in library code that you didn't personally write) without the programmer knowing it. The programmer can be reasonably sure that *if he has a mutable reference to something* then *code he didn't write isn't mutating that something at the same time*. Very powerful.... 

edit: ok this isn't really true. This was my original thought, but actually instead of working with various types of references that are checked by the compiler you can basically just roll your own raw pointers. For example you can just keep track of various usize indexes into your array of Elems instead of working with references to elems. The compiler will not catch that you have a simple index value in one place equal to the value of another index in another place. This homemade raw pointer solution is naturally found very quickly as a work around to the borrow checker........... hmmm... Now you can still benifit from rust's strict checking, but you ALSO have to make sure that you aren't tracking your own raw pointers. Even with your own handmade raw pointers it is actually still better than completely raw pointers, because you still have to localize the returned reference when you "figuratively dereference" that homemade raw pointer. Its just that you can hold the homemade raw pointers wherever you want, but yes the USE of those homemade raw pointers still has some restrictions placed on it by the rust compiler.

I feel like I am not able to fully express myself in rust, but I am starting to appreciate how the builtin invariance checking in the compiler is producing more robust programs (to the extent that it is able). Nevertheless, I can see why for some reference heavy code (aka "unsafe") someone would still choose C/C++ over rust. Especially if the lack of productivity from switching to rust is considered excessive. (Though I do know that there are ways of using RefCell, I'm not sure exactly how that works.)

### final thoughts

Also comparing with my experiences with C/C++ vs rust, it does seem that the build system for rust is much better. I much prefer the simple "cargo build / cargo run / cargo test" than the unclear logic in makefiles, and other config. My understanding (probably very flawed) is that cargo does primarily static linking (by default) (with dynamic linking of core OS libraries used in the rust runtime, and optionally external C libraries) which produces relatively portable binaries between OSes and containers of the same version. That could potentially make native deployment as easy as java (although with docker and other containers, native vs VM deployment is essentially at parity). My also very limited understanding of go language is that this is the default for golang building/deployment as well.

My dislike of java (what I currently use at work) grows as my love for rust grows. I want to say on the record though that I never liked java. That is very important to mention early and often....

As a small but interesting example of how rust was designed with long term, small "usability gains" in mind. Look at how the keyword "pub" was shortened from "public" and "fn" instead of "function". Are any of the built in keyword literals longer than 4 characters? I'm pretty sure the answer is no. That shows the level of long term planning and impact that has gone into this language. Remember most popular languages were either not expected to ever become popular (C, Python), or had initial development that was very quick and minimally planned (java, javascript, c++) and grew in an unplanned set of additions. 

Its like the difference between a planned city like Manhattan or Chicago that has all right angles for streets and avenues, vs an organic city like boston with tons of windy roads. Its not a "deal breaker" but in the long term the small bit of initial planning, adds up, producing a very coherent outcome.