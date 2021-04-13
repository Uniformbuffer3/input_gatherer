use smithay::{
    reexports::{
        input::Libinput,
        calloop::EventLoop
    },
    backend::{
        libinput::{LibinputSessionInterface,LibinputInputBackend},
        session::auto::AutoSession,
        udev::{UdevBackend,UdevEvent}
    },
    signaling::Linkable
};

fn main() {

    let (session,session_notifier) = AutoSession::new(None).expect("Failed to initialize the session");
    let session_signal = session_notifier.signaler();

    let seat_name = "seat-0";

    let mut libinput_context = Libinput::new_with_udev::<LibinputSessionInterface<AutoSession>>(session.into());
    libinput_context.udev_assign_seat(&seat_name).unwrap();

    let mut libinput_backend = LibinputInputBackend::new(libinput_context, None);
    libinput_backend.link(session_signal);

    let mut event_loop: EventLoop<()> = EventLoop::new().expect("Failed to initialize event loop");
    let event_loop_signal = event_loop.get_signal();

    let libinput_event_source = event_loop
        .handle()
        .insert_source(libinput_backend, move |event, _metadata, _data| {
            println!("{:#?}",event);
        })
        .expect("Failed to add libinput event provider to the event loop");
    let session_event_source = event_loop
        .handle()
        .insert_source(session_notifier, |event, _metadata, _data| {
            println!("{:#?}",event);
        })
        .expect("Failed to add session event provider to the event loop");


    let udev_backend = UdevBackend::new(&seat_name, None).map_err(|_| ()).unwrap();

    let udev_event_source = event_loop
        .handle()
        .insert_source(udev_backend, move |event, _, _state| match event {
            UdevEvent::Added { device_id: _, path: _ } => println!("Added udev device"),
            UdevEvent::Changed { device_id: _ } => println!("Changed udev device"),
            UdevEvent::Removed { device_id: _ } => println!("Removed udev device"),
        })
        .map_err(|e| -> std::io::Error { e.into() })
        .unwrap();
    let start = std::time::Instant::now();
    event_loop.run(Some(std::time::Duration::from_millis(16)),&mut (),move |_|{

        //After 5 seconds the loop terminate and give the control back to the terminal
        if start.elapsed().as_secs() >= 5 {event_loop_signal.stop();}
    }).expect("Failed to dispatch events");

    event_loop.handle().remove(udev_event_source);
    event_loop.handle().remove(session_event_source);
    event_loop.handle().remove(libinput_event_source);
}
