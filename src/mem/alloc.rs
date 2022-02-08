use memmap2::{MmapMut};
use std::io::Error;

pub unsafe fn morecore(num_bytes: usize) -> Result<MmapMut, Error> {
    MmapMut::map_anon(num_bytes) 
}

pub unsafe fn free(mem: MmapMut) {
    drop(mem);
}
