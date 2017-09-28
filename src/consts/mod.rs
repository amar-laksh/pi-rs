#![allow(dead_code)]
pub const OUTPUT: i8 = 1;
pub const INPUT: i8 = 0;
pub const ALT0: i8 = 4;
pub const ALT1: i8 = 5;
pub const ALT2: i8 = 6;
pub const ALT3: i8 = 7;
pub const ALT4: i8 = 3;
pub const ALT5: i8 = 2;

pub const HIGH: i8 = 1;
pub const LOW: i8 = 0;

//: u32 = GPIO Function selects ;
pub const GPFSEL0: u32 = 0x20200000;
pub const GPFSEL1: u32 = 0x20200004;
pub const GPFSEL2: u32 = 0x20200008;
pub const GPFSEL3: u32 = 0x2020000C;
pub const GPFSEL4: u32 = 0x20200010;
pub const GPFSEL5: u32 = 0x20200014;

//: u32 = GPIO Pin Output Sets;
pub const GPSET0: u32 =  0x2020001C;
pub const GPSET1: u32 =  0x20200020;

//: u32 = GPIO Pin Output Clears;
pub const GPCLR0: u32 =  0x20200028;
pub const GPCLR1: u32 =  0x2020002C;

//: u32 = GPIO Pin Levels;
pub const GPLEV0: u32 =  0x20200034;
pub const GPLEV1: u32 =  0x20200038;

//: u32 = GPIO Event Detect Status;
pub const GPEDS0: u32 =  0x20200040;
pub const GPEDS1: u32 =  0x20200044;

//: u32 = GPIO Pin Rising Edge Detect Enables;
pub const GPREN0: u32 =  0x2020004C;
pub const GPREN1: u32 =  0x20200050;

//: u32 = GPIO Pin Falling Edge Detect Enables;
pub const GPFEN0: u32 =  0x20200058;
pub const GPFEN1: u32 =  0x2020005C;

//: u32 = GPIO Pin Falling High Detect Enables;
pub const GPHEN0: u32 =  0x20200064;
pub const GPHEN1: u32 =  0x20200068;

//: u32 = GPIO Pin Falling Low Detect Enables;
pub const GPLEN0: u32 =  0x20200070;
pub const GPLEN1: u32 =  0x20200074;

//: u32 = GPIO Pin Async: Rising Edge Detect Enables;
pub const GPAREN0: u32 =  0x2020007C;
pub const GPAREN1: u32 =  0x20200080;

//: u32 = GPIO Pin Async: Falling Edge Detect Enables;
pub const GPAFEN0: u32 =  0x20200088;
pub const GPAFEN1: u32 =  0x2020008C;

//: u32 = GPIO Pin Pull-up/down Enable;
pub const GPPUD: u32 =  0x20200094;

//: u32 = GPIO Pin Pull-up/down Enable Clocks;
pub const GPPUDCLK0: u32 =  0x20200098;
pub const GPPUDCLK1: u32 =  0x2020009C;

pub const TEST: u32 =  0x202000B0;

pub const AUX_IRQ:u32 = 0x20215000;
pub const AUX_ENABLES:u32 = 0x20215004;
pub const AUX_MU_IO_REG:u32 = 0x20215040;
pub const AUX_MU_IER_REG:u32 = 0x20215044;
pub const AUX_MU_IIR_REG :u32 = 0x20215048;
pub const AUX_MU_LCR_REG :u32 = 0x2021504C;
pub const AUX_MU_MCR_REG :u32 = 0x20215050;
pub const AUX_MU_LSR_REG :u32 = 0x20215054;
pub const AUX_MU_MSR_REG :u32 = 0x20215058;
pub const AUX_MU_SCRATCH :u32 = 0x2021505C;
pub const AUX_MU_CNTL_REG:u32 = 0x20215060;
pub const AUX_MU_STAT_REG:u32 = 0x20215064;
pub const AUX_MU_BAUD_REG:u32 = 0x20215068;

pub const UART_BASE:u32 = 0x20201000;
pub const UART_DR:u32 = 0x20201000;
pub const UART_RSRECR:u32 = 0x20201004;
pub const UART_FR:u32 = 0x20201018;
pub const UART_ILPR:u32 = 0x20201020;
pub const UART_IBRD:u32 = 0x20201024;
pub const UART_FBRD:u32 = 0x20201028;
pub const UART_LCRH:u32 = 0x2020102C;
pub const UART_CR:u32 = 0x20201030;
pub const UART_IFLS:u32 = 0x20201034;
pub const UART_IMSC:u32 = 0x20201038;
pub const UART_RIS:u32 = 0x2020103C;
pub const UART_MIS:u32 = 0x20201040;
pub const UART_ICR:u32 = 0x20201044;
pub const UART_DMACR:u32 = 0x20201048;
pub const UART_ITCR:u32 = 0x20201080;
pub const UART_ITIP:u32 = 0x20201084;
pub const UART_ITOP:u32 = 0x20201088;
pub const UART_TDR:u32 = 0x2020108C;