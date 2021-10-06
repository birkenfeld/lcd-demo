#![no_main]
#![no_std]

use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{pac, delay::Delay, spi::Spi};
use st7735_lcd::ST7735;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let core = pac::CorePeripherals::take().unwrap();
    let peri = pac::Peripherals::take().unwrap();
    let mut rcc = peri.RCC.constrain();
    let mut flash = peri.FLASH.constrain();
    let mut afio = peri.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = peri.GPIOA.split(&mut rcc.apb2);

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .pclk1(36.mhz())
        .sysclk(72.mhz())
        .freeze(&mut flash.acr);

    let dc   = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
    let rst  = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    let sclk = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let spi = Spi::spi1(
        peri.SPI1, (sclk, miso, mosi), &mut afio.mapr,
        embedded_hal::spi::MODE_0, 16.mhz(), clocks, &mut rcc.apb2
    );

    let mut display = ST7735::new(spi, dc, rst, true, false, 160, 128);
    let mut delay = Delay::new(core.SYST, clocks);
    display.init(&mut delay).expect("init display failed");

    loop {
        cortex_m::asm::wfi();
    }
}
