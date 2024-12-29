// 引入子模块
mod engine;
mod block;
mod file;
mod index;
mod partition;

// 对外暴露子模块的部分内容
pub use engine::{StorageEngine, ColumnStorage};
pub use block::{DataBlock, BlockMetadata};
pub use file::{FileStorage, FileHandler};
pub use index::{Index, BTreeIndex};
pub use partition::{PartitionManager, Partition};

// 定义公共常量
pub const DEFAULT_BLOCK_SIZE: usize = 64 * 1024; // 默认块大小（64KB）

/// `Storage` 结构体是存储模块的统一入口
pub struct Storage {
    engine: engine::StorageEngine,
    partition_manager: partition::PartitionManager,
}

impl Storage {
    /// 创建一个新的 `Storage` 实例
    pub fn new(engine: engine::StorageEngine, partition_manager: partition::PartitionManager) -> Self {
        Self {
            engine,
            partition_manager,
        }
    }

    /// 存储一个数据块
    pub fn store_block(&mut self, block: block::DataBlock) -> Result<(), String> {
        // 使用分区管理器分配分区
        let partition_id = self.partition_manager.assign_partition(&block)?;
        // 使用存储引擎存储数据块
        self.engine.write_block(partition_id, block)
    }

    /// 读取一个数据块
    pub fn read_block(&self, block_id: u64) -> Result<block::DataBlock, String> {
        self.engine.read_block(block_id)
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_operations() {
        // 初始化存储引擎和分区管理器
        let engine = engine::StorageEngine::new();
        let partition_manager = partition::PartitionManager::new();
        let mut storage = Storage::new(engine, partition_manager);

        // 创建数据块
        let block = block::DataBlock::new(1, vec![1, 2, 3]);

        // 测试存储数据块
        storage.store_block(block.clone()).expect("Failed to store block");

        // 测试读取数据块
        let retrieved_block = storage.read_block(1).expect("Failed to read block");
        assert_eq!(retrieved_block, block);
    }
}
