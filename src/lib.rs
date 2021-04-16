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

pub struct KeyboadDecoder {
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: xkb::State,
}
impl KeyboadDecoder {
    pub fn new() -> Self {
        let context = xkb::Context::new(0);
        let keymap = xkb::Keymap::new_from_names(&context, "", "", "", "", None, 0)
            .expect("Fauled to create keymap");
        let state = xkb::State::new(&keymap);

        Self {
            context,
            keymap,
            state,
        }
    }
    pub fn decode(&self, keycode: u32) -> &[xkb::Keysym] {
        self.state.key_get_syms(keycode + 8)
    }
}

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

    let mut gatherer = InputGatherer::new();
    let keyboard_decoder = KeyboadDecoder::new();

    let start = std::time::Instant::now();
    let mut running = true;
    while running {
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
                    for key in keyboard_decoder.decode(event.key()) {
                        match *key {
                            keysyms::KEY_Escape => {
                                println!("Esc pressed, exiting");
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
