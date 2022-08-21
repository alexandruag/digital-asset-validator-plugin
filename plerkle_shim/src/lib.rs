use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

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
        // Spawn the thread in `on_load` if all the operations within
        // need to happen after the validator starts sending events to
        // the plugin.
        let thread = thread::spawn(move || {
            // ...
        });

        PlerkleShim {
            inner: Arc::new(Mutex::new(None)),
            thread,
        }
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
