#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "20 - qspi"]
    QSPI = 20,
    #[doc = "27 - uart0"]
    UART0 = 27,
    #[doc = "28 - uart1"]
    UART1 = 28,
    #[doc = "29 - uart2"]
    UART2 = 29,
    #[doc = "30 - i2c0"]
    I2C0 = 30,
    #[doc = "31 - i2c1"]
    I2C1 = 31,
    #[doc = "32 - i2c2"]
    I2C2 = 32,
    #[doc = "33 - spi0"]
    SPI0 = 33,
    #[doc = "34 - spi1"]
    SPI1 = 34,
    #[doc = "35 - spi2"]
    SPI2 = 35,
    #[doc = "40 - uart3"]
    UART3 = 40,
    #[doc = "41 - uart4"]
    UART4 = 41,
    #[doc = "42 - uart5"]
    UART5 = 42,
    #[doc = "43 - i2c3"]
    I2C3 = 43,
    #[doc = "44 - i2c4"]
    I2C4 = 44,
    #[doc = "45 - i2c5"]
    I2C5 = 45,
    #[doc = "46 - i2c6"]
    I2C6 = 46,
    #[doc = "47 - spi3"]
    SPI3 = 47,
    #[doc = "48 - spi4"]
    SPI4 = 48,
    #[doc = "49 - spi5"]
    SPI5 = 49,
    #[doc = "50 - spi6"]
    SPI6 = 50,
    #[doc = "106 - pmu"]
    PMU = 106,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            20 => Ok(Interrupt::QSPI),
            27 => Ok(Interrupt::UART0),
            28 => Ok(Interrupt::UART1),
            29 => Ok(Interrupt::UART2),
            30 => Ok(Interrupt::I2C0),
            31 => Ok(Interrupt::I2C1),
            32 => Ok(Interrupt::I2C2),
            33 => Ok(Interrupt::SPI0),
            34 => Ok(Interrupt::SPI1),
            35 => Ok(Interrupt::SPI2),
            40 => Ok(Interrupt::UART3),
            41 => Ok(Interrupt::UART4),
            42 => Ok(Interrupt::UART5),
            43 => Ok(Interrupt::I2C3),
            44 => Ok(Interrupt::I2C4),
            45 => Ok(Interrupt::I2C5),
            46 => Ok(Interrupt::I2C6),
            47 => Ok(Interrupt::SPI3),
            48 => Ok(Interrupt::SPI4),
            49 => Ok(Interrupt::SPI5),
            50 => Ok(Interrupt::SPI6),
            106 => Ok(Interrupt::PMU),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
