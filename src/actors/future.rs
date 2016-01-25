use std::any::Any;
use std::mem;
use std::sync::Mutex;

use actors::{Actor, ActorCell, ActorContext, ActorPath, ActorRef};

enum FutureMessages {
    /// We complete the future with the value inside the enum.
    Complete(Box<Any>),
    /// We apply the following closure to the value inside the Future and update it with the
    /// result.
    ///
    /// *  Extracted will extract the result from the future and kill it.
    /// *  NewValue will update the value inside the Future.
    /// *  Done will kill the Future after the calculations are done.
    ///
    /// Note that Done and Extracted might be a double of each other, I'll try to remove it
    /// afterwards.
    Calculation(Box<Fn(Box<Any>, ActorCell) -> FutureState>),
}

enum FutureState {
    Uncompleted,
    Computing(Box<Any + Send>),
    Terminated,
    Extracted,
}

struct Future {
    state: Mutex<FutureState>,
}

impl Future {
    fn new() -> Future {
        Future {
            state: Mutex::new(FutureState::Uncompleted),
        }
    }
}

impl Actor for Future {
    fn receive(&self, message: Box<Any>, context: ActorCell) {
        // NOTE: We may want to fail if the message is not correct.
        if let Ok(message) = Box::<Any>::downcast::<FutureMessages>(message) {
            match *message {
                FutureMessages::Complete(msg) => {
                    let mut state = self.state.lock().unwrap();
                    match *state {
                        FutureState::Uncompleted => {
                            *state = FutureState::Computing(unsafe {
                                mem::transmute::<Box<Any>, Box<Any + Send>>(msg)
                            });
                        },
                        _ => {
                            // NOTE: Send a failure to the sender instead.
                            panic!("Tried to complete a Future twice");
                        }
                    }
                },
                FutureMessages::Calculation(func) => {
                    // FIXME(gamazeps): check the state.
                    let mut state = self.state.lock().unwrap();
                    match *state {
                        FutureState::Computing(value) => {
                            let res = (*func)(value, context.clone());
                            match res {
                                FutureState::Computing(v) => *state = FutureState::Computing(v),
                                FutureState::Terminated => {
                                    *state = FutureState::Terminated;
                                    context.kill_me();
                                }
                                FutureState::Extracted => {
                                    *state = FutureState::Extracted;
                                    context.kill_me();}
                                ,
                                FutureState::Uncompleted => panic!("A future closure returned Uncompleted, this should not happen"),
                            }
                        },
                        FutureState::Uncompleted => panic!("A closure was called on an uncompleted Future."),
                        FutureState::Terminated => panic!("A closure was called on a Terminated Future."),
                        FutureState::Extracted => panic!("A closure was called on an extracted Future."),
                    }
                },
            }
        }
    }
}
