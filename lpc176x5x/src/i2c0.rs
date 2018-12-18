#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Set Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is set. Writing a zero has no effect on the corresponding bit in the I2C control register."]
    pub conset: CONSET,
    #[doc = "0x04 - I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed."]
    pub stat: STAT,
    #[doc = "0x08 - I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register."]
    pub dat: DAT,
    #[doc = "0x0c - I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
    pub adr0: ADR0,
    #[doc = "0x10 - SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock."]
    pub sclh: SCLH,
    #[doc = "0x14 - SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. SCLL and SCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode."]
    pub scll: SCLL,
    #[doc = "0x18 - I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register."]
    pub conclr: CONCLR,
    #[doc = "0x1c - Monitor mode control register."]
    pub mmctrl: MMCTRL,
    #[doc = "0x20 - I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
    pub adr1: ADR,
    #[doc = "0x24 - I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
    pub adr2: ADR,
    #[doc = "0x28 - I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
    pub adr3: ADR,
    #[doc = "0x2c - Data buffer register. The contents of the 8 MSBs of the DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus."]
    pub data_buffer: DATA_BUFFER,
    #[doc = "0x30 - I2C Slave address mask register"]
    pub mask: [MASK; 4],
}
#[doc = "I2C Control Set Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is set. Writing a zero has no effect on the corresponding bit in the I2C control register."]
pub struct CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Control Set Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is set. Writing a zero has no effect on the corresponding bit in the I2C control register."]
pub mod conset;
#[doc = "I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed."]
pub mod stat;
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register."]
pub struct DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register."]
pub mod dat;
#[doc = "I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
pub struct ADR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Address Register 0. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
pub mod adr0;
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock."]
pub struct SCLH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock."]
pub mod sclh;
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. SCLL and SCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode."]
pub struct SCLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. SCLL and SCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode."]
pub mod scll;
#[doc = "I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register."]
pub struct CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register."]
pub mod conclr;
#[doc = "Monitor mode control register."]
pub struct MMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor mode control register."]
pub mod mmctrl;
#[doc = "I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
pub struct ADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address."]
pub mod adr;
#[doc = "Data buffer register. The contents of the 8 MSBs of the DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus."]
pub struct DATA_BUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data buffer register. The contents of the 8 MSBs of the DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus."]
pub mod data_buffer;
#[doc = "I2C Slave address mask register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave address mask register"]
pub mod mask;