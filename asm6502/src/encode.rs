use regex::{Regex, Captures};


macro_rules! return_match {
    ( $pattern:expr, $opcode:expr, $line:expr ) => {
        {
            let re = Regex::new($pattern).unwrap();
            if re.is_match($line) {
                let caps =  re.captures_iter($line).collect::<Vec<Captures>>();
                return ($opcode, caps[0][1].to_string());
            }
        }
    };
}


// takes a line with an instruction and returns opcode, args string, arg_length,
// uses a massive branching statement
pub fn get_opcode_and_arguments(line: String, line_number: u16) -> (u8, String) {

    // all the impls and As
    // TODO this block needs to be fixed to use the macro
    if line      == "brk" { return (0x00, "".to_string()); }
    else if line == "rti" { return (0x40, "".to_string()); }
    else if line == "rts" { return (0x60, "".to_string()); }
    else if line == "php" { return (0x08, "".to_string()); }
    else if line == "clc" { return (0x18, "".to_string()); }
    else if line == "plp" { return (0x28, "".to_string()); }
    else if line == "sec" { return (0x38, "".to_string()); }
    else if line == "pha" { return (0x48, "".to_string()); }
    else if line == "cli" { return (0x58, "".to_string()); }
    else if line == "pla" { return (0x68, "".to_string()); }
    else if line == "sei" { return (0x78, "".to_string()); }
    else if line == "dey" { return (0x88, "".to_string()); }
    else if line == "tya" { return (0x98, "".to_string()); }
    else if line == "tay" { return (0xa8, "".to_string()); }
    else if line == "clv" { return (0xb8, "".to_string()); }
    else if line == "iny" { return (0xc8, "".to_string()); }
    else if line == "cld" { return (0xd8, "".to_string()); }
    else if line == "inx" { return (0xe8, "".to_string()); }
    else if line == "sed" { return (0xf8, "".to_string()); }
    else if line == "txa" { return (0x8a, "".to_string()); }
    else if line == "txs" { return (0x9a, "".to_string()); }
    else if line == "tax" { return (0xaa, "".to_string()); }
    else if line == "tsx" { return (0xba, "".to_string()); }
    else if line == "dex" { return (0xca, "".to_string()); }
    else if line == "nop" { return (0xea, "".to_string()); }

    else if line == "asl" { return (0x0a, "".to_string()); }
    else if line == "rol" { return (0x2a, "".to_string()); }
    else if line == "lsr" { return (0x4a, "".to_string()); }
    else if line == "ror" { return (0x6a, "".to_string()); }


    // rels (labeled)
    return_match!(r"^bpl\s+([a-zA-Z]\w*)$", 0x10, &line);
    return_match!(r"^bmi\s+([a-zA-Z]\w*)$", 0x30, &line);
    return_match!(r"^bvc\s+([a-zA-Z]\w*)$", 0x50, &line);
    return_match!(r"^bvs\s+([a-zA-Z]\w*)$", 0x70, &line);
    return_match!(r"^bcc\s+([a-zA-Z]\w*)$", 0x90, &line);
    return_match!(r"^bcs\s+([a-zA-Z]\w*)$", 0xb0, &line);
    return_match!(r"^bne\s+([a-zA-Z]\w*)$", 0xd0, &line);
    return_match!(r"^beq\s+([a-zA-Z]\w*)$", 0xf0, &line);

    // rels unlabeled (relative offset)
    return_match!(r"^bpl\s+\$([0-9a-f]{1,2})$", 0x10, &line);
    return_match!(r"^bmi\s+\$([0-9a-f]{1,2})$", 0x30, &line);
    return_match!(r"^bvc\s+\$([0-9a-f]{1,2})$", 0x50, &line);
    return_match!(r"^bvs\s+\$([0-9a-f]{1,2})$", 0x70, &line);
    return_match!(r"^bcc\s+\$([0-9a-f]{1,2})$", 0x90, &line);
    return_match!(r"^bcs\s+\$([0-9a-f]{1,2})$", 0xb0, &line);
    return_match!(r"^bne\s+\$([0-9a-f]{1,2})$", 0xd0, &line);
    return_match!(r"^beq\s+\$([0-9a-f]{1,2})$", 0xf0, &line);

    // immediates
    return_match!(r"^ldy\s+#\$?([0-9a-f]{1,2})", 0xa0, &line);
    return_match!(r"^ldx\s+#\$?([0-9a-f]{1,2})", 0xa2, &line);
    return_match!(r"^cpy\s+#\$?([0-9a-f]{1,2})", 0xc0, &line);
    return_match!(r"^cpx\s+#\$?([0-9a-f]{1,2})", 0xe0, &line);
    return_match!(r"^ora\s+#\$?([0-9a-f]{1,2})", 0x09, &line);
    return_match!(r"^and\s+#\$?([0-9a-f]{1,2})", 0x29, &line);
    return_match!(r"^eor\s+#\$?([0-9a-f]{1,2})", 0x49, &line);
    return_match!(r"^adc\s+#\$?([0-9a-f]{1,2})", 0x69, &line);
    return_match!(r"^lda\s+#\$?([0-9a-f]{1,2})", 0xa9, &line);
    return_match!(r"^cmp\s+#\$?([0-9a-f]{1,2})", 0xc9, &line);
    return_match!(r"^sbc\s+#\$?([0-9a-f]{1,2})", 0xe9, &line);

    // indirect
    return_match!(r"^ora\s+\(\$([0-9a-f]{1,2}),x\)$", 0x01, &line);
    return_match!(r"^ora\s+\(\$([0-9a-f]{1,2})\),y", 0x11, &line);
    return_match!(r"^and\s+\(\$([0-9a-f]{1,2}),x\)$", 0x21, &line);
    return_match!(r"^and\s+\(\$([0-9a-f]{1,2})\),y", 0x31, &line);
    return_match!(r"^eor\s+\(\$([0-9a-f]{1,2}),x\)$", 0x41, &line);
    return_match!(r"^eor\s+\(\$([0-9a-f]{1,2})\),y", 0x51, &line);
    return_match!(r"^adc\s+\(\$([0-9a-f]{1,2}),x\)$", 0x61, &line);
    return_match!(r"^adc\s+\(\$([0-9a-f]{1,2})\),y", 0x71, &line);
    return_match!(r"^sta\s+\(\$([0-9a-f]{1,2}),x\)$", 0x81, &line);
    return_match!(r"^sta\s+\(\$([0-9a-f]{2})\),y", 0x91, &line);
    return_match!(r"^lda\s+\(\$([0-9a-f]{1,2}),x\)$", 0xa1, &line);
    return_match!(r"^lda\s+\(\$([0-9a-f]{1,2})\),y", 0xb1, &line);
    return_match!(r"^cmp\s+\(\$([0-9a-f]{1,2}),x\)$", 0xc1, &line);
    return_match!(r"^cmp\s+\(\$([0-9a-f]{1,2})\),y", 0xd1, &line);
    return_match!(r"^sbc\s+\(\$([0-9a-f]{1,2}),x\)$", 0xe1, &line);
    return_match!(r"^sbc\s+\(\$([0-9a-f]{1,2})\),y", 0xf1, &line);

    return_match!(r"^jmp\s+\(\$([0-9a-f]{1,4})\)$",   0x6c, &line);

    // zpgs
    return_match!(r"^ora\s+\$([0-9a-f]{1,2})$",     0x05, &line);
    return_match!(r"^ora\s+\$([0-9a-f]{1,2}),x$",   0x15, &line);
    return_match!(r"^and\s+\$([0-9a-f]{1,2})$",     0x25, &line);
    return_match!(r"^and\s+\$([0-9a-f]{1,2}),x$",  0x35, &line);
    return_match!(r"^eor\s+\$([0-9a-f]{1,2})$",    0x45, &line);
    return_match!(r"^eor\s+\$([0-9a-f]{1,2}),x$",  0x55, &line);
    return_match!(r"^adc\s+\$([0-9a-f]{1,2})$",    0x65, &line);
    return_match!(r"^adc\s+\$([0-9a-f]{1,2}),x$",  0x75, &line);
    return_match!(r"^sta\s+\$([0-9a-f]{1,2})$",    0x85, &line);
    return_match!(r"^sta\s+\$([0-9a-f]{1,2}),x$",  0x95, &line);
    return_match!(r"^lda\s+\$([0-9a-f]{1,2})$",    0xa5, &line);
    return_match!(r"^lda\s+\$([0-9a-f]{1,2}),x$",  0xb5, &line);
    return_match!(r"^cmp\s+\$([0-9a-f]{1,2})$",    0xc5, &line);
    return_match!(r"^cmp\s+\$([0-9a-f]{1,2}),x$",  0xd5, &line);
    return_match!(r"^sbc\s+\$([0-9a-f]{1,2})$",    0xe5, &line);
    return_match!(r"^sbc\s+\$([0-9a-f]{1,2}),x$",  0xf5, &line);
    return_match!(r"^asl\s+\$([0-9a-f]{1,2})$",    0x06, &line);
    return_match!(r"^asl\s+\$([0-9a-f]{1,2}),x$",  0x16, &line);
    return_match!(r"^rol\s+\$([0-9a-f]{1,2})$",    0x26, &line);
    return_match!(r"^rol\s+\$([0-9a-f]{1,2}),x$",  0x36, &line);
    return_match!(r"^lsr\s+\$([0-9a-f]{1,2})$",    0x46, &line);
    return_match!(r"^lsr\s+\$([0-9a-f]{1,2}),x$",  0x56, &line);
    return_match!(r"^ror\s+\$([0-9a-f]{1,2})$",    0x66, &line);
    return_match!(r"^ror\s+\$([0-9a-f]{1,2}),x$",  0x76, &line);
    return_match!(r"^stx\s+\$([0-9a-f]{1,2})$",    0x86, &line);
    return_match!(r"^stx\s+\$([0-9a-f]{1,2}),y$",  0x96, &line);
    return_match!(r"^ldx\s+\$([0-9a-f]{1,2})$",    0xa6, &line);
    return_match!(r"^ldx\s+\$([0-9a-f]{1,2}),y$",  0xb6, &line);
    return_match!(r"^dec\s+\$([0-9a-f]{1,2})$",    0xc6, &line);
    return_match!(r"^dec\s+\$([0-9a-f]{1,2}),x$",  0xd6, &line);
    return_match!(r"^inc\s+\$([0-9a-f]{1,2})$",    0xe6, &line);
    return_match!(r"^inc\s+\$([0-9a-f]{1,2}),x$",  0xf6, &line);
    return_match!(r"^sty\s+\$([0-9a-f]{1,2})$",    0x84, &line);
    return_match!(r"^sty\s+\$([0-9a-f]{1,2}),x$",  0x94, &line);
    return_match!(r"^ldy\s+\$([0-9a-f]{1,2})$",    0xa4, &line);
    return_match!(r"^ldy\s+\$([0-9a-f]{1,2}),x$",  0xb4, &line);
    return_match!(r"^bit\s+\$([0-9a-f]{1,2})$",    0x24, &line);
    return_match!(r"^cpy\s+\$([0-9a-f]{1,2})$",    0xc4, &line);
    return_match!(r"^cpx\s+\$([0-9a-f]{1,2})$",    0xe4, &line);
    
    // absolutes
    return_match!(r"^ora\s+\$([0-9a-f]{1,4})$",    0x0d, &line);
    return_match!(r"^ora\s+\$([0-9a-f]{1,4}),x$",  0x1d, &line);
    return_match!(r"^and\s+\$([0-9a-f]{1,4})$",    0x2d, &line);
    return_match!(r"^and\s+\$([0-9a-f]{1,4}),x$",  0x3d, &line);
    return_match!(r"^eor\s+\$([0-9a-f]{1,4})$",    0x4d, &line);
    return_match!(r"^eor\s+\$([0-9a-f]{1,4}),x$",  0x5d, &line);
    return_match!(r"^adc\s+\$([0-9a-f]{1,4})$",    0x6d, &line);
    return_match!(r"^adc\s+\$([0-9a-f]{1,4}),x$",  0x7d, &line);
    return_match!(r"^sta\s+\$([0-9a-f]{1,4})$",    0x8d, &line);
    return_match!(r"^sta\s+\$([0-9a-f]{1,4}),x$",  0x9d, &line);
    return_match!(r"^lda\s+\$([0-9a-f]{1,4})$",    0xad, &line);
    return_match!(r"^lda\s+\$([0-9a-f]{1,4}),x$",  0xbd, &line);
    return_match!(r"^cmp\s+\$([0-9a-f]{1,4})$",    0xcd, &line);
    return_match!(r"^cmp\s+\$([0-9a-f]{1,4}),x$",  0xdd, &line);
    return_match!(r"^sbc\s+\$([0-9a-f]{1,4})$",    0xed, &line);
    return_match!(r"^sbc\s+\$([0-9a-f]{1,4}),x$",  0xfd, &line);
    return_match!(r"^asl\s+\$([0-9a-f]{1,4})$",    0x0e, &line);
    return_match!(r"^asl\s+\$([0-9a-f]{1,4}),x$",  0x1e, &line);
    return_match!(r"^rol\s+\$([0-9a-f]{1,4})$",    0x2e, &line);
    return_match!(r"^rol\s+\$([0-9a-f]{1,4}),x$",  0x3e, &line);
    return_match!(r"^lsr\s+\$([0-9a-f]{1,4})$",    0x4e, &line);
    return_match!(r"^lsr\s+\$([0-9a-f]{1,4}),x$",  0x5e, &line);
    return_match!(r"^ror\s+\$([0-9a-f]{1,4})$",    0x6e, &line);
    return_match!(r"^ror\s+\$([0-9a-f]{1,4}),x$",  0x7e, &line);
    return_match!(r"^stx\s+\$([0-9a-f]{1,4})$",    0x8e, &line);
    return_match!(r"^ldx\s+\$([0-9a-f]{1,4})$",    0x9e, &line);
    return_match!(r"^ldx\s+\$([0-9a-f]{1,4}),y$",  0xae, &line);
    return_match!(r"^dec\s+\$([0-9a-f]{1,4})$",    0xbe, &line);
    return_match!(r"^dec\s+\$([0-9a-f]{1,4}),x$",  0xce, &line);
    return_match!(r"^inc\s+\$([0-9a-f]{1,4})$",    0xde, &line);
    return_match!(r"^inc\s+\$([0-9a-f]{1,4}),x$",  0xee, &line);
    return_match!(r"^sty\s+\$([0-9a-f]{1,4})$",    0xfe, &line);
    return_match!(r"^ldy\s+\$([0-9a-f]{1,4})$",    0xac, &line);
    return_match!(r"^ldy\s+\$([0-9a-f]{1,4}),x$",  0xbc, &line);
    return_match!(r"^bit\s+\$([0-9a-f]{1,4})$",    0x2c, &line);
    return_match!(r"^cpy\s+\$([0-9a-f]{1,4})$",    0xcc, &line);
    return_match!(r"^cpx\s+\$([0-9a-f]{1,4})$",    0xec, &line);

    return_match!(r"^jmp\s+\$([0-9a-f]{1,4})$",    0x6c, &line);
    return_match!(r"^jmp\s+([a-zA-Z]\w*)$",        0x6c, &line);

    return_match!(r"^jsr\s+\$([0-9a-f]{1,4})$",    0x20, &line);
    return_match!(r"^jsr\s+([a-zA-Z]\w*)$",        0x20, &line);



    // If the function hasn't returned by now, there is a problem
    panic!("the line {}, \"{}\" was not a valid instruction.", line_number, line);
}