#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod vga_buffer;
mod interrupts;
mod memory;
mod allocator;
mod shell; // MODUL BARU!

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::OffsetPageTable, VirtAddr};

    println!("------------------------------------");
    println!("  PROGRAMMEROS JAYATOS v0.1.0-alpha ");
    println!("------------------------------------");
    
    // 1. Inisialisasi Interrupts
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize(); }
    x86_64::instructions::interrupts::enable();
    println!("Interrupts:    [ ENABLED ]");

    // 2. Inisialisasi Paging
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    // 3. Inisialisasi Heap Allocator
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    println!("Heap Init:     [ OK ]");

    // 4. Inisialisasi Shell
    shell::init();
    println!("Shell Status:  [ READY ]");
    println!("------------------------------------");
    println!("Welcome, Master. Ready for commands.");
    shell::print_prompt();

    loop {
        x86_64::instructions::hlt();
    }
}

// Frame Allocator sederhana (seperti sebelumnya)
use x86_64::structures::paging::{FrameAllocator, Size4KiB, PhysFrame, UnusedPhysFrame};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator { memory_map, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item = UnusedPhysFrame> {
        self.memory_map.iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(x86_64::PhysAddr::new(addr)))
            .map(|frame| unsafe { UnusedPhysFrame::new(frame) })
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<UnusedPhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
