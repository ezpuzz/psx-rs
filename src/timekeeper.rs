/// Struct keeping track of the various peripheral's emulation advancement.
pub struct TimeKeeper {
    /// Counter keeping track of the current date. Unit is a period of
    /// the CPU clock at 33.8685MHz (~29.5ns)
    now: CpuTime,
    /// Time sheets for keeping track of the various peripherals
    timesheets: [TimeSheet; 1],
}

impl TimeKeeper {
    pub fn new() -> TimeKeeper {
        TimeKeeper {
            now: 0,
            timesheets: [TimeSheet::new(); 1],
        }
    }

    pub fn tick(&mut self, cycles: CpuTime) {
        self.now += cycles;
    }

    /// Synchronize the timesheet for the given peripheral and return
    /// the elapsed time synce the last sync.
    pub fn sync(&mut self, who: Peripheral) -> CpuTime {
        self.timesheets[who as usize].sync(self.now)
    }
}

#[derive(Clone,Copy)]
/// Struct used to keep track of individual peripherals
struct TimeSheet {
    /// Date of the last synchronization
    last: CpuTime,
}

impl TimeSheet {

    fn new() -> TimeSheet {
        TimeSheet {
            last: 0,
        }
    }

    /// Forward the time sheet to the current date and return the
    /// elapsed time since the last sync.
    fn sync(&mut self, now: CpuTime) -> CpuTime {
        let delta = now - self.last;

        self.last = now;

        delta
    }
}

/// List of all peripherals requiring a TimeSheet. The value of the
/// enum is used as the index in the table
pub enum Peripheral {
    /// Graphics Processing Unit
    Gpu = 0,
}

/// 64bit timestamps will wrap in roughly 17271 years with a CPU clock
/// at 33.8685MHz so it should be plenty enough.
pub type CpuTime = u64;
