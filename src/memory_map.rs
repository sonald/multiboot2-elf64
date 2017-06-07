#[derive(Debug)]
#[repr(C)]
pub struct MemoryMapTag {
    typ: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    first_area: MemoryArea,
}

impl MemoryMapTag {
    pub fn memory_areas(&self) -> MemoryAreaIter {
        let self_ptr = self as *const MemoryMapTag;
        let start_area = (&self.first_area) as *const MemoryArea;
        MemoryAreaIter {
            current_area: unsafe {&*start_area as &'static _},
            last_area: unsafe {&*((self_ptr as u64 + (self.size - self.entry_size) as u64)
                          as *const MemoryArea)}
                as &'static MemoryArea,
            entry_size: self.entry_size,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryArea {
    pub base_addr: u64,
    pub length: u64,
    typ: u32,
    _reserved: u32,
}

#[derive(Clone, Debug)]
pub struct MemoryAreaIter {
    current_area: &'static MemoryArea,
    last_area: &'static MemoryArea,
    entry_size: u32,
}

impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;
    fn next(&mut self) -> Option<&'static MemoryArea> {
        if self.current_area as *const _ as usize > self.last_area as *const _ as usize {
            None
        } else {
            let area = self.current_area;
            let current = self.current_area as *const _ as u64;
            self.current_area = unsafe {&*((current + self.entry_size as u64) as *const MemoryArea)
                as &'static MemoryArea};
            if area.typ == 1 {
                Some(area)
            } else {self.next()}
        }
    }
}
