use smithay::{
    reexports::{
        input::Libinput
    },
    backend::{
        libinput::{LibinputSessionInterface},
        session::auto::AutoSession,
    }
};

#[macro_use]
extern crate slog;
use slog::Drain;

fn main() {

    let log = slog::Logger::root(
        slog_async::Async::default(slog_term::term_full().fuse()).fuse(),
        o!(),
    );

    let (session,_session_notifier) = AutoSession::new(log.clone()).expect("Failed to initialize the session");

    let seat_name = "seat-0";

    let mut libinput_context = Libinput::new_with_udev::<LibinputSessionInterface<AutoSession>>(session.into());
    libinput_context.udev_assign_seat(&seat_name).unwrap();

    let start = std::time::Instant::now();
    loop {
        libinput_context.dispatch().unwrap();

        if let Some(event) = libinput_context.next()
        {
            println!("{:#?}",&event)
        }

        //After 5 seconds the loop terminate and give the control back to the terminal
        if start.elapsed().as_secs() >= 5 {break;}
    }
}
