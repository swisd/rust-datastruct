// custom types

#![allow(non_camel_case_types)]
// unsigned -12

struct u12 {
    a: u8,
    b: u8,
}

struct u24 {
    a: u8,
    b: u8,
    c: u8,
}

struct u48 {
    a: u16,
    b: u16,
}

struct u72 {
    a: u32,
    b: u32,
}

struct u96 {
    a: u64,
    b: u64,
}

struct u192 {
    a: u128,
    b: u128,
}

// signed -12

struct i12 {
    a: i8,
    b: i8,
}

struct i24 {
    a: i8,
    b: i8,
    c: i8,
}

struct i48 {
    a: i16,
    b: i16,
}

struct i72 {
    a: i32,
    b: i32,
}

struct i96 {
    a: i64,
    b: i64,
}

struct i192 {
    a: i128,
    b: i128,
}


// impls

impl u12 {
    fn get_bytes(&self) -> (u8, u8) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: u8, b: u8) -> u32 {
        ((a as u32) << 8) | (b as u32)
    }
    pub fn to_bytes(&self) -> (u8, u8) {
        self.get_bytes()
    }
}

impl i12 {
    fn get_bytes(&self) -> (i8, i8) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: i8, b: i8) -> i32 {
        ((a as i32) << 8) | (b as i32)
    }
    pub fn to_bytes(&self) -> (i8, i8) {
        self.get_bytes()
    }
}

impl u24 {
    fn get_bytes(&self) -> (u8, u8, u8) {
        (self.a, self.b, self.c)
    }
    pub fn from_bytes(a: u8, b: u8, c: u8) -> u32 {
        ((a as u32) << 16) | ((b as u32) << 8) | (c as u32)
    }
    pub fn to_bytes(&self) -> (u8, u8, u8) {
        self.get_bytes()
    }
}

impl i24 {
    fn get_bytes(&self) -> (i8, i8, i8) {
        (self.a, self.b, self.c)
    }
    pub fn from_bytes(a: i8, b: i8, c: i8) -> i32 {
        ((a as i32) << 16) | ((b as i32) << 8) | (c as i32)
    }
    pub fn to_bytes(&self) -> (i8, i8, i8) {
        self.get_bytes()
    }
}

impl u48 {
    fn get_bytes(&self) -> (u16, u16) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: u16, b: u16) -> u64 {
        ((a as u64) << 16) | (b as u64)
    }
    pub fn to_bytes(&self) -> (u16, u16) {
        self.get_bytes()
    }
}

impl i48 {
    fn get_bytes(&self) -> (i16, i16) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: i16, b: i16) -> i64 {
        ((a as i64) << 16) | (b as i64)
    }
    pub fn to_bytes(&self) -> (i16, i16) {
        self.get_bytes()
    }
}

impl u72 {
    fn get_bytes(&self) -> (u32, u32) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: u32, b: u32) -> u128 {
        ((a as u128) << 32) | (b as u128)
    }
    pub fn to_bytes(&self) -> (u32, u32) {
        self.get_bytes()
    }
}

impl i72 {
    fn get_bytes(&self) -> (i32, i32) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: i32, b: i32) -> i128 {
        ((a as i128) << 32) | (b as i128)
    }
    pub fn to_bytes(&self) -> (i32, i32) {
        self.get_bytes()
    }
}

impl u96 {
    fn get_bytes(&self) -> (u64, u64) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: u64, b: u64) -> u192 {
        ((a as u192) << 64) | (b as u192)
    }
    pub fn to_bytes(&self) -> (u64, u64) {
        self.get_bytes()
    }
}

impl i96 {
    fn get_bytes(&self) -> (i64, i64) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: i64, b: i64) -> i192 {
        ((a as i192) << 64) | (b as i192)
    }
    pub fn to_bytes(&self) -> (i64, i64) {
        self.get_bytes()
    }
}

impl u192 {
    fn get_bytes(&self) -> (u128, u128) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: u128, b: u128) -> u192 {
        ((a as u192) << 128) | (b as u192)
    }
    pub fn to_bytes(&self) -> (u128, u128) {
        self.get_bytes()
    }
}

impl i192 {
    fn get_bytes(&self) -> (i128, i128) {
        (self.a, self.b)
    }
    pub fn from_bytes(a: i128, b: i128) -> i192 {
        ((a as i192) << 128) | (b as i192)
    }
    pub fn to_bytes(&self) -> (i128, i128) {
        self.get_bytes()
    }
}