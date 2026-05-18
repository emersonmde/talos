#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MmioRegion {
    pub name: &'static str,
    pub base: usize,
    pub size: usize,
}

impl MmioRegion {
    pub const fn new(name: &'static str, base: usize, size: usize) -> Self {
        Self { name, base, size }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MmioMap {
    regions: &'static [MmioRegion],
}

impl MmioMap {
    pub const fn new(regions: &'static [MmioRegion]) -> Self {
        Self { regions }
    }

    pub const fn regions(self) -> &'static [MmioRegion] {
        self.regions
    }
}
