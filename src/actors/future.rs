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
    /// Future is created, but the value is not yet filled.
    New,
    /// The value has been completed, but computation have not been started yet.
    Completed,
    /// Computations have been run.
    /// NOTE: this does not mean that a computation is running right now, thus that is not super
    /// Sync.
    Computing,
    /// The value from the Future has been extracted, nothing can be done with it now.
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
        // FIXME(gamazeps): check that the state is clean in a better way.
        if *self.state.lock().unwrap() == FutureState::Completed ||
            *self.state.lock().unwrap() == FutureState::Computing {
                let func = self.closures.lock().unwrap().pop_front();
                if let Some(func) = func {
                    let mut value = self.value.lock().unwrap();
                    *value = Some(func(value.take().unwrap()));
                }
                *self.state.lock().unwrap() = FutureState::Computing;
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
