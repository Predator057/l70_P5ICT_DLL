use std::io::Read;
use std::{fs, time::Duration};
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
    let mut port_nm = String::new();
    match fs::File::open("acceptor"){
        Ok(mut file)=>{
            let port = file.read_to_string(&mut port_nm);
        }
        Err(e) => {
            return -222
        }
    }

    let port_result = serialport::new(port_nm, 9600)
        .timeout(Duration::from_millis(20))
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open().map_err(|e|{
            println!("error {}", e);
            return -2;
        });
    
    match port_result {
        Ok(mut port) => {
            let _ = port.write(&[RESET]);
            for _ in 0..100 {
                let mut buffer = [0u8; 4];
                match port.read(buffer.as_mut_slice()){
                    Ok(n)=>{
                        println!("buffer {:?}", buffer);
                        for i in 0..buffer.len() {
                            if buffer[i] == 0x80{
                                let _ = port.write(&[ACCEPT]);
                                println!("принято");
                            }
                            if buffer[i] == 0x8F {
                                let _ = port.write(&[ACCEPT]);
                                println!("принято");
                                sleep(Duration::from_millis(3000));
                                let _ = port.write(&[DISABLE]);
                                println!("отключение");
                                println!("успешно");
                                return 0;
                            } 
                        }
                        // sleep(Duration::from_millis(3000));
                        // let _ = port.write(&[DISABLE]);
                        // println!("отключение");
                        // println!("успешно");
                    }
                    Err(_) =>{}
                }
            }
            return -3;
        }
        Err(e)=>{
            return -1;
        }
    }

}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "C" fn StartAcceptor() -> i16{
    let mut port_nm = String::new();
    match fs::File::open("acceptor"){
        Ok(mut file)=>{
            let port = file.read_to_string(&mut port_nm);
        }
        Err(e) => {
            return -222
        }
    }

    let port_result = serialport::new(port_nm, 9600)
        .timeout(Duration::from_millis(20))
        .data_bits(serialport::DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open().map_err(|e|{
            return -2;
        });

    match port_result {
        Ok(mut port) =>{
            let mut rx_buffer: Vec<u8> = Vec::new();
            let _ = port.write(&[ENABLE]);
            loop{
                //01 - 50  руб
                //02 - 100 руб
                //03 - 500 руб
                //04 - 1000 руб
                let mut temp = [0u8; 2];
                match port.read(temp.as_mut_slice()){
                    Ok(n)=>{
                        rx_buffer.extend_from_slice(&temp[..n]);
                        println!("буфер купюры {:?}", rx_buffer);
                        if rx_buffer.len() < 2{
                            sleep(Duration::from_millis(100));
                            continue;
                        }
                        for i in 0..temp.len(){
                            if rx_buffer[0] == 0x81{
                                match rx_buffer[1] {
                                    0x41 => {
                                        let _ = port.write(&[0x02]);
                                        return 50;
                                    }
                                    0x42 => {
                                        let _ = port.write(&[0x02]);
                                        return 100;
                                    }
                                    0x43 => {
                                        let _ = port.write(&[0x02]);
                                        return 500;
                                    }
                                    0x44 => {
                                        let _ = port.write(&[0x0F]);
                                        continue;
                                    }
                                    0x45 => {
                                        let _ = port.write(&[0x0F]);
                                        continue;
                                    }
                                    _ => {
                                        let _ = port.write(&[0x0F]);
                                        continue;
                                    }
                                }
                            }
                        }
                        rx_buffer.clear();
                    }
                    Err(_) =>{rx_buffer.clear()}
                }
                sleep(Duration::from_millis(100));
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
    let mut port_nm = String::new();
    match fs::File::open("acceptor"){
        Ok(mut file)=>{
            let port = file.read_to_string(&mut port_nm);
        }
        Err(e) => {
            return -222
        }
    }

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
