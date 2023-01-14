// would be 0xFE000000 for raspberry pi 4
pub const MMIO_BASE:            u32 = 0x3F000000;
pub const GPFSEL1:              u32 = MMIO_BASE + 0x00200004;
pub const GPPUD:                u32 = MMIO_BASE + 0x00200094;
pub const GPPUDCLK0:            u32 = MMIO_BASE + 0x00200098;
