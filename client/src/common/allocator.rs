use std::ptr::NonNull;
use std::cell::UnsafeCell;
use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::cell::Cell;

#[derive(Debug,Copy,Clone)]
pub struct Ptr<T>{
    item : NonNull<T>,
    index : usize,
    generation : usize,
}

impl<T> Ptr<T> {
    pub fn GetMut(&self) -> Pin<&mut T>{
        unsafe { Pin::new_unchecked(&mut *self.item.as_ptr()) }
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.item.as_ptr() }
    }
}

/*
 * Wrapper around a pointer to a buffer
 * capacity: total amount of elements we can allocate
 * free_bits: bitfield that tells you what elements are free. 0 is free 1 is used  
 */
#[derive(Debug)]
struct AllocatorInt<T> {
    buffer : NonNull<T>,
    capacity : usize,
    free_bits : std::vec::Vec<Cell<i64>>,
}

pub struct Allocator<T>{
    a : UnsafeCell<AllocatorInt<T>>,
}


impl<T : Copy> Allocator<T> {
    pub fn new(capacity : usize) -> Self{
        unsafe{
            let layout = Layout::array::<T>(capacity).expect("aa");
            let ptr = alloc(layout);
            let bitfield_len : usize = (capacity / 64) + 1;            
            return Self{
                a : UnsafeCell::new(
                    AllocatorInt {
                        buffer : NonNull::new(ptr as *mut T).expect("Failed Alloc"),
                        capacity : capacity,
                        free_bits : vec![Cell::new(i64::MAX); bitfield_len],
                    }
                ),
            }    
        }
    }    

    pub fn New(&self) -> Option<Ptr<T>> {
        let allocator : &mut AllocatorInt<T> = self.Allocator();
        for (index, bitfield) in allocator.free_bits.iter().enumerate() { 
            if bitfield.get() != 0 {
                /**
                 * 1 is free, 0 is used
                 */
                let free_bit = 63 -  (bitfield.get().leading_zeros() as usize);
                let global_index = index*64 + free_bit;
                println!("{} {}",free_bit,bitfield.get());
                bitfield.set(bitfield.get() & ( !(1 << free_bit)));
                unsafe{
                    let ptr = allocator.buffer.as_ptr().add(global_index);
                    return Some(Ptr{
                        item : NonNull::new(ptr).expect("New Failed"),
                        index : global_index,
                        generation : 0,
                    });
                }
            }
        }
        return None;
    }

    pub fn NewInit(&self, init_value : &T) -> Ptr<T>{
        let ptr = self.New().unwrap();
        let mut value = ptr.GetMut();
        value.set(*init_value);
        return ptr;
    }

    pub fn Free(&self, ptr : Ptr<T>) {
        let hyper_index = (ptr.index / 64) as i64;
        let local_index = (ptr.index % 64) as i64;
        let mut allocator = self.Allocator();
        
        let bitfield : &Cell<i64> = &allocator.free_bits[hyper_index as usize];
        bitfield.set(bitfield.get() | ( 1 << local_index ));
    }

    fn Allocator(&self) -> &mut AllocatorInt<T> {
        unsafe{
            return &mut *self.a.get()
        }
    }
}
