pub mod compute_fn {
    use fnv::FnvHasher;
    use std::hash::Hasher;

    /// 生成哈希值以便于区分文件唯一性
    pub fn gen_hash_from<T: std::hash::Hash>(t: &T) -> u64 {
        let mut hasher = FnvHasher::default();
        t.hash(&mut hasher);
        hasher.finish()
    }
}