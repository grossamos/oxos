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
