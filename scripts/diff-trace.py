#!/usr/bin/env python3

import csv

MWEMU_TRACE_PATH = '/tmp/output.csv' # mwemu
X64DBG_TRACE_PATH = '/Users/brandon/Desktop/pe_loader2-export-20250116-230152.csv' # x64dbg
EXPECTED_HEADERS = ["Index", "Address", "Bytes", "Disassembly", "Registers", "Memory", "Comments"]
EXPECTED_ENTRY = 0x000000014005D7F0

def validate_headers(headers):
    if headers != EXPECTED_HEADERS:
        raise ValueError(f"Invalid CSV headers. Expected {EXPECTED_HEADERS}, got {headers}")

def parse_hex(s):
    # Strip any leading/trailing whitespace and remove any '0x' prefix
    s = s.strip().replace('0x', '')
    # Handle empty strings
    if not s:
        return 0
    return int(s, 16)

def calculate_offset(first_addr):
    """Calculate the offset needed to normalize addresses to expected RVA"""
    return first_addr - EXPECTED_ENTRY

def compare_traces():
    print(f"Opening files:")
    print(f"mwemu trace: {MWEMU_TRACE_PATH}")
    print(f"x64dbg trace: {X64DBG_TRACE_PATH}")

    lines_processed = 0
    
    with open(MWEMU_TRACE_PATH, 'r') as f_mwemu, open(X64DBG_TRACE_PATH, 'r') as f_x64dbg:
        print("Files opened successfully")
        
        mwemu_reader = csv.DictReader(f_mwemu)
        x64dbg_reader = csv.DictReader(f_x64dbg)

        # Get first row to calculate offsets
        mwemu_row = next(mwemu_reader)
        x64dbg_row = next(x64dbg_reader)

        # add Source column to traces
        mwemu_row['Source'] = 'mwemu'
        x64dbg_row['Source'] = 'x64dbg'

        # parse addresses
        mwemu_addr = parse_hex(mwemu_row['Address'].split()[0])
        x64dbg_addr = parse_hex(x64dbg_row['Address'].split()[0])
        
        # Calculate offsets based on first address
        mwemu_offset = calculate_offset(mwemu_addr)
        x64dbg_offset = calculate_offset(x64dbg_addr)

        # check if x64dbg row is out of range
        entry_rva = 0x005D7F0
        x64dbg_base = x64dbg_addr - entry_rva
        text_start = 0x1000
        text_end = 0x60000

        if x64dbg_addr < x64dbg_base + text_start or x64dbg_addr > x64dbg_base + text_end:
            print(f"x64dbg row is out of range: 0x{x64dbg_addr:x}")
            return

        print(f"x64dbg base: 0x{x64dbg_base:x}")
        print(f"text start: 0x{text_start:x}")
        print(f"text end: 0x{text_end:x}")
        
        print(f"mwemu trace offset: 0x{mwemu_offset:x}")
        print(f"x64dbg trace offset: 0x{x64dbg_offset:x}")

        # Add buffer for previous lines
        mwemu_prev_lines = [(parse_hex(mwemu_row['Index']), 
                        parse_hex(mwemu_row['Address'].split()[0]) - mwemu_offset, 
                        mwemu_row)]
        x64dbg_prev_lines = [(parse_hex(x64dbg_row['Index']), 
                        parse_hex(x64dbg_row['Address'].split()[0]) - x64dbg_offset, 
                        x64dbg_row)]
        max_history = 10

        # Compare all rows
        for row_num, (mwemu_row, x64dbg_row) in enumerate(zip(mwemu_reader, x64dbg_reader), start=2):
            mwemu_idx = parse_hex(mwemu_row['Index'])
            x64dbg_idx = parse_hex(x64dbg_row['Index'])
            mwemu_addr = parse_hex(mwemu_row['Address'].split()[0]) - mwemu_offset
            x64dbg_addr = parse_hex(x64dbg_row['Address'].split()[0]) - x64dbg_offset

            # add Source column to traces
            mwemu_row['Source'] = 'mwemu'
            x64dbg_row['Source'] = 'x64dbg'

            # Store current line in history
            mwemu_prev_lines.append((mwemu_idx, mwemu_addr, mwemu_row))
            x64dbg_prev_lines.append((x64dbg_idx, x64dbg_addr, x64dbg_row))
            if len(mwemu_prev_lines) > max_history:
                mwemu_prev_lines.pop(0)
                x64dbg_prev_lines.pop(0)

            if mwemu_idx != x64dbg_idx or mwemu_addr != x64dbg_addr:
                print(f"\nDifference found at row {row_num}:")

                print(f"mwemu_idx: {mwemu_idx:x}")
                print(f"x64dbg_idx: {x64dbg_idx:x}")
                print(f"mwemu_addr: {mwemu_addr:x}")
                print(f"x64dbg_addr: {x64dbg_addr + x64dbg_offset:x}")

                print(f"\nPrevious {max_history} lines from mwemu trace:")
                for prev_idx, prev_addr, prev_row in mwemu_prev_lines:
                    print(f"{prev_row}")
                print(f"\nPrevious {max_history} lines from x64dbg trace:")
                for prev_idx, prev_addr, prev_row in x64dbg_prev_lines:
                    print(f"{prev_row}")
                return
            
            lines_processed += 1
            if lines_processed % 100000 == 0:
                print(f"Processed {lines_processed} lines")

if __name__ == '__main__':
    try:
        compare_traces()
    except Exception as e:
        print(f"Error: {e}")
