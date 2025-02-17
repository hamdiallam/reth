use alloy_eips::BlockNumHash;

/// A head of the ExEx. It contains the highest host block committed to the
/// internal ExEx state. I.e. the latest block that the ExEx has fully
/// processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExExHead {
    /// The head block.
    pub block: BlockNumHash,
}

impl ExExHead {
    /// Creates a new instance for the given head block.
    pub const fn new(block: BlockNumHash) -> Self {
        Self { block }
    }
}
