use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

/// BmbpCacheItem 缓存对象
#[allow(dead_code)]
pub struct BmbpCacheItem<T> {
    /// 标识
    key: String,
    /// 缓存数据
    data: T,
    /// 开始时间
    start: usize,
    /// 存活时长
    live: isize,
}

impl<T> BmbpCacheItem<T> {
    pub fn data(&self) -> &T {
        self.data.borrow()
    }
}

/// BmbpCache 缓存模块
pub struct BmbpCache<T> {
    store: HashMap<String, BmbpCacheItem<T>>,
}

impl<T> BmbpCache<T> {
    pub fn new() -> BmbpCache<T> {
        let mp: HashMap<String, BmbpCacheItem<T>> = HashMap::new();
        BmbpCache { store: mp }
    }

    pub fn store(&self) -> &HashMap<String, BmbpCacheItem<T>> {
        self.store.borrow()
    }

    pub fn set(&mut self, key: String, t: T) {
        let item = BmbpCacheItem {
            key: key.clone(),
            data: t,
            start: 0,
            live: 0,
        };
        self.store.insert(key, item);
    }

    pub fn get(&self, key: String) -> Option<&T> {
        let item = self.store.get(key.as_str());
        return match item {
            Some(data) => Some(data.data.borrow()),
            None => None,
        };
    }

    pub fn get_mut(&mut self, key: String) -> Option<&mut T> {
        let item = self.store.get_mut(key.as_str());
        return match item {
            Some(data) => Some(data.data.borrow_mut()),
            None => None,
        };
    }
    pub fn clear(&mut self) {
        self.store.clear();
    }
    pub fn size(&self) -> usize {
        self.store.len()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::types::cache::BmbpCache;

    #[test]
    fn test_cache() {
        let mut ca = BmbpCache::<HashMap<&str, &str>>::new();
        let mut m = HashMap::new();
        m.insert("item", "item");
        ca.set("1".to_string(), m);

        assert_eq!(ca.size(), 1);

        let ke = ca.get_mut("1".to_string());

        match ke {
            Some(v) => {
                v.insert("item2", "item2");
                assert_eq!(v.len(), 2);
            }
            None => {
                assert_eq!(1, 2)
            }
        }

        let ke = ca.get("1".to_string());
        match ke {
            Some(v) => {
                println!("{}", v.len());
                assert_eq!(v.len(), 2);
            }
            None => {
                assert_eq!(1, 2)
            }
        }

        ca.clear();
        assert_eq!(ca.size(), 0)
    }
}
