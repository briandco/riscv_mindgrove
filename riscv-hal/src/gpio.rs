use riscv_pac::Gpio;

pub struct GpioPin {
    port: &'static Gpio, // Replace with actual PAC GPIO module
    pin: u8,
}

impl GpioPin{
    pub fn new(port:&'static Gpio, pin:u8) -> Self{
        Self{port, pin}
    }

    pub fn set_output(&self){
        // Set pin to output
        self.port.gpio_direction().write(|w| unsafe {w.bits(0 << self.pin)});
    }

    pub fn set_input(&self){
        // Set pin to input
        self.port.gpio_direction().write(|w| unsafe {w.bits(1 << self.pin)});
    }

    pub fn set_high(&self){
        // Write to the GpioSet register to set the pin
        self.port.gpio_set().write(|w| unsafe{w.bits(1 << self.pin)});
    }

    pub fn set_low(&self){
        // Write to the GpioSet register to clear the pin
        self.port.gpio_clear().write(|w| unsafe{w.bits(1 << self.pin)});
    }

    pub fn gpio_read(&self)->bool{
        // Read the gpio_data
        let value = self.port.gpio_data().read().bits() & (1 << self.pin);
        value != 0
    }

    pub fn gpio_intr(&self){
        
    }
}