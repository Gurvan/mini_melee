use std::collections::VecDeque;
use rusb::{DeviceHandle, Error, UsbContext};
use std::time::Duration;

const GCADAPTER_VENDOR_ID: u16 = 0x057E;
const GCADAPTER_PRODUCT_ID: u16 = 0x0337;

#[derive(Debug, Copy, Clone)]
pub struct Deadzone {
    pub stick_x:    u8,
    pub stick_y:    u8,
    pub c_x:  u8,
    pub c_y:  u8,
    pub l_analog:  u8,
    pub r_analog:  u8,
}

#[allow(dead_code)]
impl Deadzone {
    pub fn empty() -> Self {
        Deadzone {
            stick_x:    0,
            stick_y:    0,
            c_x:  0,
            c_y:  0,
            l_analog:  0,
            r_analog:  0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ControllerInput {
    pub stick_x: f32,
    pub stick_y: f32,
    pub c_x: f32,
    pub c_y: f32,
    pub l_analog: f32,
    pub r_analog: f32,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub l: bool,
    pub r: bool,
    pub start: bool,
}

impl ControllerInput {
    fn new() -> ControllerInput {
        ControllerInput {
            stick_x: 0.0,
            stick_y: 0.0,
            c_x: 0.0,
            c_y: 0.0,
            l_analog: 0.0,
            r_analog: 0.0,
            a: false,
            b: false,
            x: false,
            y: false,
            z: false,
            l: false,
            r: false,
            start: false,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    // pub current: ControllerInput,
    // pub previous: ControllerInput,
    // pub previous2: ControllerInput,
    pub list: VecDeque<ControllerInput>,
    pub deadzone: Option<Deadzone>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            // current: ControllerInput::new(),
            // previous: ControllerInput::new(),
            // previous2: ControllerInput::new(),
            list: VecDeque::from(vec![ControllerInput::new(); 3]),
            deadzone: None,
        }
    }
    pub fn update(&mut self, input: ControllerInput) {
        self.list.push_front(input);
        self.list.pop_back();
    }
    pub fn current(&self) -> ControllerInput {
        self.list[0]
    }
    pub fn previous(&self) -> ControllerInput {
        self.list[1]
    }
    pub fn previous2(&self) -> ControllerInput {
        self.list[2]
    }
}

enum InputSource {
    GCAdapter { handle: DeviceHandle<rusb::Context> },
    RawInput {},
}

pub struct InputManager {
    input_sources: Vec<InputSource>,
    pub raw_data: [u8; 8],
}

impl InputManager {
    pub fn new() -> Result<InputManager, Error> {
        let device_list = rusb::Context::new()?.devices()?;
        // let mut adapter_handles = vec![];
        let mut adapter_handles: Vec<DeviceHandle<rusb::Context>> = Vec::new();

        for device in device_list.iter() {
            let desc = device.device_descriptor()?;

            if desc.vendor_id() != GCADAPTER_VENDOR_ID || desc.product_id() != GCADAPTER_PRODUCT_ID {
                continue;
            }
            match device.open() {
                Ok(mut handle) => {
                    if let Ok(true) = handle.kernel_driver_active(0) {
                        handle.detach_kernel_driver(0).unwrap();
                    }
                    match handle.claim_interface(0) {
                        Ok(_) => {
                            // Tell adapter to start reading
                            let payload = [0x13];
                            if let Ok(_) = handle.write_interrupt(0x2, &payload, Duration::new(1, 0)) {
                                adapter_handles.push(handle);
                                println!("GC adapter: Setup complete");
                            }
                        }
                        Err(e) => println!("GC adapter: Failed to claim interface: {}", e)
                    }
                }
                Err(e) => {
                    InputManager::handle_open_error(e);
                }
            }
        }

        let mut input_sources = vec!();
        for handle in adapter_handles {
            input_sources.push(InputSource::GCAdapter { handle });
        }
        if input_sources.len() == 0 {
            input_sources.push(InputSource::RawInput {});
        }

        Ok(InputManager {
            input_sources,
            raw_data: [0u8; 8],
        })
    }

    fn handle_open_error(e: Error) {
        let access_solution = if cfg!(target_os = "linux") { r#":
    You need to set a udev rule so that the adapter can be accessed.
    To fix this on most Linux distributions, run the following command and then restart your computer.
    echo 'SUBSYSTEM=="usb", ENV{DEVTYPE}=="usb_device", ATTRS{idVendor}=="057e", ATTRS{idProduct}=="0337", TAG+="uaccess"' | sudo tee /etc/udev/rules.d/51-gcadapter.rules"#
        } else { "" };

        let driver_solution = if cfg!(target_os = "windows") { r#":
    To use your GC adapter you must:
    1. Download and run Zadig: http://zadig.akeo.ie/
    2. Options -> List all devices
    3. In the pulldown menu, Select WUP-028
    4. On the right ensure WinUSB is selected
    5. Select Replace Driver
    6. Select yes in the dialog box
    7. Restart PF Sandbox"#
        } else { "" };

        match e {
            Error::Access => {
                println!("GC adapter: Permissions error{}", access_solution);
            }
            Error::NotSupported => {
                println!("GC adapter: Not supported error{}", driver_solution);
            }
            _ => { println!("GC adapter: Failed to open handle: {:?}", e); }
        }
    }

    pub fn step(&mut self, input: &mut Input, port: usize) {
        // read input from controllers
        // input.previous2 = input.previous;
        // input.previous = input.current;
        let mut new_input: Option<ControllerInput> = None;
        for source in &mut self.input_sources {
            new_input = match source {
                &mut InputSource::GCAdapter { ref mut handle }
                    => read_gc_adapter(handle, &mut input.deadzone, port),
                    // => read_gc_adapter(handle, &mut input.current, &mut input.deadzone, port),
                &mut InputSource::RawInput {}
                    => read_raw_input(&mut self.raw_data),
                    // => read_raw_input(&mut self.raw_data, &mut input.current),
            };
        };
        match new_input {
            Some(i) => input.update(i),
            None => (),
        }
    }

    // #[allow(dead_code)]
    // pub fn step_current(&mut self, input: &mut Input) {
    //     // read input from controllers
    //     for source in &mut self.input_sources {
    //         match source {
    //             &mut InputSource::GCAdapter { ref mut handle }
    //                 => read_gc_adapter(handle, &mut input.current),
    //             &mut InputSource::RawInput {}
    //                 => read_raw_input(&mut self.raw_data, &mut input.current),
    //         }
    //     }
    // }
}


// pub fn read_raw_input(data: &mut [u8; 8], input: &mut ControllerInput) {
pub fn read_raw_input(data: &mut [u8; 8]) -> Option<ControllerInput> {
    let mut input = ControllerInput::new();
    let (stick_x, stick_y) = stick_filter(data[2], data[3]);
    let a = (data[0] & 0b00000001) != 0;
    let x = (data[0] & 0b00000100) | (data[0] & 0b00001000) != 0;
    input.stick_x = stick_x;
    input.stick_y = stick_y;
    input.a = a;
    input.x = x;
    Some(input)
}


// fn read_gc_adapter<T: rusb::UsbContext>(handle: &mut DeviceHandle<T>, input: &mut ControllerInput, deadzone: &mut Option<Deadzone>, port: usize) -> Option<ControllerInput> {
fn read_gc_adapter<T: rusb::UsbContext>(handle: &mut DeviceHandle<T>, deadzone: &mut Option<Deadzone>, port: usize) -> Option<ControllerInput> {
    let mut data: [u8; 37] = [0; 37];
    if let Ok(_) = handle.read_interrupt(0x81, &mut data, Duration::new(1, 0)) {
        // println!("{:?}", &data[..32]);
        if let Some(deadzone) = deadzone {
            // Stick
            let (stick_x, stick_y) = stick_filter(stick_deadzone(data[9*port + 4], deadzone.stick_x), stick_deadzone(data[9*port + 5], deadzone.stick_y));
            let (c_x, c_y) = stick_filter(stick_deadzone(data[9*port + 6], deadzone.c_x), stick_deadzone(data[9*port + 7], deadzone.c_y));
            // input.stick_x = stick_x;
            // input.stick_y = stick_y;
            // input.c_x = c_x;
            // input.c_y = c_y;
            // input.l_analog = trigger_filter(data[9*port+8].saturating_sub(deadzone.l_analog));
            // input.r_analog = trigger_filter(data[9*port+9].saturating_sub(deadzone.r_analog));
            // // A
            // input.a = (data[9*port + 2] & 0b00000001) != 0;
            // // B
            // input.b = (data[9*port + 2] & 0b00000010) != 0;
            // // X
            // input.x = (data[9*port + 2] & 0b00000100) != 0;
            // // Y
            // input.y = (data[9*port + 2] & 0b00001000) != 0;
            // // Z
            // input.z = (data[9*port + 3] & 0b00000010) != 0;
            // // L
            // input.l = (data[9*port + 3] & 0b00001000) != 0;
            // // R
            // input.r = (data[9*port + 3] & 0b00000100) != 0;
            // // Start
            // input.start = (data[9*port + 3] & 0b00000001) != 0;
            return Some(ControllerInput {
                stick_x: stick_x,
                stick_y: stick_y,
                c_x: c_x,
                c_y: c_y,
                l_analog: trigger_filter(data[9*port+8].saturating_sub(deadzone.l_analog)),
                r_analog: trigger_filter(data[9*port+9].saturating_sub(deadzone.r_analog)),
                a: (data[9*port + 2] & 0b00000001) != 0,
                b: (data[9*port + 2] & 0b00000010) != 0,
                x: (data[9*port + 2] & 0b00000100) != 0,
                y: (data[9*port + 2] & 0b00001000) != 0,
                z: (data[9*port + 3] & 0b00000010) != 0,
                l: (data[9*port + 3] & 0b00001000) != 0,
                r: (data[9*port + 3] & 0b00000100) != 0,
                start: (data[9*port + 3] & 0b00000001) != 0,
            })
        } else {
            // Set deadzone
            *deadzone = Some(Deadzone{
                stick_x: data[9*port + 4],
                stick_y: data[9*port + 5],
                c_x: data[9*port + 6],
                c_y: data[9*port + 7],
                l_analog: data[9*port + 8],
                r_analog: data[9*port + 9],
            });
        }
    }
    None
}

        // for port in 0..4 {
        //     let plugged_in    = data[9*port+1] == 20 || data[9*port+1] == 16;
        //     let raw_stick_x   = data[9*port+4];
        //     let raw_stick_y   = data[9*port+5];
        //     let raw_c_stick_x = data[9*port+6];
        //     let raw_c_stick_y = data[9*port+7];
        //     let raw_l_trigger = data[9*port+8];
        //     let raw_r_trigger = data[9*port+9];


        //     inputs.push(ControllerInput {
        //         up:    data[9*port+2] & 0b10000000 != 0,
        //         down:  data[9*port+2] & 0b01000000 != 0,
        //         right: data[9*port+2] & 0b00100000 != 0,
        //         left:  data[9*port+2] & 0b00010000 != 0,
        //         y:     data[9*port+2] & 0b00001000 != 0,
        //         x:     data[9*port+2] & 0b00000100 != 0,
        //         b:     data[9*port+2] & 0b00000010 != 0,
        //         a:     data[9*port+2] & 0b00000001 != 0,
        //         l:     data[9*port+3] & 0b00001000 != 0,
        //         r:     data[9*port+3] & 0b00000100 != 0,
        //         z:     data[9*port+3] & 0b00000010 != 0,
        //         start: data[9*port+3] & 0b00000001 != 0,
        //         stick_x,
        //         stick_y,
        //         c_stick_x,
        //         c_stick_y,
        //         l_trigger,
        //         r_trigger,
        //         plugged_in,
        //     });
        // }
//     }
// }


pub fn stick_deadzone(current: u8, first: u8) -> u8 {
    if current > first {
        128u8.saturating_add(current - first)
    } else {
        128u8.saturating_sub(first - current)
    }
}


fn abs_min(a: f32, b: f32) -> f32 {
    if (a >= 0.0 && a > b) || (a <= 0.0 && a < b) {
        b
    } else {
        a
    }
}

fn stick_filter(in_stick_x: u8, in_stick_y: u8) -> (f32, f32) {
    let raw_stick_x = in_stick_x as f32 - 128.0;
    let raw_stick_y = in_stick_y as f32 - 128.0;
    let angle = (raw_stick_y).atan2(raw_stick_x);

    let max_x = (angle.cos() * 80.0).trunc();
    let max_y = (angle.sin() * 80.0).trunc();
    let stick_x = if in_stick_x == 128 { // avoid raw_stick_x = 0 and thus division by zero in the atan2)
        0.0
    } else {
        abs_min(raw_stick_x, max_x) / 80.0
    };
    let stick_y = abs_min(raw_stick_y, max_y) / 80.0;

    let deadzone = 0.2875;
    (
        if stick_x.abs() < deadzone { 0.0 } else { stick_x },
        if stick_y.abs() < deadzone { 0.0 } else { stick_y }
    )
}

pub fn trigger_filter(trigger: u8) -> f32 {
    let value = (trigger as f32) / 140.0;
    if value > 1.0 {
        1.0
    } else {
        value
    }
}


// mflr r0              # move link register to r0
// stw r0,4(sp)
// stwu sp,-0x8(sp)
// lis	r3,0x804c	# 804c1f9e joystick and cstick
// lbz	r3,0x1f9f(r3)	# load L/R analog normalization byte
// bl INT_TO_FLOAT	# get analog resolution as float (can I just force it to be 140 instead of loading byte?)
// #lis r3,0x430c	# float 140.0
// #stw r3,0(r2)
// #lfs f2,0(r2)	# load float 140.0 into fpr
// fmuls	f1,f1,f2	# multiply resolution by normalization
// fctiwz f1,f1
// stfd f1,0(r2)
// lwz r3,4(r2)	# load int
// lwz r0,0xc(sp)
// addi sp,sp,8
// mtlr r0              # move r0 to link register
// blr


impl std::fmt::Debug for InputManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "InputManager")
    }
}