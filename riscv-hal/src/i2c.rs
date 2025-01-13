use core::ptr;

use riscv_pac::pwm0::clock_ctrl;

//I2C Control register
const I2C_ACK:u32 = 0x01;
const I2C_STO:u32 = 0x02;
const I2C_STA:u32 = 0x04;
const I2C_ESO:u32 = 0x40;
const I2C_PIN:u32 = 0x80;

//I2C Start, Stop and Idle state
const I2C_START:u32 = I2C_PIN | I2C_ESO | I2C_STA | I2C_ACK;
const I2C_STOP :u32 = I2C_PIN | I2C_ESO | I2C_STO | I2C_ACK; 
const I2C_IDLE: u32 = I2C_ESO | I2C_ACK;

//I2C Clock Frequency
const CLOCK_FREQUENCY_BASE: u32 = 12_000_000;

//Memory mapped address for I2C0 and I2C1
const I2C0_BASE:usize = 0x0004_4000;
const I2C1_BASE:usize = 0x0004_4100;

const I2C_OFFSET: usize = 0x100;

#[derive(Debug)]
pub enum I2cError {
    InvalidInstance,
    NoAckForAddress,
    NoAckForData,
}

#[derive(Debug)]
#[repr(C)]
struct I2C_Register{
    prescalar:u32,
    _reserverd1:u32,
    control:u32,
    _reserved2:u32,
    data:u32,
    _reserved3:u32,
    status:u32,
    _reserved4:[u32; 7],
    clock_division:u32,
}

impl I2C_Register{
    fn get(i2c_instance:u8) -> &'static mut Self{
        let base_address = match i2c_instance{
            0 => I2C0_BASE,
            1 => I2C1_BASE,
            _ => panic!("Invalid I2C instance"),
        };
        unsafe { &mut *(base_address as *mut Self)}
    }

    pub fn init(instance_number:u8, clock_freq:u32)-> Result<(),I2cError>{
    
        let regs = I2C_Register::get(instance_number);
        regs.control = I2C_PIN;  // Disable serial interface, reset all status register
        let scl_div = CLOCK_FREQUENCY_BASE / ( 2 * 2 * clock_freq);
        regs.prescalar = 1;
        regs.clock_division = scl_div;
        regs.control = I2C_IDLE; // Enable serial interface
        Ok(())
    }

    pub fn transmit(&mut self, slave_address:u8, data: &[u8], start:bool, stop:bool)-> Result<(), I2cError>{
        if start{
            
            while self.status & 0x01 == 0 {} // wait for the i2c bus to be free
            
            // load the slave address in the data
            // I2C 8th bit is set 0 for write operation and 1 for read operation
            self.data = (slave_address as u32) << 1;
            self.control = I2C_START;
        }

        // wait for other devices to acknowledge the address
        while self.control & I2C_PIN != 0 {}

        if self.status & 0x08 == 0 {
            for &byte in data{
                self.data = byte as u32;
                // wait for other devices to acknowledge the data
                while self.control & I2C_PIN != 0 {}
                if self.status & 0x08 != 0 {
                    self.control = I2C_STOP;
                    return Err(I2cError::NoAckForData);
                }
            }
        }
        else {
            self.control = I2C_STOP;
            return Err(I2cError::NoAckForAddress)
        }

        if stop{
            self.control = I2C_STOP;
        }

        Ok(())
    }

    pub fn receive(&mut self, slave_address:u8, buffer:&mut[u8], start:bool, stop:bool)->Result<(), I2cError>{
        if start{
            while self.status & 0x01 == 0 {} // wait for the i2c bus to be free
            
            // load the slave address in the data
            // I2C 8th bit is set 0 for write operation and 1 for read operation
            self.data = ((slave_address as u32) << 1) | 1;
            self.control = I2C_START;
        }

        // wait for other devices to acknowledge the address
        while self.control & I2C_PIN != 0 {}

        if self.status & 0x08 == 0{
            for i in 0 .. buffer.len(){
                buffer[i] = self.data as u8;
                if i == buffer.len() - 1{
                    // Send NACK on last byte to end transmission
                    self.control = I2C_ESO; // NACK (do not set ACK)
                }
                else {
                    // Send ACK for each byte except the last one
                    self.control = I2C_ESO | I2C_ACK; // Keep ACK enabled
                }
                while self.control & I2C_PIN != 0 {}
            }
        }
        else {
            self.control = I2C_STOP;
            return Err(I2cError::NoAckForAddress)
        }

        if stop{
            self.control = I2C_STOP;
        }

        Ok(())
    }

}



