use evdev::{Device};

#[derive(Debug)]
struct State {
	device: String,

	shift_up_code: u16,
	shift_up: bool,

	shift_down_code: u16,
	shift_down: bool,

	steering_code: u16,
	steering_min: i32,
	steering_max: i32,
	steering: i32,
	invert_steering: bool,

	throttle_code: u16,
	throttle_min: i32,
	throttle_max: i32,
	throttle: i32,
	invert_throttle: bool,

	brake_code: u16,
	brake_min: i32,
	brake_max: i32,
	brake: i32,
	invert_brake: bool,

	clutch_code: u16,
	clutch_min: i32,
	clutch_max: i32,
	clutch: i32,
	invert_clutch: bool,

	handbrake_code: u16,
	handbrake_min: i32,
	handbrake_max: i32,
	handbrake: i32,
	invert_handbrake: bool
}

fn get_info_by_code(code:u16, device:&Device) -> Option<(evdev::EventType, i32, i32, i32)>{
	match device.supported_keys(){
		Some(keys) => {
			if keys.contains(evdev::Key(code)){
				match device.get_key_state(){
					Ok(active_keys) => {
						return Some((evdev::EventType::KEY, 0, 1, if active_keys.contains(evdev::Key(code)){1}else{0}));
					},
					Err(_) => {panic!("failed fetching key state during init");}
				}
			}
		},
		None => {}
	};

	match device.supported_absolute_axes(){
		Some(axes) => {
			if axes.contains(evdev::AbsoluteAxisType(code)){
				match device.get_abs_state(){
					Ok(info) => {
						let index:usize = code.into();
						return Some((evdev::EventType::ABSOLUTE, info[index].minimum, info[index].maximum, info[index].value))
					},
					Err(_) => {panic!("failed fetching abs state during init");}
				}
			}
		},
		None => {}
	};

	return None;
}

fn print_codes(device:&Device){
	match device.supported_keys(){
		Some(keys) => {
			let mut itr = keys.iter();
			loop{
				match itr.next(){
					Some(key) => {
						println!("KEY {}", key.0);
					},
					None => {break;}
				}
			};
		},
		None => {println!("device has no KEY events");}
	}

	match device.supported_absolute_axes(){
		Some(axes) => {
			match device.get_abs_state(){
				Ok(info) => {
					let mut itr = axes.iter();
					loop{
						match itr.next(){
							Some(axis) => {
								let index:usize = axis.0.into();
								println!("ABS {}: min {}, max {}, current {}, fuzz {}, flat {}, resolution {}", index, info[index].minimum, info[index].maximum, info[index].value, info[index].fuzz, info[index].flat, info[index].resolution);
							},
							None => {break;}
						}
					}					
				},
				Err(_) => {println!("device has ABS events, but state cannot be fetched");}
			}

		}
		None => {println!("device has no ABS events");}
	}
}

fn display_state(state:&State){
	println!("{:#?}", state);	
}

fn poller(initial_state:State){
	let mut state = initial_state;

	// open device
	let mut device = match Device::open(&state.device){
		Ok(d) => d,
		Err(_) => {
			panic!("failed opening {}", &state.device);
		}
	};
	println!("opened {} for polling", &state.device);
	match device.unique_name(){
		Some(s) => {println!("device name is {}", s);},
		None => {}
	};

	print_codes(&device);
	// check codes for event type and min/max
	match get_info_by_code(state.shift_up_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.shift_up = if value != 0 {true} else {false};
		},
		None => {panic!("cannot fetch info of shift up");}
	}

	match get_info_by_code(state.shift_down_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.shift_down = if value != 0 {true} else {false};
		},
		None => {panic!("cannot fetch info of shift down");}
	}

	match get_info_by_code(state.steering_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.steering_min = min;
			state.steering_max = max;
			state.steering = value;
		},
		None => {panic!("cannot fetch info of steering");}
	}

	match get_info_by_code(state.throttle_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.throttle_min = min;
			state.throttle_max = max;
			state.throttle = value;
		},
		None => {panic!("cannot fetch info of throttle");}
	}

	match get_info_by_code(state.brake_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.brake_min = min;
			state.brake_max = max;
			state.brake = value;
		},
		None => {panic!("cannot fetch info of brake");}
	}

	match get_info_by_code(state.clutch_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.clutch_min = min;
			state.clutch_max = max;
			state.clutch = value;
		},
		None => {panic!("cannot fetch info of clutch");}
	}

	match get_info_by_code(state.handbrake_code, &device){
		Some(info) => {
			let (t, min, max, value) = info;
			state.handbrake_min = min;
			state.handbrake_max = max;
			state.handbrake = value;
		},
		None => {panic!("cannot fetch info of handbrake");}
	}

	println!("{:#?}", state);

	loop{
		match device.fetch_events(){
			Ok(mut events) => {
				loop{
					match events.next(){
						Some(event) => {
							if event.event_type() != evdev::EventType::SYNCHRONIZATION{
								if event.code() == state.shift_up_code{
									state.shift_up = if event.value() != 0 {true} else {false};
								}else if event.code() == state.shift_down_code{
									state.shift_down = if event.value() != 0 {true} else {false};
								}else if event.code() == state.steering_code{
									state.steering = event.value();
								}else if event.code() == state.throttle_code{
									state.throttle = event.value();
								}else if event.code() == state.brake_code{
									state.brake = event.value();
								}else if event.code() == state.clutch_code{
									state.clutch = event.value();
								}else if event.code() == state.handbrake_code{
									state.handbrake = event.value();
								}
							}
						},
						None => {break;}
					}
					display_state(&state);
				}
			},
			Err(_) => {
				panic!("failed fetching events, terminating");
			}
		}
	}
}

// TODO shifter when I have one
#[argopt::cmd]
fn main(
	#[opt(short = 'D', long = "device")]
	device: String,
	#[opt(short = 'u', long = "shift_up")]
	shift_up: u16,
	#[opt(short = 'd', long = "shift_down")]
	shift_down: u16,
	#[opt(short = 's', long = "steering")]
	steering: u16,
	#[opt(short = 't', long = "throttle")]
	throttle: u16,
	#[opt(short = 'b', long = "brake")]
	brake: u16,
	#[opt(short = 'h', long = "handbrake")]
	handbrake: u16,
	#[opt(short = 'c', long = "clutch")]
	clutch: u16,
	#[opt(long = "invert_steering")]
	invert_steering: bool,
	#[opt(long = "invert_throttle")]
	invert_throttle: bool,
	#[opt(long = "invert_brake")]
	invert_brake: bool,
	#[opt(long = "invert_clutch")]
	invert_clutch: bool,
	#[opt(long = "invert_handbrake")]
	invert_handbrake: bool
) {
	println!("evdev device {device}");
	println!("shift up event code {shift_up}");
	println!("shift down event code {shift_down}");
	println!("steering event code {steering}");
	println!("invert steering {invert_steering}");
	println!("throttle event code {throttle}");
	println!("invert throttle {invert_throttle}");
	println!("brake event code {brake}");
	println!("invert brake {invert_brake}");
	println!("clutch event code {clutch}");
	println!("invert clutch {invert_clutch}");
	println!("handbrake event code {handbrake}");
	println!("invert handbrake {invert_handbrake}");

	let initial_state = State{
		device: device,

		shift_up_code: shift_up,
		shift_up: false,

		shift_down_code: shift_down,
		shift_down: false,

		steering_code: steering,
		steering_min: 0,
		steering_max: 0,
		steering: 0,
		invert_steering: invert_steering,

		throttle_code: throttle,
		throttle_min: 0,
		throttle_max: 0,
		throttle: 0,
		invert_throttle: invert_throttle,
		
		brake_code: brake,
		brake_min: 0,
		brake_max: 0,
		brake: 0,
		invert_brake: invert_brake,

		clutch_code: clutch,
		clutch_min: 0,
		clutch_max: 0,
		clutch: 0,
		invert_clutch: invert_clutch,

		handbrake_code: handbrake,
		handbrake_min: 0,
		handbrake_max: 0,
		handbrake: 0,
		invert_handbrake: invert_handbrake
	};

	poller(initial_state);
}
