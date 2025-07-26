mod advapi32;
mod crypt32;
mod dnsapi;
pub mod helper;
mod iphlpapi;
pub mod kernel32;
mod kernelbase;
mod libgcc;
mod mscoree;
mod msvcrt;
mod ntdll;
mod oleaut32;
mod shlwapi;
mod user32;
mod wincrt;
mod wininet;
mod ws2_32;

use crate::emu;

pub fn gateway(addr: u32, name: &str, emu: &mut emu::Emu) {
    emu.regs.sanitize32();
    match name {
        "kernel32.text" => kernel32::gateway(addr, emu),
        "kernel32.rdata" => kernel32::gateway(addr, emu),
        "ntdll.text" => ntdll::gateway(addr, emu),
        "user32.text" => user32::gateway(addr, emu),
        "ws2_32.text" => ws2_32::gateway(addr, emu),
        "wininet.text" => wininet::gateway(addr, emu),
        "advapi32.text" => advapi32::gateway(addr, emu),
        "crypt32.text" => crypt32::gateway(addr, emu),
        "dnsapi.text" => dnsapi::gateway(addr, emu),
        "mscoree.text" => mscoree::gateway(addr, emu),
        "msvcrt.text" => msvcrt::gateway(addr, emu),
        "shlwapi.text" => shlwapi::gateway(addr, emu),
        "oleaut32.text" => oleaut32::gateway(addr, emu),
        "kernelbase.text" => kernelbase::gateway(addr, emu),
        "iphlpapi.text" => iphlpapi::gateway(addr, emu),
        "libgcc_s_dw2-1.text" => libgcc::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.text" => wincrt::gateway(addr, emu),
        "not_loaded" => {
            // TODO: banzai check?
            emu.pe32.as_ref().unwrap().import_addr_to_name(addr)
        }
        _ => panic!("/!\\ trying to execute on {} at 0x{:x}", name, addr),
    };
    emu.call_stack.pop();
}
