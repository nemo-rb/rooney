use super::{db, Command, Cooldown, Error, Result};
use std::cell::RefCell;
use std::time::{Duration, Instant};


const COOLDOWN: u64 = 3;


pub(super) struct Remark {
    last_call: RefCell<Option<Instant>>
}


impl Remark {
    pub(super) fn new() -> Self {
        Self {
            last_call: RefCell::new(None)
        }
    }
}


impl Cooldown for Remark {
    fn get_last_call(&self) -> Option<Instant> {
        *self.last_call.borrow()
    }

    fn set_last_call(&self) {
        *self.last_call.borrow_mut() = Some(Instant::now())
    }

    fn on_cooldown(&self) -> bool {
        match self.get_last_call() {
            None => {
                self.set_last_call();
                false
            },
            last_call => {
                let elapsed = last_call.unwrap_or_else(Instant::now).elapsed();
                if elapsed > Duration::new(60*COOLDOWN, 0) {
                    self.set_last_call();
                    false
                } else {
                    true
                }
            }
        }
    }
}


impl Command for Remark {
    fn name(&self) -> &'static str {
        "remark"
    }

    fn run(&self, db: &db::DB, msg: &Option<&str>) -> Result<String> {
        if self.on_cooldown() {
            Err(Error::Contact)
        } else {
            Ok(db.get_remark(msg.unwrap()).unwrap())
        }
    }

    fn help(&self) -> &'static str {
        ""
    }
}