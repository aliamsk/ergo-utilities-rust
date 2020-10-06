pub mod predicated_boxes;
pub mod protocol;
pub mod stage;

pub use ergo_lib::ast::Constant;
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_offchain_utilities::{NanoErg, P2SAddressString};
pub use predicated_boxes::{ErgsBox, StageBox};
pub use protocol::Protocol;
pub use stage::{BoxVerificationError, Result, Stage, StageType};
