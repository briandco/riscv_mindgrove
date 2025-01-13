use core::ptr::{read_volatile, write_volatile};

// Base address for spi instances
const SPI0_BASE:usize = 0x00020000;
const SPI1_BASE:usize = 0x00020100;
const SPI2_BASE:usize = 0x00020200;
const SPI3_BASE:usize = 0x00020300;

// Offsets for SPI registers relative to base address
const SPI_CTRL_OFFSET       :usize = 0x0000;
const SPI_CLK_CTRL_OFFSET   :usize = 0x0004;
const SPI_TX_OFFSET         :usize = 0x0008;
const SPI_RX_OFFSET         :usize = 0x000C;
const SPI_INTR_EN_OFFSET    :usize = 0x0010;
const SPI_FIFO_STATUS_OFFSET:usize = 0x0014;
const SPI_COMM_STATUS_OFFSET:usize = 0x0018;

// Bit mask for SPI status flags
const SPI_TX_FULL : u32 = 1 << 0;  // TX FIFO is full
const SPI_TX_EMPTY: u32 = 1 << 1;  // TX FIFO is empty
const SPI_RX_EMPTY: u32 = 1 << 2;  // RX FIFO is empty

// Functions for accesseing memory mapped register safely
unsafe fn mmio_read32(addr: usize) -> u32{
    read_volatile(addr as *const u32)
}

unsafe fn mmio_write32(addr: usize, value: u32){
    write_volatile(addr as *mut u32, value);
}

// Configuration structure for SPI with relevant parameters
#[derive(Debug, Copy, Clone)]
struct SPIConfig{
    base_address:usize, // Base address for SPI instance
    mode        :u8,    // SPI communication mode
    clk_polarity:bool,  // Clock polarity: true for high idle and false for low idle
    clk_phase   :bool,  // Clock phase: true for sampling on the second edge
    prescaler   :u8,    // Clock prescaler for frequency control
} 

impl SPIConfig{
    /// Creates a new SPI configuration with the specified parameters.
    /// Panics if the instance number is invalid.
}