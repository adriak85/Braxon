#include <stdint.h>

uint64_t braxon_build_tool_probe(uint64_t x) {
    return (x * 37u) ^ 0xBADC0FFEEu;
}

int main(void) {
    return (int)(braxon_build_tool_probe(7) & 0);
}
