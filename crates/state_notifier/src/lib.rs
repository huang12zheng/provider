//! source from https://github.com/rrousselGit/state_notifier/blob/master/packages/state_notifier/lib/state_notifier.dart
//! 20230516 version 0.7.2+1
//!
//! * Other Info:
//! export 'package:state_notifier/state_notifier.dart' **hide** Listener, LocatorMixin;
#![feature(linked_list_cursors)]
use common::{
    eyre::{bail, Result},
    getset::{Getters, Setters},
    itertools::join,
    thiserror::Error,
};

type Listener<T> = fn(&T) -> Result<()>;
// type RemoveListener = fn();
type RemoveListener<'a> = dyn FnMut() + Send + Sync + 'a;
type ErrorListener = fn(error: IError);
// use common::atlist_rs::LinkedList;
use std::{
    any::type_name,
    // collections::LinkedList,

    // error::Report,
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
    // unimplemented,
    todo,
};
// use Vec as LinkedList;
use common::generational_token_list::GenerationalTokenList as LinkedList;

#[derive(Error, Debug)]
pub enum IError {
    #[error(
        r#"At least listener of the StateNotifier {state_notifier:?} threw an exception
when the notifier tried to update its state.

The exceptions thrown are:

{errors:?}
"#
    )]
    StateNotifierListenerError {
        errors: String,
        state_notifier: String,
    },
    // #[error("tried to call [LocatorMixin.read<{0}>()], but the [{0}] was not found")]
    // DependencyNotFoundException {
    //     phantom: std::marker::PhantomData<T>,
    // },
    // #[error("Bad state: {message}")]
    // StateError { message: String },
}
fn default_on_error(e: IError) {}

#[derive(Getters, Setters)]
pub struct StateNotifier<T> {
    listeners: LinkedList<ListenerEntry<T>>,
    #[getset(set = "pub")]
    on_error: ErrorListener,
    // #[derivative(Default(value = "true"))]
    #[getset(get = "pub")]
    mounted: bool,
    #[getset(get = "pub")]
    state: T,
}

struct ListenerEntry<T> {
    pub listener: Listener<T>,
}

impl<'a, T: PartialEq + Clone + Sync + Send> StateNotifier<T> {
    pub fn new(state: T) -> Self {
        StateNotifier {
            state,
            mounted: true,
            listeners: LinkedList::new(),
            on_error: default_on_error,
        }
    }
    pub fn set_state(&mut self, value: &T) -> Result<()> {
        // if (!updateShouldNotify(previousState, value))
        if &self.state == value {
            // if std::ptr::eq(&self.state, value) {
            return Ok(());
        }
        // _controller?.add(value);

        let mut errors = vec![];
        self.listeners.iter().for_each(|e| {
            (e.listener)(value).unwrap_or_else(|err| errors.push(err));
        });

        self.state = value.clone();
        if errors.is_empty() {
            Ok(())
        } else {
            let e = IError::StateNotifierListenerError {
                errors: join(errors, "\n"),
                state_notifier: type_name::<T>().to_owned(),
            };
            bail!(e)
        }
    }

    pub fn has_listeners(&self) -> bool {
        !self.listeners.is_empty()
    }

    pub fn add_listener(
        &mut self,
        listener: Listener<T>,
        fire_immediately: bool,
    ) -> Result<Arc<RemoveListener>> // ->
    {
        let listener_entry = ListenerEntry {
            listener: listener.clone(),
        };
        let index = self.listeners.push_back(listener_entry);

        if fire_immediately {
            let _ = (listener)(&self.state);
        }

        let callback = move || {
            self.listeners.remove(index);
        };
        Ok(Arc::new(callback))
    }

    // pub fn set_on_error(&mut self, error_listener: ErrorListener) {
    //     self.on_error = Some(error_listener);
    // }

    // #[cfg(test)]
    // pub fn debug_state(&self) -> T {
    //     self.state
    // }
    // fn updateShouldNotify
}
// const LOCATOR: Locator<T> = || unimplemented!();
// <T>() => throw DependencyNotFoundException<T>();
// remove it
// pub trait LocatorMixin {
//     fn locator<T>(&self) -> Result<T> {
//         // todo!()
//         panic!("{:?}"，Error::DependencyNotFoundException::<T>())
//     }

// fn read<T>(&self) -> Result<Locator<T>, Error<T>> {}

// fn set_read<T>(&mut self, read: Box<dyn Fn() -> Result<T, DependencyNotFoundException<T>>>);

// fn debug_mock_dependency<T, Dependency>(&mut self, value: Dependency)
// where
//     T: 'static,
//     Dependency: 'static;

// fn init_state(&mut self);

// fn update(&mut self, watch: Box<dyn Fn() -> Result<(), DependencyNotFoundException<()>>>>;

// fn debug_update(&mut self);
// }
// #[derive(Debug, Error)]
// #[error("At least listener of the StateNotifier {0} threw an exception when the notifier tried to update its state.\n\nThe exceptions thrown are:\n\n{0:?}")]
// struct StateNotifierListenerError {
//     errors: Vec<Box<dyn std::error::Error>>,
//     stack_traces: Vec<Option<std::backtrace::Backtrace>>,
//     state_notifier: StateNotifier,
// }

// fn p() {
//     let a: Listener<u32> = |a| {};
// }
// type Locator = dyn Fn() -> T;

// struct StateNotifier<T> {
//     state: T,
//     listeners: Vec<Listener<T>>,
//     error_listener: Option<ErrorListener>,
// }

// impl<T> StateNotifier<T> {
//     fn new(initial_state: T) -> StateNotifier<T> {
//         StateNotifier {
//             state: initial_state,
//             listeners: Vec::new(),
//             error_listener: None,
//         }
//     }

//     fn add_listener(&mut self, listener: Listener<T>) -> RemoveListener {
//         self.listeners.push(listener);
//         let index = self.listeners.len() - 1;
//         let remove_listener: RemoveListener = || {
//             self.listeners.remove(index);
//         };
// struct StateNotifier<T> {
//     state: T,
//     listeners: Vec<Listener<T>>,
//     error_listener: Option<ErrorListener>,
// }

// impl<T> StateNotifier<T> {
//     fn new(initial_state: T) -> StateNotifier<T> {
//         StateNotifier {
//             state: initial_state,
//             listeners: Vec::new(),
//             error_listener: None,
//         }
//     }

//     fn add_listener(&mut self, listener: Listener<T>) -> RemoveListener {
//         self.listeners.push(listener);
//         let index = self.listeners.len() - 1;
//         let remove_listener: RemoveListener = || {
//             self.listeners.remove(index);
//         };
//         remove_listener
//     }

//     fn set_state(&mut self, new_state: T) {
//         self.state = new_state;
//         for listener in &self.listeners {
//             listener(self.state.clone());
//         }
//     }

//     fn on_error(&mut self, error_listener: ErrorListener) {
//         self.error_listener = Some(error_listener);
//     }

//     fn notify_error(
//         &self,
//         error: &dyn std::error::Error,
//         stack_trace: Option<&std::backtrace::Backtrace>,
//     ) {
//         if let Some(error_listener) = &self.error_listener {
//             error_listener(error, stack_trace);
//         }
//     }

//     fn with_state<R>(&self, locator: Locator, callback: fn(&T) -> R) -> R {
//         let state = (locator)();
//         callback(&state)
//     }
// }
