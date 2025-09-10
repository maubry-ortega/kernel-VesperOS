pub unsafe fn kreset() -> ! {
    println!("kreset");
    loop {
        core::hint::spin_loop();
    }
}

pub unsafe fn emergency_reset() -> ! {
    // Emergency reset - halt the system
    loop {
        core::hint::spin_loop();
    }
}

pub unsafe fn kstop() -> ! {
    println!("kstop");
    loop {
        core::hint::spin_loop();
    }
}
