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


pub use smithay::backend::input::{InputBackend, InputEvent};
pub use smithay::reexports::input::event::keyboard::KeyboardEventTrait;

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
impl Default for InputGatherer {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_gatherer() {
    use keystroke_decoder::{KeystrokeDecoder,keysyms};

    //Creating the gatherer
    let mut gatherer = InputGatherer::new();
    //Creating the keyboard decoder
    let keystroke_decoder = KeystrokeDecoder::new();

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
                    //Decoding keys into chars
                    for key in keystroke_decoder.decode_as_chars(event.key()) {
                        println!("{}", key);
                    }

                    //Decoding keys into keysym
                    for key in keystroke_decoder.decode_as_keysym(event.key()) {
                        match *key {
                            keysyms::KEY_Escape => {
                                println!("Esc pressed, early exit");
                                running = false;
                            }
                            _ => {}
                        }
                    }
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


