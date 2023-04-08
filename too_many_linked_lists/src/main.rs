pub mod ch01_bad_stack;
pub mod ch02_ok_linked_stack;
mod ch03_persistent_stack;
mod ch04_bad_doubly_linked_deque;
mod ch05_ok_unsafe_doubly_linked_queue;
fn main() {
    unsafe {
        let mut data = Box::new(10);
        let ptr1 = (&mut *data) as *mut i32;
        let ptr2 = ptr1;

        *ptr1 += 1;
        *ptr2 += 1;

        println!("{}", data);
    }
}
