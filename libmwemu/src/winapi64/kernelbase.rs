use crate::emu;
use crate::serialization;
use crate::winapi64;
//use crate::constants;
//use crate::winapi32::helper;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "PathCombineA" => PathCombineA(emu),
        "PathCombineW" => PathCombineW(emu),
        "IsCharAlphaNumericA" => IsCharAlphaNumericA(emu),
        "GetTokenInformation" => GetTokenInformation(emu),
        "GetFileVersionInfoSizeA" => GetFileVersionInfoSizeA(emu),
        "GetFileVersionInfoA" => GetFileVersionInfoA(emu),
        "VerQueryValueA" => VerQueryValueA(emu),
        "_initterm_e" => _initterm_e(emu),
        "_initterm" => _initterm(emu),
        "exit" => exit(emu),
        "_exit" => _exit(emu),
        "atexit" => atexit(emu),

        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(
                        &emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs.rip
            );
            return api;
        }
    }

    String::new()
}

pub fn PathCombineA(emu: &mut emu::Emu) {
    let dst: u64 = emu.regs.rcx;
    let dir = emu.regs.rdx;
    let file = emu.regs.r8;

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = emu.maps.read_string(dir);
    }
    if file > 0 {
        path2 = emu.maps.read_string(file);
    }

    log::info!(
        "{}** {} kernelbase!PathCombineA path1: {} path2: {} {}",
        emu.colors.light_red,
        emu.pos,
        path1,
        path2,
        emu.colors.nc
    );

    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps.write_string(dst, &format!("{}\\{}", path1, path2));
    }

    emu.regs.rax = dst;
}

pub fn PathCombineW(emu: &mut emu::Emu) {
    let dst: u64 = emu.regs.rcx;
    let dir = emu.regs.rdx;
    let file = emu.regs.r8;

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = emu.maps.read_wide_string(dir);
    }
    if file > 0 {
        path2 = emu.maps.read_wide_string(file);
    }

    log::info!(
        "{}** {} kernelbase!PathCombineW path1: {} path2: {} {}",
        emu.colors.light_red,
        emu.pos,
        path1,
        path2,
        emu.colors.nc
    );

    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        emu.maps
            .write_wide_string(dst, &format!("{}\\{}", path1, path2));
    }

    emu.regs.rax = dst;
}

pub fn IsCharAlphaNumericA(emu: &mut emu::Emu) {
    let c = emu.regs.rcx as u8 as char;

    log::info!(
        "{}** {} kernelbase!IsCharAlphaNumericA char: {} {}",
        emu.colors.light_red,
        emu.pos,
        c,
        emu.colors.nc
    );

    emu.regs.rax = if c.is_ascii_alphanumeric() { 1 } else { 0 };
}

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = emu.regs.rdx;
    let token_information_class = emu.regs.rcx;
    let token_information = emu.regs.r8;
    let token_information_length = emu.regs.r9;
    let return_length = emu.maps.read_qword(emu.regs.rsp + 0x20);

    log::info!(
        "{}** {} kernelbase!GetTokenInformation token_information_class: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        token_information_class,
        emu.colors.nc
    );

    emu.regs.rax = 1;
}

/*
DWORD GetFileVersionInfoSizeA(
  [in]            LPCSTR  lptstrFilename,
  [out, optional] LPDWORD lpdwHandle
);
*/
fn GetFileVersionInfoSizeA(emu: &mut emu::Emu) {
    let lptstr_filename = emu.regs.rcx as usize;
    let lpdw_handle = emu.regs.rdx as usize;
    log_red!(
        emu,
        "** {} kernelbase!GetFileVersionInfoSizeA lptstr_filename: 0x{:x} lpdw_handle: 0x{:x}",
        emu.pos,
        lptstr_filename,
        lpdw_handle
    );
    // TODO: just putting a rough number for now
    emu.regs.rax = 0x100;
}

/*
BOOL GetFileVersionInfoA(
  [in]  LPCSTR lptstrFilename,
        DWORD  dwHandle,
  [in]  DWORD  dwLen,
  [out] LPVOID lpData
);
*/
fn GetFileVersionInfoA(emu: &mut emu::Emu) {
    let lptstr_filename = emu.regs.rcx as usize;
    let dw_handle = emu.regs.rdx as usize;
    let dw_len = emu.regs.rcx as usize;
    let lp_data = emu.regs.r8 as usize;
    log_red!(emu, "** {} kernelbase!GetFileVersionInfoA lptstr_filename: 0x{:x} dw_handle: 0x{:x} dw_len: 0x{:x} lp_data: 0x{:x}", 
        emu.pos,
        lptstr_filename,
        dw_handle,
        dw_len,
        lp_data
    );
    // TODO: write to lp_data
    emu.regs.rax = 1;
}

/*
BOOL VerQueryValueA(
  [in]  LPCVOID pBlock,
  [in]  LPCSTR  lpSubBlock,
  [out] LPVOID  *lplpBuffer,
  [out] PUINT   puLen
);
*/
fn VerQueryValueA(emu: &mut emu::Emu) {
    let p_block = emu.regs.rcx as usize;
    let lp_sub_block = emu.regs.rdx as usize;
    let lplp_buffer = emu.regs.rcx as usize;
    let pu_len = emu.regs.r8 as usize;
    log_red!(emu, "** {} kernelbase!VerQueryValueA p_block: 0x{:x} lp_sub_block: {} lplp_buffer: 0x{:x} pu_len: 0x{:x}", 
        emu.pos,
        p_block,
        lp_sub_block,
        lplp_buffer,
        pu_len
    );
    // TODO: write more structured data
    let base = emu.maps.alloc(0x100).expect("out of memory");
    emu.maps.write_qword(lplp_buffer as u64, base);
    emu.maps.write_qword(pu_len as u64, 0x100);
    emu.regs.rax = 1;
}

fn _initterm_e(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernelbase!_initterm_e  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs.rax = 0;
}

fn _initterm(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernelbase!_initterm  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs.rax = 0;
}

fn exit(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernelbase!exit  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    panic!("exit called");
}

fn _exit(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernelbase!_exit  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    panic!("_exit called");
}

fn atexit(emu: &mut emu::Emu) {
    let fptr = emu.regs.rcx;
    log::info!(
        "{}** {} kernelbase!atexit fptr: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        fptr,
        emu.colors.nc
    );
    emu.regs.rax = 0;
}
