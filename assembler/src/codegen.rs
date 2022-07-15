use crate::tokenizer;
use std::collections::*;

struct SymbolNeedToImplement {
    addr: i32,
    symbol: String,
}

fn regstr_to_regno(regstr: &str) -> u8 {
    match regstr {
        "R0" => 0x00,
        "R1" => 0x01,
        "R2" => 0x02,
        "R3" => 0x03,
        "R4" => 0x04,
        "R5" => 0x05,
        "R6" => 0x06,
        "R7" => 0x07,
        "R8" => 0x08,
        "R9" => 0x09,
        _ => panic!("Please use valid register! (current: {})", regstr),
    }
}

fn is_regstr(s: &str) -> bool {
    match s {
        "R0" | "R1" | "R2" | "R3" | "R4" | "R5" | "R6" | "R7" | "R8" | "R9" => true,
        _ => false,
    }
}

fn break_i32(num: i32, idx: u8) -> u8 {
    match idx {
        0 => (num >> 24) as u8,
        1 => (num << 8 >> 8 >> 16) as u8,
        2 => (num << 16 >> 16 >> 8) as u8,
        3 => (num << 24 >> 24) as u8,
        _ => panic!("Invalid call"),
    }
}

pub fn gen_code_from(tk: Vec<tokenizer::Token>) -> Vec<u8> {
    let mut v_start_addr = 0x0;
    let mut i = 0;
    let mut state_code = 0;
    let mut symbol_table = HashMap::<&str, i32>::new();
    let mut result = Vec::<u8>::new();
    let mut byte_pointer = 0;
    let mut next_ret_val = -1;
    let mut need_to_fill_in = Vec::<SymbolNeedToImplement>::new();

    while i < tk.len() {
        match state_code {

            0 => {
                // BOOKMARK - Main State
                match tk[i].t_type {
                    tokenizer::TokenType::MetaTag => {
                        if tk[i].clip == "@VSTART" {
                            state_code = 1;
                            i += 1;
                            continue;
                        }
                        if tk[i].clip == "@P" {
                            state_code = 2;
                            i += 1;
                            continue;
                        }
                        if tk[i].clip == "@DBGPRINTTABLE" {
                            println!("{:?}", symbol_table);
                            i += 1;
                            continue;
                        }
                    }
                    tokenizer::TokenType::Identifier => {
                        if tk[i].clip == "NOP" {
                            result.push(0x00);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "MOV" {
                            state_code = 4;
                            result.push(0x01);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "MOV_ANS" {
                            state_code = 6;
                            result.push(0x03);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "INTR" {
                            state_code = 7;
                        }
                        if tk[i].clip == "STORE" {
                            state_code = 8;
                            result.push(0x06);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "LOAD" {
                            state_code = 8;
                            result.push(0x07);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "ADD" {
                            state_code = 8;
                            result.push(0x08);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "SUB" {
                            state_code = 8;
                            result.push(0x09);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "MUL" {
                            state_code = 8;
                            result.push(0x0A);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "DIV" {
                            state_code = 8;
                            result.push(0x0B);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "MOD" {
                            state_code = 8;
                            result.push(0x0C);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "AND" {
                            state_code = 8;
                            result.push(0x0D);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "OR" {
                            state_code = 8;
                            result.push(0x0E);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "XOR" {
                            state_code = 8;
                            result.push(0x0F);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "NOT" {
                            state_code = 10;
                            result.push(0x10);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "JMP" {
                            state_code = 11;
                        }
                        if tk[i].clip == "LSF" {
                            state_code = 10;
                            result.push(0x13);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "RSF" {
                            state_code = 10;
                            result.push(0x14);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "JMP_COND" {
                            state_code = 12;
                        }
                        if tk[i].clip == "STOP" {
                            result.push(0x17);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "CMP" {
                            result.push(0x18);
                            byte_pointer += 1;
                            state_code = 15;
                        }
                        if tk[i].clip == "CALL" {
                            state_code = 13;
                        }
                        if tk[i].clip == "RET" {
                            result.push(0x21);
                            byte_pointer += 1;
                        }
                        if tk[i].clip == "PUSH" {
                            state_code = 14;
                        }
                        if tk[i].clip == "POP" {
                            result.push(0x24);
                            byte_pointer += 1;
                        }
                        // TODO ^ rin
                    }
                    _ => {}
                }
            }

            1 => {
                // BOOKMARK - Process meta tag @vstart
                match tk[i].t_type {
                    tokenizer::TokenType::Number => {
                        v_start_addr = tk[i].clip.parse().expect("unreachable");
                        state_code = 0;
                    }
                    _ => {
                        panic!("invalid argument for @vstart");
                    }
                }
            }

            2 => {
                // BOOKMARK - Process meta tag @per
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        symbol_table.insert(&tk[i].clip, v_start_addr + byte_pointer as i32);
                        state_code = 0;
                    }
                    _ => {}
                }
            }

            3 => {
                // BOOKMARK - Expect a comma
                match tk[i].t_type {
                    tokenizer::TokenType::Comma => {
                        state_code = next_ret_val;
                        next_ret_val = -1;
                    }
                    _ => {
                        panic!("expected , here")
                    }
                }
            }

            4 => {
                // BOOKMARK - mov, arg1
                match tk[i].t_type {
                    // mov register->register
                    tokenizer::TokenType::Identifier => {
                        result[byte_pointer - 1] = 0x02;
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        next_ret_val = 5;
                        state_code = 3;
                    }
                    tokenizer::TokenType::Number => {
                        let num = tk[i].clip.parse::<i32>().expect("imposible");
                        result.push(break_i32(num, 0));
                        result.push(break_i32(num, 1));
                        result.push(break_i32(num, 2));
                        result.push(break_i32(num, 3));
                        byte_pointer += 4;
                        next_ret_val = 5;
                        state_code = 3;
                    }
                    _ => panic!("caonima"),
                }
            }

            5 => {
                // BOOKMARK - mov, arg2
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        state_code = 0;
                    }
                    _ => panic!("caonima"),
                }
            }

            6 => {
                // BOOKMARK - normal arg1, id
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        state_code = 0;
                    }
                    _ => panic!("fuck, {:?}", tk[i]),
                }
            }

            7 => {
                // BOOKMARK - intr
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(0x05);
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 2;
                        state_code = 0;
                    }
                    tokenizer::TokenType::Number => {
                        result.push(0x04);
                        result.push(tk[i].clip.parse::<u8>().expect("FUCK"));
                        byte_pointer += 2;
                        state_code = 0;
                    }
                    _ => panic!("shit! {:?}", tk[i]),
                }
            }

            8 => {
                // BOOKMARK - general a1, a2 (arg 1)
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        // expect a comma
                        next_ret_val = 9;
                        state_code = 3;
                    }
                    _ => panic!("bitch! {:?}", tk[i]),
                }
            }

            9 => {
                // BOOKMARK - general a1, a2 (arg2)
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        state_code = 0;
                    }
                    _ => panic!("bitch! {:?}", tk[i]),
                }
            }

            10 => {
                // BOOKMARK - general a1
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        state_code = 0;
                    }
                    _ => panic!("damn it {:?}", tk[i]),
                }
            }

            11 => {
                // BOOKMARK - jmp
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        if is_regstr(&tk[i].clip) {
                            result.push(0x11);
                            result.push(regstr_to_regno(&tk[i].clip));
                            byte_pointer += 2;
                            state_code = 0;
                        } else {
                            let addr_gay = symbol_table.get(&tk[i].clip.as_str());
                            match addr_gay {
                                Some(addr) => {
                                    let a = *addr;
                                    result.push(0x12);
                                    result.push(break_i32(a, 0));
                                    result.push(break_i32(a, 1));
                                    result.push(break_i32(a, 2));
                                    result.push(break_i32(a, 3));
                                    byte_pointer += 5;
                                    state_code = 0;
                                }
                                None => {
                                    result.push(0x12);
                                    result.push(0x00);
                                    result.push(0x00);
                                    result.push(0x00);
                                    result.push(0x00);
                                    need_to_fill_in.push(SymbolNeedToImplement{addr: (byte_pointer + 1) as i32, symbol: (tk[i].clip).clone()});
                                    byte_pointer += 5;
                                    state_code = 0;
                                }
                            }
                        }
                    }
                    tokenizer::TokenType::Number => {
                        result.push(0x12);
                        let a = tk[i].clip.parse::<i32>().expect("FUCK");
                        result.push(break_i32(a, 0));
                        result.push(break_i32(a, 1));
                        result.push(break_i32(a, 2));
                        result.push(break_i32(a, 3));
                        byte_pointer += 5;
                        state_code = 0;
                    }
                    _ => panic!("...{:?}", tk[i]),
                }
            }

            12 => {
                // BOOKMARK - jmp_cond
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        if is_regstr(&tk[i].clip) {
                            result.push(0x15);
                            result.push(regstr_to_regno(&tk[i].clip));
                            byte_pointer += 2;
                            state_code = 0;
                        } else {
                            let addr_gay = symbol_table.get(&tk[i].clip.as_str());
                            match addr_gay {
                                Some(addr) => {
                                    let a = *addr;
                                    result.push(0x16);
                                    result.push(break_i32(a, 0));
                                    result.push(break_i32(a, 1));
                                    result.push(break_i32(a, 2));
                                    result.push(break_i32(a, 3));
                                    byte_pointer += 5;
                                    state_code = 0;
                                }
                                None => {
                                    result.push(0x16);
                                    result.push(0x00);
                                    result.push(0x00);
                                    result.push(0x00);
                                    result.push(0x00);
                                    need_to_fill_in.push(SymbolNeedToImplement{addr: (byte_pointer + 1) as i32, symbol: (tk[i].clip).clone()});
                                    byte_pointer += 5;
                                    state_code = 0;
                                }
                            }
                        }
                    }
                    tokenizer::TokenType::Number => {
                        result.push(0x16);
                        let a = tk[i].clip.parse::<i32>().expect("FUCK");
                        result.push(break_i32(a, 0));
                        result.push(break_i32(a, 1));
                        result.push(break_i32(a, 2));
                        result.push(break_i32(a, 3));
                        byte_pointer += 5;
                        state_code = 0;
                    }
                    _ => panic!("...{:?}", tk[i]),
                }
            }

            13 => {
                // BOOKMARK - call
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        if is_regstr(&tk[i].clip) {
                            result.push(0x20);
                            result.push(regstr_to_regno(&tk[i].clip));
                            byte_pointer += 2;
                            state_code = 0;
                        } else {
                            let addr_gay = symbol_table.get(&tk[i].clip.as_str());
                            match addr_gay {
                                Some(addr) => {
                                    let a = *addr;
                                    result.push(0x19);
                                    result.push(break_i32(a, 0));
                                    result.push(break_i32(a, 1));
                                    result.push(break_i32(a, 2));
                                    result.push(break_i32(a, 3));
                                    byte_pointer += 5;
                                    state_code = 0;
                                }
                                None => {
                                    result.push(0x19);
                                    result.push(0x00);
                                    result.push(0x00);
                                    result.push(0x00);
                                    result.push(0x00);
                                    need_to_fill_in.push(SymbolNeedToImplement{addr: (byte_pointer + 1) as i32, symbol: (tk[i].clip).clone()});
                                    byte_pointer += 5;
                                    state_code = 0;
                                }
                            }
                        }
                    }
                    tokenizer::TokenType::Number => {
                        result.push(0x19);
                        let a = tk[i].clip.parse::<i32>().expect("FUCK");
                        result.push(break_i32(a, 0));
                        result.push(break_i32(a, 1));
                        result.push(break_i32(a, 2));
                        result.push(break_i32(a, 3));
                        byte_pointer += 5;
                        state_code = 0;
                    }
                    _ => panic!("...{:?}", tk[i]),
                }
            }

            14 => {
                // BOOKMARK - push
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(0x23);
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 2;
                        state_code = 0;
                    }
                    tokenizer::TokenType::Number => {
                        result.push(0x22);
                        result.push(tk[i].clip.parse::<u8>().unwrap());
                    }
                    _ => panic!("{:?}", tk[i])
                }
            }

            15 => {
                // BOOKMARK - cmp [a1,] a2, #mode: i8#
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        
                        next_ret_val = 16;
                        state_code = 3;
                    }
                    _ => panic!("")
                }
            }

            16 => {
                // BOOKMARK - cmp a1, [a2,] #mode: i8#
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(regstr_to_regno(&tk[i].clip));
                        byte_pointer += 1;
                        
                        next_ret_val = 17;
                        state_code = 3;
                    }
                    _ => panic!("")
                }
            }

            17 => {
                // BOOKMARK - cmp a1, a2, [#mode: i8#]
                match tk[i].t_type {
                    tokenizer::TokenType::Identifier => {
                        result.push(tk[i].clip.parse::<u8>().unwrap());
                        byte_pointer += 1;
                        
                        state_code = 0;
                    }
                    _ => panic!("")
                }
            }

            _ => {}
        }
        i += 1;
    }

    for i in need_to_fill_in {
        let sa = *symbol_table.get(i.symbol.as_str()).unwrap();
        result[i.addr as usize] = break_i32(sa, 0);
        result[i.addr as usize + 1] = break_i32(sa, 1);
        result[i.addr as usize + 2] = break_i32(sa, 2);
        result[i.addr as usize + 3] = break_i32(sa, 3);
    }
    result
}
