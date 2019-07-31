This is a 2 pass compiler. Sub routines can be defined after they are used. Labels are not case sensitive.

The target system is a MOS6502.

Sources:
 - https://www.masswerk.at/6502/6502_instruction_set.html
 - https://skilldrick.github.io/easy6502

### Build
`cargo build` in this directory

### Test
`cargo test` in this directory

### Run
`cargo run asm_code/snake.6502asm out.bin` in this directory