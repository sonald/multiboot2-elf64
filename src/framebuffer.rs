#[derive(Debug)]
#[repr(u8)]
pub enum FramebufferType {
    Indexed = 0,
    Rgb = 1,
    EgaText = 2,
}

#[derive(Debug)]
#[repr(packed)] // repr(C) would add unwanted padding before first_section
pub struct FramebufferTag {
    typ: u32,
    size: u32,
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub frame_type: FramebufferType,
    reserved: u16
}

