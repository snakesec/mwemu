use crate::maps::mem64::Mem64;
use crate::maps::Maps;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

////// PEB / TEB //////

#[derive(Debug)]
pub struct ListEntry {
    pub flink: u32,
    pub blink: u32,
}

impl Default for ListEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl ListEntry {
    pub fn new() -> ListEntry {
        ListEntry { flink: 0, blink: 0 }
    }

    pub fn load(addr: u64, maps: &Maps) -> ListEntry {
        ListEntry {
            flink: maps.read_dword(addr).unwrap(),
            blink: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.flink);
        maps.write_dword(addr + 4, self.blink);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ListEntry64 {
    pub flink: u64,
    pub blink: u64,
}

impl Default for ListEntry64 {
    fn default() -> Self {
        Self::new()
    }
}

impl ListEntry64 {
    pub fn new() -> ListEntry64 {
        ListEntry64 { flink: 0, blink: 0 }
    }

    pub fn load(addr: u64, maps: &Maps) -> ListEntry64 {
        ListEntry64 {
            flink: maps.read_qword(addr).unwrap(),
            blink: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.flink);
        maps.write_qword(addr + 8, self.blink);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct LdrDataTableEntry {
    pub in_load_order_links: ListEntry,           // +0x00 (8 bytes)
    pub in_memory_order_links: ListEntry,         // +8  (8 bytes)
    pub in_initialization_order_links: ListEntry, // +16 (8 bytes)
    pub dll_base: u32,                            // +24 +0x18 (4 bytes)
    pub entry_point: u32,                         // +28 +0x1C (4 bytes)
    pub size_of_image: u32,                       // +32 +0x20 (4 bytes)
    pub full_dll_name: UnicodeString,             // +36 +0x24 (8 bytes)
    pub base_dll_name: UnicodeString,             // +44 +0x2C (8 bytes)
    pub flags: u32,                               // +52 +0x34 (4 bytes)
    pub load_count: u16,                          // +56 +0x38 (2 bytes)
    pub tls_index: u16,                           // +58 +0x3A (2 bytes)
    pub hash_links: ListEntry,                    // +60 +0x3C (8 bytes)
    pub time_date_stamp: u32,                     // +68 +0x44 (4 bytes)
}

impl Default for LdrDataTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl LdrDataTableEntry {
    pub fn size() -> usize {
        72
    }

    pub fn new() -> LdrDataTableEntry {
        LdrDataTableEntry {
            in_load_order_links: ListEntry::new(),
            in_memory_order_links: ListEntry::new(),
            in_initialization_order_links: ListEntry::new(),
            dll_base: 0,
            entry_point: 0,
            size_of_image: 0,
            full_dll_name: UnicodeString::new(),
            base_dll_name: UnicodeString::new(),
            flags: 0,
            load_count: 0,
            tls_index: 0,
            hash_links: ListEntry::new(),
            time_date_stamp: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> LdrDataTableEntry {
        LdrDataTableEntry {
            in_load_order_links: ListEntry::load(addr, maps),            // +0x00
            in_memory_order_links: ListEntry::load(addr + 8, maps),      // +0x08
            in_initialization_order_links: ListEntry::load(addr + 16, maps), // +0x10
            dll_base: maps.read_dword(addr + 24).unwrap(),               // +0x18
            entry_point: maps.read_dword(addr + 28).unwrap(),            // +0x1C
            size_of_image: maps.read_dword(addr + 32).unwrap(),          // +0x20
            full_dll_name: UnicodeString::load(addr + 36, maps),         // +0x24
            base_dll_name: UnicodeString::load(addr + 44, maps),         // +0x2C
            flags: maps.read_dword(addr + 52).unwrap(),                  // +0x34
            load_count: maps.read_word(addr + 56).unwrap(),              // +0x38
            tls_index: maps.read_word(addr + 58).unwrap(),               // +0x3A
            hash_links: ListEntry::load(addr + 60, maps),                // +0x3C
            time_date_stamp: maps.read_dword(addr + 68).unwrap(),        // +0x44
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.in_load_order_links.save(addr, maps);               // +0x00
        self.in_memory_order_links.save(addr + 8, maps);         // +0x08
        self.in_initialization_order_links.save(addr + 16, maps); // +0x10
        maps.write_dword(addr + 24, self.dll_base);             // +0x18
        maps.write_dword(addr + 28, self.entry_point);          // +0x1C
        maps.write_dword(addr + 32, self.size_of_image);       // +0x20
        self.full_dll_name.save(addr + 36, maps);               // +0x24
        self.base_dll_name.save(addr + 44, maps);               // +0x2C
        maps.write_dword(addr + 52, self.flags);               // +0x34
        maps.write_word(addr + 56, self.load_count);           // +0x38
        maps.write_word(addr + 58, self.tls_index);            // +0x3A
        self.hash_links.save(addr + 60, maps);                  // +0x3C
        maps.write_dword(addr + 68, self.time_date_stamp);     // +0x44
    }


    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct PebLdrData {
    pub length: u32,
    pub initializated: u32,
    pub sshandle: u32,
    pub in_load_order_module_list: ListEntry, // 0x0c (12)
    pub in_memory_order_module_list: ListEntry,
    pub in_initialization_order_module_list: ListEntry,
    pub entry_in_progress: u32,
    pub shutdown_in_progress: u32,
    pub shutdown_thread_id: u32,
}

impl Default for PebLdrData {
    fn default() -> Self {
        Self::new()
    }
}

impl PebLdrData {
    pub fn size() -> usize {
        48
    }

    pub fn new() -> PebLdrData {
        PebLdrData {
            length: 48,
            initializated: 0,
            sshandle: 0,
            in_load_order_module_list: ListEntry::new(),
            in_memory_order_module_list: ListEntry::new(),
            in_initialization_order_module_list: ListEntry::new(),
            entry_in_progress: 0,
            shutdown_in_progress: 0,
            shutdown_thread_id: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PebLdrData {
        PebLdrData {
            length: maps.read_dword(addr).unwrap(),
            initializated: maps.read_dword(addr + 4).unwrap(),
            sshandle: maps.read_dword(addr + 8).unwrap(),
            in_load_order_module_list: ListEntry::load(addr + 12, maps),
            in_memory_order_module_list: ListEntry::load(addr + 20, maps),
            in_initialization_order_module_list: ListEntry::load(addr + 28, maps),
            entry_in_progress: maps.read_dword(addr + 36).unwrap(),
            shutdown_in_progress: maps.read_dword(addr + 40).unwrap(),
            shutdown_thread_id: maps.read_dword(addr + 44).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.length);
        maps.write_dword(addr + 4, self.initializated);
        maps.write_dword(addr + 8, self.sshandle);
        self.in_load_order_module_list.save(addr + 12, maps);
        self.in_memory_order_module_list.save(addr + 20, maps);
        self.in_initialization_order_module_list
            .save(addr + 28, maps);
        maps.write_dword(addr + 36, self.entry_in_progress);
        maps.write_dword(addr + 40, self.shutdown_in_progress);
        maps.write_dword(addr + 44, self.shutdown_thread_id);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct PebLdrData64 {
    pub length: u32,
    pub initializated: u32,
    pub sshandle: u64,
    pub in_load_order_module_list: ListEntry64,
    pub in_memory_order_module_list: ListEntry64,
    pub in_initialization_order_module_list: ListEntry64,
    pub entry_in_progress: ListEntry64,
}

impl Default for PebLdrData64 {
    fn default() -> Self {
        Self::new()
    }
}

impl PebLdrData64 {
    pub fn size() -> usize {
        80
    }

    pub fn new() -> PebLdrData64 {
        PebLdrData64 {
            length: 80, // prev:72
            initializated: 0,
            sshandle: 0,
            in_load_order_module_list: ListEntry64::new(),
            in_memory_order_module_list: ListEntry64::new(),
            in_initialization_order_module_list: ListEntry64::new(),
            entry_in_progress: ListEntry64::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PebLdrData64 {
        PebLdrData64 {
            length: maps.read_dword(addr).unwrap(),
            initializated: maps.read_dword(addr + 4).unwrap(),
            sshandle: maps.read_qword(addr + 8).unwrap(),
            in_load_order_module_list: ListEntry64::load(addr + 0x10, maps),
            in_memory_order_module_list: ListEntry64::load(addr + 0x20, maps),
            in_initialization_order_module_list: ListEntry64::load(addr + 0x30, maps),
            entry_in_progress: ListEntry64::load(addr + 0x40, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.length);
        maps.write_dword(addr + 4, self.initializated);
        maps.write_qword(addr + 8, self.sshandle);
        self.in_load_order_module_list.save(addr + 0x10, maps);
        self.in_memory_order_module_list.save(addr + 0x20, maps);
        self.in_initialization_order_module_list
            .save(addr + 0x30, maps);
        self.entry_in_progress.save(addr + 0x40, maps);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct OrdinalTable {
    pub func_name: String,
    pub ordinal_tbl_rva: u64,
    pub ordinal_tbl: u64,
    pub ordinal: u64,
    pub func_addr_tbl_rva: u64,
    pub func_addr_tbl: u64,
    pub func_rva: u64,
    pub func_va: u64,
}

impl Default for OrdinalTable {
    fn default() -> Self {
        Self::new()
    }
}

impl OrdinalTable {
    pub fn new() -> OrdinalTable {
        OrdinalTable {
            func_name: String::new(),
            ordinal_tbl_rva: 0,
            ordinal_tbl: 0,
            ordinal: 0,
            func_addr_tbl_rva: 0,
            func_addr_tbl: 0,
            func_rva: 0,
            func_va: 0,
        }
    }
}

#[derive(Debug)]
pub struct NtTib32 {
    pub exception_list: u32,
    pub stack_base: u32,
    pub stack_limit: u32,
    pub sub_system_tib: u32,
    pub fiber_data: u32,
    pub arbitrary_user_pointer: u32,
    pub self_pointer: u32,
}

impl Default for NtTib32 {
    fn default() -> Self {
        Self::new()
    }
}

impl NtTib32 {
    pub fn size() -> usize {
        28
    }

    pub fn new() -> NtTib32 {
        NtTib32 {
            exception_list: 0,
            stack_base: 0,
            stack_limit: 0,
            sub_system_tib: 0,
            fiber_data: 0,
            arbitrary_user_pointer: 0,
            self_pointer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> NtTib32 {
        NtTib32 {
            exception_list: maps.read_dword(addr).unwrap(),
            stack_base: maps.read_dword(addr + 4).unwrap(),
            stack_limit: maps.read_dword(addr + 8).unwrap(),
            sub_system_tib: maps.read_dword(addr + 12).unwrap(),
            fiber_data: maps.read_dword(addr + 16).unwrap(),
            arbitrary_user_pointer: maps.read_dword(addr + 20).unwrap(),
            self_pointer: maps.read_dword(addr + 24).unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> NtTib32 {
        NtTib32 {
            exception_list: map.read_dword(addr),
            stack_base: map.read_dword(addr + 4),
            stack_limit: map.read_dword(addr + 8),
            sub_system_tib: map.read_dword(addr + 12),
            fiber_data: map.read_dword(addr + 16),
            arbitrary_user_pointer: map.read_dword(addr + 20),
            self_pointer: map.read_dword(addr + 24),
        }
    }

    pub fn save(&self, addr: u64, mem: &mut Mem64) {
        mem.write_dword(addr, self.exception_list);
        mem.write_dword(addr + 4, self.stack_base);
        mem.write_dword(addr + 8, self.stack_limit);
        mem.write_dword(addr + 12, self.sub_system_tib);
        mem.write_dword(addr + 16, self.fiber_data);
        mem.write_dword(addr + 20, self.arbitrary_user_pointer);
        mem.write_dword(addr + 24, self.self_pointer);
    }
}

#[derive(Debug)]
pub struct TEB {
    pub nt_tib: NtTib32,
    pub environment_pointer: u32,
    pub process_id: u32,
    pub thread_id: u32,
    pub active_rpc_handle: u32,
    pub thread_local_storage_pointer: u32,
    pub process_environment_block: u32, // PEB 0x30
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u32,
    pub win32_thread_info: u32,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 6],
    pub wow32_reserved: u32,
    pub current_locale: u32,
    pub fp_software_status_register: u32,
    pub system_reserved1: [u64; 54],
    pub exception_code: u32,
    pub activation_context_stack_pointer: u32,
    pub spare_bytes: [u8; 24],
    pub tx_fs_context: u32,
}

impl TEB {
    pub fn size() -> usize {
        1000
    }

    pub fn new(peb_addr: u32) -> TEB {
        TEB {
            nt_tib: NtTib32::new(),
            environment_pointer: 0,
            process_id: 3240,
            thread_id: 1,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: peb_addr, // PEB 0x30
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: 0,
            current_locale: 0,
            fp_software_status_register: 0,
            system_reserved1: [0; 54],
            exception_code: 0,
            activation_context_stack_pointer: 0,
            spare_bytes: [0; 24],
            tx_fs_context: 0,
        }
    }

    pub fn set_last_error(&mut self, err: u32) {
        self.last_error_value = err;
    }

    pub fn load(addr: u64, maps: &Maps) -> TEB {
        TEB {
            nt_tib: NtTib32::load(addr, maps),
            environment_pointer: maps.read_dword(addr + 28).unwrap(),
            process_id: maps.read_dword(addr + 32).unwrap(),
            thread_id: maps.read_dword(addr + 36).unwrap(),
            active_rpc_handle: maps.read_dword(addr + 40).unwrap(),
            thread_local_storage_pointer: maps.read_dword(addr + 44).unwrap(),
            process_environment_block: maps.read_dword(addr + 48).unwrap(),
            last_error_value: maps.read_dword(addr + 52).unwrap(),
            count_of_owned_critical_sections: maps.read_dword(addr + 56).unwrap(),
            csr_client_thread: maps.read_dword(addr + 60).unwrap(),
            win32_thread_info: maps.read_dword(addr + 64).unwrap(),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: maps.read_dword(addr + 70).unwrap(),
            current_locale: maps.read_dword(addr + 74).unwrap(),
            fp_software_status_register: maps.read_dword(addr + 78).unwrap(),
            system_reserved1: [0; 54],
            exception_code: maps.read_dword(addr + 82).unwrap(),
            activation_context_stack_pointer: maps.read_dword(addr + 86).unwrap(),
            spare_bytes: [0; 24],
            tx_fs_context: maps.read_dword(addr + 190).unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> TEB {
        TEB {
            nt_tib: NtTib32::load_map(addr, map),
            environment_pointer: map.read_dword(addr + 28),
            process_id: map.read_dword(addr + 32),
            thread_id: map.read_dword(addr + 36),
            active_rpc_handle: map.read_dword(addr + 40),
            thread_local_storage_pointer: map.read_dword(addr + 44),
            process_environment_block: map.read_dword(addr + 48),
            last_error_value: map.read_dword(addr + 52),
            count_of_owned_critical_sections: map.read_dword(addr + 56),
            csr_client_thread: map.read_dword(addr + 60),
            win32_thread_info: map.read_dword(addr + 64),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: map.read_dword(addr + 70),
            current_locale: map.read_dword(addr + 74),
            fp_software_status_register: map.read_dword(addr + 78),
            system_reserved1: [0; 54],
            exception_code: map.read_dword(addr + 82),
            activation_context_stack_pointer: map.read_dword(addr + 86),
            spare_bytes: [0; 24],
            tx_fs_context: map.read_dword(addr + 190),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        self.nt_tib.save(base, mem);
        mem.write_dword(base + 28, self.environment_pointer);
        mem.write_dword(base + 32, self.process_id);
        mem.write_dword(base + 36, self.thread_id);
        mem.write_dword(base + 40, self.active_rpc_handle);
        mem.write_dword(base + 44, self.thread_local_storage_pointer);
        mem.write_dword(base + 48, self.process_environment_block);
        mem.write_dword(base + 52, self.last_error_value);
        mem.write_dword(base + 56, self.count_of_owned_critical_sections);
        mem.write_dword(base + 60, self.csr_client_thread);
        mem.write_dword(base + 64, self.win32_thread_info);
        //maps.write_dword(addr + 68, self.user32_reserved);
        //maps.write_dword(addr + 70, self.user_reserved);
        mem.write_dword(base + 70, self.wow32_reserved);
        mem.write_dword(base + 74, self.current_locale);
        mem.write_dword(base + 78, self.fp_software_status_register);
        mem.write_dword(base + 82, self.exception_code);
        mem.write_dword(base + 86, self.activation_context_stack_pointer);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct PEB {
    inheritet_addr_space: u8,
    read_img_file_exec_options: u8,
    pub being_debugged: u8,
    speer_bool: u8,
    padding: u32,
    pub image_base_addr: u32,
    pub ldr: u32, // ptr to PEB_LDR_DATA  +0x0c
    process_parameters: u32,
    reserved4: [u32; 3],
    alt_thunk_list_ptr: u32,
    reserved5: u32,
    reserved6: u32,
    reserved7: u32,
    reserved8: u32,
    alt_thunk_list_ptr_32: u32, // +52 + 45*4 + 96
    reserved9: [u32; 45],
    reserved10: [u8; 96],
    post_process_init_routine: u32,
    reserved11: [u32; 128],
    reserved12: u32,
    session_id: u32,
}

impl PEB {
    pub fn size() -> usize {
        800 // TODO: std::mem::size_of_val
    }

    pub fn new(image_base_addr: u32, ldr: u32, process_parameters: u32) -> PEB {
        PEB {
            inheritet_addr_space: 0,
            read_img_file_exec_options: 0,
            being_debugged: 0,
            speer_bool: 0,
            padding: 0,
            image_base_addr,
            ldr,
            process_parameters,
            reserved4: [0; 3],
            alt_thunk_list_ptr: 0,
            reserved5: 0,
            reserved6: 0,
            reserved7: 0,
            reserved8: 0,
            alt_thunk_list_ptr_32: 0,
            reserved9: [0; 45],
            reserved10: [0; 96],
            post_process_init_routine: 0,
            reserved11: [0; 128],
            reserved12: 0,
            session_id: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PEB {
        PEB {
            inheritet_addr_space: maps.read_byte(addr).unwrap(),
            read_img_file_exec_options: maps.read_byte(addr + 1).unwrap(),
            being_debugged: maps.read_byte(addr + 2).unwrap(),
            speer_bool: maps.read_byte(addr + 3).unwrap(),
            padding: maps.read_dword(addr + 4).unwrap(),
            image_base_addr: maps.read_dword(addr + 8).unwrap(),
            ldr: maps.read_dword(addr + 12).unwrap(),
            process_parameters: maps.read_dword(addr + 16).unwrap(),
            reserved4: [
                maps.read_dword(addr + 20).unwrap(),
                maps.read_dword(addr + 24).unwrap(),
                maps.read_dword(addr + 28).unwrap(),
            ],
            alt_thunk_list_ptr: maps.read_dword(addr + 32).unwrap(),
            reserved5: maps.read_dword(addr + 36).unwrap(),
            reserved6: maps.read_dword(addr + 40).unwrap(),
            reserved7: maps.read_dword(addr + 44).unwrap(),
            reserved8: maps.read_dword(addr + 48).unwrap(),
            alt_thunk_list_ptr_32: maps.read_dword(addr + 52).unwrap(),
            reserved9: [0; 45],
            reserved10: [0; 96],
            post_process_init_routine: maps.read_dword(addr + 56).unwrap(),
            reserved11: [0; 128],
            reserved12: maps.read_dword(addr + 60).unwrap(),
            session_id: maps.read_dword(addr + 64).unwrap(),
        }
    }

    pub fn set_image_base(&mut self, image_base: u32) {
        self.image_base_addr = image_base;
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_byte(base, self.inheritet_addr_space);
        mem.write_byte(base + 1, self.read_img_file_exec_options);
        mem.write_byte(base + 2, self.being_debugged);
        mem.write_byte(base + 3, self.speer_bool);
        mem.write_dword(base + 4, self.padding);
        mem.write_dword(base + 8, self.image_base_addr);
        mem.write_dword(base + 12, self.ldr);
        mem.write_dword(base + 16, self.process_parameters);
        mem.write_dword(base + 20, self.reserved4[0]);
        mem.write_dword(base + 24, self.reserved4[1]);
        mem.write_dword(base + 28, self.reserved4[2]);
        mem.write_dword(base + 32, self.alt_thunk_list_ptr);
        mem.write_dword(base + 36, self.reserved5);
        mem.write_dword(base + 40, self.reserved6);
        mem.write_dword(base + 44, self.reserved7);
        mem.write_dword(base + 48, self.reserved8);
        mem.write_dword(base + 52, self.alt_thunk_list_ptr_32);
        mem.write_dword(base + 56, self.post_process_init_routine);
        mem.write_dword(base + 60, self.reserved12);
        mem.write_dword(base + 64, self.session_id);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

// 64bits
// https://bytepointer.com/resources/tebpeb64.htm   (from xp to win8)
// https://www.tssc.de/winint/Win10_19042_ntoskrnl/_PEB64.htm (win10)

#[derive(Debug)]
pub struct PEB64 {
    inheritet_addr_space: u8,
    read_img_file_exec_options: u8,
    pub being_debugged: u8,
    system_dependent_01: u8,
    dummy_align: u32,
    mutant: u64,
    pub image_base_addr: u64,
    pub ldr: u64,
    process_parameters: u64,
    subsystem_data: u64,
    process_heap: u64,
    fast_peb_lock: u64,
    system_dependent_02: u64,
    system_dependent_03: u64,
    system_dependent_04: u64,
    kernel_callback_table: u64,
    system_reserved: u32,
    system_dependent_05: u32,
    system_dependent_06: u64,
    tls_expansion_counter: u64,
    tls_bitmap: u64,
    tls_bitmap_bits: [u32; 2],
    read_only_shared_memory_base: u64,
    system_dependent_07: u64,
    read_only_static_server_data: u64,
    ansi_code_page_data: u64,
    oem_code_page_data: u64,
    unicode_case_table_data: u64,
    number_of_processors: u32,
    nt_global_flag: u32,
    critical_section_timeout: u64,
    heap_segment_reserve: u64,
    heap_segment_commit: u64,
    heap_decommit_total_free_threshold: u64,
    heap_decommit_free_block_threshold: u64,
    number_of_heaps: u32,
    max_number_of_heaps: u32,
    process_heaps: u64,
    gdi_share_handle_table: u64,
    process_starter_helper: u64,
    gdi_dc_attribute_list: u64,
    loader_lock: u64,
    os_major_version: u32,
    os_minor_version: u32,
    os_build_number: u16,
    oscsd_version: u16,
    os_platform_id: u32,
    image_subsystem: u32,
    image_subsystem_major_version: u32,
    image_subsystem_minor_version: u64,
    active_process_afinity_mask: u64,
    gdi_handle_buffer: [u64; 30],
    post_process_init_routine: u64,
    tls_expansion_bitmap: u64,
    tls_expansion_bitmap_bits: [u32; 32],
    session_id: u64,
    app_compat_flags: u64,
    app_compat_flags_user: u64,
    p_shim_data: u64,
    app_compat_info: u64,
    csd_version: [u64; 2],
    activate_context_data: u64,
    process_assembly_storage_map: u64,
    system_default_activation_context_data: u64,
    system_assembly_storage_map: u64,
    minimum_stack_commit: u64,
}

impl PEB64 {
    pub fn size() -> usize {
        800 // std::mem::size_of_val
    }

    pub fn new(image_base_addr: u64, ldr: u64, process_parameters: u64) -> PEB64 {
        PEB64 {
            inheritet_addr_space: 0x0,
            read_img_file_exec_options: 0x0,
            being_debugged: 0x0,
            system_dependent_01: 0x0,
            dummy_align: 0x0,
            mutant: 0xffffffffffffffff,
            image_base_addr,
            ldr,
            process_parameters,
            subsystem_data: 0x0,
            process_heap: 0x520000,
            fast_peb_lock: 0x7710a900,
            system_dependent_02: 0x0,
            system_dependent_03: 0x0,
            system_dependent_04: 0x2,
            kernel_callback_table: 0x76f59500,
            system_reserved: 0x0,
            system_dependent_05: 0x0,
            system_dependent_06: 0x7feff2f0000,
            tls_expansion_counter: 0x0,
            tls_bitmap: 0x77102590,
            tls_bitmap_bits: [0x1fff, 0x0],
            read_only_shared_memory_base: 0x7efe0000,
            system_dependent_07: 0x0,
            read_only_static_server_data: 0x7efe0a90,
            ansi_code_page_data: 0x7fffffb0000,
            oem_code_page_data: 0x7fffffc0228,
            unicode_case_table_data: 0x7fffffd0650,
            number_of_processors: 0x1,
            nt_global_flag: 0x70,
            critical_section_timeout: 0xffffe86d079b8000,
            heap_segment_reserve: 0x100000,
            heap_segment_commit: 0x2000,
            heap_decommit_total_free_threshold: 0x10000,
            heap_decommit_free_block_threshold: 0x10000,
            number_of_heaps: 0x4,
            max_number_of_heaps: 0x10,
            process_heaps: 0x7710a6c0,
            gdi_share_handle_table: 0x920000,
            process_starter_helper: 0x0,
            gdi_dc_attribute_list: 0x14,
            loader_lock: 0x77107490,
            os_major_version: 0x6,
            os_minor_version: 0x1,
            os_build_number: 0x1db1,
            oscsd_version: 0x100,
            os_platform_id: 0x2,
            image_subsystem: 0x3,
            image_subsystem_major_version: 0x5,
            image_subsystem_minor_version: 0x2,
            active_process_afinity_mask: 0x1,
            gdi_handle_buffer: [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            post_process_init_routine: 0x0,
            tls_expansion_bitmap: 0x77102580,
            tls_expansion_bitmap_bits: [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            session_id: 0x1,
            app_compat_flags: 0x0,
            app_compat_flags_user: 0x0,
            p_shim_data: 0x0,
            app_compat_info: 0x0,
            csd_version: [0x1e001c, 0x7efe0afa],
            activate_context_data: 0x0,
            process_assembly_storage_map: 0x0,
            system_default_activation_context_data: 0x230000,
            system_assembly_storage_map: 0x0,
            minimum_stack_commit: 0x0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PEB64 {
        PEB64 {
            inheritet_addr_space: maps.read_byte(addr).unwrap(),
            read_img_file_exec_options: maps.read_byte(addr + 0x1).unwrap(),
            being_debugged: maps.read_byte(addr + 0x2).unwrap(),
            system_dependent_01: maps.read_byte(addr + 0x3).unwrap(),
            dummy_align: 0,
            mutant: maps.read_qword(addr + 0x8).unwrap(),
            image_base_addr: maps.read_qword(addr + 0x10).unwrap(),
            ldr: maps.read_qword(addr + 0x18).unwrap(),
            process_parameters: maps.read_qword(addr + 0x20).unwrap(),
            subsystem_data: maps.read_qword(addr + 0x28).unwrap(),
            process_heap: maps.read_qword(addr + 0x30).unwrap(),
            fast_peb_lock: maps.read_qword(addr + 0x38).unwrap(),
            system_dependent_02: maps.read_qword(addr + 0x40).unwrap(),
            system_dependent_03: maps.read_qword(addr + 0x48).unwrap(),
            system_dependent_04: maps.read_qword(addr + 0x50).unwrap(),
            kernel_callback_table: maps.read_qword(addr + 0x58).unwrap(),
            system_reserved: maps.read_dword(addr + 0x60).unwrap(),
            system_dependent_05: maps.read_dword(addr + 0x64).unwrap(),
            system_dependent_06: maps.read_qword(addr + 0x68).unwrap(),
            tls_expansion_counter: maps.read_qword(addr + 0x70).unwrap(),
            tls_bitmap: maps.read_qword(addr + 0x78).unwrap(),
            tls_bitmap_bits: [
                maps.read_dword(addr + 0x80).unwrap(),
                maps.read_dword(addr + 0x84).unwrap(),
            ],
            read_only_shared_memory_base: maps.read_qword(addr + 0x88).unwrap(),
            system_dependent_07: maps.read_qword(addr + 0x90).unwrap(),
            read_only_static_server_data: maps.read_qword(addr + 0x98).unwrap(),
            ansi_code_page_data: maps.read_qword(addr + 0xa0).unwrap(),
            oem_code_page_data: maps.read_qword(addr + 0xa8).unwrap(),
            unicode_case_table_data: maps.read_qword(addr + 0xb0).unwrap(),
            number_of_processors: maps.read_dword(addr + 0xb8).unwrap(),
            nt_global_flag: maps.read_dword(addr + 0xbc).unwrap(),
            critical_section_timeout: maps.read_qword(addr + 0xc0).unwrap(),
            heap_segment_reserve: maps.read_qword(addr + 0xc8).unwrap(),
            heap_segment_commit: maps.read_qword(addr + 0xd0).unwrap(),
            heap_decommit_total_free_threshold: maps.read_qword(addr + 0xd8).unwrap(),
            heap_decommit_free_block_threshold: maps.read_qword(addr + 0xd8).unwrap(),
            number_of_heaps: maps.read_dword(addr + 0xe8).unwrap(),
            max_number_of_heaps: maps.read_dword(addr + 0xec).unwrap(),
            process_heaps: maps.read_qword(addr + 0xf0).unwrap(),
            gdi_share_handle_table: maps.read_qword(addr + 0xf8).unwrap(),
            process_starter_helper: maps.read_qword(addr + 0x100).unwrap(),
            gdi_dc_attribute_list: maps.read_qword(addr + 0x108).unwrap(),
            loader_lock: maps.read_qword(addr + 0x110).unwrap(),
            os_major_version: maps.read_dword(addr + 0x118).unwrap(),
            os_minor_version: maps.read_dword(addr + 0x11c).unwrap(),
            os_build_number: maps.read_word(addr + 0x120).unwrap(),
            oscsd_version: maps.read_word(addr + 0x122).unwrap(),
            os_platform_id: maps.read_dword(addr + 0x124).unwrap(),
            image_subsystem: maps.read_dword(addr + 0x128).unwrap(),
            image_subsystem_major_version: maps.read_dword(addr + 0x12c).unwrap(),
            image_subsystem_minor_version: maps.read_qword(addr + 0x130).unwrap(),
            active_process_afinity_mask: maps.read_qword(addr + 0x138).unwrap(),
            gdi_handle_buffer: [0; 30],
            post_process_init_routine: maps.read_qword(addr + 0x230).unwrap(),
            tls_expansion_bitmap: maps.read_qword(addr + 0x238).unwrap(),
            tls_expansion_bitmap_bits: [0; 32],
            session_id: maps.read_qword(addr + 0x2c0).unwrap(),
            app_compat_flags: maps.read_qword(addr + 0x2c8).unwrap(),
            app_compat_flags_user: maps.read_qword(addr + 0x2d0).unwrap(),
            p_shim_data: maps.read_qword(addr + 0x2d8).unwrap(),
            app_compat_info: maps.read_qword(addr + 0x2e0).unwrap(),
            csd_version: [
                maps.read_qword(addr + 0x2e8).unwrap(),
                maps.read_qword(addr + 0x2f0).unwrap(),
            ],
            activate_context_data: maps.read_qword(addr + 0x2f8).unwrap(),
            process_assembly_storage_map: maps.read_qword(addr + 0x300).unwrap(),
            system_default_activation_context_data: maps.read_qword(addr + 0x308).unwrap(),
            system_assembly_storage_map: maps.read_qword(addr + 0x310).unwrap(),
            minimum_stack_commit: maps.read_qword(addr + 0x318).unwrap(),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_byte(base, self.inheritet_addr_space);
        mem.write_byte(base + 1, self.read_img_file_exec_options);
        mem.write_byte(base + 2, self.being_debugged);
        mem.write_byte(base + 3, self.system_dependent_01);
        mem.write_dword(base + 4, self.dummy_align);
        mem.write_qword(base + 8, self.mutant);
        mem.write_qword(base + 16, self.image_base_addr);
        mem.write_qword(base + 24, self.ldr);
        mem.write_qword(base + 32, self.process_parameters);
        mem.write_qword(base + 40, self.subsystem_data);
        mem.write_qword(base + 48, self.process_heap);
        mem.write_qword(base + 56, self.fast_peb_lock);
        mem.write_qword(base + 64, self.system_dependent_02);
        mem.write_qword(base + 72, self.system_dependent_03);
        mem.write_qword(base + 80, self.system_dependent_04);
        mem.write_qword(base + 88, self.kernel_callback_table);
        mem.write_dword(base + 96, self.system_reserved);
        mem.write_dword(base + 100, self.system_dependent_05);
        mem.write_qword(base + 104, self.system_dependent_06);
        mem.write_qword(base + 112, self.tls_expansion_counter);
        mem.write_qword(base + 120, self.tls_bitmap);
        mem.write_dword(base + 128, self.tls_bitmap_bits[0]);
        mem.write_dword(base + 132, self.tls_bitmap_bits[1]);
        mem.write_qword(base + 136, self.read_only_shared_memory_base);
        mem.write_qword(base + 144, self.system_dependent_07);
        mem.write_qword(base + 152, self.read_only_static_server_data);
        mem.write_qword(base + 160, self.ansi_code_page_data);
        mem.write_qword(base + 168, self.oem_code_page_data);
        mem.write_qword(base + 176, self.unicode_case_table_data);
        mem.write_dword(base + 184, self.number_of_processors);
        mem.write_dword(base + 188, self.nt_global_flag);
        mem.write_qword(base + 192, self.critical_section_timeout);
        mem.write_qword(base + 200, self.heap_segment_reserve);
        mem.write_qword(base + 208, self.heap_segment_commit);
        mem.write_qword(base + 216, self.heap_decommit_total_free_threshold);
        mem.write_qword(base + 224, self.heap_decommit_free_block_threshold);
        mem.write_dword(base + 232, self.number_of_heaps);
        mem.write_dword(base + 236, self.max_number_of_heaps);
        mem.write_qword(base + 240, self.process_heaps);
        mem.write_qword(base + 248, self.gdi_share_handle_table);
        mem.write_qword(base + 256, self.process_starter_helper);
        mem.write_qword(base + 264, self.gdi_dc_attribute_list);
        mem.write_qword(base + 272, self.loader_lock);
        mem.write_dword(base + 280, self.os_major_version);
        mem.write_dword(base + 284, self.os_minor_version);
        mem.write_word(base + 288, self.os_build_number);
        mem.write_word(base + 290, self.oscsd_version);
        mem.write_dword(base + 292, self.os_platform_id);
        mem.write_dword(base + 296, self.image_subsystem);
        mem.write_dword(base + 300, self.image_subsystem_major_version);
        mem.write_qword(base + 304, self.image_subsystem_minor_version);
        mem.write_qword(base + 312, self.active_process_afinity_mask);
        let mut idx = base + 312 + 8;
        for i in 0..30 {
            mem.write_qword(idx, self.gdi_handle_buffer[i as usize]);
            idx += 8;
        }
        mem.write_qword(idx, self.post_process_init_routine);
        mem.write_qword(idx + 8, self.tls_expansion_bitmap);
        idx += 8;
        for i in 0..32 {
            mem.write_dword(idx, self.tls_expansion_bitmap_bits[i]);
            idx += 4;
        }
        mem.write_qword(idx, self.session_id);
        mem.write_qword(idx + 8, self.app_compat_flags);
        mem.write_qword(idx + 16, self.app_compat_flags_user);
        mem.write_qword(idx + 24, self.p_shim_data);
        mem.write_qword(idx + 32, self.app_compat_info);
        mem.write_qword(idx + 40, self.csd_version[0]);
        mem.write_qword(idx + 48, self.csd_version[1]);
        mem.write_qword(idx + 56, self.activate_context_data);
        mem.write_qword(idx + 64, self.process_assembly_storage_map);
        mem.write_qword(idx + 72, self.system_default_activation_context_data);
        mem.write_qword(idx + 80, self.system_assembly_storage_map);
        mem.write_qword(idx + 88, self.minimum_stack_commit);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct NtTib64 {
    pub exception_list: u64,
    pub stack_base: u64,
    pub stack_limit: u64,
    pub sub_system_tib: u64,
    pub fiber_data: u64,
    pub arbitrary_user_pointer: u64,
    pub self_pointer: u64,
}

impl Default for NtTib64 {
    fn default() -> Self {
        Self::new()
    }
}

impl NtTib64 {
    pub fn new() -> NtTib64 {
        NtTib64 {
            exception_list: 0,
            stack_base: 0,
            stack_limit: 0,
            sub_system_tib: 0,
            fiber_data: 0,
            arbitrary_user_pointer: 0,
            self_pointer: 0,
        }
    }

    pub fn size() -> usize {
        56
    }

    pub fn load(addr: u64, maps: &Maps) -> NtTib64 {
        NtTib64 {
            exception_list: maps.read_qword(addr).unwrap(),
            stack_base: maps.read_qword(addr + 8).unwrap(),
            stack_limit: maps.read_qword(addr + 16).unwrap(),
            sub_system_tib: maps.read_qword(addr + 24).unwrap(),
            fiber_data: maps.read_qword(addr + 32).unwrap(),
            arbitrary_user_pointer: maps.read_qword(addr + 40).unwrap(),
            self_pointer: maps.read_qword(addr + 48).unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> NtTib64 {
        NtTib64 {
            exception_list: map.read_qword(addr),
            stack_base: map.read_qword(addr + 8),
            stack_limit: map.read_qword(addr + 16),
            sub_system_tib: map.read_qword(addr + 24),
            fiber_data: map.read_qword(addr + 32),
            arbitrary_user_pointer: map.read_qword(addr + 40),
            self_pointer: map.read_qword(addr + 48),
        }
    }

    pub fn save(&self, base: u64, mem: &mut Mem64) {
        mem.write_qword(base, self.exception_list);
        mem.write_qword(base + 8, self.stack_base);
        mem.write_qword(base + 16, self.stack_limit);
        mem.write_qword(base + 24, self.sub_system_tib);
        mem.write_qword(base + 32, self.fiber_data);
        mem.write_qword(base + 40, self.arbitrary_user_pointer);
        mem.write_qword(base + 48, self.self_pointer);
    }
}

#[derive(Debug)]
pub struct TEB64 {
    pub nt_tib: NtTib64,
    pub environment_pointer: u64,
    pub process_id: u64,
    pub thread_id: u64,
    pub active_rpc_handle: u64,
    pub thread_local_storage_pointer: u64,
    pub process_environment_block: u64, // PEB64
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u64,
    pub win32_thread_info: u64,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 6],
    pub wow32_reserved: u64,
    pub current_locale: u32,
    pub fp_software_status_register: u32,
    pub system_reserved1: [u64; 54],
    pub exception_code: u32,
    pub activation_context_stack_pointer: u64,
}

impl TEB64 {
    pub fn new(peb_addr: u64) -> TEB64 {
        TEB64 {
            nt_tib: NtTib64::new(),
            environment_pointer: 0,
            process_id: 3240,
            thread_id: 1,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: peb_addr,
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: 0,
            current_locale: 0,
            fp_software_status_register: 0,
            system_reserved1: [0; 54],
            exception_code: 0,
            activation_context_stack_pointer: 0,
        }
    }

    pub fn size() -> usize {
        0x2d0
    }

    pub fn load(addr: u64, maps: &Maps) -> TEB64 {
        TEB64 {
            nt_tib: NtTib64::load(addr, maps),
            environment_pointer: maps.read_qword(addr + 0x38).unwrap(),
            process_id: maps.read_qword(addr + 0x40).unwrap(),
            thread_id: maps.read_qword(addr + 0x48).unwrap(),
            active_rpc_handle: maps.read_qword(addr + 0x50).unwrap(),
            thread_local_storage_pointer: maps.read_qword(addr + 0x58).unwrap(),
            process_environment_block: maps.read_qword(addr + 0x60).unwrap(),
            last_error_value: maps.read_dword(addr + 0x68).unwrap(),
            count_of_owned_critical_sections: maps.read_dword(addr + 0x6c).unwrap(),
            csr_client_thread: maps.read_qword(addr + 0x70).unwrap(),
            win32_thread_info: maps.read_qword(addr + 0x78).unwrap(),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: maps.read_qword(addr + 0x100).unwrap(),
            current_locale: maps.read_dword(addr + 0x108).unwrap(),
            fp_software_status_register: maps.read_dword(addr + 0x10c).unwrap(),
            system_reserved1: [0; 54],
            exception_code: maps.read_dword(addr + 0x2c0).unwrap(),
            activation_context_stack_pointer: maps.read_qword(addr + 0x2c8).unwrap(),
        }
    }
    pub fn set_last_error(&mut self, err: u32) {
        self.last_error_value = err;
    }

    pub fn load_map(addr: u64, map: &Mem64) -> TEB64 {
        TEB64 {
            nt_tib: NtTib64::load_map(addr, map),
            environment_pointer: map.read_qword(addr + 0x38),
            process_id: map.read_qword(addr + 0x40),
            thread_id: map.read_qword(addr + 0x48),
            active_rpc_handle: map.read_qword(addr + 0x50),
            thread_local_storage_pointer: map.read_qword(addr + 0x58),
            process_environment_block: map.read_qword(addr + 0x60),
            last_error_value: map.read_dword(addr + 0x68),
            count_of_owned_critical_sections: map.read_dword(addr + 0x6c),
            csr_client_thread: map.read_qword(addr + 0x70),
            win32_thread_info: map.read_qword(addr + 0x78),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: map.read_qword(addr + 0x100),
            current_locale: map.read_dword(addr + 0x108),
            fp_software_status_register: map.read_dword(addr + 0x10c),
            system_reserved1: [0; 54],
            exception_code: map.read_dword(addr + 0x2c0),
            activation_context_stack_pointer: map.read_qword(addr + 0x2c8),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        self.nt_tib.save(base, mem);
        mem.write_qword(base + 0x38, self.environment_pointer);
        mem.write_qword(base + 0x40, self.process_id);
        mem.write_qword(base + 0x48, self.thread_id);
        mem.write_qword(base + 0x50, self.active_rpc_handle);
        mem.write_qword(base + 0x58, self.thread_local_storage_pointer);
        mem.write_qword(base + 0x60, self.process_environment_block);
        mem.write_dword(base + 0x68, self.last_error_value);
        mem.write_dword(base + 0x6c, self.count_of_owned_critical_sections);
        mem.write_qword(base + 0x70, self.csr_client_thread);
        mem.write_qword(base + 0x78, self.win32_thread_info);
        let mut idx = base + 0x100;
        for i in 0..26 {
            mem.write_dword(idx, self.user32_reserved[i]);
            idx += 4;
        }
        let mut idx = base + 0x108;
        for i in 0..6 {
            mem.write_dword(idx, self.user_reserved[i]);
            idx += 4;
        }
        mem.write_qword(base + 0x100, self.wow32_reserved);
        mem.write_dword(base + 0x108, self.current_locale);
        mem.write_dword(base + 0x10c, self.fp_software_status_register);
        mem.write_dword(base + 0x2c0, self.exception_code);
        mem.write_qword(base + 0x2c8, self.activation_context_stack_pointer);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct UnicodeString {
    pub length: u16,         // 0x58          0x68
    pub maximum_length: u16, // 0x5a  0x6a
    pub buffer: u32,         // 0x60         0x70
}

impl Default for UnicodeString {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeString {
    pub fn size() -> u32 {
        0x10
    }

    pub fn new() -> UnicodeString {
        UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString {
        UnicodeString {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            buffer: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_dword(addr + 4, self.buffer);
    }
}

#[derive(Debug)]
pub struct UnicodeString64 {
    pub length: u16,         // 0x58          0x68
    pub maximum_length: u16, // 0x5a  0x6a
    pub padding: u32,        // 0x5c         0x6c
    pub buffer: u64,         // 0x60         0x70
}

impl Default for UnicodeString64 {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeString64 {
    pub fn size() -> u64 {
        0x10
    }

    pub fn new() -> UnicodeString64 {
        UnicodeString64 {
            length: 0,
            maximum_length: 0,
            padding: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString64 {
        UnicodeString64 {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            padding: maps.read_dword(addr + 4).unwrap(),
            buffer: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_dword(addr + 4, self.padding);
        maps.write_qword(addr + 8, self.buffer);
    }
}

#[derive(Debug)]
pub struct LdrDataTableEntry64 {
    pub in_load_order_links: ListEntry64,
    pub in_memory_order_links: ListEntry64,
    pub in_initialization_order_links: ListEntry64,
    pub dll_base: u64,
    pub entry_point: u64,
    pub size_of_image: u64,
    pub full_dll_name: UnicodeString64,
    pub base_dll_name: UnicodeString64,
    pub flags: u32,
    pub load_count: u16,
    pub tls_index: u16,
    pub hash_links: ListEntry64,
    pub time_date_stamp: u32,
}

impl Default for LdrDataTableEntry64 {
    fn default() -> Self {
        Self::new()
    }
}

impl LdrDataTableEntry64 {
    pub fn size() -> u64 {
        0x100
    }

    pub fn new() -> LdrDataTableEntry64 {
        LdrDataTableEntry64 {
            in_load_order_links: ListEntry64::new(),
            in_memory_order_links: ListEntry64::new(),
            in_initialization_order_links: ListEntry64::new(),
            dll_base: 0,
            entry_point: 0,
            size_of_image: 0,
            full_dll_name: UnicodeString64::new(),
            base_dll_name: UnicodeString64::new(),
            flags: 0,
            load_count: 0,
            tls_index: 0,
            hash_links: ListEntry64::new(),
            time_date_stamp: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> LdrDataTableEntry64 {
        LdrDataTableEntry64 {
            in_load_order_links: ListEntry64::load(addr, maps),
            in_memory_order_links: ListEntry64::load(addr + 0x10, maps),
            in_initialization_order_links: ListEntry64::load(addr + 0x20, maps),
            dll_base: maps.read_qword(addr + 0x30).unwrap(),
            entry_point: maps.read_qword(addr + 0x38).unwrap(),
            size_of_image: maps.read_qword(addr + 0x40).unwrap(),
            full_dll_name: UnicodeString64::load(addr + 0x48, maps),
            base_dll_name: UnicodeString64::load(addr + 0x58, maps),
            flags: maps.read_dword(addr + 0x68).unwrap(), // cc 22 00 00   c4 a2 00 00   cc a2 c0 00
            load_count: maps.read_word(addr + 0x7b).unwrap(), // ff ff
            tls_index: maps.read_word(addr + 0x7d).unwrap(), // ff ff
            hash_links: ListEntry64::load(addr + 0x7f, maps),
            time_date_stamp: maps.read_dword(addr + 0x8f).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.in_load_order_links.save(addr, maps);
        self.in_memory_order_links.save(addr + 0x10, maps);
        self.in_initialization_order_links.save(addr + 0x20, maps);
        maps.write_qword(addr + 0x30, self.dll_base);
        maps.write_qword(addr + 0x38, self.entry_point);
        maps.write_qword(addr + 0x40, self.size_of_image);
        self.full_dll_name.save(addr + 0x48, maps);
        self.base_dll_name.save(addr + 0x58, maps);
        maps.write_dword(addr + 0x68, self.flags);
        maps.write_word(addr + 0x7b, self.load_count);
        maps.write_word(addr + 0x7d, self.tls_index);
        self.hash_links.save(addr + 0x7f, maps);
        maps.write_dword(addr + 0x8f, self.time_date_stamp);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageExportDirectory {
    characteristics: u32,
    time_date_stamp: u32,
    major_version: u16,
    minor_version: u16,
    name: u32,
    base: u32,
    number_of_functions: u32,
    number_of_names: u32,
    address_of_functions: u32,
    address_of_names: u32,
    address_of_ordinals: u32,
}

impl ImageExportDirectory {
    pub fn load(addr: u64, maps: &Maps) -> ImageExportDirectory {
        ImageExportDirectory {
            characteristics: maps.read_dword(addr).unwrap(),
            time_date_stamp: maps.read_dword(addr + 4).unwrap(),
            major_version: maps.read_word(addr + 8).unwrap(),
            minor_version: maps.read_word(addr + 10).unwrap(),
            name: maps.read_dword(addr + 12).unwrap(),
            base: maps.read_dword(addr + 16).unwrap(),
            number_of_functions: maps.read_dword(addr + 20).unwrap(),
            number_of_names: maps.read_dword(addr + 24).unwrap(),
            address_of_functions: maps.read_dword(addr + 28).unwrap(),
            address_of_names: maps.read_dword(addr + 32).unwrap(),
            address_of_ordinals: maps.read_dword(addr + 36).unwrap(),
        }
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

////// EXCEPTIONS //////

/*
ypedef struct _SCOPETABLE_ENTRY {
 DWORD EnclosingLevel;
 PVOID FilterFunc;
 PVOID HandlerFunc;
} SCOPETABLE_ENTRY, *PSCOPETABLE_ENTRY;
*/

#[derive(Debug)]
pub struct PScopeTableEntry {
    enclosing_level: u32,
    filter_func: u32,
    handler_func: u32,
}

impl PScopeTableEntry {
    pub fn load(addr: u64, maps: &Maps) -> PScopeTableEntry {
        PScopeTableEntry {
            enclosing_level: maps.read_dword(addr).unwrap(),
            filter_func: maps.read_dword(addr + 4).unwrap(),
            handler_func: maps.read_dword(addr + 8).unwrap(),
        }
    }

    pub fn size() -> u64 {
        12
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct CppEhRecord {
    old_esp: u32,
    exc_ptr: u32,
    next: u32, // ptr to _EH3_EXCEPTION_REGISTRATION
    exception_handler: u32,
    scope_table: PScopeTableEntry,
    try_level: u32,
}

impl CppEhRecord {
    pub fn load(addr: u64, maps: &Maps) -> CppEhRecord {
        CppEhRecord {
            old_esp: maps.read_dword(addr).unwrap(),
            exc_ptr: maps.read_dword(addr + 4).unwrap(),
            next: maps.read_dword(addr + 8).unwrap(),
            exception_handler: maps.read_dword(addr + 12).unwrap(),
            scope_table: PScopeTableEntry::load(addr + 16, maps),
            try_level: maps
                .read_dword(addr + 16 + PScopeTableEntry::size())
                .unwrap(),
        }
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ExceptionPointers {
    exception_record: u32,
    context_record: u32,
}

impl ExceptionPointers {
    pub fn load(addr: u64, maps: &Maps) -> ExceptionPointers {
        ExceptionPointers {
            exception_record: maps.read_dword(addr).unwrap(),
            context_record: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn size() -> u64 {
        8
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct Eh3ExceptionRegistration {
    next: u32,
    exception_handler: u32,
    scope_table: PScopeTableEntry,
    try_level: u32,
}

impl Eh3ExceptionRegistration {
    pub fn load(addr: u64, maps: &Maps) -> Eh3ExceptionRegistration {
        Eh3ExceptionRegistration {
            next: maps.read_dword(addr).unwrap(),
            exception_handler: maps.read_dword(addr + 4).unwrap(),
            scope_table: PScopeTableEntry::load(addr + 8, maps),
            try_level: maps
                .read_dword(addr + 8 + PScopeTableEntry::size())
                .unwrap(),
        }
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct MemoryBasicInformation {
    pub base_address: u32,
    pub allocation_base: u32,
    pub allocation_protect: u32,
    pub partition_id: u16,
    pub region_size: u32,
    pub state: u32,
    pub protect: u32,
    pub typ: u32,
}

impl MemoryBasicInformation {
    pub fn guess(addr: u64, maps: &mut Maps) -> MemoryBasicInformation {
        match maps.get_mem_by_addr_mut(addr) {
            Some(mem) => MemoryBasicInformation {
                base_address: mem.get_base() as u32,
                allocation_base: mem.get_base() as u32,
                allocation_protect: 0xff,
                partition_id: 0,
                region_size: mem.size() as u32,
                state: 0,
                protect: 0xff,
                typ: 0,
            },
            None => MemoryBasicInformation {
                base_address: 0,
                allocation_base: 0,
                allocation_protect: 0xff,
                partition_id: 0,
                region_size: 0,
                state: 0,
                protect: 0xff,
                typ: 0,
            },
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> MemoryBasicInformation {
        MemoryBasicInformation {
            base_address: maps.read_dword(addr).unwrap(),
            allocation_base: maps.read_dword(addr + 4).unwrap(),
            allocation_protect: maps.read_dword(addr + 8).unwrap(),
            partition_id: maps.read_word(addr + 12).unwrap(),
            region_size: maps.read_dword(addr + 14).unwrap(),
            state: maps.read_dword(addr + 18).unwrap(),
            protect: maps.read_dword(addr + 22).unwrap(),
            typ: maps.read_dword(addr + 26).unwrap(),
        }
    }

    pub fn size() -> u64 {
        30
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.base_address);
        maps.write_dword(addr + 4, self.allocation_base);
        maps.write_dword(addr + 8, self.allocation_protect);
        maps.write_word(addr + 12, self.partition_id);
        maps.write_dword(addr + 14, self.region_size);
        maps.write_dword(addr + 18, self.state);
        maps.write_dword(addr + 22, self.protect);
        maps.write_dword(addr + 26, self.typ);
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

// TLS

#[derive(Debug)]
pub struct TlsDirectory32 {
    tls_data_start: u32,
    tls_data_end: u32,
    tls_index: u32, // DS:[FS:[2Ch]] + tls_index *4
    tls_callbacks: u32,
    zero_fill_size: u32, // size = tls_data_end - tls_data_start + zero_fill_size
    characteristic: u32,
}

impl TlsDirectory32 {
    pub fn load(addr: u64, maps: &Maps) -> TlsDirectory32 {
        TlsDirectory32 {
            tls_data_start: maps.read_dword(addr).unwrap(),
            tls_data_end: maps.read_dword(addr + 4).unwrap(),
            tls_index: maps.read_dword(addr + 8).unwrap(),
            tls_callbacks: maps.read_dword(addr + 12).unwrap(),
            zero_fill_size: maps.read_dword(addr + 16).unwrap(),
            characteristic: maps.read_dword(addr + 20).unwrap(),
        }
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct TlsDirectory64 {
    tls_data_start: u64,
    tls_data_end: u64,
    tls_index: u64, // DS:[FS:[2Ch]] + tls_index *4
    tls_callbacks: u64,
    zero_fill_size: u32, // size = tls_data_end - tls_data_start + zero_fill_size
    characteristic: u32,
}

impl TlsDirectory64 {
    pub fn load(addr: u64, maps: &Maps) -> TlsDirectory64 {
        TlsDirectory64 {
            tls_data_start: maps.read_qword(addr).unwrap(),
            tls_data_end: maps.read_qword(addr + 8).unwrap(),
            tls_index: maps.read_qword(addr + 16).unwrap(),
            tls_callbacks: maps.read_qword(addr + 24).unwrap(),
            zero_fill_size: maps.read_dword(addr + 32).unwrap(),
            characteristic: maps.read_dword(addr + 34).unwrap(),
        }
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageTlsCallback {
    // every tls callback has this structure
    dll_handle: u32,
    reason: u32,
    reserved: u32,
}

#[derive(Debug)]
pub struct OsVersionInfo {
    version_info_size: u32,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    platform_id: u32,
    version: [u8; 128],
}

impl Default for OsVersionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl OsVersionInfo {
    pub fn new() -> OsVersionInfo {
        let mut ovi = OsVersionInfo {
            version_info_size: 284,
            major_version: 10,
            minor_version: 0,
            build_number: 19042,
            platform_id: 2,
            version: [0; 128],
        };

        "Service Pack 0"
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &byte)| {
                ovi.version[i] = byte;
            });

        ovi
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.version_info_size);
        maps.write_dword(addr + 4, self.major_version);
        maps.write_dword(addr + 8, self.minor_version);
        maps.write_dword(addr + 12, self.build_number);
        maps.write_dword(addr + 16, self.platform_id);
        maps.write_buffer(addr + 20, &self.version);
    }
}

#[derive(Debug)]
pub struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    millis: u16,
}

impl SystemTime {
    pub fn now() -> SystemTime {
        let now = Utc::now();
        let systime = SystemTime {
            year: now.year() as u16,
            month: now.month() as u16,
            day_of_week: now.weekday() as u16,
            day: now.day() as u16,
            hour: now.hour() as u16,
            minute: now.minute() as u16,
            second: now.second() as u16,
            millis: now.timestamp_millis() as u16,
        };

        systime
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.year);
        maps.write_word(addr + 2, self.month);
        maps.write_word(addr + 4, self.day_of_week);
        maps.write_word(addr + 6, self.day);
        maps.write_word(addr + 8, self.hour);
        maps.write_word(addr + 10, self.minute);
        maps.write_word(addr + 12, self.second);
        maps.write_word(addr + 14, self.millis);
    }
}

#[derive(Debug)]
pub struct StartupInfo32 {
    cb: u32,
    reserved: u32,
    desktop: u32,
    title: u32,
    x: u32,
    y: u32,
    x_size: u32,
    y_size: u32,
    x_count_chars: u32,
    y_count_chars: u32,
    fill_attribute: u32,
    flags: u32,
    show_window: u16,
    cb_reserved2: u16,
    lp_reserved2: u32,
    std_input: u32,
    std_output: u32,
    std_error: u32,
}

impl Default for StartupInfo32 {
    fn default() -> Self {
        Self::new()
    }
}

impl StartupInfo32 {
    pub fn new() -> StartupInfo32 {
        StartupInfo32 {
            cb: 68,
            reserved: 0,
            desktop: 0,
            title: 0,
            x: 10,
            y: 10,
            x_size: 300,
            y_size: 200,
            x_count_chars: 0,
            y_count_chars: 0,
            fill_attribute: 0,
            flags: 0,
            show_window: 1,
            cb_reserved2: 0,
            lp_reserved2: 0,
            std_input: 0,
            std_output: 0,
            std_error: 0,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.cb);
        maps.write_dword(addr + 4, self.reserved);
        maps.write_dword(addr + 8, self.desktop);
        maps.write_dword(addr + 12, self.title);
        maps.write_dword(addr + 16, self.x);
        maps.write_dword(addr + 20, self.y);
        maps.write_dword(addr + 24, self.x_size);
        maps.write_dword(addr + 28, self.y_size);
        maps.write_dword(addr + 32, self.x_count_chars);
        maps.write_dword(addr + 36, self.y_count_chars);
        maps.write_dword(addr + 40, self.fill_attribute);
        maps.write_dword(addr + 44, self.flags);
        maps.write_word(addr + 48, self.show_window);
        maps.write_word(addr + 50, self.cb_reserved2);
        maps.write_dword(addr + 52, self.lp_reserved2);
        maps.write_dword(addr + 56, self.std_input);
        maps.write_dword(addr + 60, self.std_output);
        maps.write_dword(addr + 64, self.std_error);
    }
}

#[derive(Debug)]
pub struct StartupInfo64 {
    cb: u32,
    reserved: u64,
    desktop: u64,
    title: u64,
    x: u32,
    y: u32,
    x_size: u32,
    y_size: u32,
    x_count_chars: u32,
    y_count_chars: u32,
    fill_attribute: u32,
    flags: u32,
    show_window: u16,
    cb_reserved2: u16,
    lp_reserved2: u64,
    std_input: u32,
    std_output: u32,
    std_error: u32,
}

impl Default for StartupInfo64 {
    fn default() -> Self {
        Self::new()
    }
}

impl StartupInfo64 {
    pub fn new() -> StartupInfo64 {
        StartupInfo64 {
            cb: 84,
            reserved: 0,
            desktop: 0,
            title: 0,
            x: 10,
            y: 10,
            x_size: 300,
            y_size: 200,
            x_count_chars: 0,
            y_count_chars: 0,
            fill_attribute: 0,
            flags: 0,
            show_window: 1,
            cb_reserved2: 0,
            lp_reserved2: 0,
            std_input: 0,
            std_output: 0,
            std_error: 0,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.cb);
        maps.write_qword(addr + 4, self.reserved);
        maps.write_qword(addr + 12, self.desktop);
        maps.write_qword(addr + 20, self.title);
        maps.write_dword(addr + 28, self.x);
        maps.write_dword(addr + 32, self.y);
        maps.write_dword(addr + 36, self.x_size);
        maps.write_dword(addr + 40, self.y_size);
        maps.write_dword(addr + 44, self.x_count_chars);
        maps.write_dword(addr + 48, self.y_count_chars);
        maps.write_dword(addr + 52, self.fill_attribute);
        maps.write_dword(addr + 56, self.flags);
        maps.write_word(addr + 60, self.show_window);
        maps.write_word(addr + 62, self.cb_reserved2);
        maps.write_qword(addr + 64, self.lp_reserved2);
        maps.write_dword(addr + 72, self.std_input);
        maps.write_dword(addr + 76, self.std_output);
        maps.write_dword(addr + 80, self.std_error);
    }
}

pub struct SystemInfo32 {
    oem_id: u32,
    processor_architecture: u32,
    reserved: u16,
    page_size: u32,
    min_app_addr: u32,
    max_app_addr: u32,
    active_processor_mask: u32,
    number_of_processors: u32,
    processor_type: u32,
    alloc_granularity: u32,
    processor_level: u16,
    processor_revision: u16,
}

impl Default for SystemInfo32 {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo32 {
    pub fn new() -> SystemInfo32 {
        SystemInfo32 {
            oem_id: 0x1337,
            processor_architecture: 9,
            reserved: 0,
            page_size: 4090,
            min_app_addr: 0,
            max_app_addr: 0,
            active_processor_mask: 1,
            number_of_processors: 4,
            processor_type: 586,
            alloc_granularity: 65536,
            processor_level: 5,
            processor_revision: 255,
        }
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.oem_id);
        maps.write_dword(addr + 4, self.processor_architecture);
        maps.write_word(addr + 8, self.reserved);
        maps.write_dword(addr + 10, self.page_size);
        maps.write_dword(addr + 14, self.min_app_addr);
        maps.write_dword(addr + 18, self.max_app_addr);
        maps.write_dword(addr + 22, self.active_processor_mask);
        maps.write_dword(addr + 26, self.number_of_processors);
        maps.write_dword(addr + 30, self.processor_type);
        maps.write_dword(addr + 34, self.alloc_granularity);
        maps.write_word(addr + 38, self.processor_level);
        maps.write_word(addr + 40, self.processor_revision);
    }

    pub fn size(&self) -> usize {
        42
    }
}

#[derive(Debug)]
pub struct SystemInfo64 {
    oem_id: u32,
    processor_architecture: u32,
    reserved: u16,
    page_size: u32,
    min_app_addr: u64,
    max_app_addr: u64,
    active_processor_mask: u64,
    number_of_processors: u32,
    processor_type: u32,
    alloc_granularity: u32,
    processor_level: u16,
    processor_revision: u16,
}

impl Default for SystemInfo64 {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo64 {
    pub fn new() -> SystemInfo64 {
        SystemInfo64 {
            oem_id: 0,
            processor_architecture: 9, // PROCESSOR_ARCHITECTURE_AMD64
            reserved: 0,
            page_size: 4096,
            min_app_addr: 0x10000,
            max_app_addr: 0x7FFFFFFEFFFF,
            active_processor_mask: 0xFF,
            number_of_processors: 8,
            processor_type: 8664,
            alloc_granularity: 65536,
            processor_level: 6,
            processor_revision: 0xA201,
        }
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        // First union/struct (4 bytes total)
        maps.write_word(addr + 0, self.processor_architecture as u16);
        maps.write_word(addr + 2, self.reserved);

        // Rest of the structure
        maps.write_dword(addr + 4, self.page_size);
        maps.write_qword(addr + 8, self.min_app_addr);
        maps.write_qword(addr + 16, self.max_app_addr);
        maps.write_qword(addr + 24, self.active_processor_mask);
        maps.write_dword(addr + 32, self.number_of_processors);
        maps.write_dword(addr + 36, self.processor_type);
        maps.write_dword(addr + 40, self.alloc_granularity);
        maps.write_word(addr + 44, self.processor_level);
        maps.write_word(addr + 46, self.processor_revision);
    }

    pub fn size(&self) -> usize {
        48
    }
}

/// Linux ////

#[derive(Debug)]
pub struct Statx64Timestamp {
    pub tv_sec: i64,
    pub tv_nsec: u32,
    pub reserved: i32,
}

#[derive(Debug)]
pub struct Statx64 {
    pub mask: u32,
    pub blksize: u32,
    pub attrib: u64,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub mode: u16,
    pub spare0: u16,
    pub inode: u64,
    pub size: u64,
    pub blocks: u64,
    pub attrib_mask: u64,
    pub atime: Statx64Timestamp,
    pub btime: Statx64Timestamp,
    pub ctime: Statx64Timestamp,
    pub mtime: Statx64Timestamp,
    pub rdev_major: u32,
    pub rdev_minor: u32,
    pub dev_major: u32,
    pub dev_minor: u32,
    pub mnt_id: u64,
    pub spare2: u64,
    pub spare3: [u64; 12],
}

#[derive(Debug)]
pub struct Stat {
    // used by fstat syscall
    pub dev: u64,
    pub ino: u64,
    pub nlink: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub pad0: u32,
    pub rdev: u64,
    pub size: i64,
    pub blksize: i64,
    pub blocks: i64,
    pub atime_sec: u64,
    pub atime_nsec: u64,
    pub mtime_sec: u64,
    pub mtime_nsec: u64,
    pub ctime_sec: u64,
    pub ctime_nsec: u64,
    pub reserved: [i64; 3],
}

impl Stat {
    pub fn fake() -> Stat {
        Stat {
            dev: 64769,
            ino: 41946037,
            nlink: 1,
            mode: 33188,
            uid: 0,
            gid: 0,
            pad0: 0,
            rdev: 0,
            size: 2794,
            blksize: 4096,
            blocks: 8,
            atime_sec: 1692634621,
            atime_nsec: 419117625,
            mtime_sec: 1690443336,
            mtime_nsec: 991482376,
            ctime_sec: 1690443336,
            ctime_nsec: 995482376,
            reserved: [0; 3],
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.dev);
        maps.write_qword(addr + 8, self.ino);
        maps.write_qword(addr + 16, self.nlink);
        maps.write_dword(addr + 24, self.mode);
        maps.write_dword(addr + 28, self.uid);
        maps.write_dword(addr + 32, self.gid);
        maps.write_dword(addr + 36, self.pad0);
        maps.write_qword(addr + 40, self.rdev);
        maps.write_qword(addr + 48, self.size as u64);
        maps.write_qword(addr + 56, self.blksize as u64);
        maps.write_qword(addr + 64, self.blocks as u64);
        maps.write_qword(addr + 72, self.atime_sec);
        maps.write_qword(addr + 80, self.atime_nsec);
        maps.write_qword(addr + 88, self.mtime_sec);
        maps.write_qword(addr + 96, self.mtime_nsec);
        maps.write_qword(addr + 104, self.ctime_sec);
        maps.write_qword(addr + 112, self.ctime_nsec);
        maps.write_qword(addr + 120, self.reserved[0] as u64);
        maps.write_qword(addr + 128, self.reserved[1] as u64);
        maps.write_qword(addr + 136, self.reserved[2] as u64);
    }

    pub fn size() -> usize {
        144
    }
}

pub struct Hostent {
    pub hname: u64,
    pub alias_list: u64,
    pub addr_type: u16,
    pub length: u16,
    pub addr_list: u64,
    // (gdb) 0x7ffff7fa0b60 -> 0x5555555595d0 -> 0x5555555595cc -> IP
}

impl Default for Hostent {
    fn default() -> Self {
        Self::new()
    }
}

impl Hostent {
    pub fn new() -> Hostent {
        Hostent {
            hname: 0,
            alias_list: 0,
            addr_type: 0,
            length: 4,
            addr_list: 0,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.hname);
        maps.write_qword(addr + 8, self.alias_list);
        maps.write_word(addr + 16, self.addr_type);
        maps.write_word(addr + 20, self.length);
        maps.write_qword(addr + 24, self.addr_list);
    }

    pub fn size() -> usize {
        32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOperation {
    /// Position/step counter in the emulation
    pub pos: u64,
    /// Instruction pointer at time of operation
    pub rip: u64,
    /// Type of memory operation ("read" or "write")
    pub op: String,
    /// Size of the operation in bits (8, 16, 32, 64)
    pub bits: u32,
    /// Memory address being accessed
    pub address: u64,
    /// Old value before the operation
    pub old_value: u64,
    /// New value after the operation
    pub new_value: u64,
    /// Name of the memory region being accessed
    pub name: String,
}

// ... existing code ...

#[derive(Debug)]
pub struct CpInfo {
    pub max_char_size: u32,
    pub default_char: [u8; 2], // MAX_DEFAULTCHAR = 2
    pub lead_byte: [u8; 12],   // MAX_LEADBYTES = 12
}

impl Default for CpInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CpInfo {
    pub fn new() -> CpInfo {
        CpInfo {
            max_char_size: 1,
            default_char: [0x3F, 0], // '?' character as default
            lead_byte: [0; 12],
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> CpInfo {
        let mut info = CpInfo::new();
        info.max_char_size = maps.read_dword(addr).unwrap();

        // Read default char array
        for i in 0..2 {
            info.default_char[i] = maps.read_byte(addr + 4 + i as u64).unwrap();
        }

        // Read lead byte array
        for i in 0..12 {
            info.lead_byte[i] = maps.read_byte(addr + 6 + i as u64).unwrap();
        }

        info
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.max_char_size);

        // Write default char array
        for i in 0..2 {
            maps.write_byte(addr + 4 + i as u64, self.default_char[i]);
        }

        // Write lead byte array
        for i in 0..12 {
            maps.write_byte(addr + 6 + i as u64, self.lead_byte[i]);
        }
    }

    pub fn size() -> usize {
        18 // 4 bytes for max_char_size + 2 bytes for default_char + 12 bytes for lead_byte
    }

    pub fn print(&self) {
        log::info!("{:#x?}", self);
    }
}

/******* resources *******/

pub struct ImageResourceDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub number_of_named_entries: u16,
    pub number_of_id_entries: u16,
}

impl ImageResourceDirectory {
    pub fn new() -> ImageResourceDirectory {
        ImageResourceDirectory {
            characteristics: 0,
            time_date_stamp: 0,
            major_version: 0,
            minor_version: 0,
            number_of_named_entries: 0,
            number_of_id_entries: 0,
        }
    }

    pub fn size() -> usize {
        16
    }
}

#[derive(Debug)]
pub struct ImageResourceDirectoryEntry {
    pub name_or_id: u32,
    pub data_or_directory: u32,
}

impl ImageResourceDirectoryEntry {
    pub fn new() -> ImageResourceDirectoryEntry {
        ImageResourceDirectoryEntry {
            name_or_id: 0,
            data_or_directory: 0,
        }
    }

    pub fn size() -> usize {
        8
    }

    pub fn print(&self) {
        log::info!(
            "name_or_id: {:x} data_or_directory: {:x}",
            self.name_or_id,
            self.data_or_directory
        );
    }

    pub fn is_name(&self) -> bool {
        self.name_or_id & 0x8000_0000 != 0
    }

    pub fn is_id(&self) -> bool {
        self.name_or_id & 0x8000_0000 == 0
    }

    pub fn get_name_or_id(&self) -> u32 {
        self.name_or_id & 0x7FFF_FFFF
    }

    pub fn is_directory(&self) -> bool {
        self.data_or_directory & 0x8000_0000 != 0
    }

    pub fn get_offset(&self) -> u32 {
        self.data_or_directory & 0x7FFF_FFFF
    }
}

pub struct ImageResourceDataEntry32 {
    pub offset_to_data: u32,
    pub size: u32,
    pub code_page: u32,
    pub reserved: u32,
}

impl ImageResourceDataEntry32 {
    pub fn new() -> ImageResourceDataEntry32 {
        ImageResourceDataEntry32 {
            offset_to_data: 0,
            size: 0,
            code_page: 0,
            reserved: 0,
        }
    }
}

pub struct ImageResourceDataEntry64 {
    pub offset_to_data: u64,
    pub size: u64,
    pub code_page: u64,
    pub reserved: u64,
}

impl ImageResourceDataEntry64 {
    pub fn new() -> ImageResourceDataEntry64 {
        ImageResourceDataEntry64 {
            offset_to_data: 0,
            size: 0,
            code_page: 0,
            reserved: 0,
        }
    }
}

pub struct ActCtxSectionKeyedData32 {
    pub cb_size: u32,
    pub ul_data_format_version: u32,
    pub lp_data: u32,
    pub ul_length: u32,
    pub lp_section_global_data: u32,
    pub ul_section_global_data_length: u32,
    pub lp_section_base: u32,
    pub ul_section_total_length: u32,
    pub h_act_ctx: u32,
    pub ul_assembly_roster_index: u32,
    pub ul_flags: u32,
    pub assembly_metadata: [u8; 64],
}

impl ActCtxSectionKeyedData32 {
    pub fn new() -> ActCtxSectionKeyedData32 {
        ActCtxSectionKeyedData32 {
            cb_size: 0,
            ul_data_format_version: 0,
            lp_data: 0,
            ul_length: 0,
            lp_section_global_data: 0,
            ul_section_global_data_length: 0,
            lp_section_base: 0,
            ul_section_total_length: 0,
            h_act_ctx: 0,
            ul_assembly_roster_index: 0,
            ul_flags: 0,
            assembly_metadata: [0; 64],
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.cb_size);
        maps.write_dword(addr + 4, self.ul_data_format_version);
        maps.write_dword(addr + 8, self.lp_data);
        maps.write_dword(addr + 12, self.ul_length);
        maps.write_dword(addr + 16, self.lp_section_global_data);
        maps.write_dword(addr + 20, self.ul_section_global_data_length);
        maps.write_dword(addr + 24, self.lp_section_base);
        maps.write_dword(addr + 28, self.ul_section_total_length);
        maps.write_dword(addr + 32, self.h_act_ctx);
        maps.write_dword(addr + 36, self.ul_assembly_roster_index);
        maps.write_dword(addr + 40, self.ul_flags);
        maps.write_bytes(addr + 44, self.assembly_metadata.to_vec());
    }
}

pub struct ActCtxSectionKeyedData64 {
    pub cb_size: u32,
    pub ul_data_format_version: u32,
    pub lp_data: u64,
    pub ul_length: u32,
    pub lp_section_global_data: u64,
    pub ul_section_global_data_length: u32,
    pub lp_section_base: u64,
    pub ul_section_total_length: u32,
    pub h_act_ctx: u64,
    pub ul_assembly_roster_index: u32,
    pub ul_flags: u32,
    pub assembly_metadata: [u8; 64],
}

impl ActCtxSectionKeyedData64 {
    pub fn new() -> ActCtxSectionKeyedData64 {
        ActCtxSectionKeyedData64 {
            cb_size: 0,
            ul_data_format_version: 0,
            lp_data: 0,
            ul_length: 0,
            lp_section_global_data: 0,
            ul_section_global_data_length: 0,
            lp_section_base: 0,
            ul_section_total_length: 0,
            h_act_ctx: 0,
            ul_assembly_roster_index: 0,
            ul_flags: 0,
            assembly_metadata: [0; 64],
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.cb_size);
        maps.write_dword(addr + 4, self.ul_data_format_version);
        maps.write_qword(addr + 8, self.lp_data);
        maps.write_dword(addr + 16, self.ul_length);
        maps.write_qword(addr + 24, self.lp_section_global_data);
        maps.write_dword(addr + 32, self.ul_section_global_data_length);
        maps.write_qword(addr + 40, self.lp_section_base);
        maps.write_dword(addr + 48, self.ul_section_total_length);
        maps.write_qword(addr + 56, self.h_act_ctx);
        maps.write_dword(addr + 64, self.ul_assembly_roster_index);
        maps.write_dword(addr + 68, self.ul_flags);
        maps.write_bytes(addr + 72, self.assembly_metadata.to_vec());
    }
}
