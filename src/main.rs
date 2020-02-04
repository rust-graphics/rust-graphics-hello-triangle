extern crate rust_graphics_gl as gl;
extern crate rust_graphics_main as main;

use {
    self::gl::{
        constants as glc,
        log::{log_i, result_f, unwrap_f},
        window::{
            event::{Data as EventData, Event, Listener as EventListener},
            Window,
        },
    },
    main::{main, Arg},
    std::sync::{Arc, RwLock},
};

struct Listener {
    pub running: bool,
}

impl EventListener for Listener {
    fn on_event(&mut self, event: &Event) -> bool {
        match event.get_data() {
            &EventData::Quit => self.running = false,
            _e @ _ => {
                #[cfg(feature = "verbose-log")]
                log_i!("{:?}", _e);
            }
        }
        return false;
    }
}

fn start(arg: Arg) {
    let w = Window::new(arg);
    let listener = Arc::new(RwLock::new(Listener { running: true }));
    let l: Arc<RwLock<dyn EventListener>> = listener.clone();
    w.get_event_engine().add(0, Arc::downgrade(&l));

    let gl_manager = unwrap_f!(gl::manager::Manager::new(w.clone()));
    let glf = gl_manager.get_loader();
    (glf.clear_color)(1.0, 0.0, 0.0, 0.0);
    while { result_f!(listener.read()).running } {
        w.fetch_events();
        (glf.clear)(glc::COLOR_BUFFER_BIT);
        gl_manager.swap_buffers();
    }
    log_i!("Program ended.");
}

main!(start);
