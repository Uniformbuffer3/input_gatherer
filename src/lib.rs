use smithay::{
    backend::{
        libinput::{LibinputInputBackend, LibinputSessionInterface},
        session::{
            auto::{AutoSession, AutoSessionNotifier},
            Session,
        },
    },
    reexports::input::Libinput,
};

use xkbcommon::xkb;

/**
This structure initialize the xkb components to decode key strokes to an abstract xkb representation,
making possible to handle multiple keyboards layouts.
*/
pub struct KeyboadDecoder {
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: xkb::State,
}
impl KeyboadDecoder {
    fn detect_keyboard_layout_from_env()->Result<String,()>{
        for (var,value) in std::env::vars()
        {
            if var == "XKB_DEFAULT_LAYOUT" {return Ok(value);}
        }
        Err(())
    }

    fn detect_keyboard_layout_from_file()->Result<String,()>{
        let regex = regex::Regex::new(r"\s*XKBLAYOUT\s*=(.+)").unwrap();

        let file_data = std::fs::read_to_string("/etc/default/keyboard").unwrap();
        for line in file_data.lines()
        {
            match regex.captures(line) {
                Some(capture)=>{return Ok(capture.get(1).unwrap().as_str().to_string());}
                None=>{}
            };
        }

        return Err(());
    }

    fn detect_keyboard_layout()->Result<String,()>{

        //Try to detect from env
        match Self::detect_keyboard_layout_from_env(){
            Ok(layout)=>return Ok(layout),
            Err(_)=>{}
        }

        //Try to detect from file
        match Self::detect_keyboard_layout_from_file(){
            Ok(layout)=>return Ok(layout),
            Err(_)=>{}
        }
        return Err(());
    }

    pub fn new() -> Self {
        // Initializing the xkb context with no flags
        let context = xkb::Context::new(0);

        let keyboard_layout = match Self::detect_keyboard_layout(){
            Ok(keyboard_layout)=>{println!("Detected layout: {}",&keyboard_layout);keyboard_layout}
            Err(_)=>String::from("")
        };

        // Initializing the keymap using empty values ("").
        // This will make xkb detect automatically the system keymap.
        let keymap = xkb::Keymap::new_from_names(&context, "", "", &keyboard_layout, "", None, 0)
            .expect("Fauled to create keymap");

        // Initializing the xkb state that will be used to decode keystrokes
        let state = xkb::State::new(&keymap);

        Self {
            context,
            keymap,
            state
        }
    }
    /// This function will decode the key into an abstract xkb representation (Keysym).
    /// The keycode will be increased by 8 because the evdev XKB rules reflect X's
    /// broken keycode system, which starts at 8
    pub fn decode_as_keysym(&self, keycode: u32) -> &[xkb::Keysym] {
        self.state.key_get_syms(keycode + 8)
    }
    pub fn decode_as_chars(&self, keycode: u32) -> Vec<char> {
        self.state.key_get_utf8(keycode + 8).chars().collect()
    }
}

/**
This structure will initialize the session using AutoSession,
that will automatically detect the context and initialze accordingly.
The session will be used to create the Libinput context from where inputs will be gathered.
Finally the Libinput is wrapped into the LibinputInputBackend that provide some convenience
interface to Libinput.
*/
pub struct InputGatherer(LibinputInputBackend, AutoSessionNotifier);
impl InputGatherer {
    pub fn new() -> Self {
        let (session, session_notifier) =
            AutoSession::new(None).expect("Failed to initialize the session");
        let seat_name = session.seat();

        let mut context =
            Libinput::new_with_udev::<LibinputSessionInterface<AutoSession>>(session.into());
        context.udev_assign_seat(&seat_name).unwrap();

        let backend = LibinputInputBackend::new(context, None);

        Self(backend, session_notifier)
    }
}
impl std::ops::Deref for InputGatherer {
    type Target = LibinputInputBackend;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for InputGatherer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_gatherer() {
    use smithay::backend::input::{InputBackend, InputEvent};
    use smithay::reexports::input::event::keyboard::KeyboardEventTrait;
    use xkb::keysyms;

    //Creating the gatherer
    let mut gatherer = InputGatherer::new();
    //Creating the keyboard decoder
    let keyboard_decoder = KeyboadDecoder::new();

    let start = std::time::Instant::now();
    let mut running = true;
    while running {
        //Dispatching events
        gatherer
            .dispatch_new_events(|event, _config| match event {
                InputEvent::NewSeat(seat) => {
                    println!("Seat added: {:#?}", seat);
                }
                InputEvent::SeatChanged(seat) => {
                    println!("Seat changed: {:#?}", seat);
                }
                InputEvent::SeatRemoved(seat) => {
                    println!("Seat removed: {:#?}", seat);
                }
                InputEvent::Keyboard { seat: _, event } => {
                    //Decoding keys
                    for key in keyboard_decoder.decode_as_chars(event.key()) {
                        println!("{}",key);
                        /*
                        match *key {
                            keysyms::KEY_Escape => {
                                println!("Esc pressed, early exit");
                                running = false;
                            }
                            _ => {}
                        }
                        */
                    }
                    /*
                    for key in keyboard_decoder.decode_as_keysym(event.key()) {
                        match *key {
                            keysyms::KEY_Escape => {
                                println!("Esc pressed, early exit");
                                running = false;
                            }
                            _ => {}
                        }
                    }
                    */
                }
                InputEvent::PointerMotion { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::PointerMotionAbsolute { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::PointerButton { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::PointerAxis { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::TouchDown { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::TouchMotion { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::TouchUp { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::TouchCancel { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::TouchFrame { seat: _, event } => {
                    println!("{:#?}", event);
                }
                InputEvent::Special(event) => {
                    println!("{:#?}", event);
                }
            })
            .unwrap();

        //After 5 seconds the loop terminate and give the control back to the terminal
        if start.elapsed().as_secs() >= 5 {
            running = false;
        }
    }
}
