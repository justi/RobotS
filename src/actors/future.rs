use std::any::Any;
use std::collections::VecDeque;
use std::mem;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use actors::{Actor, ActorCell, ActorContext, ActorPath, ActorRef, Message};


#[derive(Clone)]
pub enum FutureMessages {
    /// We complete the future with the value inside the enum.
    Complete(Arc<Any + Send + Sync>),
    /// We apply the following closure to the value inside the Future and update it with the
    /// result.
    ///
    /// *  Extracted will extract the result from the future and kill it.
    /// *  NewValue will update the value inside the Future.
    /// *  Done will kill the Future after the calculations are done.
    ///
    /// Note that Done and Extracted might be a double of each other, I'll try to remove it
    /// afterwards.
    Calculation(Arc<Fn(Box<Any + Send>, ActorCell) -> FutureState + Send + Sync>),
}

pub enum FutureState {
    Uncompleted,
    Computing(Box<Any + Send>),
    Terminated,
    Extracted,
}

pub struct Future {
    state: Mutex<Option<FutureState>>,
    scheduled_calculations: Mutex<VecDeque<Arc<Fn(Box<Any + Send>, ActorCell) -> FutureState + Send + Sync>>>,
}

impl Future {
    pub fn new(_dummy: ()) -> Future {
        Future {
            state: Mutex::new(Some(FutureState::Uncompleted)),
            scheduled_calculations: Mutex::new(VecDeque::new()),
        }
    }

    fn do_computation(&self, value: Box<Any + Send>, func: Arc<Fn(Box<Any + Send>, ActorCell) -> FutureState + Send + Sync>, context: ActorCell) {
        let mut state = self.state.lock().unwrap();
        *state = Some((*func)(value, context.clone()));
        match *state {
            Some(FutureState::Terminated) => context.kill_me(),
            Some(FutureState::Extracted) => context.kill_me(),
            Some(FutureState::Uncompleted) => panic!("A future closure returned Uncompleted, this should not happen"),
            _ => {},
        }
    }
}

impl Actor for Future {
    fn receive(&self, message: Box<Any>, context: ActorCell) {
        // NOTE: We may want to fail if the message is not correct.
        if let Ok(message) = Box::<Any>::downcast::<FutureMessages>(message) {
            match *message {
                FutureMessages::Complete(mut msg) => {
                    let mut state = self.state.lock().unwrap();
                    match *state {
                        Some(FutureState::Uncompleted) => {
                            *state = Some(FutureState::Computing(unsafe {
                                let msg = Arc::get_mut(&mut msg).unwrap();
                                Box::<Any + Send>::from_raw(msg)
                            }));
                            println!("I have been completed");
                            let mut scheduled_calculations = self.scheduled_calculations.lock().unwrap();
                            while let Some(func) = scheduled_calculations.pop_front() {
                                // FIXME(gamazeps) compute for real..
                                panic!("I should be computing");
                            }
                        },
                        Some(_) => {
                            // NOTE: Send a failure to the sender instead.
                            panic!("Tried to complete a Future twice");
                        },
                        None => unreachable!(),
                    }
                },
                FutureMessages::Calculation(func) => {
                    let mut state = self.state.lock().unwrap();
                    let s = state.take().expect("lol");
                    match s {
                        FutureState::Computing(value) => self.do_computation(value, func, context),
                        FutureState::Uncompleted => {
                            *state = Some(s);
                            println!("keeping the calculation for later");
                            self.scheduled_calculations.lock().unwrap().push_back(func);
                        },
                        FutureState::Terminated => {
                            *state = Some(s);
                            panic!("A closure was called on a Terminated Future.");
                        },
                        FutureState::Extracted => {
                            *state = Some(s);
                            panic!("A closure was called on an extracted Future.");
                        },
                    }
                }
            }
        }
    }
}

pub struct FutureExtractor<T: Message> {
    future: ActorRef,
    channel: Arc<Mutex<Sender<T>>>,
}

impl<T: Message> FutureExtractor<T> {
    pub fn new(args: (ActorRef, Arc<Mutex<Sender<T>>>)) -> FutureExtractor<T> {
        FutureExtractor {
            future: args.0,
            channel: args.1,
        }
    }
}

impl<T: Message> Actor for FutureExtractor<T> {
    // Here when the extractor is created it tells the future to forward it its result.
    fn pre_start(&self, context: ActorCell) {
        context.forward_result::<T>(self.future.clone(), context.actor_ref());
    }

    // It then receives the result and will send it through its channel.
    fn receive(&self, message: Box<Any>, context: ActorCell) {
        if let Ok(message) = Box::<Any>::downcast::<T>(message) {
            self.channel.lock().unwrap().send(*message);
            // Once we have sent the message through the channel, we want this actor to be dropped.
            context.kill_me();
        }
    }
}
