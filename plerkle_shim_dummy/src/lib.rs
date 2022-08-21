use std::ffi::c_void;

use plerkle_shim::PlerkleShimInner;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, SlotStatus,
};

#[derive(Debug)]
struct DummyShim {}

impl GeyserPlugin for DummyShim {
    fn name(&self) -> &'static str {
        todo!()
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
        _account: ReplicaAccountInfoVersions,
        _slot: u64,
        _is_startup: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn notify_end_of_startup(
        &mut self,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn update_slot_status(
        &mut self,
        _slot: u64,
        _parent: Option<u64>,
        _status: SlotStatus,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn notify_transaction(
        &mut self,
        _transaction: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn notify_block_metadata(
        &mut self,
        _blockinfo: ReplicaBlockInfoVersions,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        todo!()
    }

    fn account_data_notifications_enabled(&self) -> bool {
        todo!()
    }

    fn transaction_notifications_enabled(&self) -> bool {
        todo!()
    }
}

impl PlerkleShimInner for DummyShim {
    fn sup(&self) {
        println!("sup dummy!")
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_shim() -> *mut c_void {
    let shim: Box<dyn PlerkleShimInner> = Box::new(DummyShim {});
    Box::into_raw(shim) as *mut c_void
}
