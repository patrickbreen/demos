extern crate regex;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::hash_map::HashMap;
use std::vec::Vec;
use std::env;
use std::io::Write;

use regex::Regex;

mod encode;
use encode::{compile_patterns, get_opcode_and_arguments};

#[derive(Debug, Clone)]
struct Line {
    bin_position: u16, // address would be (bin_position + start_mem_address)
    line_number: u16,

    op_code: u8,
    // Note that arguments can be either labels or numbers, so in general treat them as strings
    argument: String,
}

#[derive(Debug, Clone)]
struct Define {
    original: String,
    replace: String,
}

// This function strips out comments and applies "define" statements
fn preprocess(raw_lines: Vec<String>) -> Vec<String> {

    // strip comments
    let mut lines_no_comments = Vec::new();
    for line in raw_lines {
        let tokens = line.split(";").collect::<Vec<&str>>();

        lines_no_comments.push(String::from(tokens[0]));
    }

    // get defines, non-defines
    let mut defines: Vec<Define> = Vec::new();
    let mut not_defines = Vec::new();

    for line in lines_no_comments.clone() {
        if line.starts_with("define") {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();

            let d = Define{ 
                original:String::from(tokens[1]),
                replace:String::from(tokens[2])
            };
            defines.push(d);
            not_defines.push("".to_string());
            
        } else {
            not_defines.push(line);
        }
    }

    // find/replace defines
    let mut after_defines = Vec::new();

    for line in not_defines.clone() {
        let mut new_line = line.clone();

        for define in defines.clone() {
            new_line = str::replace(&line, &define.original, &define.replace);

            if line != new_line {
                break;
            }

        }
        after_defines.push(new_line);
    }
    after_defines
}

// note endianess
fn u16_to_two_u8s(arg: u16) -> [u8; 2] {
    [arg as u8, (arg >> 8) as u8]
}

fn parse(after_defines: Vec<String>,
         compiled_patterns: Vec<(Regex, u8, &'static str)>) -> 
         (HashMap<String, u16>, Vec<u8>, Vec<String>, Vec<u16>, Vec<&'static str>) {
    // First pass: put labels in as arguments if nessisary
    // and build label map (label-> absolute address)

    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut opcodes: Vec<u8> = Vec::new();
    let mut args: Vec<String> = Vec::new();
    let mut line_numbers: Vec<u16> = Vec::new();
    let mut instr_types: Vec<&'static str> = Vec::new();

    let mut line_number = 1;
    let mut position = 0;

    // first pass, just get the location of labels
    for line in after_defines.clone() {


        // blank line
        if line == "" {
            line_number += 1;
        }
        // label 
        else if line.ends_with(":") {
            let label = line[0..(line.len()-1)].to_string();

            let mut addr = 0;
            if position == 0 {
                addr = 0;
            } else {
                addr = position - 1;
            }
            labels.insert(label.to_lowercase(), addr);
            line_number += 1;
        }

        // instruction with zero or one argument
        else {
            let (opcode, arg, instr_type) = get_opcode_and_arguments(
                                    line.to_lowercase().trim().to_string(),
                                    line_number, &compiled_patterns);
            if instr_type == "u8" || instr_type == "label_rel" {
                position += 2;
            } else if instr_type == "u16" || instr_type == "label_abs" {
                position += 3;
            } else if instr_type == "no_arg" {
                position += 1;
            } else {
                panic!("unknown instr type");
            }

            opcodes.push(opcode);
            args.push(arg);
            line_numbers.push(line_number);
            instr_types.push(instr_type);
            line_number += 1;
        }
        
    }
    (labels, opcodes, args, line_numbers, instr_types)
}



fn assemble(in_file_path: String, start_mem_address: u16) -> Vec<u8> {

    // Read file. File closes automatically at the end of the scope.
    let f = File::open(in_file_path).unwrap();
    let f = BufReader::new(f);
    
    let mut raw_lines = Vec::new();

    for line in f.lines() {
        raw_lines.push(String::from(line.unwrap().trim()));
    }

    let after_defines = preprocess(raw_lines);
    let compiled_patterns = compile_patterns();

    let (labels, opcodes, args, line_numbers, instr_types) = parse(after_defines, compiled_patterns);


    let mut output_bin_bytes: Vec<u8> = Vec::new();
    for i in 0..opcodes.len() {

        output_bin_bytes.push(opcodes[i]);

        if instr_types[i] == "label_rel" {

            let current_position = output_bin_bytes.len() as i32;
            let jump_addr = *labels.get(&args[i]).unwrap() as i32;
            let diff = (jump_addr - current_position) as i8;
            
            output_bin_bytes.push(diff as u8);

        } else if instr_types[i] == "label_abs" {

            let val = *labels.get(&args[i]).unwrap() + start_mem_address + 1;
            let vals = u16_to_two_u8s(val);
            output_bin_bytes.push(vals[0]);
            output_bin_bytes.push(vals[1]);

        } else if instr_types[i] == "u8" {
            let val = u8::from_str_radix(&args[i], 16).unwrap();
            output_bin_bytes.push(val);
        } else if instr_types[i] == "u16" {
            let val = u16::from_str_radix(&args[i], 16).unwrap();
            let vals = u16_to_two_u8s(val);
            output_bin_bytes.push(vals[0]);
            output_bin_bytes.push(vals[1]);
        }
    }
    output_bin_bytes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // All binaries are placed at 0x0600 offset
    let start_mem_address: u16 = 0x0600;
    let bin_bytes = assemble(args[1].to_string(), start_mem_address);

    let mut output_file = File::create(args[2].to_string()).unwrap();
    output_file.write_all(&bin_bytes).unwrap();

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple() {
        let bin_bytes = assemble("asm_code/simple.6502asm".to_string(), 0x0600);
        let expected = vec![0xa9, 0x01, 0x8d, 0x00, 0x02,
                            0xa9, 0x05, 0x8d, 0x01, 0x02,
                            0xa9, 0x08, 0x8d, 0x02, 0x02];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_registers() {
        let bin_bytes = assemble("asm_code/registers.6502asm".to_string(), 0x0600);
        let expected = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x69, 0xc4, 0x00];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_branching() {
        let bin_bytes = assemble("asm_code/branching.6502asm".to_string(), 0x0600);
        let expected = vec![0xa2, 0x08, 0xca, 0x8e, 0x00,
                            0x02, 0xe0, 0x03, 0xd0, 0xf8,
                            0x8e, 0x01, 0x02, 0x00];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_indirect() {
        let bin_bytes = assemble("asm_code/indirect.6502asm".to_string(), 0x0600);
        let expected = vec![0xa9, 0x01, 0x85, 0xf0, 0xa9,
                            0xcc, 0x85, 0xf1, 0x6c, 0xf0, 0x00];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_indexed_indirect() {
        let bin_bytes = assemble("asm_code/indexed_indirect.6502asm".to_string(), 0x0600);
        let expected = vec![0xa2, 0x01, 0xa9, 0x05, 0x85,
                            0x01, 0xa9, 0x07, 0x85, 0x02,
                            0xa0, 0x0a, 0x8c, 0x05, 0x07,
                            0xa1, 0x00 ];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_indirect_index() {
        let bin_bytes = assemble("asm_code/indirect_indexed.6502asm".to_string(), 0x0600);
        let expected = vec![0xa0, 0x01, 0xa9, 0x03, 0x85,
                            0x01, 0xa9, 0x07, 0x85, 0x02,
                            0xa2, 0x0a, 0x8e, 0x04, 0x07,
                            0xb1, 0x01 ];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_stack() {
        let bin_bytes = assemble("asm_code/stack.6502asm".to_string(), 0x0600);
        let expected = vec![0xa2, 0x00, 0xa0, 0x00, 0x8a,
                            0x99, 0x00, 0x02, 0x48, 0xe8,
                            0xc8, 0xc0, 0x10, 0xd0, 0xf5,
                            0x68, 0x99, 0x00, 0x02, 0xc8,
                            0xc0, 0x20, 0xd0, 0xf7];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_jsr() {
        let bin_bytes = assemble("asm_code/jsr.6502asm".to_string(), 0x0600);
        let expected = vec![0x20, 0x09, 0x06, 0x20, 0x0c,
                            0x06, 0x20, 0x12, 0x06, 0xa2,
                            0x00, 0x60, 0xe8, 0xe0, 0x05,
                            0xd0, 0xfb, 0x60, 0x00];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_preprocess() {
        let bin_bytes = assemble("asm_code/preprocess.6502asm".to_string(), 0x0600);
        let expected = vec![0xa5, 0xfe, 0xa2, 0x0c];
        assert_eq!(bin_bytes, expected);
    }

    #[test]
    fn test_snake() {
        let bin_bytes = assemble("asm_code/snake.6502asm".to_string(), 0x0600);
        let expected = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
        0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
        0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
        0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
        0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
        0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
        0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
        0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
        0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
        0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
        0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
        0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
        0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
        0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
        0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
        0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
        0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
        0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
        0xea, 0xca, 0xd0, 0xfb, 0x60
        ];

        assert_eq!(bin_bytes, expected);
    }
}