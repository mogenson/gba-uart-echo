#![allow(dead_code)]
use gba::{newtype, newtype_enum};
use gba_proc_macro::phantom_fields;
use voladdress::VolAddress;

pub const SIOCNT: VolAddress<SioControlSetting> = unsafe { VolAddress::new(0x400_0128) };
pub const SIODATA8: VolAddress<u16> = unsafe { VolAddress::new(0x400_012A) };
pub const RCNT: VolAddress<IoControlSetting> = unsafe { VolAddress::new(0x400_0134) };

newtype!(SioControlSetting, u16);

impl SioControlSetting {
    phantom_fields! {
        self.0: u16,
        baud_rate: 0-1=BaudRate<Bps9600,Bps38400,Bps57600,Bps115200>,
        flow_control: 2,
        parity_odd: 3,
        tx_full: 4,
        rx_empty: 5,
        error: 6,
        data_length_8bit: 7,
        fifo_enable:8,
        parity_enable: 9,
        tx_enable: 10,
        rx_enable: 11,
        mode: 12-13=SioMode<Normal8Bit,MultiPlayer,Normal32Bit,Uart>,
        irq_enable: 14,
    }
}

newtype_enum! {
    BaudRate = u16,
    Bps9600 = 0,
    Bps38400 = 1,
    Bps57600 = 2,
    Bps115200 = 3,
}

newtype_enum! {
    SioMode = u16,
    Normal8Bit = 0,
    MultiPlayer = 1,
    Normal32Bit = 2,
    Uart = 3,
}

newtype!(IoControlSetting, u16);

impl IoControlSetting {
    phantom_fields! {
        self.0: u16,
        sc: 0,
        sd: 1,
        si: 2,
        so: 3,
        sc_output_enable: 4,
        sd_output_enable: 5,
        si_output_enable: 6,
        so_output_enable: 7,
        si_irq_enable: 8,
        mode: 14-15=IoMode<Disabled,Invalid,GPIO,JoyBus>,
    }
}

newtype_enum! {
    IoMode = u16,
    Disabled = 0,
    Invalid = 1,
    GPIO = 2,
    JoyBus = 3,
}

pub fn init(baud: BaudRate) {
    RCNT.write(IoControlSetting::new());
    SIOCNT.write(
        SioControlSetting::new()
            .with_baud_rate(baud)
            .with_data_length_8bit(true)
            .with_mode(SioMode::Uart)
            .with_rx_enable(true)
            .with_tx_enable(true),
    );
}

pub fn getc() -> u8 {
    while SIOCNT.read().rx_empty() {}
    SIODATA8.read() as u8
}

pub fn putc(c: u8) {
    while SIOCNT.read().tx_full() {}
    SIODATA8.write(c as u16);
}
