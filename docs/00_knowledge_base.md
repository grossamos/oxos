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

## GNU Assembler
- format of instructions
```bash
mov $0, %vax
```
