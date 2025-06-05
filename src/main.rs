#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustic_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use rustic_os::println;
use rustic_os::task::{Task, executor::Executor, keyboard};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustic_os::allocator;
    use rustic_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rustic_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustic_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustic_os::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}