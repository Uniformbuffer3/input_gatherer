use smithay::{
    backend::{
        input::{InputBackend, InputEvent},
        libinput::{LibinputInputBackend, LibinputSessionInterface},
        session::{auto::AutoSession, Session},
    },
    reexports::input::{Libinput},
};

use std::convert::TryInto;

pub struct KeyboadDecoder;
impl KeyboadDecoder {
    pub fn decode(keycode: u32) -> keycode::KeyMappingId {
        let keymap: keycode::KeyMap = keycode::KeyMapping::Xkb((keycode+8) as u16).try_into().unwrap();
        keymap.id
    }
}

pub struct InputGatherer(LibinputInputBackend);
impl InputGatherer {
    pub fn new() -> Self {
        let (session, _session_notifier) =
            AutoSession::new(None).expect("Failed to initialize the session");
        let seat_name = session.seat();

        let mut context =
            Libinput::new_with_udev::<LibinputSessionInterface<AutoSession>>(session.into());
        context.udev_assign_seat(&seat_name).unwrap();

        let backend = LibinputInputBackend::new(context, None);

        Self(backend)
    }
}
impl std::ops::Deref for InputGatherer
{
    type Target = LibinputInputBackend;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for InputGatherer
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_raw()
{
    use smithay::reexports::input::event::keyboard::KeyboardEventTrait;

    let (session, _session_notifier) =
        AutoSession::new(None).expect("Failed to initialize the session");
    let seat_name = session.seat();

    let mut context =
        Libinput::new_with_udev::<LibinputSessionInterface<AutoSession>>(session.into());
    context.udev_assign_seat(&seat_name).unwrap();

    let mut gatherer = LibinputInputBackend::new(context, None);

    let start = std::time::Instant::now();
    loop{
        gatherer.dispatch_new_events(|event,_config|match event {
            InputEvent::NewSeat(seat) => {println!("Seat added: {:#?}",seat);}
            InputEvent::SeatChanged(seat) => {println!("Seat changed: {:#?}",seat);}
            InputEvent::SeatRemoved(seat) => {println!("Seat removed: {:#?}",seat);}
            InputEvent::Keyboard { seat: _, event } => {
                println!("KEYBOARD_EVENT_DETECTED");
                //println!("{:#?}", KeyboadDecoder::decode(event.key()));
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
        }).unwrap();
        //After 5 seconds the loop terminate and give the control back to the terminal
        if start.elapsed().as_secs() >= 5 {
            break;
        }
    }
}


#[test]
fn test_gatherer()
{
    use smithay::reexports::input::event::keyboard::KeyboardEventTrait;

    let mut gatherer = InputGatherer::new();

    let start = std::time::Instant::now();
    loop{
        gatherer.dispatch_new_events(|event,_config|match event {
            InputEvent::NewSeat(seat) => {println!("Seat added: {:#?}",seat);}
            InputEvent::SeatChanged(seat) => {println!("Seat changed: {:#?}",seat);}
            InputEvent::SeatRemoved(seat) => {println!("Seat removed: {:#?}",seat);}
            InputEvent::Keyboard { seat: _, event } => {
                println!("KEYBOARD_EVENT_DETECTED");
                //println!("{:#?}", KeyboadDecoder::decode(event.key()));
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
        }).unwrap();
        //After 5 seconds the loop terminate and give the control back to the terminal
        if start.elapsed().as_secs() >= 5 {
            break;
        }
    }
}
