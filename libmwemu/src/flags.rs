use serde::{Deserialize, Serialize};

pub const MIN_I8: i8 = -128;
pub const MAX_I8: i8 = 0x7f;
pub const MIN_U8: u8 = 0;
pub const MAX_U8: u8 = 0xff;

pub const MIN_I16: i16 = -32768;
pub const MAX_I16: i16 = 0x7fff;
pub const MIN_U16: u16 = 0;
pub const MAX_U16: u16 = 0xffff;

pub const MIN_I32: i32 = -2147483648;
pub const MAX_I32: i32 = 0x7fffffff;
pub const MIN_U32: u32 = 0;
pub const MAX_U32: u32 = 0xffffffff;

pub const MIN_I64: i64 = -9223372036854775808;
pub const MAX_I64: i64 = 0x7fffffffffffffff;
pub const MIN_U64: u64 = 0;
pub const MAX_U64: u64 = 0xffffffffffffffff;

// instead of table we generate the table at compile time to make sure it is correct
// the parity table calculate true if the number of zero-bit in the lsb 8-bit is even and false otherwise.
const fn build_parity_table() -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = i.count_ones() % 2 == 0;
        i += 1;
    }
    table
}

pub const PARITY_LOOKUP_TABLE: [bool; 256] = build_parity_table();


macro_rules! get_bit {
    ($val:expr, $count:expr) => {
        ($val & (1 << $count)) >> $count
    };
}

macro_rules! set_bit {
    ($val:expr, $count:expr, $bit:expr) => {
        if $bit == 1 {
            $val |= 1 << $count;
        } else {
            $val &= !(1 << $count);
        }
    };
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Flags {
    pub f_cf: bool,
    pub f_pf: bool,
    pub f_af: bool,
    pub f_zf: bool,
    pub f_sf: bool,
    pub f_tf: bool,
    pub f_if: bool,
    pub f_df: bool,
    pub f_of: bool,
    pub f_iopl1: bool,
    pub f_iopl2: bool,
    pub f_nt: bool,
    pub f_rf: bool,
    pub f_vm: bool,
    pub f_ac: bool,
    pub f_vif: bool,
    pub f_vip: bool,
    pub f_id: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            f_cf: false,
            f_pf: false,
            f_af: false,
            f_zf: false,
            f_sf: false,
            f_tf: false,
            f_if: false,
            f_df: false,
            f_of: false,
            f_iopl1: false,
            f_iopl2: false,
            f_nt: false,
            f_rf: false,
            f_vm: false,
            f_ac: false,
            f_vif: false,
            f_vip: false,
            f_id: false,
        }
    }

    pub fn clear(&mut self) {
        self.f_cf = false;
        self.f_pf = false;
        self.f_af = false;
        self.f_zf = false;
        self.f_sf = false;
        self.f_tf = false;
        self.f_if = false;
        self.f_df = false;
        self.f_of = false;
        self.f_iopl1 = false;
        self.f_iopl2 = false;
        self.f_nt = false;
        self.f_rf = false;
        self.f_vm = false;
        self.f_ac = false;
        self.f_vif = false;
        self.f_vip = false;
        self.f_id = false;
    }

    pub fn print_trace(&self, pos: u64) {
        let mut fs = String::new();
        fs.push_str("[ ");
        if self.f_cf {
            fs.push_str("CF ");
        }
        if self.f_pf {
            fs.push_str("PF ");
        }
        if self.f_af {
            fs.push_str("AF ");
        }
        if self.f_zf {
            fs.push_str("ZF ");
        }
        if self.f_sf {
            fs.push_str("SF ");
        }
        if self.f_tf {
            fs.push_str("TF ");
        }
        if self.f_if {
            fs.push_str("IF ");
        }
        if self.f_df {
            fs.push_str("DF ");
        }
        if self.f_of {
            fs.push_str("OF ");
        }
        fs.push_str("]");
        log::info!("\t{} flags: 0x{:x} {}", pos, self.dump(), fs);
        
    }

    pub fn print(&self) {
        log::info!("--- flags ---");
        log::info!("0x{:x}", self.dump());
        log::info!("cf: {}", self.f_cf);
        log::info!("pf: {}", self.f_pf);
        log::info!("af: {}", self.f_af);
        log::info!("zf: {}", self.f_zf);
        log::info!("sf: {}", self.f_sf);
        log::info!("tf: {}", self.f_tf);
        log::info!("if: {}", self.f_if);
        log::info!("df: {}", self.f_df);
        log::info!("of: {}", self.f_of);
        log::info!("iopl1: {}", self.f_iopl1);
        log::info!("iopl2: {}", self.f_iopl2);
        log::info!("nt: {}", self.f_nt);
        log::info!("rf: {}", self.f_rf);
        log::info!("vm: {}", self.f_vm);
        log::info!("ac: {}", self.f_ac);
        log::info!("vif: {}", self.f_vif);
        log::info!("vip: {}", self.f_vip);
        log::info!("id: {}", self.f_id);
        log::info!("---");
    }

    pub fn diff(a: Flags, b: Flags) -> String {
        let mut output = String::new();
        // f_cf
        if a.f_cf != b.f_cf {
            output = format!("{}{}: {} -> {} ", output, "cf", a.f_cf, b.f_cf);
        }
        // f_pf
        if a.f_pf != b.f_pf {
            output = format!("{}{}: {} -> {} ", output, "pf", a.f_pf, b.f_pf);
        }
        // f_af
        if a.f_af != b.f_af {
            output = format!("{}{}: {} -> {} ", output, "af", a.f_af, b.f_af);
        }
        // f_zf
        if a.f_zf != b.f_zf {
            output = format!("{}{}: {} -> {} ", output, "zf", a.f_zf, b.f_zf);
        }
        // f_sf
        if a.f_sf != b.f_sf {
            output = format!("{}{}: {} -> {} ", output, "sf", a.f_sf, b.f_sf);
        }
        // f_tf
        if a.f_tf != b.f_tf {
            output = format!("{}{}: {} -> {} ", output, "tf", a.f_tf, b.f_tf);
        }
        // f_if
        if a.f_if != b.f_if {
            output = format!("{}{}: {} -> {} ", output, "if", a.f_if, b.f_if);
        }
        // f_df
        if a.f_df != b.f_df {
            output = format!("{}{}: {} -> {} ", output, "df", a.f_df, b.f_df);
        }
        // f_of
        if a.f_of != b.f_of {
            output = format!("{}{}: {} -> {} ", output, "of", a.f_of, b.f_of);
        }
        // f_iopl1
        if a.f_iopl1 != b.f_iopl1 {
            output = format!("{}{}: {} -> {} ", output, "iopl1", a.f_iopl1, b.f_iopl1);
        }
        // f_iopl2
        if a.f_iopl2 != b.f_iopl2 {
            output = format!("{}{}: {} -> {} ", output, "iopl2", a.f_iopl2, b.f_iopl2);
        }
        // f_nt
        if a.f_nt != b.f_nt {
            output = format!("{}{}: {} -> {} ", output, "nt", a.f_nt, b.f_nt);
        }
        // f_rf
        if a.f_rf != b.f_rf {
            output = format!("{}{}: {} -> {} ", output, "rf", a.f_rf, b.f_rf);
        }
        // f_vm
        if a.f_vm != b.f_vm {
            output = format!("{}{}: {} -> {} ", output, "vm", a.f_vm, b.f_vm);
        }
        // f_ac
        if a.f_ac != b.f_ac {
            output = format!("{}{}: {} -> {} ", output, "ac", a.f_ac, b.f_ac);
        }
        // f_vif
        if a.f_vif != b.f_vif {
            output = format!("{}{}: {} -> {} ", output, "vif", a.f_vif, b.f_vif);
        }
        // f_vip
        if a.f_vip != b.f_vip {
            output = format!("{}{}: {} -> {} ", output, "vip", a.f_vip, b.f_vip);
        }
        // f_id
        if a.f_id != b.f_id {
            output = format!("{}{}: {} -> {} ", output, "id", a.f_id, b.f_id);
        }
        output
    }

    pub fn dump(&self) -> u32 {
        let mut flags: u32 = 0;

        if self.f_cf {
            set_bit!(flags, 0, 1);
        }
        set_bit!(flags, 1, 1); // always 1 in EFLAGS
        if self.f_pf {
            set_bit!(flags, 2, 1);
        }
        // 3 is reserved
        if self.f_af {
            set_bit!(flags, 4, 1);
        }
        // 5 is reserved
        if self.f_zf {
            set_bit!(flags, 6, 1);
        }
        if self.f_sf {
            set_bit!(flags, 7, 1);
        }
        if self.f_tf {
            set_bit!(flags, 8, 1);
        }
        if self.f_if {
            set_bit!(flags, 9, 1);
        }
        if self.f_df {
            set_bit!(flags, 10, 1);
        }
        if self.f_of {
            set_bit!(flags, 11, 1);
        }

        if self.f_iopl1 {
            set_bit!(flags, 12, 1);
        }
        if self.f_iopl2 {
            set_bit!(flags, 13, 1);
        }

        if self.f_nt {
            set_bit!(flags, 14, 1);
        }
        set_bit!(flags, 15, 0);
        if self.f_rf {
            set_bit!(flags, 16, 1);
        }
        if self.f_vm {
            set_bit!(flags, 17, 1);
        }
        if self.f_ac {
            set_bit!(flags, 18, 1);
        }
        if self.f_vif {
            set_bit!(flags, 19, 1);
        }
        if self.f_vip {
            set_bit!(flags, 20, 1);
        }
        if self.f_id {
            set_bit!(flags, 21, 1);
        }

        flags
    }

    pub fn load(&mut self, flags: u32) {
        self.f_cf = get_bit!(flags, 0) == 1;
        self.f_pf = get_bit!(flags, 2) == 1;
        self.f_af = get_bit!(flags, 4) == 1;
        self.f_zf = get_bit!(flags, 6) == 1;
        self.f_sf = get_bit!(flags, 7) == 1;
        self.f_tf = get_bit!(flags, 8) == 1;
        self.f_if = get_bit!(flags, 9) == 1;
        self.f_df = get_bit!(flags, 10) == 1;
        self.f_of = get_bit!(flags, 11) == 1;
        self.f_iopl1 = get_bit!(flags, 12) == 1;
        self.f_iopl2 = get_bit!(flags, 13) == 1;
        self.f_nt = get_bit!(flags, 14) == 1;
        self.f_rf = get_bit!(flags, 16) == 1;
        self.f_vm = get_bit!(flags, 17) == 1;
        self.f_ac = get_bit!(flags, 18) == 1;
        self.f_vif = get_bit!(flags, 19) == 1;
        self.f_vip = get_bit!(flags, 20) == 1;
        self.f_id = get_bit!(flags, 21) == 1;
    }

    /// FLAGS ///
    ///
    /// overflow 0xffffffff + 1     
    /// carry    0x7fffffff + 1     or  0x80000000 - 1       or   0 - 1
    pub fn check_carry_sub_byte(&mut self, a: u64, b: u64) {
        self.f_cf = (b as u8) > (a as u8);
    }

    pub fn check_overflow_sub_byte(&mut self, a: u64, b: u64) -> i8 {
        let cf = false;

        let rs: i16 = if cf {
            (a as i8) as i16 - (b as i8) as i16 - 1
        } else {
            (a as i8) as i16 - (b as i8) as i16
        };

        self.f_of = rs < MIN_I8 as i16 || rs > MAX_I8 as i16;

        (((rs as u16) & 0xff) as u8) as i8
    }

    pub fn check_carry_sub_word(&mut self, a: u64, b: u64) {
        self.f_cf = (b as u16) > (a as u16);
    }

    pub fn check_overflow_sub_word(&mut self, a: u64, b: u64) -> i16 {
        let cf = false;

        let rs: i32 = if cf {
            (a as i16) as i32 - (b as i16) as i32 - 1
        } else {
            (a as i16) as i32 - (b as i16) as i32
        };

        self.f_of = rs < MIN_I16 as i32 || rs > MAX_I16 as i32;
        (((rs as u32) & 0xffff) as u16) as i16
    }

    pub fn check_carry_sub_qword(&mut self, a: u64, b: u64) {
        self.f_cf = b > a;
    }

    pub fn check_carry_sub_dword(&mut self, a: u64, b: u64) {
        self.f_cf = (b as u32) > (a as u32);
    }

    pub fn check_overflow_sub_qword(&mut self, a: u64, b: u64) -> i64 {
        let cf = false;

        let rs: i128 = if cf {
            (a as i64) as i128 - (b as i64) as i128 - 1
        } else {
            (a as i64) as i128 - (b as i64) as i128
        };

        self.f_of = rs < MIN_I64 as i128 || rs > MAX_I64 as i128;
        (((rs as u128) & 0xffffffff_ffffffff) as u64) as i64
    }

    pub fn check_overflow_sub_dword(&mut self, a: u64, b: u64) -> i32 {
        let cf = false;

        let rs: i64 = if cf {
            (a as i32) as i64 - (b as i32) as i64 - 1
        } else {
            (a as i32) as i64 - (b as i32) as i64
        };

        self.f_of = rs < MIN_I32 as i64 || rs > MAX_I32 as i64;
        (((rs as u64) & 0xffffffff) as u32) as i32
    }

    pub fn calc_flags(&mut self, final_value: u64, bits: u32) {
        match bits {
            64 => self.f_sf = (final_value as i64) < 0,
            32 => self.f_sf = (final_value as i32) < 0,
            16 => self.f_sf = (final_value as i16) < 0,
            8 => self.f_sf = (final_value as i8) < 0,
            _ => unreachable!("weird size"),
        }

        self.calc_pf(final_value as u8);
        self.f_zf = final_value == 0;
        self.f_tf = false;
    }


    #[inline]
    pub fn calc_pf(&mut self, final_value: u8) {
        self.f_pf = PARITY_LOOKUP_TABLE[(final_value & 0xff) as usize];
    }

    #[inline]
    pub fn calc_af(&mut self, value1: u64, value2: u64, result: u64, bits: u64) {
        //let mask = bits*8-4;
        let mask = 1 << 4;
        self.f_af = ((value1 ^ value2 ^ result) & mask) != 0;
        //self.f_af = (value1 & 0x0f) + (value2 & 0x0f) > 0x09;
    }

    pub fn add64(&mut self, value1: u64, value2: u64, cf: bool, include_carry: bool) -> u64 {
        let v1 = value1;
        let v2 = value2;
        let c = if include_carry { cf as u64 } else { 0 };

        let result = v1.wrapping_add(v2).wrapping_add(c);
        let sum = v1 as u128 + v2 as u128 + c as u128;

        self.f_cf = sum > 0xFFFFFFFFFFFFFFFF;
        self.f_sf = (result as i64) < 0;
        self.f_zf = result == 0;
        self.calc_pf(result as u8);

        let sign1 = (v1 >> 63) & 1;
        let sign2 = (v2 >> 63) & 1;
        let signr = (result >> 63) & 1;
        self.f_of = (sign1 == sign2) && (sign1 != signr);

        self.calc_af(v1, v2, result, 64);
        result
    }

    pub fn add32(&mut self, value1: u32, value2: u32, cf: bool, include_carry: bool) -> u64 {
        let result = if include_carry {
            value1.wrapping_add(value2).wrapping_add(cf as u32)
        } else {
            value1.wrapping_add(value2)
        };

        let sum = if include_carry {
            value1 as u64 + value2 as u64 + cf as u64
        } else {
            value1 as u64 + value2 as u64
        };

        self.f_cf = sum > 0xFFFFFFFF;
        self.f_sf = (result as i32) < 0;
        self.f_zf = result == 0;
        self.calc_pf(result as u8);

        let sign1 = (value1 >> 31) & 1;
        let sign2 = (value2 >> 31) & 1;
        let signr = (result >> 31) & 1;
        self.f_of = (sign1 == sign2) && (sign1 != signr);

        self.calc_af(value1 as u64, value2 as u64, result as u64, 32);
        result as u64
    }

    pub fn add16(&mut self, value1: u16, value2: u16, cf: bool, include_carry: bool) -> u64 {
        let result = if include_carry {
            value1.wrapping_add(value2).wrapping_add(cf as u16)
        } else {
            value1.wrapping_add(value2)
        };

        let sum = if include_carry {
            value1 as u32 + value2 as u32 + cf as u32
        } else {
            value1 as u32 + value2 as u32
        };

        self.f_cf = sum > 0xFFFF;
        self.f_sf = (result as i16) < 0;
        self.f_zf = result == 0;
        self.calc_pf(result as u8);

        let sign1 = (value1 >> 15) & 1;
        let sign2 = (value2 >> 15) & 1;
        let signr = (result >> 15) & 1;
        self.f_of = (sign1 == sign2) && (sign1 != signr);

        self.calc_af(value1 as u64, value2 as u64, result as u64, 16);
        result as u64
    }

    pub fn add8(&mut self, value1: u8, value2: u8, cf: bool, include_carry: bool) -> u64 {
        let c = if include_carry { cf as u8 } else { 0 };
        let result = value1.wrapping_add(value2).wrapping_add(c);
        let sum = value1 as u16 + value2 as u16 + c as u16;

        self.f_cf = sum > 0xFF;
        self.f_sf = (result as i8) < 0;
        self.f_zf = result == 0;
        self.calc_pf(result);

        let sign1 = (value1 >> 7) & 1;
        let sign2 = (value2 >> 7) & 1;
        let signr = (result >> 7) & 1;
        self.f_of = (sign1 == sign2) && (sign1 != signr);

        self.calc_af(value1 as u64, value2 as u64, result as u64, 8);
        result as u64
    }

    pub fn sub64(&mut self, value1: u64, value2: u64) -> u64 {
        // let r:i64;

        let (r, carry) = (value1).overflowing_sub(value2);
        let (_, overflow) = (value1 as i64).overflowing_sub(value2 as i64);
        self.f_cf = carry;
        self.f_of = overflow;

        //self.check_carry_sub_qword(value1, value2);
        //r = self.check_overflow_sub_qword(value1, value2);
        self.f_zf = value1 == value2;

        self.f_sf = (r as i64) < 0;
        self.calc_pf(r as u8);
        self.calc_af(value1, value2, r, 64);

        /*
        let low_nibble_value1 = value1 & 0xf;
        let low_nibble_value2 = value2 & 0xf;
        self.f_af = low_nibble_value2 > low_nibble_value1;
        */

        //self.f_af = (r & 0x1000000000000000) != 0;

        r
    }

    pub fn sub32(&mut self, value1: u64, value2: u64) -> u64 {
        //let r:i32;

        let (r, carry) = (value1 as u32).overflowing_sub(value2 as u32);
        let (_, overflow) = (value1 as u32 as i32).overflowing_sub(value2 as u32 as i32);
        self.f_cf = carry;
        self.f_of = overflow;

        //self.check_carry_sub_dword(value1, value2);
        //r = self.check_overflow_sub_dword(value1, value2);
        self.f_zf = value1 == value2;

        self.f_sf = (r as i32) < 0;
        self.calc_pf(r as u8);
        //self.f_af = (r & 0x10000000) != 0;
        self.calc_af(value1, value2, r as u64, 32);

        r as u64
    }

    pub fn sub16(&mut self, value1: u64, value2: u64) -> u64 {
        //let r:i16;

        let (r, carry) = (value1 as u16).overflowing_sub(value2 as u16);
        let (_, overflow) = (value1 as u16 as i16).overflowing_sub(value2 as u16 as i16);
        self.f_cf = carry;
        self.f_of = overflow;

        //let val1 = value1 & 0xffff;
        //let val2 = value2 & 0xffff;

        //self.check_carry_sub_word(val1, val2);
        //r = self.check_overflow_sub_word(val1, val2);
        self.f_zf = value1 == value2;

        self.f_sf = (r as i16) < 0;
        self.calc_pf(r as u8);
        //self.f_af = (r & 0x1000) != 0;
        self.calc_af(value1, value2, r as u64, 16);

        r as u64
    }

    pub fn sub8(&mut self, value1: u64, value2: u64) -> u64 {
        //let r:i8;
        let (r, carry) = (value1 as u8).overflowing_sub(value2 as u8);
        let (_, overflow) = (value1 as u8 as i8).overflowing_sub(value2 as u8 as i8);
        self.f_cf = carry;
        self.f_of = overflow;

        //let val1:u64 = value1 & 0xff;
        //let val2:u64 = value2 & 0xff;

        //self.check_carry_sub_byte(val1, val2);
        //r = self.check_overflow_sub_byte(val1, val2);
        self.f_zf = value1 == value2;

        self.f_sf = (r as i8) < 0;
        self.calc_pf(r);
        //self.f_af = (r & 16) != 0;
        self.calc_af(value1, value2, r as u64, 8);

        r as u64
    }

    pub fn inc64(&mut self, value: u64) -> u64 {
        if value == 0xffffffffffffffff {
            self.f_zf = true;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = false;
            self.f_of = false;
            return 0;
        }

        self.f_of = value == 0x7fffffff_ffffffff;
        self.f_sf = value+1 > 0x7fffffff_ffffffff;
        self.calc_pf((value + 1) as u8);
        self.f_zf = false;
        value + 1
    }

    pub fn inc32(&mut self, value: u64) -> u64 {
        if value == 0xffffffff {
            self.f_zf = true;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = false;
            self.f_of = false;
            return 0;
        }
        self.f_of = value == 0x7fffffff;
        self.f_sf = value+1 > 0x7fffffff;
        self.calc_pf((value + 1) as u8);
        //self.f_pf = (((value as i32) +1) & 0xff) % 2 == 0;
        self.f_zf = false;
        value + 1
    }

    pub fn inc16(&mut self, value: u64) -> u64 {
        if value == 0xffff {
            self.f_zf = true;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = false;
            self.f_of = false;
            return 0;
        }
        self.f_of = value == 0x7fff;
        self.f_sf = value+1 > 0x7fff;
        self.calc_pf((value + 1) as u8);
        //self.f_pf = (((value as i32) +1) & 0xff) % 2 == 0;
        self.f_zf = false;
        value + 1
    }

    pub fn inc8(&mut self, value: u64) -> u64 {
        if value == 0xff {
            self.f_zf = true;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = false;
            self.f_of = false;
            return 0;
        }
        self.f_of = value == 0x7f;
        self.f_sf = value+1 > 0x7f;
        self.calc_pf((value + 1) as u8);
        //self.f_pf = (((value as i32) +1) & 0xff) % 2 == 0;
        self.f_zf = false;
        value + 1
    }

    pub fn dec64(&mut self, value: u64) -> u64 {
        if value == 0 {
            self.f_zf = false;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = true;
            self.f_of = false;
            return 0xffffffffffffffff;
        }
        self.f_of = value == 0x8000000000000000;
        self.calc_pf((value - 1) as u8);
        //self.f_pf = (((value as i64) -1) & 0xff) % 2 == 0;
        self.f_af = false;
        self.f_sf = ((value - 1) as i64) < 0;
        self.f_zf = value == 1;

        value - 1
    }

    pub fn dec32(&mut self, value: u64) -> u64 {
        if value == 0 {
            self.f_zf = false;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = true;
            self.f_of = false;
            return 0xffffffff;
        }
        self.f_of = value == 0x80000000;
        self.calc_pf((value - 1) as u8);
        //self.f_pf = (((value as i32) -1) & 0xff) % 2 == 0;
        self.f_af = false;
        self.f_sf = ((value - 1) as u32 as i32) < 0;
        self.f_zf = value == 1;

        value - 1
    }

    pub fn dec16(&mut self, value: u64) -> u64 {
        if value == 0 {
            self.f_zf = false;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = true;
            self.f_of = false;
            return 0xffff;
        }
        self.f_of = value == 0x8000;
        self.calc_pf((value - 1) as u8);
        //self.f_pf = (((value as i32) -1) & 0xff) % 2 == 0;
        self.f_af = false;
        self.f_sf = ((value - 1) as u16 as i16) < 0;
        self.f_zf = value == 1;

        value - 1
    }

    pub fn dec8(&mut self, value: u64) -> u64 {
        if value == 0 {
            self.f_zf = false;
            self.f_pf = true;
            self.f_af = true;
            self.f_sf = true;
            self.f_of = false;
            return 0xff;
        }
        self.f_of = value == 0x80;
        self.calc_pf((value - 1) as u8);
        //self.f_pf = (((value as i32) -1) & 0xff) % 2 == 0;
        self.f_af = false;
        self.f_sf = ((value - 1) as u8 as i8) < 0;
        self.f_zf = value == 1;

        value - 1
    }

    pub fn neg64(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x8000000000000000;
        self.f_cf = true;

        let mut ival = value as i64;
        if ival != i64::MIN {
            ival = -ival;
        }

        let res = ival as u64;

        self.calc_flags(res, 64);
        self.calc_pf(res as u8);
        res
    }

    pub fn neg32(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x80000000;
        self.f_cf = true;

        let mut ival = value as i32;
        if ival != i32::MIN {
            ival = -ival;
        }

        let res = ival as u32 as u64;

        self.calc_flags(res, 32);
        self.calc_pf(res as u8);
        res
    }

    pub fn neg16(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x8000;
        self.f_cf = true;

        let mut ival = value as i16;
        if ival != i16::MIN {
            ival = -ival;
        }

        let res = ival as u16 as u64;

        self.calc_flags(res, 16);
        self.calc_pf(res as u8);
        res
    }

    pub fn neg8(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x80;
        self.f_cf = true;

        let mut ival = value as i8;
        if ival != i8::MIN {
            ival = -ival;
        }

        let res = ival as u8 as u64;

        self.calc_flags(res, 8);
        self.calc_pf(res as u8);
        res
    }

    //// sal sar signed ////

    pub fn sal2p64(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p64(value0, value1)
    }

    pub fn sal2p32(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p32(value0, value1)
    }

    pub fn sal2p16(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p16(value0, value1)
    }

    pub fn sal2p8(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p8(value0, value1)
    }

    pub fn sal1p64(&mut self, value: u64) -> u64 {
        self.shl1p64(value)
    }

    pub fn sal1p32(&mut self, value: u64) -> u64 {
        self.shl1p32(value)
    }

    pub fn sal1p16(&mut self, value: u64) -> u64 {
        self.shl1p16(value)
    }

    pub fn sal1p8(&mut self, value: u64) -> u64 {
        self.shl1p8(value)
    }

    pub fn sar2p64(&mut self, value0: u64, value1: u64) -> u64 {
        let s64: i64 = value0 as i64;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x3f;
        let sResult = s64 >> count;
        let result = sResult as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 64);
        result
    }

    pub fn sar2p32(&mut self, value0: u64, value1: u64) -> u64 {
        let s32: i32 = value0 as u32 as i32;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let sResult = s32 >> count;
        let result = sResult as u32 as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 32);
        result
    }

    pub fn sar2p16(&mut self, value0: u64, value1: u64) -> u64 {
        let s16 = value0 as u16 as i16;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let sResult = s16 >> count;
        let result = sResult as u16 as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 16);
        result
    }

    pub fn sar2p8(&mut self, value0: u64, value1: u64) -> u64 {
        let s8: i8 = value0 as u8 as i8;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let sResult = s8 >> count;
        let result = sResult as u8 as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 8);
        result
    }

    pub fn sar1p64(&mut self, value: u64) -> u64 {
        let s64 = value as i64;

        let sResult = s64 >> 1;
        let result = sResult as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 64);
        result
    }

    pub fn sar1p32(&mut self, value: u64) -> u64 {
        let s32 = value as u32 as i32;
        let sResult = s32 >> 1;
        let result = sResult as u32 as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 32);
        result
    }

    pub fn sar1p16(&mut self, value: u64) -> u64 {
        let s16 = value as u16 as i16;
        let sResult = s16 >> 1;
        let result = sResult as u16 as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 16);
        result
    }

    pub fn sar1p8(&mut self, value: u64) -> u64 {
        let s16 = value as u8 as i16;
        let sResult = s16 >> 1;
        let result = sResult as u8 as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 8);
        result
    }

    //// shr shl unsigned ////

    pub fn shl2p64(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x3f;
        let result = (value0 << count) & 0xffffffffffffffff;
        self.f_cf = ((value0 >> (64 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 63)) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shl2p32(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 << count) & 0xffffffff;
        self.f_cf = ((value0 >> (32 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 31)) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shl2p16(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 << count) & 0xffff;
        self.f_cf = ((value0 >> (16 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 15)) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shl2p8(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 << count) & 0xff;
        self.f_cf = ((value0 >> (8 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 7)) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    // TODO: update shl1 the same as shl2
    pub fn shl1p64(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xffffffffffffffff;
        self.f_cf = ((value >> 63) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 63)) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shl1p32(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xffffffff;
        self.f_cf = ((value >> 32) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 32)) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shl1p16(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xffff;
        self.f_cf = ((value >> 16) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 16)) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shl1p8(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xff;
        self.f_cf = ((value >> 8) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 8)) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    pub fn shr2p64(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x3f;
        let result = (value0 >> count) & 0xffffffffffffffff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 63 & 0x1) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shr2p32(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 >> count) & 0xffffffff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 31 & 0x1) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shr2p16(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 >> count) & 0xffff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 15 & 0x1) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shr2p8(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 >> count) & 0xff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = ((((result << 1) ^ result) >> 7) & 0x1) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    pub fn shr1p64(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xffffffffffffffff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 63) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shr1p32(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xffffffff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 31) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shr1p16(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xffff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 15) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shr1p8(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 7) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    pub fn test(&mut self, value0: u64, value1: u64, sz: u32) {
        let result: u64 = value0 & value1;

        self.f_zf = result == 0;
        self.f_cf = false;
        self.f_of = false;
        self.calc_pf(result as u8);

        match sz {
            64 => self.f_sf = (result as i64) < 0,
            32 => self.f_sf = (result as i32) < 0,
            16 => self.f_sf = (result as i16) < 0,
            8 => self.f_sf = (result as i8) < 0,
            _ => unreachable!("weird size"),
        }
        //undefined behavior: self.calc_af(value0, value1, result as u64, sz as u64);
    }

    //// imul ////
    pub fn imul64p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i128 = value0 as i64 as i128 * value1 as i64 as i128;
        let uresult: u128 = result as u128;

        if uresult > 0xffffffffffffffff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }



        let res: u64 = (uresult & 0xffffffffffffffff) as u64;
        res
    }

    pub fn imul32p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i64 = value0 as i32 as i64 * value1 as i32 as i64;
        let uresult: u64 = result as u64;

        if uresult > 0xffffffff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        let res: u64 = uresult & 0xffffffff;
        res
    }

    pub fn imul16p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i32 = value0 as i16 as i32 * value1 as i16 as i32;
        let uresult: u32 = result as u32;

        if uresult > 0xffff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        let res = (uresult & 0xffff) as u64;
        res
    }

    pub fn imul8p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i16 = value0 as i8 as i16 * value1 as i8 as i16;
        let uresult: u16 = result as u16;

        if uresult > 0xff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        let res = (uresult & 0xff) as u64;
        res
    }

    pub fn rcr_of_and_cf(&mut self, value0: u64, value1: u64, sz: u32) {
        let count = value1 & 0x3f;
        let res = if count == 1 {
            (value0 >> count) | ((self.f_cf as u64) << (sz - 1))
        } else {
            (value0 >> count) | ((self.f_cf as u64) << ((sz as u64) - count)) |
                (value0 << ((sz+1) as u64 - count))
        };

        self.f_cf = ((value0 >> (count - 1) ) &  1) == 1;
        self.f_of = ((res ^ (res << 1)) >> 63) == 1;
    }

    pub fn rcr(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = value1 & mask;
        let pow = if sz == 64 { u64::MAX } else { (1u64 << sz) - 1 };
        let res = match count {
            0 => value0 & pow,
            1 => ((value0 >> 1) | ((self.f_cf as u64) << (sz - 1))) & pow,
            _ => {
                ((value0 >> count)
                    | ((self.f_cf as u64) << ((sz as u64) - count))
                    | (value0 << ((sz as u64 + 1) - count))) & pow
            }
        };

        if count != 0 {
            self.f_cf = ((value0 >> (count - 1)) & 1) != 0;
        }

        if count == 1 {
            self.f_of = (((res ^ (res << 1)) >> (sz - 1)) & 1) != 0;
        }

        res
    }


    pub fn rcr_prev(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 {
            0x3f
        } else {
            0x1f
        };
        let count = value1 & mask;
        let res = if count == 1 {
            ((value0 >> count) | ((self.f_cf as u64) << (sz - 1))) & (u64::pow(2, sz) - 1)
        } else {
            ((value0 >> count) | ((self.f_cf as u64) << ((sz as u64) - count)) |
                (value0 << ((sz+1) as u64 - count))) & (u64::pow(2, sz) - 1)
        };

        self.f_cf = ((value0 >> (count - 1) ) &  1) == 1;
        self.f_of = ((res ^ (res << 1)) >> (sz-1)) == 1;
        // don't calculate the flag zf, sf doesn't got effect
        res
    }

    pub fn rcl(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        //assert!(sz == 8 || sz == 16 || sz == 32 || sz == 64);

        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = value1 & mask;
        if count == 0 {
            let pow = if sz == 64 { u64::MAX } else { (1u64 << sz) - 1 };
            return value0 & pow;
        }

        if sz == 64 {
            let pow128 = (1u128 << 64) - 1;
            let extended = ((value0 as u128 & pow128) << 1) | (self.f_cf as u128);
            let rotated = ((extended << count) | (extended >> (65 - count))) & ((1u128 << 65) - 1);
            let res = (rotated >> 1) & pow128;
            self.f_cf = (rotated & 1) != 0;
            if count == 1 {
                let msb = (res >> 63) & 1;
                self.f_of = self.f_cf ^ (msb != 0);
            }
            return res as u64;
        } else {
            let pow = (1u64 << sz) - 1;
            let extended = ((value0 & pow) << 1) | (self.f_cf as u64);
            let rotated = ((extended << count) | (extended >> ((sz + 1) as u64 - count))) & ((1u64 << (sz + 1)) - 1);
            let res = (rotated >> 1) & pow;
            self.f_cf = (rotated & 1) != 0;
            if count == 1 {
                let msb = (res >> (sz - 1)) & 1;
                self.f_of = self.f_cf ^ (msb != 0);
            }
            return res;
        }
    }


    pub fn ror(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 {
            0x3f
        } else {
            0x1f
        };

        // input size can be only 64 32 16 and 8
        let res_mask = match sz {
            64 => 0xffffffffffffffff,
            32 => 0xffffffff,
            16 => 0xffff,
            _ => 0xff,
        };
        let count =  value1 & mask;
        let res = (value0 >> count) | (value0 << (sz as u64 - count)) & res_mask;
        let bit63 = (res >> (sz-1)) & 1;
        let bit62 = (res >> (sz-2)) & 1;

        self.f_cf = bit63 == 1;
        self.f_of = bit63 != bit62; // take this for grant
        // don't calculate the flag zf, sf doesn't got effect
        res
    }

    pub fn rol(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 {
            0x3f
        } else {
            0x1f
        };
        let res_mask = match sz {
            64 => 0xffffffffffffffff,
            32 => 0xffffffff,
            16 => 0xffff,
            _ => 0xff,
        };
        let count =  value1 & mask;
        let res = ((value0 << count) | (value0 >> (sz as u64 - count))) & res_mask;
        self.f_cf = (res & 0x1) == 1;
        self.f_of = (self.f_cf as u64 ^ (res >> (sz - 1))) == 1;
        // don't calculate the flag zf, sf doesn't got effect
        res
    }
}
