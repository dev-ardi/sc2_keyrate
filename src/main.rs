extern crate winapi;
use winapi::um::winuser::{SystemParametersInfoW, SPI_GETFILTERKEYS, SPI_SETFILTERKEYS};

#[repr(C)]
struct FILTERKEYS {
    cbSize: u32,
    dwFlags: u32,
    iWaitMSec: u32,
    iDelayMSec: u32,
    iRepeatMSec: u32,
    iBounceMSec: u32,
}
fn get_repeat_delay() -> anyhow::Result<(u32, u32)> {
    let mut fk = FILTERKEYS {
        cbSize: std::mem::size_of::<FILTERKEYS>() as u32,
        dwFlags: 0,
        iWaitMSec: 0,
        iDelayMSec: 0,
        iRepeatMSec: 0,
        iBounceMSec: 0,
    };

    let success = unsafe {
        SystemParametersInfoW(
            SPI_GETFILTERKEYS,
            fk.cbSize,
            &mut fk as *mut FILTERKEYS as *mut _,
            0,
        ) != 0
    };

    if success {
        Ok((fk.iRepeatMSec, fk.iDelayMSec))
    } else {
        Err(anyhow::anyhow!("Failed to retrieve FILTERKEYS settings"))
    }
}
fn set_repeat_delay(repeat: u32, delay: u32) -> anyhow::Result<()> {
    let mut fk = FILTERKEYS {
        cbSize: std::mem::size_of::<FILTERKEYS>() as u32,
        dwFlags: 1 | 2,
        iWaitMSec: 0,
        iDelayMSec: delay,
        iRepeatMSec: repeat,
        iBounceMSec: 0,
    };

    let success = unsafe {
        SystemParametersInfoW(
            SPI_SETFILTERKEYS,
            fk.cbSize,
            &mut fk as *mut FILTERKEYS as *mut _,
            0,
        ) != 0
    };

    if success {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to set FILTERKEYS settings"))
    }
}

fn parse_from_stdin() -> Option<u32> {
    let mut data = String::new();
    loop {
        std::io::stdin().read_line(&mut data).unwrap();
        data = data.trim().to_owned();
        if data == "" {
            return None;
        }
        match data.parse::<u32>() {
            Ok(parsed) => return Some(parsed),
            Err(e) => {
                eprint!("{data} is not a valid number.\n{e}\n> ");
                data.clear()
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    loop {
        let (repeat, delay) = get_repeat_delay()?;

        println!("Current delay rate is {delay}ms.");
        print!("Type desired delay rate and press enter (leave blank to skip)\n> ");
        let delay = parse_from_stdin().unwrap_or(delay);

        println!("Current repeat rate rate is {repeat}ms.");
        print!("Type desired repeat rate rate and press enter (leave blank to skip)\n> ");
        let repeat = parse_from_stdin().unwrap_or(repeat);

        set_repeat_delay(repeat, delay)?;

        println!("Done! you can exit the program with Control C")
    }
}
