# Knowledge Base

## System V ABI
- generic ABI for calling conventions, executables, etc in unix systems
- ELF is part of system V

## ARM
- ARM Architecture is a family of architechtures, each company and use case has its own flavor
- ARM has a *Unified Assembly Language* which can be translataed to any ARM core
- arm cores can be devided into three types
    - Cortex-M -> really small
    - Cortex-R -> real time devices
    - Cortex-A -> smartphones, PCs, TVs
- ARM only has prviliged and unpriviledged modes
    - Privileged is plit up into: fiq, irq, svc, abt, sys, und => defined by state of the proxessir when a particular exception occurs

- ARM is bi-endian (meaning both little and big endian are allowed) -> bit 9 in CPSR register
- reading <https://azeria-labs.com/writing-arm-assembly-part-1/>

### Data types
- can be signed or unsigned
- three types:
    - byte (8)
    - word (32)
    - halfwork (16)

- they have different endings:
    - byte: `-b`, `-sb`
    - half word: `-h`, `-sh`
    - word: nothing
- ex. loading signed byte: `ldsb`

### Registers
- R0-R6: General purpose
- R7: Holds Syscalls
- R8-R10: General purpose
- R11 (FP alias): Frame Pointer
- *Special purpose registers:*
- R12 (IP alias): Intra Procedure Call
- R13 (SP alias): Stack Pointer
- R14 (LR alias): Link Register
- R15 (PC alias): Programm Counter
- CPSR: Current Programm Status Counter

- R0-R12: can be used with general instructions
- SP: points to begining of Stack
- LR: stores address from function call to go back to
- PC: automatically incremented programm counter (points to 4/8 bytes after the current instruction)

#### CPSR structure
- N: negative? (1 if true)
- Z: zero? (1 if true)
- C: Carry 
- V: Overflow flag
- E: endian bit (0 is little endian)
- T: Thumb state (1 is true, 0 is arm state)
- M: current privilege level
- J: execution state that allows execution of java byte code

### Instruction set
#### Modes of operation
- arm and Thumb
- thumb is basically a compact version of the regular instruction set
- thum uses 16 bit shorthands for the 32 bit arm commands

#### Structure of instruction
- 31-28: Condition
- 27-25: Operand type 
- 24-21: Opcode
- 20: Set condition code (indicate if it should update the CPSR)
- 19-16: Operand register
- 15-12: Destination Register
- 11-0: Immediate operand (for constants and stuff)

- instructions take three clock cycles to execute but are parallelised with a pipeline -> takes one clock cycle per instruction
- neg numbers can be used via the two's compliment
- ARM is bi-endian -> uses both and can be set via the CPSR
- shifting in arm is done as a side effect of other instructions like mov
- in arm you can shift logically and arithmetically
    - logic just shifts
    - arithmetic preserves the sign present
    - rotate right -> bits dont fall off but wrap around at the other end 

#### MOV
1. `MOV RD, $imm16` -> simplest move, can only take 16 bit immediate values
2. `MOVT RD, $imm16` -> simple move, moves 16 bit value into upper 16 bits
    - filling all 32 bits of a register requires one `MOV` and one `MOVT` instruction
3. `MOV RD, RD`
4. `MOV RD, operand2` 
    - two formats: (1) a register and a shift, (2) small number and a rotation
    - ex. `MOV R1, R2, LSL #1` shift r2 by one and move result to r1
    - options are: LSR, ASR, ROR, LSL, RRX etc.
    - ROR = rotate right
    - RRX = rotate extended right
    - number provided can be 8 bit
    - for mov and shift shorthands exist: `LSL R1, R2, #1`
5. `MVN RD, operand2`
    - like MOV but it reverses 1s and 0s (aka not)
    - can be used to multiply by -1
#### ADD/ADC
1. `ADD{S} RD, RS, Operand2`
2. `ADD{S} RD, RS, #imml2`
3. `ADD{S} RD, RS1, RS2`
4. `ADC{S} RD, RS, Operand2`
5. `ADC{S} RD, RS1, RS2`

- Allways put reults in RD
- `ADD R1, #1` is equal to `ADD R1, R1, #1`
- `ADDS` changes the CPSR carry flag
- you can then use the `ADC` command to use carry

### Controll Flow
- unconditional branch (like jump) -> `b label`
- allows jumps of up to 32MB
- conditional branch is structured as follows: `b{condition} label`
    - EQ -> zero flag is set
    - NE -> zero flag is not set
    - CS/HS -> c is set
    - CC/LO -> c is clear
    - etc.

#### CMP instruction
- format of `CMP RN, operand2`
- works just like SUBS or ADDS, just it doesn't update the registers
- ex. `CMP R4, $45`
- reverse for loop could look as follwos:
```asm 
        MOV R2, $10
loop:
        SUBS R2, $1 
        BNE loop
```

### Coprocessor
- there are multiple additions to ARM, these are on the actuall chip
- muliple ones exist, p15 for example is used in the pi 
- CP15 is the system controll coprocessor (helps with stuff like the MMU)
    - can have up to 16 32bit registers
    - registers can be called by c0-c15
    - the can be read using `MRC p15, op1, RT, CRN, CRM, Op2`
    - they can be written to using `MCR p15, op1, RT, CRN, CRM, op2`
    - ex `MRC, p15, 0, R1, c0, c0, 0` reads register c0 to r1

### Logical operatiors
- format `OPERATOR{S} RD, RS, operand2`
- AND, EOR (exclusive OR), ORR (regular or), BIC (bit clear, does RS AND NOT operand2, sometimes used to endcode operand2s not possible with AND)
- ex. `AND r5, r5, $1` runs and of r5 and 1

### Memory management
- data can be defined using `.byte`, `.word` and `.ascii`
- ex. `label: .byte 74, 0112, 0b00010001, 0x4A, 'J', 'h' + 1`
- `-` will take the twos compliment and `~` will take the ones compliment of the specified number
- we can definie larger sets of memory using: `.fill repeat, size, value` 
- ex. `.fill 10, 4, 0` creates a block of memory of 10 4 byte words all with 0s
- you can also repeate data:
```asm
.rept count 
...
.endr
```
- loading is done via `LDR{type} RD, =label`
- type can be: `B`, `SB`, `H`, `SH` or empty for word
- you can also load via register indexing: `LDR{type}, RD [RS, $N]` loads what is in RS + $N to RD 
- this can also be done with two registers `LDR{type}, RD [R1, R2]`
- wrighting is done by ending it in a `!`, ex. `LDR R2 [R1, R3, LSL $2]!`

### Raspberry pi specifics
- ARMv7-A conatins special store multiple increment after (stmia) operations that can be used to store bigger chunks of data

## GNU Assembler
- format of instructions: `label opcode operands`
```bash
mov $0, %vax
```

- we have a `arm-none-eabi` compiler, which is designed to run its programms as a operating system
- the `arm-linux-gnueabihf` compiler can be used for creating programms for linux


## GDB
- run qemu with the `-s -S` flags
- ex. `qemu-system-arm -M raspi0 -d in_asm -nographic -kernel boot -d cpu -s -S`
- compile the code with debugging info with `as -g`
- open gdb with `gdb boot`
- connect via `target remote localhost:1234`
- list code with `l` and go to next line with `n` print registers with `i r`
- view memory with `x /Nfu`
    - N is number of objkects to display
    - f is display format (t == binary, x == hel, d == decimal, i == instruction, s == string)
    - u is size (b == byte, h == 16 bit, w == 32 bit, g == 64 bit)
    - example: `x /4ubft _start` oder `x /4ubfi _start`
- set breakpoint at line `b 88`
- step `s`
- continue `c`
- delete breakpoint n `delete n`

## Current commands
- compile with `arm-none-eabi-gcc -o boot.o -c boot.s -g -mcpu=cortex-a7 -ffreestanding -fpic`
