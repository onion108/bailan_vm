#include <iostream>
#include <vector>
#include <stack>
#include <map>
#include <stdexcept>
#include "vm.hh"

using std::cin;
using std::cout;
using std::endl;

int main() {
    vm_t computer;
    computer.write_to(std::vector<byte> {0x04,0x01,0x03,0x00,0x01,0x00,0x00,0x00,0x00,0x07,0x18,0x00,0x07,0x00,0x16,0x00,0x00,0x7C,0x5D,0x01,0x00,0x00,0x00,0x01,0x07,0x18,0x00,0x07,0x00,0x16,0x00,0x00,0x7C,0x5D,0x01,0x00,0x00,0x00,0x02,0x07,0x0C,0x00,0x07,0x03,0x01,0x01,0x00,0x00,0x00,0x00,0x08,0x18,0x01,0x08,0x00,0x16,0x00,0x00,0x7C,0x5D,0x01,0x00,0x00,0x00,0x01,0x08,0x08,0x07,0x08,0x03,0x07,0x18,0x07,0x00,0x03,0x16,0x00,0x00,0x7C,0x28,0x01,0x00,0x00,0x00,0x01,0x00,0x04,0x00,0x12,0x00,0x00,0x7C,0x6A,0x01,0x00,0x00,0x00,0x00,0x00,0x04,0x00,0x12,0x00,0x00,0x7C,0x6A,0x17,}, 0x7c00); 
    computer.boot();
}

