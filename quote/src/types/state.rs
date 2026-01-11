use crate::{
    distributor::Distributor,
    master::Connection,
    types::{error::QuoteError, guard::ShutdownGuard},
};
use crossbeam::queue;
#[cfg(feature = "logging")]
use log::{info, warn};
use std::{
    cell::RefCell,
    collections::HashMap,
    net::SocketAddr,
    rc::Rc,
    sync::{Arc, LockResult, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// Стейт мастера севера
pub(crate) struct MasterState {
    // последовательность: 1
    connections: Mutex<HashMap<SocketAddr, Vec<Connection>>>,
    // последовательность: 2
    distributor: Mutex<Distributor>,

    // последовательность: 3
    shutdown: RwLock<bool>,
    // последовательность: 4
    secret_key: RwLock<String>,

    // магические числа взятых последовательностей
    queue: Rc<Mutex<Vec<u8>>>,
}

fn gen_callback(numb: u8, append_pop: bool) -> impl FnOnce(Rc<RefCell<Vec<u8>>>) {
    move |queue: Rc<RefCell<Vec<u8>>>| {
        let mut queue = queue.borrow_mut();
        if !append_pop {
            queue.push(numb);
        } else {
            queue.pop_if(|v| v == &numb);
        }
    }
}

/// думаю сильно переборщил с решением по 15 пункту ревью)))
impl MasterState {
    pub(crate) fn new(
        connections: Mutex<HashMap<SocketAddr, Vec<Connection>>>,
        distributor: Mutex<Distributor>,
        shutdown: RwLock<bool>,
        secret_key: RwLock<String>,
    ) -> Self {
        Self {
            state: Arc::clone(state),
        }
    }

    pub fn secret_key(
        &self,
    ) -> Result<
        ShutdownGuard<
            RwLockReadGuard<'_, String>,
            Rc<RefCell<Vec<u8>>>,
            impl FnOnce(Rc<RefCell<Vec<u8>>>),
        >,
        QuoteError,
    > {
        let can_get = {};
        let func = gen_callback(4, false);
        let secret_key = self
            .state
            .secret_key
            .read()
            .map_err(|_| QuoteError::InternalError)?;

        Ok(ShutdownGuard::new(secret_key, self.queue.clone(), func))
    }

    pub fn shutdown(&self) -> LockResult<RwLockReadGuard<'_, bool>> {
        self.state.shutdown.read()
    }

    pub fn secret_key_mut(&self) -> LockResult<RwLockWriteGuard<'_, String>> {
        self.state.secret_key.write()
    }

    pub fn shutdown_mut(&self) -> LockResult<RwLockWriteGuard<'_, bool>> {
        self.state.shutdown.write()
    }

    pub fn get_subdata(
        &self,
    ) -> Result<
        (
            MutexGuard<'_, HashMap<SocketAddr, Vec<Connection>>>,
            MutexGuard<'_, Distributor>,
        ),
        QuoteError,
    > {
        let all_connections = self
            .state
            .connections
            .lock()
            .map_err(|_| QuoteError::InternalError)?;

        let distributor = self
            .state
            .distributor
            .lock()
            .map_err(|_| QuoteError::InternalError)?;
        return Ok((all_connections, distributor));
    }

    /// ## Чистая эвристика
    /// логика такая: магические числа диктуют последовательность
    /// если уж
    fn can_get(&self, numb: u8) -> bool {
        let queue = self.queue.borrow();
        for i in queue {
            if i == numb {
                return false;
            }
        }
        true
    }
}
