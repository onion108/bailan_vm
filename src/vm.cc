#include "vm.hh"

void vm_t::validate_regno(byte reg_no) {
        if (reg_no < 0 || reg_no >= 10) {
            throw std::invalid_argument("invalid register number");
        }
    }

int vm_t::concat_to_int(byte a_1, byte a_2, byte a_3, byte a_4) {
        return (a_1 << 24) + (a_2 << 16) + (a_3 << 8) + (a_4);
    }

int vm_t::content_of_reg(byte reg_no) {
        validate_regno(reg_no);
        return registers[reg_no];
    }

void vm_t::set_mem_at(unsigned int addr, byte val) {
        if (addr < ram.size()) {
            ram[addr] = val;
        } else {
            while(addr >= ram.size()) {
                ram.push_back(0);
            }
            ram[addr] = val;
        }
    }

    byte vm_t::get_mem_at(unsigned int addr) {
        return addr < ram.size() ? ram[addr] : 0;
    }

    void vm_t::write_to(std::vector<byte> data, unsigned start_addr) {
        int addr = start_addr;
        for (auto i : data) {
            set_mem_at(addr, i);
            addr++;
        }
    }

    void vm_t::jmp_to(unsigned int addr) {
        pc = addr;
    }

    void vm_t::call(unsigned int addr) {
        call_stack.push(pc);
        pc = addr;
    }

    void vm_t::ret() {
        pc = call_stack.top();
        call_stack.pop();
    }

    void vm_t::push(byte data) {
        mem_stack.push(data);
    }

    void vm_t::pop() {
        registers[10] = mem_stack.top();
        mem_stack.pop();
    } 

    void vm_t::mov(byte reg_no, int val) {
        validate_regno(reg_no);
        registers[reg_no] = val;
    }

    void vm_t::mov_reg(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[reg_no2] = reg_no1;
    }
    
    void vm_t::mov_ans(byte reg_no) {
        validate_regno(reg_no);
        registers[reg_no] = registers[10];
    }

    void vm_t::add(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] + registers[reg_no2];
    }

    void vm_t::sub(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] - registers[reg_no2];
    }

    void vm_t::mul(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] * registers[reg_no2];
    }

    void vm_t::div(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] / registers[reg_no2];
    }

    void vm_t::mod(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] % registers[reg_no2];
    }

    void vm_t::b_and(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] & registers[reg_no2];
    }

    void vm_t::b_or(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] | registers[reg_no2];
    }

    void vm_t::b_xor(byte reg_no1, byte reg_no2) {
        validate_regno(reg_no1);
        validate_regno(reg_no2);
        registers[10] = registers[reg_no1] ^ registers[reg_no2];
    }

    void vm_t::b_not(byte reg_no) {
        validate_regno(reg_no);
        registers[10] = ~registers[reg_no];
    }

    // TODO: more interrupts.
    void vm_t::intr(byte intr_code) {

        if (intr_code == 0x00) {
            // Output number intr.
            // r0: the number to output.
            cout << content_of_reg(0) << endl;
            return;
        }

        if (intr_code == 0x01) {
            // Read a number into the answer register.
            cin >> registers[10];
            return;
        }

        if (intr_code == 0x02) {
            // Output a character
            // r0: the character.
            cout << char(registers[0]);
            return;
        }

        if (intr_code == 0x03) {
            // Read a charater into the answer register.
            registers[10] = getchar();
            return;
        }

        if (intr_code == 0x05) {
            // Output pc's value
            cout << pc << endl;
        }

        // Otherwise, custom intr.
        call(intr_table[intr_code]);
        pc--;

    }

    void vm_t::rshift(byte reg_no) {
        validate_regno(reg_no);
        registers[reg_no] >>= 1;
    }

    void vm_t::lshift(byte reg_no) {
        validate_regno(reg_no);
        registers[reg_no] <<= 1;
    }

    void vm_t::regist_interrupt(byte intr_no, unsigned int addr) {
        intr_table[intr_no] = addr;
    }

    void vm_t::jmp_cond(unsigned int addr) {
        if (flag) {
            pc = addr;
        }
    }

    void vm_t::store(byte reg_no, byte reg_addr) {
        validate_regno(reg_no);
        validate_regno(reg_addr);
        set_mem_at(registers[reg_addr], registers[reg_no]);
    }

    void vm_t::load(byte reg_no, byte reg_addr) {
        validate_regno(reg_no);
        validate_regno(reg_addr);
        registers[reg_no] = get_mem_at(registers[reg_addr]);
    }

    void vm_t::compare(byte reg_no1, byte reg_no2, byte mode) {

        auto r1_cont = content_of_reg(reg_no1);
        auto r2_cont = content_of_reg(reg_no2);

        // TODO: implement compare;
        if (mode == 0x00) {
            // Equal
            flag = (r1_cont == r2_cont);
            return;
        }

        if (mode == 0x01) {
            // Non-Equal
            flag = (r1_cont != r2_cont);
            return;
        }

        if (mode == 0x02) {
            // Greater than
            flag = (r1_cont > r2_cont);
            return;
        }

        if (mode == 0x03) {
            // Less than
            flag = (r1_cont < r2_cont);
            return;
        }

        if (mode == 0x04) {
            // Greater or Equal
            flag = (r1_cont >= r2_cont);
            return;
        }

        if (mode == 0x05) {
            // Less than or Equal
            flag = (r1_cont <= r2_cont);
            return;
        }

    }

    void vm_t::exec_byte() {
        const int op_code = get_mem_at(pc);
        byte a_1, a_2, a_3, a_4, a_5;
        switch (op_code)
        {
        case 0x00: // nop
        break;

        case 0x01: // mov #num:i32#, a1 ;mov number to a1
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        a_3 = ram[++pc];
        a_4 = ram[++pc];
        a_5 = ram[++pc];
        mov(a_5, concat_to_int(a_1, a_2, a_3, a_4));
        break;

        case 0x02: // mov a1, a2 ; move a1 to a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        mov_reg(a_1, a_2);
        break;

        case 0x03: // mov_ans a1 ; move the content of the answer reg to the a1
        a_1 = ram[++pc];
        mov_ans(a_1);
        break;

        case 0x04: // intr #num:i8#
        a_1 = ram[++pc];
        intr(a_1);
        break;

        case 0x05: // intr a1
        a_1 = ram[++pc];
        intr(byte(content_of_reg(a_1)));

        case 0x06: // store a1, a2 ; store a1's content to the address stored in a2.
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        store(a_1, a_2);
        break;

        case 0x07: // load a1, a2 ; Same as store, but load.
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        load(a_1, a_2);
        break;

        case 0x08: // add a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        add(a_1, a_2);
        break;

        case 0x09: // sub a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        sub(a_1, a_2);
        break;

        case 0x0A: // mul a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        mul(a_1, a_2);
        break;

        case 0x0B: // div a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        div(a_1, a_2);
        break;

        case 0x0C: // mod a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        mod(a_1, a_2);
        break;

        case 0x0D: // and a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        b_and(a_1, a_2);
        break;

        case 0x0E: // or a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        b_or(a_1, a_2);
        break;

        case 0x0F: // xor a1, a2
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        b_xor(a_1, a_2);
        break;

        case 0x10: // not a1
        a_1 = ram[++pc];
        b_not(a_1);
        break;

        case 0x11: // jmp a1
        a_1 = ram[++pc];
        jmp_to(content_of_reg(a_1));
        return;
        
        case 0x12: // jmp #num:i32#
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        a_3 = ram[++pc];
        a_4 = ram[++pc];
        jmp_to(concat_to_int(a_1, a_2, a_3, a_4));
        return;

        case 0x13: // lsf a1
        a_1 = ram[++pc];
        lshift(a_1);
        break;

        case 0x14: // rsf a1
        a_1 = ram[++pc];
        rshift(a_1);
        break;

        case 0x15: // jmp_cond a1
        a_1 = ram[++pc];
        jmp_cond(content_of_reg(a_1));
        if (flag) {
            return;
        }
        break;

        case 0x16: // jmp_cond #num:i32#
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        a_3 = ram[++pc];
        a_4 = ram[++pc];
        jmp_cond(concat_to_int(a_1, a_2, a_3, a_4));
        if (flag) {
            return;
        }
        break;

        case 0x17: // stop
        throw -1;
        break;

        case 0x18: // cmp a1, a2, #mode: i8#
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        a_3 = ram[++pc];
        compare(a_1, a_2, a_3);
        break;

        case 0x19: // call #num:i32#
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        a_3 = ram[++pc];
        a_4 = ram[++pc];
        call(concat_to_int(a_1, a_2, a_3, a_4));
        return;

        case 0x20: // call a1
        a_1 = ram[++pc];
        call(content_of_reg(a_1));
        return;

        case 0x21: // ret
        ret();
        return;

        case 0x22: // push #num:i8#
        a_1 = ram[++pc];
        push(a_1);
        break;

        case 0x23: // push a1
        a_1 = ram[++pc];
        push(content_of_reg(a_1));
        break;

        case 0x24: // pop
        pop();
        break;

        case 0x25: // rin #num:i8# #num:i32# ; Register the interrupt. The first is the intr number and the second is the addr.
        a_1 = ram[++pc];
        a_2 = ram[++pc];
        a_3 = ram[++pc];
        a_4 = ram[++pc];
        a_5 = ram[++pc];
        regist_interrupt(a_1, concat_to_int(a_2, a_3, a_4, a_5));
        break;

        default:
        throw std::runtime_error("ud2: undefined instruction.");
        break;
        }
        pc++;
    }

    void vm_t::boot() {
        pc = 0x7c00; // Load MBR.
        while(1) {
            try {
                exec_byte();
                // printf("0x%08X\n", pc);
                // printf("%d\n", content_of_reg(5));
            } catch(int e) {
                if (e == -1) {
                    break;
                }
            }
        }
    }
