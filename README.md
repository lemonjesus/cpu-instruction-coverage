# CPU Instruction Set Coverage Calculator

They say you only use 10% of your brain at any given time. Is that also true of CPUs? With so many old and new x86 instructions, it's possible that some available instructions are never executed during the lifetime of the processor. As cool as that sounds, it's probably quite unlikely, but it's a question I found interesting enough: if I were to count up all of the instructions in, say, the Linux Kernel, would it cover 100% of the x86 ISA?

## What is an Instruction?
I don't know. Do I mean mneumonic? An opcode sequence? Something else? For the sake of simplicity (see below) I decided I'd get "good enough" results by defining a "unique instruction" as an entry in [this enum](https://github.com/icedland/iced/blob/85ae8b3f6d234964d2f0be299f0380880eda548a/src/rust/iced-x86/src/code.rs#L19) of [Iced](https://github.com/icedland/iced) (the instruction decoding engine that's doing the heavy lifting here).

## Coverage Calculator
A small tool I wrote that takes an object file and extracts the number of times each instruction occurs in the `.text` section of it. Once built, run the executable with a target directory as its argument. It will recursively find all of the `.o` files, extract the text section into a temporary file, and count the instructions used. The output is a JSON object between unique instruction code names ([as written in this enum](https://github.com/icedland/iced/blob/85ae8b3f6d234964d2f0be299f0380880eda548a/src/rust/iced-x86/src/code.rs#L19)) to the number of times they occured.

```bash
# you never even have to leave the source directory!
cargo run /path/to/object-files
```

If no directory is given, the current directory is assumed.

## Correctness
I have not run exhaustive tests on this. In fact, I've run almost no tests on it. I've eyeballed the output and said "yeah, that looks about right". This is probably not the most accurate or rigorous method of doing this, but it was the easiest way to get results good enough for my purposes. Any PRs increasing its accuracy are more than welcome.

This also relies on [Iced](https://github.com/icedland/iced) being correct and up to date, but by the looks of it, at the time of writing, it is. Also, forgive me, but Rust is not my first language. It's probably somewhat rough.

## Data and Visualization
In the `viz/` directory, I have the results from five Linux Kernel versions and a build of FFMPEG. There's also a `codes.json` which is just a copy of the [enum I keep talking about](https://github.com/icedland/iced/blob/85ae8b3f6d234964d2f0be299f0380880eda548a/src/rust/iced-x86/src/code.rs#L19) so you can fill in the gaps (as the data only counts instructions that exist).
