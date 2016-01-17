use std::any::Any;
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct Future {
    /// Value contained inside the future.
    value: Mutex<Option<Box<Any>>>,
    /// Closures to cal on the value.
    closures: Mutex<VecDeque<Box<Fn(Box<Any>) -> Box<Any>>>>,
    /// State of the future.
    state: Mutex<FutureState>,
}

#[derive(PartialEq)]
enum FutureState {
    New,
    Completed,
    Computing,
    Taken,
}

impl Future {
    pub fn new() -> Future {
        Future {
            value: Mutex::new(None),
            closures: Mutex::new(VecDeque::new()),
            state: Mutex::new(FutureState::New),
        }
    }

    pub fn complete(&self, value: Box<Any>) {
        let mut state = self.state.lock().unwrap();
        if *state == FutureState::New {
            *self.value.lock().unwrap() = Some(value);
            *state = FutureState::Completed;
        } else {
            panic!("Tried to complete a future in a bad state");
        }
    }

    pub fn handle(&self) {
        // FIXME(gamazeps): check that the state is clean.
        let func = self.closures.lock().unwrap().pop_front();
        if let Some(func) = func {
            let mut value = self.value.lock().unwrap();
            *value = Some(func(value.take().unwrap()));
        }

    }

    pub fn extract(&self) -> Option<Box<Any>> {
        let mut value = self.value.lock().unwrap();
        let v = value.take();
        *value = None;
        *self.state.lock().unwrap() = FutureState::Taken;
        v
    }

    // NOTE: I'm super no sure about the 'static lifetime.
    pub fn then_do<F: Fn(Box<Any>) -> Box<Any> + 'static>(&self, closure: F) {
        self.closures.lock().unwrap().push_back(Box::new(closure));
        // FIXME(gamazeps): schedule the actor.
    }
}
