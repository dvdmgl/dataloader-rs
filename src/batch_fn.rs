use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait BatchFn<K, V> {
    type Error;

    fn max_batch_size(&self) -> usize {
        200
    }

    async fn load(&self, keys: &[K]) -> HashMap<K, Result<V, Self::Error>>
    where
        K: 'async_trait,
        V: 'async_trait,
        Self::Error: 'async_trait;
}
