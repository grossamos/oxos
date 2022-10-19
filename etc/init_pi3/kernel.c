#include <stddef.h>
#include <stdint.h>

void kernel_main(uint32_t r0, uint32_t r1, uint32_t atags) {
    register uint32_t counter = 99;
    while (1)
        counter++;
}
