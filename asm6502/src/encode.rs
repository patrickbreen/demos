use regex::{Regex, Captures};

pub fn get_opcode_and_arguments(line: String, line_number: u16, compiled_patterns: &Vec<(Regex, u8)>) -> (u8, String) {

    for (re, opcode) in compiled_patterns{
        if re.is_match(&line) {
            let caps =  re.captures(&line).unwrap();
            let group1 = caps.get(1).map_or("", |m| m.as_str());
            return (*opcode, group1.to_string());
        }
    }
    // If the function hasn't returned by now, there is a problem
    panic!("the line {}, \"{}\" was not a valid instruction.", line_number, line);
}


// takes a line with an instruction and returns opcode, args string, arg_length,
// uses a massive branching statement
pub fn compile_patterns() -> Vec<(Regex, u8)> {

    let mut patterns: Vec<(&str, u8)> = Vec::new();

    // TODO - track and return if rel label or absolute label detected

    // all the impls and As
    patterns.push((r"^brk$", 0x00));
    patterns.push((r"^rti$", 0x40));
    patterns.push((r"^rts$", 0x60));
    patterns.push((r"^php$", 0x08));
    patterns.push((r"^clc$", 0x18));
    patterns.push((r"^plp$", 0x28));
    patterns.push((r"^sec$", 0x38));
    patterns.push((r"^pha$", 0x48));
    patterns.push((r"^cli$", 0x58));
    patterns.push((r"^pla$", 0x68));
    patterns.push((r"^sei$", 0x78));
    patterns.push((r"^dey$", 0x88));
    patterns.push((r"^tya$", 0x98));
    patterns.push((r"^tay$", 0xa8));
    patterns.push((r"^clv$", 0xb8));
    patterns.push((r"^iny$", 0xc8));
    patterns.push((r"^cld$", 0xd8));
    patterns.push((r"^inx$", 0xe8));
    patterns.push((r"^sed$", 0xf8));
    patterns.push((r"^txa$", 0x8a));
    patterns.push((r"^txs$", 0x9a));
    patterns.push((r"^tax$", 0xaa));
    patterns.push((r"^tsx$", 0xba));
    patterns.push((r"^dex$", 0xca));
    patterns.push((r"^nop$", 0xea));

    patterns.push((r"^asl$", 0x0a));
    patterns.push((r"^rol$", 0x2a));
    patterns.push((r"^lsr$", 0x4a));
    patterns.push((r"^ror$", 0x6a));


    // rels (relative label)
    patterns.push((r"^bpl\s+([a-zA-Z]\w*)$", 0x10));
    patterns.push((r"^bmi\s+([a-zA-Z]\w*)$", 0x30));
    patterns.push((r"^bvc\s+([a-zA-Z]\w*)$", 0x50));
    patterns.push((r"^bvs\s+([a-zA-Z]\w*)$", 0x70));
    patterns.push((r"^bcc\s+([a-zA-Z]\w*)$", 0x90));
    patterns.push((r"^bcs\s+([a-zA-Z]\w*)$", 0xb0));
    patterns.push((r"^bne\s+([a-zA-Z]\w*)$", 0xd0));
    patterns.push((r"^beq\s+([a-zA-Z]\w*)$", 0xf0));

    // rels unlabeled (relative offset)
    patterns.push((r"^bpl\s+\$([0-9a-f]{1,2})$", 0x10));
    patterns.push((r"^bmi\s+\$([0-9a-f]{1,2})$", 0x30));
    patterns.push((r"^bvc\s+\$([0-9a-f]{1,2})$", 0x50));
    patterns.push((r"^bvs\s+\$([0-9a-f]{1,2})$", 0x70));
    patterns.push((r"^bcc\s+\$([0-9a-f]{1,2})$", 0x90));
    patterns.push((r"^bcs\s+\$([0-9a-f]{1,2})$", 0xb0));
    patterns.push((r"^bne\s+\$([0-9a-f]{1,2})$", 0xd0));
    patterns.push((r"^beq\s+\$([0-9a-f]{1,2})$", 0xf0));

    // immediates
    patterns.push((r"^ldy\s+#\$?([0-9a-f]{1,2})", 0xa0));
    patterns.push((r"^ldx\s+#\$?([0-9a-f]{1,2})", 0xa2));
    patterns.push((r"^cpy\s+#\$?([0-9a-f]{1,2})", 0xc0));
    patterns.push((r"^cpx\s+#\$?([0-9a-f]{1,2})", 0xe0));
    patterns.push((r"^ora\s+#\$?([0-9a-f]{1,2})", 0x09));
    patterns.push((r"^and\s+#\$?([0-9a-f]{1,2})", 0x29));
    patterns.push((r"^eor\s+#\$?([0-9a-f]{1,2})", 0x49));
    patterns.push((r"^adc\s+#\$?([0-9a-f]{1,2})", 0x69));
    patterns.push((r"^lda\s+#\$?([0-9a-f]{1,2})", 0xa9));
    patterns.push((r"^cmp\s+#\$?([0-9a-f]{1,2})", 0xc9));
    patterns.push((r"^sbc\s+#\$?([0-9a-f]{1,2})", 0xe9));

    // indirect
    patterns.push((r"^ora\s+\(\$([0-9a-f]{1,2}),x\)$", 0x01));
    patterns.push((r"^ora\s+\(\$([0-9a-f]{1,2})\),y", 0x11));
    patterns.push((r"^and\s+\(\$([0-9a-f]{1,2}),x\)$", 0x21));
    patterns.push((r"^and\s+\(\$([0-9a-f]{1,2})\),y", 0x31));
    patterns.push((r"^eor\s+\(\$([0-9a-f]{1,2}),x\)$", 0x41));
    patterns.push((r"^eor\s+\(\$([0-9a-f]{1,2})\),y", 0x51));
    patterns.push((r"^adc\s+\(\$([0-9a-f]{1,2}),x\)$", 0x61));
    patterns.push((r"^adc\s+\(\$([0-9a-f]{1,2})\),y", 0x71));
    patterns.push((r"^sta\s+\(\$([0-9a-f]{1,2}),x\)$", 0x81));
    patterns.push((r"^sta\s+\(\$([0-9a-f]{2})\),y", 0x91));
    patterns.push((r"^lda\s+\(\$([0-9a-f]{1,2}),x\)$", 0xa1));
    patterns.push((r"^lda\s+\(\$([0-9a-f]{1,2})\),y", 0xb1));
    patterns.push((r"^cmp\s+\(\$([0-9a-f]{1,2}),x\)$", 0xc1));
    patterns.push((r"^cmp\s+\(\$([0-9a-f]{1,2})\),y", 0xd1));
    patterns.push((r"^sbc\s+\(\$([0-9a-f]{1,2}),x\)$", 0xe1));
    patterns.push((r"^sbc\s+\(\$([0-9a-f]{1,2})\),y", 0xf1));

    patterns.push((r"^jmp\s+\(\$([0-9a-f]{1,4})\)$",   0x6c));

    // zpgs
    patterns.push((r"^ora\s+\$([0-9a-f]{1,2})$",     0x05));
    patterns.push((r"^ora\s+\$([0-9a-f]{1,2}),x$",   0x15));
    patterns.push((r"^and\s+\$([0-9a-f]{1,2})$",     0x25));
    patterns.push((r"^and\s+\$([0-9a-f]{1,2}),x$",  0x35));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,2})$",    0x45));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,2}),x$",  0x55));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,2})$",    0x65));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,2}),x$",  0x75));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,2})$",    0x85));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,2}),x$",  0x95));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,2})$",    0xa5));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,2}),x$",  0xb5));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,2})$",    0xc5));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,2}),x$",  0xd5));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,2})$",    0xe5));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,2}),x$",  0xf5));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,2})$",    0x06));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,2}),x$",  0x16));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,2})$",    0x26));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,2}),x$",  0x36));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,2})$",    0x46));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,2}),x$",  0x56));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,2})$",    0x66));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,2}),x$",  0x76));
    patterns.push((r"^stx\s+\$([0-9a-f]{1,2})$",    0x86));
    patterns.push((r"^stx\s+\$([0-9a-f]{1,2}),y$",  0x96));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,2})$",    0xa6));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,2}),y$",  0xb6));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,2})$",    0xc6));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,2}),x$",  0xd6));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,2})$",    0xe6));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,2}),x$",  0xf6));
    patterns.push((r"^sty\s+\$([0-9a-f]{1,2})$",    0x84));
    patterns.push((r"^sty\s+\$([0-9a-f]{1,2}),x$",  0x94));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,2})$",    0xa4));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,2}),x$",  0xb4));
    patterns.push((r"^bit\s+\$([0-9a-f]{1,2})$",    0x24));
    patterns.push((r"^cpy\s+\$([0-9a-f]{1,2})$",    0xc4));
    patterns.push((r"^cpx\s+\$([0-9a-f]{1,2})$",    0xe4));
    
    // absolutes
    patterns.push((r"^ora\s+\$([0-9a-f]{1,4})$",    0x0d));
    patterns.push((r"^ora\s+\$([0-9a-f]{1,4}),x$",  0x1d));
    patterns.push((r"^and\s+\$([0-9a-f]{1,4})$",    0x2d));
    patterns.push((r"^and\s+\$([0-9a-f]{1,4}),x$",  0x3d));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,4})$",    0x4d));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,4}),x$",  0x5d));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,4})$",    0x6d));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,4}),x$",  0x7d));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,4})$",    0x8d));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,4}),x$",  0x9d));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,4})$",    0xad));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,4}),x$",  0xbd));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,4})$",    0xcd));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,4}),x$",  0xdd));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,4})$",    0xed));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,4}),x$",  0xfd));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,4})$",    0x0e));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,4}),x$",  0x1e));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,4})$",    0x2e));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,4}),x$",  0x3e));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,4})$",    0x4e));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,4}),x$",  0x5e));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,4})$",    0x6e));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,4}),x$",  0x7e));
    patterns.push((r"^stx\s+\$([0-9a-f]{1,4})$",    0x8e));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,4})$",    0x9e));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,4}),y$",  0xae));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,4})$",    0xbe));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,4}),x$",  0xce));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,4})$",    0xde));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,4}),x$",  0xee));
    patterns.push((r"^sty\s+\$([0-9a-f]{1,4})$",    0xfe));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,4})$",    0xac));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,4}),x$",  0xbc));
    patterns.push((r"^bit\s+\$([0-9a-f]{1,4})$",    0x2c));
    patterns.push((r"^cpy\s+\$([0-9a-f]{1,4})$",    0xcc));
    patterns.push((r"^cpx\s+\$([0-9a-f]{1,4})$",    0xec));

    patterns.push((r"^jmp\s+\$([0-9a-f]{1,4})$",    0x6c));
    patterns.push((r"^jmp\s+([a-zA-Z]\w*)$",        0x6c));

    patterns.push((r"^jsr\s+\$([0-9a-f]{1,4})$",    0x20));
    patterns.push((r"^jsr\s+([a-zA-Z]\w*)$",        0x20));

    let mut compiled_patterns = Vec::new();
    for (pattern, opcode) in patterns {
        let re = Regex::new(pattern).unwrap();
        compiled_patterns.push((re, opcode));
    }

    return compiled_patterns;
}

