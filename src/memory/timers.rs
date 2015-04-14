use timekeeper::{TimeKeeper, Peripheral};
use gpu::Gpu;

pub struct Timers {
    /// The three timers. They're mostly identical except that they
    /// can each select a unique clock source besides the regular
    /// system clock:
    ///
    /// * Timer 0: GPU pixel clock
    /// * Timer 1: GPU horizontal blanking
    /// * Timer 2: System clock / 8
    timers: [Timer; 3],
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            timers: [Timer::new(),
                     Timer::new(),
                     Timer::new(),
                     ],
        }
    }

    /// Handle timer register write
    pub fn set_reg(&mut self,
                   timekeeper: &mut TimeKeeper,
                   gpu: &mut Gpu,
                   reg: u32,
                   val: u16) {
        self.update(timekeeper, gpu);

        // XXX
    }

    fn update(&mut self, timekeeper: &mut TimeKeeper, gpu: &mut Gpu) {
        let _delta = timekeeper.sync(Peripheral::Timers);
    }
}

struct Timer {
    counter: u16,
    target: u16,
    /// EXPLODE THIS
    mode: u16,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            counter: 0,
        }
    }
}
