use regex::{Regex, Captures};

pub fn get_opcode_and_arguments(line: String,
                                line_number: u16,
                                compiled_patterns: &Vec<(Regex, u8, &'static str)>) -> 
                                (u8, String, &'static str) {

    for (re, opcode, instr_type) in compiled_patterns{
        if re.is_match(&line) {
            let caps =  re.captures(&line).unwrap();
            let group1 = caps.get(1).map_or("", |m| m.as_str());
            return (*opcode, group1.to_string(), instr_type);
        }
    }
    // If the function hasn't returned by now, there is a problem
    panic!("the line {}, \"{}\" was not a valid instruction.", line_number, line);
}


pub fn compile_patterns() -> Vec<(Regex, u8, &'static str)> {

    let mut patterns: Vec<(&str, u8, &'static str)> = Vec::new();

    // all the impls and As
    patterns.push((r"^brk$", 0x00, "no_arg"));
    patterns.push((r"^rti$", 0x40, "no_arg"));
    patterns.push((r"^rts$", 0x60, "no_arg"));
    patterns.push((r"^php$", 0x08, "no_arg"));
    patterns.push((r"^clc$", 0x18, "no_arg"));
    patterns.push((r"^plp$", 0x28, "no_arg"));
    patterns.push((r"^sec$", 0x38, "no_arg"));
    patterns.push((r"^pha$", 0x48, "no_arg"));
    patterns.push((r"^cli$", 0x58, "no_arg"));
    patterns.push((r"^pla$", 0x68, "no_arg"));
    patterns.push((r"^sei$", 0x78, "no_arg"));
    patterns.push((r"^dey$", 0x88, "no_arg"));
    patterns.push((r"^tya$", 0x98, "no_arg"));
    patterns.push((r"^tay$", 0xa8, "no_arg"));
    patterns.push((r"^clv$", 0xb8, "no_arg"));
    patterns.push((r"^iny$", 0xc8, "no_arg"));
    patterns.push((r"^cld$", 0xd8, "no_arg"));
    patterns.push((r"^inx$", 0xe8, "no_arg"));
    patterns.push((r"^sed$", 0xf8, "no_arg"));
    patterns.push((r"^txa$", 0x8a, "no_arg"));
    patterns.push((r"^txs$", 0x9a, "no_arg"));
    patterns.push((r"^tax$", 0xaa, "no_arg"));
    patterns.push((r"^tsx$", 0xba, "no_arg"));
    patterns.push((r"^dex$", 0xca, "no_arg"));
    patterns.push((r"^nop$", 0xea, "no_arg"));

    patterns.push((r"^asl$", 0x0a, "no_arg"));
    patterns.push((r"^rol$", 0x2a, "no_arg"));
    patterns.push((r"^lsr$", 0x4a, "no_arg"));
    patterns.push((r"^ror$", 0x6a, "no_arg"));


    // rels (relative label)
    patterns.push((r"^bpl\s+([a-zA-Z]\w*)$", 0x10, "label_rel"));
    patterns.push((r"^bmi\s+([a-zA-Z]\w*)$", 0x30, "label_rel"));
    patterns.push((r"^bvc\s+([a-zA-Z]\w*)$", 0x50, "label_rel"));
    patterns.push((r"^bvs\s+([a-zA-Z]\w*)$", 0x70, "label_rel"));
    patterns.push((r"^bcc\s+([a-zA-Z]\w*)$", 0x90, "label_rel"));
    patterns.push((r"^bcs\s+([a-zA-Z]\w*)$", 0xb0, "label_rel"));
    patterns.push((r"^bne\s+([a-zA-Z]\w*)$", 0xd0, "label_rel"));
    patterns.push((r"^beq\s+([a-zA-Z]\w*)$", 0xf0, "label_rel"));

    // rels unlabeled (relative offset)
    patterns.push((r"^bpl\s+\$([0-9a-f]{1,2})$", 0x10, "u8"));
    patterns.push((r"^bmi\s+\$([0-9a-f]{1,2})$", 0x30, "u8"));
    patterns.push((r"^bvc\s+\$([0-9a-f]{1,2})$", 0x50, "u8"));
    patterns.push((r"^bvs\s+\$([0-9a-f]{1,2})$", 0x70, "u8"));
    patterns.push((r"^bcc\s+\$([0-9a-f]{1,2})$", 0x90, "u8"));
    patterns.push((r"^bcs\s+\$([0-9a-f]{1,2})$", 0xb0, "u8"));
    patterns.push((r"^bne\s+\$([0-9a-f]{1,2})$", 0xd0, "u8"));
    patterns.push((r"^beq\s+\$([0-9a-f]{1,2})$", 0xf0, "u8"));

    // immediates
    patterns.push((r"^ldy\s+#\$?([0-9a-f]{1,2})", 0xa0, "u8"));
    patterns.push((r"^ldx\s+#\$?([0-9a-f]{1,2})", 0xa2, "u8"));
    patterns.push((r"^cpy\s+#\$?([0-9a-f]{1,2})", 0xc0, "u8"));
    patterns.push((r"^cpx\s+#\$?([0-9a-f]{1,2})", 0xe0, "u8"));
    patterns.push((r"^ora\s+#\$?([0-9a-f]{1,2})", 0x09, "u8"));
    patterns.push((r"^and\s+#\$?([0-9a-f]{1,2})", 0x29, "u8"));
    patterns.push((r"^eor\s+#\$?([0-9a-f]{1,2})", 0x49, "u8"));
    patterns.push((r"^adc\s+#\$?([0-9a-f]{1,2})", 0x69, "u8"));
    patterns.push((r"^lda\s+#\$?([0-9a-f]{1,2})", 0xa9, "u8"));
    patterns.push((r"^cmp\s+#\$?([0-9a-f]{1,2})", 0xc9, "u8"));
    patterns.push((r"^sbc\s+#\$?([0-9a-f]{1,2})", 0xe9, "u8"));

    // indirect
    patterns.push((r"^ora\s+\(\$([0-9a-f]{1,2}),x\)$", 0x01, "u8"));
    patterns.push((r"^ora\s+\(\$([0-9a-f]{1,2})\),y", 0x11, "u8"));
    patterns.push((r"^and\s+\(\$([0-9a-f]{1,2}),x\)$", 0x21, "u8"));
    patterns.push((r"^and\s+\(\$([0-9a-f]{1,2})\),y", 0x31, "u8"));
    patterns.push((r"^eor\s+\(\$([0-9a-f]{1,2}),x\)$", 0x41, "u8"));
    patterns.push((r"^eor\s+\(\$([0-9a-f]{1,2})\),y", 0x51, "u8"));
    patterns.push((r"^adc\s+\(\$([0-9a-f]{1,2}),x\)$", 0x61, "u8"));
    patterns.push((r"^adc\s+\(\$([0-9a-f]{1,2})\),y", 0x71, "u8"));
    patterns.push((r"^sta\s+\(\$([0-9a-f]{1,2}),x\)$", 0x81, "u8"));
    patterns.push((r"^sta\s+\(\$([0-9a-f]{1,2})\),y", 0x91, "u8"));
    patterns.push((r"^lda\s+\(\$([0-9a-f]{1,2}),x\)$", 0xa1, "u8"));
    patterns.push((r"^lda\s+\(\$([0-9a-f]{1,2})\),y", 0xb1, "u8"));
    patterns.push((r"^cmp\s+\(\$([0-9a-f]{1,2}),x\)$", 0xc1, "u8"));
    patterns.push((r"^cmp\s+\(\$([0-9a-f]{1,2})\),y", 0xd1, "u8"));
    patterns.push((r"^sbc\s+\(\$([0-9a-f]{1,2}),x\)$", 0xe1, "u8"));
    patterns.push((r"^sbc\s+\(\$([0-9a-f]{1,2})\),y", 0xf1, "u8"));

    patterns.push((r"^jmp\s+\(\$([0-9a-f]{1,4})\)$",   0x6c, "u16"));

    // zpgs
    patterns.push((r"^ora\s+\$([0-9a-f]{1,2})$",    0x05, "u8"));
    patterns.push((r"^ora\s+\$([0-9a-f]{1,2}),x$",  0x15, "u8"));
    patterns.push((r"^and\s+\$([0-9a-f]{1,2})$",    0x25, "u8"));
    patterns.push((r"^and\s+\$([0-9a-f]{1,2}),x$",  0x35, "u8"));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,2})$",    0x45, "u8"));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,2}),x$",  0x55, "u8"));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,2})$",    0x65, "u8"));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,2}),x$",  0x75, "u8"));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,2})$",    0x85, "u8"));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,2}),x$",  0x95, "u8"));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,2})$",    0xa5, "u8"));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,2}),x$",  0xb5, "u8"));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,2})$",    0xc5, "u8"));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,2}),x$",  0xd5, "u8"));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,2})$",    0xe5, "u8"));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,2}),x$",  0xf5, "u8"));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,2})$",    0x06, "u8"));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,2}),x$",  0x16, "u8"));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,2})$",    0x26, "u8"));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,2}),x$",  0x36, "u8"));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,2})$",    0x46, "u8"));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,2}),x$",  0x56, "u8"));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,2})$",    0x66, "u8"));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,2}),x$",  0x76, "u8"));
    patterns.push((r"^stx\s+\$([0-9a-f]{1,2})$",    0x86, "u8"));
    patterns.push((r"^stx\s+\$([0-9a-f]{1,2}),y$",  0x96, "u8"));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,2})$",    0xa6, "u8"));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,2}),y$",  0xb6, "u8"));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,2})$",    0xc6, "u8"));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,2}),x$",  0xd6, "u8"));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,2})$",    0xe6, "u8"));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,2}),x$",  0xf6, "u8"));
    patterns.push((r"^sty\s+\$([0-9a-f]{1,2})$",    0x84, "u8"));
    patterns.push((r"^sty\s+\$([0-9a-f]{1,2}),x$",  0x94, "u8"));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,2})$",    0xa4, "u8"));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,2}),x$",  0xb4, "u8"));
    patterns.push((r"^bit\s+\$([0-9a-f]{1,2})$",    0x24, "u8"));
    patterns.push((r"^cpy\s+\$([0-9a-f]{1,2})$",    0xc4, "u8"));
    patterns.push((r"^cpx\s+\$([0-9a-f]{1,2})$",    0xe4, "u8"));
    
    // absolutes
    patterns.push((r"^ora\s+\$([0-9a-f]{1,4})$",    0x0d, "u16"));
    patterns.push((r"^ora\s+\$([0-9a-f]{1,4}),x$",  0x1d, "u16"));
    patterns.push((r"^ora\s+\$([0-9a-f]{1,4}),y$",  0x19, "u16"));
    patterns.push((r"^and\s+\$([0-9a-f]{1,4})$",    0x2d, "u16"));
    patterns.push((r"^and\s+\$([0-9a-f]{1,4}),x$",  0x3d, "u16"));
    patterns.push((r"^and\s+\$([0-9a-f]{1,4}),y$",  0x39, "u16"));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,4})$",    0x4d, "u16"));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,4}),x$",  0x5d, "u16"));
    patterns.push((r"^eor\s+\$([0-9a-f]{1,4}),y$",  0x59, "u16"));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,4})$",    0x6d, "u16"));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,4}),x$",  0x7d, "u16"));
    patterns.push((r"^adc\s+\$([0-9a-f]{1,4}),y$",  0x79, "u16"));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,4})$",    0x8d, "u16"));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,4}),x$",  0x9d, "u16"));
    patterns.push((r"^sta\s+\$([0-9a-f]{1,4}),y$",  0x99, "u16"));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,4})$",    0xad, "u16"));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,4}),x$",  0xbd, "u16"));
    patterns.push((r"^lda\s+\$([0-9a-f]{1,4}),y$",  0xb9, "u16"));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,4})$",    0xcd, "u16"));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,4}),x$",  0xdd, "u16"));
    patterns.push((r"^cmp\s+\$([0-9a-f]{1,4}),y$",  0xd9, "u16"));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,4})$",    0xed, "u16"));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,4}),x$",  0xfd, "u16"));
    patterns.push((r"^sbc\s+\$([0-9a-f]{1,4}),y$",  0xf9, "u16"));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,4})$",    0x0e, "u16"));
    patterns.push((r"^asl\s+\$([0-9a-f]{1,4}),x$",  0x1e, "u16"));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,4})$",    0x2e, "u16"));
    patterns.push((r"^rol\s+\$([0-9a-f]{1,4}),x$",  0x3e, "u16"));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,4})$",    0x4e, "u16"));
    patterns.push((r"^lsr\s+\$([0-9a-f]{1,4}),x$",  0x5e, "u16"));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,4})$",    0x6e, "u16"));
    patterns.push((r"^ror\s+\$([0-9a-f]{1,4}),x$",  0x7e, "u16"));
    patterns.push((r"^stx\s+\$([0-9a-f]{1,4})$",    0x8e, "u16"));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,4})$",    0x9e, "u16"));
    patterns.push((r"^ldx\s+\$([0-9a-f]{1,4}),y$",  0xae, "u16"));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,4})$",    0xbe, "u16"));
    patterns.push((r"^dec\s+\$([0-9a-f]{1,4}),x$",  0xce, "u16"));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,4})$",    0xde, "u16"));
    patterns.push((r"^inc\s+\$([0-9a-f]{1,4}),x$",  0xee, "u16"));
    patterns.push((r"^sty\s+\$([0-9a-f]{1,4})$",    0x8c, "u16"));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,4})$",    0xac, "u16"));
    patterns.push((r"^ldy\s+\$([0-9a-f]{1,4}),x$",  0xbc, "u16"));
    patterns.push((r"^bit\s+\$([0-9a-f]{1,4})$",    0x2c, "u16"));
    patterns.push((r"^cpy\s+\$([0-9a-f]{1,4})$",    0xcc, "u16"));
    patterns.push((r"^cpx\s+\$([0-9a-f]{1,4})$",    0xec, "u16"));

    patterns.push((r"^jmp\s+\$([0-9a-f]{1,4})$",    0x4c, "u16"));
    patterns.push((r"^jmp\s+([a-zA-Z]\w*)$",        0x4c, "label_abs"));

    patterns.push((r"^jsr\s+\$([0-9a-f]{1,4})$",    0x20, "u16"));
    patterns.push((r"^jsr\s+([a-zA-Z]\w+)$",        0x20, "label_abs"));

    let mut compiled_patterns = Vec::new();
    for (pattern, opcode, instr_type) in patterns {
        let re = Regex::new(pattern).unwrap();
        compiled_patterns.push((re, opcode, instr_type));
    }

    return compiled_patterns;
}

