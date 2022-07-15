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
    computer.write_to(std::vector<byte> {0x04,0x01,0x03,0x00,0x04,0x01,0x03,0x01,0x08,0x00,0x01,0x03,0x00,0x04,0x00,0x17,}, 0x7c00); 
    computer.boot();
}

