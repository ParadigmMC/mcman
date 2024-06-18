use digest::{Digest, DynDigest, FixedOutput, FixedOutputReset, OutputSizeUser, Reset, Update};

#[derive(Clone)]
pub struct CurseforgeHasher(Vec<u8>);

impl CurseforgeHasher {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Update for CurseforgeHasher {
    fn update(&mut self, data: &[u8]) {
        self.0.extend(
            data.iter()
                .copied()
                .filter(|&e| e != 9 && e != 10 && e != 13 && e != 32),
        )
    }
}

impl Reset for CurseforgeHasher {
    fn reset(&mut self) {
        self.0 = Vec::new()
    }
}

impl OutputSizeUser for CurseforgeHasher {
    type OutputSize = digest::typenum::U4;
}

impl FixedOutput for CurseforgeHasher {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        *out = murmur2::murmur2(&self.0, 1).to_be_bytes().into();
    }
}

impl FixedOutputReset for CurseforgeHasher {
    fn finalize_into_reset(&mut self, out: &mut digest::Output<Self>) {
        *out = murmur2::murmur2(&self.0, 1).to_be_bytes().into();
        self.0 = Vec::new();
    }
}
