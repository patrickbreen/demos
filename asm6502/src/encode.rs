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


    // rels (assume they are all labeled... for now)
    return_match!(r"^bvc\s+(w{1, 20})$", 0x50, &line);
    return_match!(r"^bvs\s+(w{1, 20})$", 0x90, &line);
    return_match!(r"^bne\s+(w{1, 20})$", 0xd0, &line);
    return_match!(r"^beq\s+(w{1, 20})$", 0xf0, &line);

    // immediates
    return_match!(r"^ldy\s+#\$([0-9a-f]{2})", 0xa0, &line);
    return_match!(r"^ldx\s+#\$([0-9a-f]{2})", 0xa2, &line);
    return_match!(r"^cpy\s+#\$([0-9a-f]{2})", 0xc0, &line);
    return_match!(r"^ora\s+#\$([0-9a-f]{2})", 0x09, &line);
    return_match!(r"^and\s+#\$([0-9a-f]{2})", 0x29, &line);
    return_match!(r"^eor\s+#\$([0-9a-f]{2})", 0x49, &line);
    return_match!(r"^adc\s+#\$([0-9a-f]{2})", 0x69, &line);
    return_match!(r"^lda\s+#\$([0-9a-f]{2})", 0xa9, &line);
    return_match!(r"^cmp\s+#\$([0-9a-f]{2})", 0xc9, &line);
    return_match!(r"^sbc\s+#\$([0-9a-f]{2})", 0xe9, &line);

    // indirect
    return_match!(r"^ora\s+\(\$([0-9a-f]{2}),x\)$", 0x01, &line);
    return_match!(r"^ora\s+\(\$([0-9a-f]{2}),y\)$", 0x11, &line);
    return_match!(r"^and\s+\(\$([0-9a-f]{2}),x\)$", 0x21, &line);
    return_match!(r"^and\s+\(\$([0-9a-f]{2}),y\)$", 0x31, &line);
    return_match!(r"^eor\s+\(\$([0-9a-f]{2}),x\)$", 0x41, &line);
    return_match!(r"^eor\s+\(\$([0-9a-f]{2}),y\)$", 0x51, &line);
    return_match!(r"^adc\s+\(\$([0-9a-f]{2}),x\)$", 0x61, &line);
    return_match!(r"^adc\s+\(\$([0-9a-f]{2}),y\)$", 0x71, &line);
    return_match!(r"^sta\s+\(\$([0-9a-f]{2}),x\)$", 0x81, &line);
    return_match!(r"^sta\s+\(\$([0-9a-f]{2}),y\)$", 0x91, &line);
    return_match!(r"^lda\s+\(\$([0-9a-f]{2}),x\)$", 0xa1, &line);
    return_match!(r"^lda\s+\(\$([0-9a-f]{2}),y\)$", 0xb1, &line);
    return_match!(r"^cmp\s+\(\$([0-9a-f]{2}),x\)$", 0xc1, &line);
    return_match!(r"^cmp\s+\(\$([0-9a-f]{2}),y\)$", 0xd1, &line);
    return_match!(r"^sbc\s+\(\$([0-9a-f]{2}),x\)$", 0xe1, &line);
    return_match!(r"^sbc\s+\(\$([0-9a-f]{2}),y\)$", 0xf1, &line);
    return_match!(r"^jmp\s+\(\$([0-9a-f]{4})\)$",   0x6c, &line);

    // zpgs
    let re_ora_zpg =    Regex::new(r"^ora\s+\$([0-9a-f]{2})$").unwrap();
    let re_ora_zpg_x =  Regex::new(r"^ora\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_and_zpg =    Regex::new(r"^and\s+\$([0-9a-f]{2})$").unwrap();
    let re_and_zpg_x =  Regex::new(r"^and\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_eor_zpg =    Regex::new(r"^eor\s+\$([0-9a-f]{2})$").unwrap();
    let re_eor_zpg_x =  Regex::new(r"^eor\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_adc_zpg =    Regex::new(r"^adc\s+\$([0-9a-f]{2})$").unwrap();
    let re_adc_zpg_x =  Regex::new(r"^adc\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_sta_zpg =    Regex::new(r"^sta\s+\$([0-9a-f]{2})$").unwrap();
    let re_sta_zpg_x =  Regex::new(r"^sta\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_lda_zpg =    Regex::new(r"^lda\s+\$([0-9a-f]{2})$").unwrap();
    let re_lda_zpg_x =  Regex::new(r"^lda\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_cmp_zpg =    Regex::new(r"^cmp\s+\$([0-9a-f]{2})$").unwrap();
    let re_cmp_zpg_x =  Regex::new(r"^cmp\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_sbc_zpg =    Regex::new(r"^sbc\s+\$([0-9a-f]{2})$").unwrap();
    let re_sbc_zpg_x =  Regex::new(r"^sbc\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_asl_zpg =    Regex::new(r"^asl\s+\$([0-9a-f]{2})$").unwrap();
    let re_asl_zpg_x =  Regex::new(r"^asl\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_rol_zpg =    Regex::new(r"^rol\s+\$([0-9a-f]{2})$").unwrap();
    let re_rol_zpg_x =  Regex::new(r"^rol\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_lsr_zpg =    Regex::new(r"^lsr\s+\$([0-9a-f]{2})$").unwrap();
    let re_lsr_zpg_x =  Regex::new(r"^lsr\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_ror_zpg =    Regex::new(r"^ror\s+\$([0-9a-f]{2})$").unwrap();
    let re_ror_zpg_x =  Regex::new(r"^ror\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_stx_zpg =    Regex::new(r"^stx\s+\$([0-9a-f]{2})$").unwrap();
    let re_stx_zpg_y =  Regex::new(r"^stx\s+\$([0-9a-f]{2}),y$").unwrap();
    let re_ldx_zpg =    Regex::new(r"^ldx\s+\$([0-9a-f]{2})$").unwrap();
    let re_ldx_zpg_y =  Regex::new(r"^ldx\s+\$([0-9a-f]{2}),y$").unwrap();
    let re_dec_zpg =    Regex::new(r"^dec\s+\$([0-9a-f]{2})$").unwrap();
    let re_dec_zpg_x =  Regex::new(r"^dec\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_inc_zpg =    Regex::new(r"^inc\s+\$([0-9a-f]{2})$").unwrap();
    let re_inc_zpg_x =  Regex::new(r"^inc\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_sty_zpg =    Regex::new(r"^sty\s+\$([0-9a-f]{2})$").unwrap();
    let re_sty_zpg_x =  Regex::new(r"^sty\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_ldy_zpg =    Regex::new(r"^ldy\s+\$([0-9a-f]{2})$").unwrap();
    let re_ldy_zpg_x =  Regex::new(r"^ldy\s+\$([0-9a-f]{2}),x$").unwrap();
    let re_bit_zpg =    Regex::new(r"^bit\s+\$([0-9a-f]{2})$").unwrap();
    let re_cpy_zpg =    Regex::new(r"^cpy\s+\$([0-9a-f]{2})$").unwrap();
    let re_cpx_zpg =    Regex::new(r"^cpy\s+\$([0-9a-f]{2})$").unwrap();
    
    // absolutes
    let re_ora_abs =    Regex::new(r"^ora\s+\$([0-9a-f]{4})$").unwrap();
    let re_ora_abs_x =  Regex::new(r"^ora\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_and_abs =    Regex::new(r"^and\s+\$([0-9a-f]{4})$").unwrap();
    let re_and_abs_x =  Regex::new(r"^and\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_eor_abs =    Regex::new(r"^eor\s+\$([0-9a-f]{4})$").unwrap();
    let re_eor_abs_x =  Regex::new(r"^eor\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_adc_abs =    Regex::new(r"^adc\s+\$([0-9a-f]{4})$").unwrap();
    let re_adc_abs_x =  Regex::new(r"^adc\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_sta_abs =    Regex::new(r"^sta\s+\$([0-9a-f]{4})$").unwrap();
    let re_sta_abs_x =  Regex::new(r"^sta\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_lda_abs =    Regex::new(r"^lda\s+\$([0-9a-f]{4})$").unwrap();
    let re_lda_abs_x =  Regex::new(r"^lda\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_cmp_abs =    Regex::new(r"^cmp\s+\$([0-9a-f]{4})$").unwrap();
    let re_cmp_abs_x =  Regex::new(r"^cmp\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_sbc_abs =    Regex::new(r"^sbc\s+\$([0-9a-f]{4})$").unwrap();
    let re_sbc_abs_x =  Regex::new(r"^sbc\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_asl_abs =    Regex::new(r"^asl\s+\$([0-9a-f]{4})$").unwrap();
    let re_asl_abs_x =  Regex::new(r"^asl\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_rol_abs =    Regex::new(r"^rol\s+\$([0-9a-f]{4})$").unwrap();
    let re_rol_abs_x =  Regex::new(r"^rol\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_lsr_abs =    Regex::new(r"^lsr\s+\$([0-9a-f]{4})$").unwrap();
    let re_lsr_abs_x =  Regex::new(r"^lsr\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_ror_abs =    Regex::new(r"^ror\s+\$([0-9a-f]{4})$").unwrap();
    let re_ror_abs_x =  Regex::new(r"^ror\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_stx_abs =    Regex::new(r"^stx\s+\$([0-9a-f]{4})$").unwrap();
    let re_ldx_abs =    Regex::new(r"^ldx\s+\$([0-9a-f]{4})$").unwrap();
    let re_ldx_abs_y =  Regex::new(r"^ldx\s+\$([0-9a-f]{4}),y$").unwrap();
    let re_dec_abs =    Regex::new(r"^dec\s+\$([0-9a-f]{4})$").unwrap();
    let re_dec_abs_x =  Regex::new(r"^dec\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_inc_abs =    Regex::new(r"^inc\s+\$([0-9a-f]{4})$").unwrap();
    let re_inc_abs_x =  Regex::new(r"^inc\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_sty_abs =    Regex::new(r"^sty\s+\$([0-9a-f]{4})$").unwrap();
    let re_ldy_abs =    Regex::new(r"^ldy\s+\$([0-9a-f]{4})$").unwrap();
    let re_ldy_abs_x =  Regex::new(r"^ldy\s+\$([0-9a-f]{4}),x$").unwrap();
    let re_bit_abs =    Regex::new(r"^bit\s+\$([0-9a-f]{4})$").unwrap();
    let re_cpy_abs =    Regex::new(r"^cpy\s+\$([0-9a-f]{4})$").unwrap();
    let re_cpx_abs =    Regex::new(r"^cpy\s+\$([0-9a-f]{4})$").unwrap();

    let re_jmp_abs =    Regex::new(r"^jmp\s+\$([0-9a-f]{4})$").unwrap();


    // all the impls and As
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

    else if line == "asl a" { return (0x0a, "".to_string()); }
    else if line == "rol a" { return (0x2a, "".to_string()); }
    else if line == "lsr a" { return (0x4a, "".to_string()); }
    else if line == "ror a" { return (0x6a, "".to_string()); }

    // all the rels 

    // all the indirect
    else if re_ora_ind_x.is_match(&line) {
        let caps =  re_ora_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x01, caps[0][1].to_string());
    }
    else if re_ora_ind_y.is_match(&line) {
        let caps =  re_ora_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x11, caps[0][1].to_string());
    }
    else if re_and_ind_x.is_match(&line) {
        let caps =  re_and_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x21, caps[0][1].to_string());
    }
    else if re_and_ind_y.is_match(&line) {
        let caps =  re_and_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x31, caps[0][1].to_string());
    }
    else if re_eor_ind_x.is_match(&line) {
        let caps =  re_eor_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x41, caps[0][1].to_string());
    }
    else if re_eor_ind_y.is_match(&line) {
        let caps =  re_eor_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x51, caps[0][1].to_string());
    }
    else if re_adc_ind_x.is_match(&line) {
        let caps =  re_adc_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x61, caps[0][1].to_string());
    }
    else if re_adc_ind_y.is_match(&line) {
        let caps =  re_adc_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x71, caps[0][1].to_string());
    }
    else if re_sta_ind_x.is_match(&line) {
        let caps =  re_sta_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x81, caps[0][1].to_string());
    }
    else if re_sta_ind_y.is_match(&line) {
        let caps =  re_sta_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x91, caps[0][1].to_string());
    }
    else if re_lda_ind_x.is_match(&line) {
        let caps =  re_lda_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0xa1, caps[0][1].to_string());
    }
    else if re_lda_ind_y.is_match(&line) {
        let caps =  re_lda_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0xb1, caps[0][1].to_string());
    }
    else if re_cmp_ind_x.is_match(&line) {
        let caps =  re_cmp_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0xc1, caps[0][1].to_string());
    }
    else if re_cmp_ind_y.is_match(&line) {
        let caps =  re_cmp_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0xd1, caps[0][1].to_string());
    }
    else if re_sbc_ind_x.is_match(&line) {
        let caps =  re_sbc_ind_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0xe1, caps[0][1].to_string());
    }
    else if re_sbc_ind_y.is_match(&line) {
        let caps =  re_sbc_ind_y.captures_iter(&line).collect::<Vec<Captures>>();
        return (0xf1, caps[0][1].to_string());
    }
    else if re_jmp_ind.is_match(&line) {
        let caps =  re_jmp_ind.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x6c, caps[0][1].to_string());
    }

    // all the zpgs
    else if re_ora_zpg.is_match(&line) {
        let caps =  re_ora_zpg.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x05, caps[0][1].to_string());
    }
    else if re_ora_zpg_x.is_match(&line) {
        let caps =  re_ora_zpg_x.captures_iter(&line).collect::<Vec<Captures>>();
        return (0x15, caps[0][1].to_string());
    }

    // all the absolutes


    // let re = Regex::new(r"([A-Za-z]").unwrap();
    // let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    // for cap in re.captures_iter(text) {
    //     println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    // }

    else {
        panic!("the line {}, ({}) was not a valid instruction.", line_number, line);
    }
}