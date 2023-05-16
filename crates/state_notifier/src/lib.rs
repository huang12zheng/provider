//! source from https://github.com/rrousselGit/state_notifier/blob/master/packages/state_notifier/lib/state_notifier.dart
//! 20230516 version 0.7.2+1
//!
//! * Other Info:
//! export 'package:state_notifier/state_notifier.dart' **hide** Listener, LocatorMixin;
#![feature(linked_list_cursors)]
use common::{
    eyre::{bail, ensure, Report, Result},
    getset::{Getters, Setters},
    itertools::join,
    thiserror::Error,
};

use common::crossbeam::channel;

type Listener<T> = fn(&T) -> Result<()>;
type RemoveListener<'a> = dyn FnMut() + Send + Sync + 'a;
type ErrorListener = fn(error: &Report);
use common::generational_token_list::GenerationalTokenList as LinkedList;
use std::{any::type_name, fmt::Debug, sync::Arc};

#[derive(Error, Debug)]
pub enum IError {
    #[error(
        r#"At least listener of the StateNotifier {runtime_type:?} threw an exception
    when the notifier tried to update its state.

    The exceptions thrown are:

    {errors:?}
    "#
    )]
    StateNotifierListenerError {
        errors: String,
        runtime_type: String,
    },
    #[error(
        r#"Tried to use $runtime_type after `dispose` was called.

Consider checking `mounted`."#
    )]
    StateError { runtime_type: String },
    #[error("Concurrent modification during iteration.")]
    ConcurrentModificationError,
}

fn default_on_error(e: &Report) {}

#[derive(Getters, Setters)]
pub struct StateNotifier<T> {
    #[cfg(debug_assertions)]
    _debug_can_add_listeners: bool,
    sender: Option<channel::Sender<T>>,
    receiver: Option<channel::Receiver<T>>,
    listeners: LinkedList<ListenerEntry<T>>,
    #[getset(set = "pub")]
    on_error: ErrorListener,
    #[getset(get = "pub")]
    mounted: bool,
    state: T,
}

struct ListenerEntry<T> {
    pub listener: Listener<T>,
}

impl<'a, T: PartialEq + Clone + Sync + Send> StateNotifier<T> {
    pub fn new(state: T) -> Self {
        StateNotifier {
            _debug_can_add_listeners: true,
            sender: None,
            receiver: None,
            state,
            mounted: true,
            listeners: LinkedList::new(),
            on_error: default_on_error,
        }
    }
    pub fn stream(&mut self) -> &channel::Receiver<T> {
        if self.sender.is_none() {
            let (sender, receiver) = channel::bounded::<T>(0);
            self.sender = Some(sender);
            self.receiver = Some(receiver);
        }
        self.receiver.as_ref().unwrap()
    }
    pub fn state(&mut self) -> &T {
        self._debug_is_mounted().unwrap();
        &self.state
    }
    pub fn set_state(&mut self, value: T) -> Result<()> {
        self._debug_is_mounted().unwrap();
        // if (!updateShouldNotify(previousState, value))
        if self.state.eq(&value) {
            // if std::ptr::eq(&self.state, value) {
            return Ok(());
        }

        // _controller?.add(value);
        if let Some(ref sender) = self.sender {
            let _ = sender.send(value.clone());
        };

        let mut errors = vec![];
        self.listeners.iter().for_each(|e| {
            (e.listener)(&value).unwrap_or_else(|err| {
                (self.on_error)(&err);
                errors.push(err);
            });
        });

        self.state = value.clone();
        if errors.is_empty() {
            let e = IError::StateNotifierListenerError {
                errors: join(errors, "\n"),
                runtime_type: value.type_name(),
            };
            bail!(e); // no need return
        }
        Ok(())
    }

    pub fn has_listeners(&self) -> bool {
        self._debug_is_mounted().unwrap();
        !self.listeners.is_empty()
    }

    pub fn add_listener(
        &mut self,
        listener: Listener<T>,
        fire_immediately: bool,
    ) -> Result<Arc<RemoveListener>> {
        #[cfg(debug_assertions)]
        {
            if !self._debug_can_add_listeners {
                bail!(IError::ConcurrentModificationError)
            }
            self._debug_is_mounted().unwrap();
        }
        let listener_entry = ListenerEntry {
            listener: listener.clone(),
        };
        let index = self.listeners.push_back(listener_entry);

        #[cfg(debug_assertions)]
        self._debug_set_can_add_listeners(false);
        if fire_immediately {
            let _ = (listener)(&self.state).unwrap_or_else(|err| {
                self.listeners.remove(index);
                (self.on_error)(&err)
            });
        }
        #[cfg(debug_assertions)]
        self._debug_set_can_add_listeners(true);

        let callback = move || {
            self.listeners.remove(index);
        };
        Ok(Arc::new(callback))
    }

    pub fn dispose(&mut self) {
        self._debug_is_mounted().unwrap();
        self.listeners.clear();
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        self.mounted = false;
    }
}
impl<T> TypeName for T {}
pub trait TypeName {
    fn type_name(&self) -> String {
        type_name::<Self>().to_owned()
    }
}

impl<T> StateNotifier<T> {
    #[cfg(debug_assertions)]
    fn _debug_set_can_add_listeners(&mut self, value: bool) -> bool {
        self._debug_can_add_listeners = value;
        true
    }
    fn _debug_is_mounted(&self) -> Result<bool> {
        ensure!(
            !self.mounted,
            IError::StateError {
                runtime_type: self.type_name()
            }
        );
        Ok(true)
    }
    #[cfg(debug_assertions)]
    pub fn debug_state(&self) -> &T {
        &self.state
    }
}
