use winput::{Vk, Mouse, Action};
use winput::message_loop;
use core::time;
use std::thread;

fn main() {
    let receiver = message_loop::start().unwrap();
    let mut prev_input: Vk = Vk::StartApp1;
    let sleep_milis = time::Duration::from_millis(33);
    loop{
        match receiver.next_event(){
            message_loop::Event::Keyboard { 
                vk,
                scan_code,  
                action: Action::Press 
            } =>{
                if vk == Vk::Escape{
                    break;
                }else if vk == Vk::Z{
                    match prev_input{
                        Vk::U=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("két");
                            Mouse::scroll(-1.0);
                            println!("Switched to Z from U");
                            println!("-----------");
                        },
                        Vk::I=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            println!("Switched to Z from I");
                            println!("-----------");
                        },
                        Vk::O=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            println!("Switched to Z from O");
                            println!("-----------");
                        },
                        _ => {
                            println!("Same button pressed!");
                        },
                    }
                    prev_input = vk;
                    println!("Z pressed");
                }else if vk == Vk::U{
                    match prev_input{
                        Vk::I=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("két");
                            Mouse::scroll(-1.0);
                            println!("Switched to U from I");
                            println!("-----------");
                        },
                        Vk::O=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            println!("Switched to U from O");
                            println!("-----------");
                        },
                        Vk::Z=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            println!("Switched to U from Z");
                            println!("-----------");
                        },
                        _ => {
                            println!("Same button pressed!");
                        },
                    }
                    prev_input = vk;
                    println!("U pressed");
                }else if vk == Vk::I{
                    match prev_input{
                        Vk::O=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("két");
                            Mouse::scroll(-1.0);
                            println!("Switched to I from O");
                            println!("-----------");
                        },
                        Vk::Z=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            Mouse::scroll(-1.0);
                            println!("Switched to I from Z");
                            println!("-----------");
                        },
                        Vk::U=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            println!("Switched to I from U");
                            println!("-----------");
                        },
                        _ => {
                            println!("Same button pressed!");
                        },
                    }
                    prev_input = vk;
                    println!("I pressed");
                }else if vk == Vk::O{
                    match prev_input{
                        Vk::Z=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("két");
                            Mouse::scroll(-1.0);
                            println!("Switched to O from Z");
                            println!("-----------");
                        },
                        Vk::U=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            thread::sleep(sleep_milis);
                            println!("egy");
                            Mouse::scroll(-1.0);
                            println!("Switched to -O from U");
                            println!("-----------");
                        },
                        Vk::I=>{
                            println!("-----------");
                            Mouse::scroll(-1.0);
                            println!("Switched to O from I");
                            println!("-----------");
                        },
                        _ => {
                            println!("Same button pressed!");
                        },
                    }
                    prev_input = vk;
                    println!("O pressed");
                }
            },
            _ => (),   
        }
    }
    message_loop::stop();
}
