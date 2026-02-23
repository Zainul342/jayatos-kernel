use x86_64::{
    structures::paging::{PageTable, OffsetPageTable},
    VirtAddr,
};

/// Inisialisasi OffsetPageTable yang baru.
/// Fungsi ini 'unsafe' karena pemanggil harus menjamin bahwa virtual_page_table_offset
/// yang diberikan valid untuk seluruh pemetaan memori fisik.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Mengambil referensi mutable ke Level 4 Page Table yang sedang aktif.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // Unsafe!
}
