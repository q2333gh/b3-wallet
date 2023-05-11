use b3_shared::{b3_sha256_hex, types::Blob};

use crate::types::{SystemWasm, WasmHash};

impl Default for SystemWasm {
    fn default() -> Self {
        Self(Blob::new())
    }
}

impl SystemWasm {
    pub fn load(&mut self, blob: &Blob) -> usize {
        self.extend(blob);

        self.len()
    }

    pub fn get(&self) -> Blob {
        self.0.clone()
    }

    pub fn extend(&mut self, blob: &Blob) {
        self.0.extend(blob);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_loading(&self, size: usize) -> bool {
        self.is_empty() || self.len() < size
    }

    pub fn is_loaded(&self, size: usize) -> bool {
        self.len() == size
    }

    pub fn generate_hash(&self) -> WasmHash {
        if self.is_empty() {
            return WasmHash::default();
        }

        b3_sha256_hex(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut wasm = SystemWasm::default();
        let blob = vec![1, 2, 3];

        assert_eq!(wasm.load(&blob), 3);
        assert_eq!(wasm.0, vec![1, 2, 3]);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());
    }

    #[test]
    fn test_load_multiple() {
        let mut wasm = SystemWasm::default();
        let blob = vec![1, 2, 3];
        let blob2 = vec![4, 5, 6];

        assert_eq!(wasm.load(&blob), 3);
        assert_eq!(wasm.load(&blob2), 6);
        assert_eq!(wasm.0, vec![1, 2, 3, 4, 5, 6]);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());

        if wasm.is_loaded(6) {
            wasm.clear();
        }

        assert_eq!(wasm.len(), 0);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());
    }
}
