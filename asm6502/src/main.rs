extern crate regex;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::hash_map::HashMap;
use std::vec::Vec;
use regex::Regex;


mod encode;
use encode::{compile_patterns, get_opcode_and_arguments};

#[derive(Debug, Clone)]
struct Line {
    bin_position: u16, // address would be (bin_position + start_mem_address)
    line_number: u16,

    op_code: u8,
    // Note that arguments can be either labels or numbers, so in general treat them as strings
    // In the case that there are no arguments, these would be empty string(s)
    // also note that the argument can be one or two bytes, and this is reflected in the string
    // ie leading zeros mean something
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

// does endianess matter?
fn u16_to_two_u8s(arg: u16) -> [u8; 2] {
    [(arg >> 8) as u8, arg as u8]
}

fn parse(after_defines: Vec<String>,
         compiled_patterns: Vec<(Regex, u8, &'static str)>) -> 
         (HashMap<String, u16>, Vec<u8>, Vec<String>, Vec<u16>, Vec<&'static str>) {
    // First pass: Use massive branching statement to parse everything
    // put labels in as arguments if nessisary
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
            let label = line[0..(line.len()-2)].to_string();
            labels.insert(label, position);
            line_number += 1;
        }

        // instruction with zero or one argument
        else {
            // TODO, the position can be either 2 or 3. 1 for the opcode, and either 1 or 2 for the argument.
            // use this to get opcodes: https://www.masswerk.at/6502/6502_instruction_set.html
            // also use this: https://skilldrick.github.io/easy6502
            // just use a massive branching statement
            let (opcode, arg, instr_type) = get_opcode_and_arguments(
                                    line.to_lowercase().trim().to_string(),
                                    line_number, &compiled_patterns);
            position += (1+args.len()/2) as u16;
            opcodes.push(opcode);
            args.push(arg);
            line_numbers.push(line_number);
            instr_types.push(instr_type);
            line_number += 1;
        }
        
    }
    (labels, opcodes, args, line_numbers, instr_types)
}



fn main() {
    // All binaries are placed at 0x0600 offset (starting position)
    let start_mem_address: u16 = 0x0600;


    // read file
    let f = File::open("example.6502asm").unwrap();
    let f = BufReader::new(f);
    
    let mut raw_lines = Vec::new();

    for line in f.lines() {
        raw_lines.push(String::from(line.unwrap().trim()));
    }

    let after_defines = preprocess(raw_lines);
    println!("--------------------\n\n\n");
    println!("After defines:");

    for line in after_defines.clone() {
        println!("{:?}", line);
    }
    println!("--------------------\n\n\n");

    let compiled_patterns = compile_patterns();

    let (labels, opcodes, args, line_numbers, instr_types) = parse(after_defines, compiled_patterns);


    let mut output_bin_bytes: Vec<u8> = Vec::new();
    for i in 0..opcodes.len() {
        println!("{:?}: {:?}, {:?} {:?}", line_numbers[i], opcodes[i], args[i], instr_types[i]);
        output_bin_bytes.push(opcodes[i]);

        if instr_types[i] == "label" && labels.contains_key(&args[i]) {

            // TODO resolve the label
            output_bin_bytes.push(0x01);

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

    println!("--------------------\n\n\n");
    println!("{:?}", output_bin_bytes);

}
