use std::any::Any;
use std::mem;
use std::sync::Mutex;

use actors::{Actor, ActorCell, ActorContext, ActorPath, ActorRef};

enum FutureMessages {
    Complete(Box<Any>),
    // NOTE: the Send in the Box is so that the Future actor is Send as it has to store
    // this inside it.
    // Here I put an Option, but it is supposed to act as an Error, I just wanted to have
    // first version rolling.
    Calculation(Box<Fn(Box<Any>, ActorCell) -> Option<Box<Any + Send>>>),
    Extract,
}

#[derive(PartialEq)]
enum FutureState {
    Uncompleted,
    Computing,
    Terminated,
    Extracted,
}

struct Future {
    value: Mutex<Option<Box<Any + Send>>>,
    state: Mutex<FutureState>,
}

impl Future {
    fn new() -> Future {
        Future {
            value: Mutex::new(None),
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
                    if *state != FutureState::Uncompleted {
                        // NOTE: Send a failure to the sender instead.
                        panic!("Tried to complete a Future twice");
                    }
                    *self.value.lock().unwrap() = Some(unsafe {
                        mem::transmute::<Box<Any>, Box<Any + Send>>(msg)
                    });
                    *state = FutureState::Computing;
                },
                FutureMessages::Calculation(func) => {
                    // FIXME(gamazeps): check the state.
                    let mut value = self.value.lock().unwrap();
                    let v = value.take().unwrap();
                    *value = (*func)(v, context.clone());
                    if let None = *value {
                        *self.state.lock().unwrap() = FutureState::Terminated;
                        context.kill_me();
                    }
                },
                FutureMessages::Extract => {
                    let mut value = self.value.lock().unwrap();
                    let v = value.take();
                    *value = None;
                    *self.state.lock().unwrap() = FutureState::Extracted;
                    // FIXME: Put the value directly in the sender's mailbox.
                    // Indeed, the tell method is generaic and here we would need to but a
                    // Box<Box<Any>> and that sounds pretty bad.
                    context.tell(context.sender(), v);
                    //context.sender().receive(v, context.actor_ref());
                    context.kill_me()
                },
            }
        }
    }
}
