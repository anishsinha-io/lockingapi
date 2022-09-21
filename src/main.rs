use lazy_static::lazy_static;
use parking_lot::lock_api::RawRwLockUpgrade;
use parking_lot::{lock_api::RawRwLock as _, RwLock};
use rand::Rng;
use rayon::ThreadPoolBuilder;
use std::sync::Arc;

struct RawLockTest {
    data: usize,
}

impl RawLockTest {}

lazy_static! {
    static ref SINGLETON: Arc<RwLock<RawLockTest>> = Arc::new(RwLock::new(RawLockTest { data: 0 }));
}

unsafe fn acquire_shared() {
    SINGLETON.raw().lock_shared();
}

unsafe fn acquire_upgradable() {
    SINGLETON.raw().lock_upgradable();
}

unsafe fn acquire_excl() {
    SINGLETON.raw().lock_exclusive();
}

unsafe fn upgrade_shared() {
    SINGLETON.raw().upgrade();
}

unsafe fn release_shared() {
    SINGLETON.raw().unlock_shared();
}

unsafe fn release_upgradable() {
    SINGLETON.raw().unlock_upgradable();
}

unsafe fn release_excl() {
    SINGLETON.raw().unlock_exclusive();
}

unsafe fn echo_thread() {
    loop {
        acquire_shared();
        println!("\nacquired shared lock");
        println!("data: {}", (*SINGLETON.data_ptr()).data);
        println!("releasing shared lock\n");
        release_shared();
        use std::thread;
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

unsafe fn determine() -> bool {
    acquire_upgradable();
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("\nacquired upgradable lock. y={}", y);
    if y > 0.5 {
        return true;
    }
    return false;
}

unsafe fn sometimes_modify_thread() {
    loop {
        use std::thread;
        if determine() {
            upgrade_shared();
            println!("incremented data by five");
            (*SINGLETON.data_ptr()).data += 5;
            println!("releasing lock upgraded to write\n");
            release_excl();
            thread::sleep(std::time::Duration::from_secs(1));
            return;
        }
        println!("releasing upgradable lock\n");
        release_upgradable();
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

unsafe fn modify_thread() {
    loop {
        acquire_excl();
        println!("\nacquired exclusive lock");
        (*SINGLETON.data_ptr()).data += 1;
        println!("incremented data");
        println!("releasing exclusive lock\n");
        release_excl();
        use std::thread;
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    unsafe {
        let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
        pool.spawn(|| echo_thread());
        pool.spawn(|| echo_thread());
        pool.spawn(|| echo_thread());
        pool.spawn(|| echo_thread());
        pool.spawn(|| modify_thread());
        pool.spawn(|| modify_thread());
        pool.spawn(|| sometimes_modify_thread());
        pool.spawn(|| sometimes_modify_thread());
        loop {}
    }
}
