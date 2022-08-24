use core::arch::asm;

// IO output (fast)
#[inline]
pub fn out8(port: u16, v: u8) {
    unsafe {
        asm! {
            "out dx, al",
            in("al") v,
            in("dx") port,
        };
    }
}

#[inline]
pub fn out16(port: u16, v: u16) {
    unsafe {
        asm! {
            "out dx, ax",
            in("ax") v,
            in("dx") port,
        };
    }
}

#[inline]
pub fn out32(port: u16, v: u32) {
    unsafe {
        asm! {
            "out dx, eax",
            in("eax") v,
            in("dx") port,
        };
    }
}

// IO output (with delay)
#[inline]
pub fn out8_delay(port: u16, v: u8) {
    out8(port, v);
    todo!()
}

#[inline]
pub fn out16_delay(port: u16, v: u16) {
    out16(port, v);
    todo!()
}

#[inline]
pub fn out32_delay(port: u16, v: u32) {
    out32(port, v);
    todo!()
}

// IO input (fast)
#[inline]
pub fn in8(port: u16) -> u8 {
    let mut out: i8;
    unsafe {
        asm! {
            "in al, dx",
            out("al") out,
            in("dx") port,
        };
    }
    out as u8
}

#[inline]
pub fn in16(port: u16) -> u16 {
    let mut out: u16;
    unsafe {
        asm! {
            "in al, dx",
            out("ax") out,
            in("dx") port,
        };
    }
    out
}

#[inline]
pub fn in32(port: u16) -> u32 {
    let mut out: u32;
    unsafe {
        asm! {
            "in al, dx",
            out("eax") out,
            in("dx") port,
        };
    }
    out
}

// IO input (with delay)
#[inline]
pub fn in8_delay(port: u16) {
    in8(port);
    todo!()
}

#[inline]
pub fn in16_delay(port: u16) {
    in16(port);
    todo!()
}

#[inline]
pub fn in32_delay(port: u16) {
    in32(port);
    todo!()
}
