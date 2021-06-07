use std::io::{Error, Read};
use std::mem::*;
use std::slice::*;

pub trait MemRead {
    fn get<U>(&mut self) -> Result<U, Error>;
    fn get_be<U>(&mut self) -> Result<U, Error>;
    fn get_str(&mut self) -> Result<String, Error>;
    fn get_str_sized<U: Into<usize>>(&mut self) -> Result<String, Error>;
    fn get_str_sized_be<U: Into<usize>>(&mut self) -> Result<String, Error>;
    fn get_str_slice(&mut self, size: usize) -> Result<String, Error>;
    fn skip(&mut self, count: u64) -> Result<u64, Error>;
}

impl<T: Read> MemRead for T {
    fn get<U>(&mut self) -> Result<U, Error> {
        unsafe {
            let mut x: U = MaybeUninit::uninit().assume_init();
            let slice = from_raw_parts_mut(&mut x as *mut U as *mut u8, size_of::<U>());
            self.read_exact(slice).map(|_| x)
        }
    }

    fn get_be<U>(&mut self) -> Result<U, Error> {
        unsafe {
            let mut x: U = MaybeUninit::uninit().assume_init();
            let slice = from_raw_parts_mut(&mut x as *mut U as *mut u8, size_of::<U>());
            let res = self.read_exact(slice);
            slice.reverse();
            res.map(|_| x)
        }
    }

    fn get_str(&mut self) -> Result<String, Error> {
        let size = self.get::<u16>()?;
        self::MemRead::get_str_slice(self, size as _)
    }

    fn get_str_sized<U: Into<usize>>(&mut self) -> Result<String, Error> {
        let size = self.get::<U>()?;
        self::MemRead::get_str_slice(self, size.into())
    }

    fn get_str_sized_be<U: Into<usize>>(&mut self) -> Result<String, Error> {
        let size = self.get_be::<U>()?;
        self::MemRead::get_str_slice(self, size.into())
    }

    fn get_str_slice(&mut self, size: usize) -> Result<String, Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        self.take(size as u64)
            .read_to_end(&mut buffer)
            .map(|_| String::from_utf8_lossy(&buffer).into())
    }

    fn skip(&mut self, count: u64) -> Result<u64, Error> {
        std::io::copy(&mut self.take(count), &mut std::io::sink())
    }
}