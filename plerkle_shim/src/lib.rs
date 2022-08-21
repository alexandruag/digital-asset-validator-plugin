use std::ops::DerefMut;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

use libloading::{Library, Symbol};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, SlotStatus,
};

pub trait PlerkleShimInner: GeyserPlugin {
    fn sup(&self);
}

#[derive(Debug)]
struct PlerkleShim {
    inner: Arc<Mutex<Option<Box<dyn PlerkleShimInner>>>>,
    thread: JoinHandle<()>,
}

impl PlerkleShim {
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(None));
        let inner_clone = inner.clone();

        // Spawn the thread in `on_load` if all the operations within
        // need to happen after the validator starts sending events to
        // the outer (shim) plugin.
        let thread = thread::spawn(move || {
            // This thread is responsible for:
            // - receiving and acting on external input (i.e. unload/update inner plugin,
            //   new settings, etc).
            // - when unloading the inner plugin, waiting for the entire operation to
            //   complete; in theory this can take quite long and may otherwise cause
            //   head-of-line blocking. The same applies to any other non-instantaneous ops.

            // ...

            // Example when loading an inner
            {
                // Safe because ...
                unsafe {
                    let _lib = Library::new("/path/to/lib.so").expect("handle error");
                    let _func: Symbol<unsafe extern "C" fn() -> *mut dyn PlerkleShimInner> =
                        _lib.get(b"create_shim").expect("handle error");

                    let shim = Box::<dyn PlerkleShimInner>::from_raw(_func());

                    let old = inner_clone.lock().unwrap().take();
                    if let Some(plugin) = old {
                        // Should unload old plugin first (if any).
                    }

                    inner_clone.lock().unwrap().replace(shim);
                }
            }

            // ...
        });

        PlerkleShim { inner, thread }
    }

    fn inner_guard(&self) -> MutexGuard<Option<Box<dyn PlerkleShimInner>>> {
        // The `unwrap` only fails if another thread panicked while
        // holding the lock.
        self.inner.lock().unwrap()
    }
}

impl GeyserPlugin for PlerkleShim {
    fn name(&self) -> &'static str {
        if let Some(inner) = self.inner_guard().as_ref() {
            inner.name()
        } else {
            "empty"
        }
    }

    fn on_load(
        &mut self,
        _config_file: &str,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn on_unload(&mut self) {
        todo!()
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        if let Some(inner) = self.inner_guard().as_mut() {
            if inner.account_data_notifications_enabled() {
                return inner.update_account(account, slot, is_startup);
            }
        }

        Ok(())
    }

    fn notify_end_of_startup(
        &mut self,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        if let Some(inner) = self.inner_guard().as_mut() {
            return inner.update_slot_status(slot, parent, status);
        }

        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        if let Some(inner) = self.inner_guard().as_mut() {
            if inner.transaction_notifications_enabled() {
                return inner.notify_transaction(transaction, slot);
            }
        }

        Ok(())
    }

    fn notify_block_metadata(
        &mut self,
        blockinfo: ReplicaBlockInfoVersions,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        if let Some(inner) = self.inner_guard().as_mut() {
            return inner.notify_block_metadata(blockinfo);
        }

        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = PlerkleShim::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
