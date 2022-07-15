#ifndef C914F58E_7BA3_4F91_BBEC_F0750860CEDD
#define C914F58E_7BA3_4F91_BBEC_F0750860CEDD

#include <iostream>
#include <vector>
#include <stack>
#include <map>
#include <stdexcept>

using std::cin;
using std::cout;
using std::endl;

typedef unsigned char byte;

class vm_t {
    std::stack<byte> call_stack;
    std::stack<byte> mem_stack;
    std::vector<byte> ram;
    std::map<byte, unsigned int> intr_table;
    unsigned int pc;
    int registers[11] = {0};
    bool flag = false;

    void validate_regno(byte reg_no);

    int concat_to_int(byte a_1, byte a_2, byte a_3, byte a_4);

    int content_of_reg(byte reg_no);

    public:

    vm_t() : ram(1024) {}

    void set_mem_at(unsigned int addr, byte val);

    byte get_mem_at(unsigned int addr);

    void write_to(std::vector<byte> data, unsigned start_addr);

    void jmp_to(unsigned int addr);

    void call(unsigned int addr);

    void ret();

    void push(byte data);

    void pop();

    void mov(byte reg_no, int val);

    void mov_reg(byte reg_no1, byte reg_no2);
    
    void mov_ans(byte reg_no);

    void add(byte reg_no1, byte reg_no2);

    void sub(byte reg_no1, byte reg_no2);

    void mul(byte reg_no1, byte reg_no2);

    void div(byte reg_no1, byte reg_no2);

    void mod(byte reg_no1, byte reg_no2);

    void b_and(byte reg_no1, byte reg_no2);

    void b_or(byte reg_no1, byte reg_no2);

    void b_xor(byte reg_no1, byte reg_no2);

    void b_not(byte reg_no);

    // TODO: more interrupts.
    void intr(byte intr_code);

    void rshift(byte reg_no);

    void lshift(byte reg_no);

    void regist_interrupt(byte intr_no, unsigned int addr);

    void jmp_cond(unsigned int addr);

    void store(byte reg_no, byte reg_addr);

    void load(byte reg_no, byte reg_addr);

    void compare(byte reg_no1, byte reg_no2, byte mode);

    void exec_byte();

    void boot();

};

#endif /* C914F58E_7BA3_4F91_BBEC_F0750860CEDD */
