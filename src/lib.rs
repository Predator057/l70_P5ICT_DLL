use std::time::Duration;
use std::thread::sleep;
use windows::{Win32::{Foundation::*, System::SystemServices::*}, core::{s}};
use serialport::{self, DataBits, Parity, StopBits};


//static SEND_BILL_VALIDATED: u8 = 0x81;
//static BILL_TYPE1: u8 = 0x40;
//static BILL_TYPE2: u8 = 0x41;
//static BILL_TYPE3: u8 = 0x42;
//static BILL_TYPE4: u8 = 0x43;
//static BILL_TYPE5: u8 = 0x44;
static ACCEPT: u8 = 0x02;
//static HOLD: u8 = 0x18;
//static STACKING: u8 = 0x10;
static RESET: u8 = 0x30;
//static CHECKSTATUS: u8 = 0x0C;
static ENABLE: u8 = 0x3E;
static DISABLE: u8 = 0x5E;
//static POWER_SUPPLY_ON1: u8 = 0x80;
//static POWER_SUPPLY_ON2: u8 = 0x8F;
#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
        }
        DLL_PROCESS_DETACH => {
            DisableAcceptor();
        }
        _ => (),
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "C" fn InitAcceptor() -> i16{
    let port_nm = String::from("COM6");

    let port_result = serialport::new(port_nm, 9600)
        .timeout(Duration::from_millis(60))
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open().map_err(|e|{
            return -2;
        });
    
    match port_result {
        Ok(mut port) => {
            let buf: [u8; 1] = [RESET];
            let _ = port.write(&buf);

            loop{
                let mut buffer = vec![0; 128];
                match port.read(buffer.as_mut_slice()){
                    Ok(_)=>{
                        for i in 0..128{
                            if buffer[i] == 0x80 || buffer[i] == 0x8F{
                                let buf: [u8; 1] = [ACCEPT];
                                let _ = port.write(&buf);
                                let buf: [u8; 1] = [DISABLE];
                                let _ = port.write(&buf);
                                return 0;
                            }
                        }
                    }
                    Err(_) =>{}
                }
                sleep(Duration::from_millis(100));
            }
        }
        Err(e)=>{
            return e
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "C" fn StartAcceptor() -> i16{
    let port_nm = String::from("COM6");

    let port_result = serialport::new(port_nm, 9600)
        .timeout(Duration::from_millis(60))
        .data_bits(serialport::DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open().map_err(|e|{
            return -2;
        });

    match port_result {
        Ok(mut port) =>{
            
            let buf: [u8; 1] = [ENABLE];
            let _ = port.write(&buf);
            loop{
                //01 - 50  руб
                //02 - 100 руб
                //03 - 500 руб
                //04 - 1000 руб
                let mut buffer = vec![0; 128];
                match port.read(buffer.as_mut_slice()){
                    Ok(_)=>{
                        for i in 0..128{
                            if buffer[i] == 0x80 || buffer[i] == 0x8F{
                                let buf: [u8; 1] = [0x02];
                                let _ = port.write(&buf);
                            }
                            match buffer[i]{
                                0x41 => {
                                    let buf: [u8; 1] = [0x02];
                                    let _ = port.write(&buf);
                                    return 50;
                                }
                                0x42 => {
                                    let buf: [u8; 1] = [0x02];
                                    let _ = port.write(&buf);
                                    return 100;
                                }
                                0x43 => {
                                    let buf: [u8; 1] = [0x02];
                                    let _ = port.write(&buf);
                                    return 500;
                                }
                                0x44 => {
                                    let buf: [u8; 1] = [0x02];
                                    let _ = port.write(&buf);
                                    return 1000;
                                }
                                0x45 => {
                                    let buf: [u8; 1] = [0x02];
                                    let _ = port.write(&buf);
                                    return 5000;
                                }
                                0x81 => {
                                    let buf: [u8; 1] = [0x02];
                                    let _ = port.write(&buf);
                                }
                                _ => {
                                    return -1;
                                }
                            }
                        }
                    }
                    Err(_) =>{}
                }
            }
        }
        Err(e) => {

        }
    }
    -2
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "C" fn DisableAcceptor() -> i16{
    let port_nm = String::from("COM6");

    let port_result = serialport::new(port_nm, 9600)
        .timeout(Duration::from_millis(60))
        .data_bits(serialport::DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open().map_err(|e|{
            return -2;
        });
    match port_result{
        Ok(mut pr)=>{
            let buf: [u8; 1] = [DISABLE];
            let _ = pr.write(&buf);
        }
        Err(err)=>{
            return -2;
        }
    }
    0
}
