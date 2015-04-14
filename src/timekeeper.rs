pub struct TimeKeeper {
    /// Counter keeping track of the current date
    now: TimeStamp,
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

    pub fn tick(&mut self, cycles: TimeStamp) {
        self.now += cycles;
    }

    /// Synchronize the timesheet for the given peripheral and return
    /// the elapsed time synce the last sync.
    pub fn sync(&mut self, who: Peripheral) -> TimeStamp {
        self.timesheets[who as usize].sync(self.now)
    }
}

#[derive(Clone,Copy)]
/// Struct used to keep track of individual peripherals
struct TimeSheet {
    /// Date of the last synchronization
    last: TimeStamp,
}

impl TimeSheet {

    fn new() -> TimeSheet {
        TimeSheet {
            last: 0,
        }
    }

    /// Forward the time sheet to the current date and return the
    /// elapsed time since the last sync.
    fn sync(&mut self, now: TimeStamp) -> TimeStamp {
        let delta = now - self.last;

        self.last = now;

        delta
    }
}

/// List of all peripherals requiring a TimeSheet. The value of the
/// enum is used as the index in the table
pub enum Peripheral {
    /// System timers
    Timers = 0,
}

pub type TimeStamp = u32;
