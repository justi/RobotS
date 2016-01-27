use std::any::Any;
use std::sync::Arc;

use actors::{ActorContext, InnerMessage, Message, SystemMessage};
use actors::actor_cell::ActorCell;
use actors::cthulhu::Cthulhu;

#[derive(Debug, Eq, Hash, PartialEq)]
/// Path to an actor.
///
/// This enum contains the information for actors whether they are local or distant.
pub enum ActorPath {
    /// Logical path to a local actor.
    Local(String),
    /// Logical path and connection information for a distant actor.
    Distant(ConnectionInfo),
}

impl ActorPath {
    /// Creates a new local ActorPath variant with the given logical_path.
    pub fn new_local(path: String) -> Arc<ActorPath> {
        Arc::new(ActorPath::Local(path))
    }

    /// Creates a new distant ActorPath.
    pub fn new_distant(distant_logical_path: String, addr_port: String) -> Arc<ActorPath> {
        Arc::new(ActorPath::Distant(
                ConnectionInfo {
                    distant_logical_path: distant_logical_path,
                    addr_port: addr_port,
                }))
    }

    /// Gives a reference to the logical path of an actor.
    ///
    /// Note that this gives the local logical path whether the actor is local or not.
    pub fn logical_path(&self) -> &String {
        match *self {
            ActorPath::Local(ref s) => s,
            ActorPath::Distant(ref c) => &(c.distant_logical_path),
        }
    }

    /// Creates an ActorPath for a child of an actor.
    ///
    /// This gives a Local variant, because actors are always created locally.
    pub fn child(&self, name: String) -> Arc<ActorPath> {
        match *self {
            ActorPath::Local(ref s) => {
                let path = format!("{}/{}", s, name);
                ActorPath::new_local(path)
            },
            ActorPath::Distant(_) => panic!("Cannot create a child for a distant actor."),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
/// This gives connection informations on how to get to the distant actors.
///
/// *  The distant_logical_path is something like "/user/distant/actor".
/// *  The addr_port is something like "127.0.0.1:12345"
///
/// Note that the storage of the addr_port could be improved, but is not a concern for now.
pub struct ConnectionInfo {
    distant_logical_path: String,
    addr_port: String,
}


impl ConnectionInfo {
    /// Distant logical path.
    pub fn distant_logical_path(&self) -> &String {
        &self.distant_logical_path
    }
    /// Address and port of the distant actor.
    pub fn addr_port(&self) -> &String {
        &self.addr_port
    }
}

#[derive(Clone)]
enum InnerActor {
    Cthulhu(Cthulhu),
    Actor(ActorCell),
}

/// An `ActorRef` is the way used to interract with something that acts as an actor.
///
/// This can represent either an ACtor, a Future or Cthulhu (the original actor) whether distant or
/// local.
///
/// It gives the Actor API, it can receive messages, be told to send messages, be asked something
/// and give its ActorPath.
pub struct ActorRef {
    inner_actor: Option<InnerActor>,
    path: Arc<ActorPath>,
}

impl ActorRef {
    /// Creates a new ActorRef to a distant actor, with the given ActorPath.
    pub fn new_distant(path: Arc<ActorPath>) -> ActorRef {
        ActorRef {
            inner_actor: None,
            path: path,
        }
    }

    /// Creates a new ActorRef to Cthulhu, this should only be called once.
    pub fn with_cthulhu(cthulhu: Cthulhu) -> ActorRef {
        let path = ActorPath::new_local("/".to_owned());
        ActorRef {
            inner_actor: Some(InnerActor::Cthulhu(cthulhu)),
            path: path,
        }
    }

    /// Creates a new ActorRef for a local Actor, with the given ActorCell.
    pub fn with_cell(cell: ActorCell, path: Arc<ActorPath>) -> ActorRef {
        ActorRef {
            inner_actor: Some(InnerActor::Actor(cell)),
            path: path,
        }
    }

    /// Receives a system message such as `Start`, `Restart` or a `Failure(ActorRef)`, puts it in
    /// the system mailbox and schedules the actor if needed.
    pub fn receive_system_message(&self, system_message: SystemMessage) {
        info!("{} receiving a system message", self.path().logical_path());
        let inner = self.inner_actor.as_ref().expect("Tried to put a system message in the mailbox of a distant actor.");
        match *inner {
            InnerActor::Actor(ref actor) => actor.receive_system_message(system_message),
            InnerActor::Cthulhu(ref cthulhu) => cthulhu.receive_system_message(),
        };
    }

    /// Receives a regular message and puts it in the mailbox and schedules the actor if needed.
    pub fn receive(&self, message: InnerMessage, sender: ActorRef) {
        info!("{} receiving a message", self.path().logical_path());
        let inner = self.inner_actor.as_ref().expect("Tried to put a message in the mailbox of a distant actor.");
        match *inner {
            InnerActor::Actor(ref actor) => actor.receive_message(message, sender),
            InnerActor::Cthulhu(ref cthulhu) => cthulhu.receive(),
        };
    }

    /// Handles a messages by calling the `receive` method of the underlying actor.
    pub fn handle(&self) {
        info!("{} handling a message", self.path().logical_path());
        let inner = self.inner_actor.as_ref().expect("");
        match *inner {
            InnerActor::Actor(ref actor) => actor.handle_envelope(),
            InnerActor::Cthulhu(ref cthulhu) => cthulhu.handle(),
        };
    }

    /// Gives a clone of the ActorPath.
    pub fn path(&self) -> Arc<ActorPath> {
        self.path.clone()
    }

    /// Makes this ActorRef send a message to anther ActorRef.
    pub fn tell_to<MessageTo: Message>(&self, to: ActorRef, message: MessageTo) {
        let inner = self.inner_actor.as_ref().expect("");
        info!("an actor is telling something to another");
        let message: Box<Any + Send> = Box::new(message);
        to.receive(InnerMessage::Message(message), self.clone())
    }
}

impl Clone for ActorRef {
    fn clone(&self) -> ActorRef {
        ActorRef {
            inner_actor: self.inner_actor.clone(),
            path: self.path.clone(),
        }
    }
}
