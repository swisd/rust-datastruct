// custom types

#![allow(non_camel_case_types)]

#[derive(Debug)]
pub struct u1 {
    a: u8,
}

#[derive(Debug)]
pub struct i1 {
    a: i8,
}

#[derive(Debug)]
pub struct u2 {
    a: u8,
}

#[derive(Debug)]
pub struct i2 {
    a: i8,
}

#[derive(Debug)]
pub struct u4 {
    a: u8,
}

pub struct i4 {
    a: i8,
}

// unsigned -12

#[derive(Debug)]
pub struct u12 {
    a: u8,
    b: u8,
}

#[derive(Debug)]
pub struct u24 {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Debug)]
pub struct u48 {
    a: u16,
    b: u16,
}

#[derive(Debug)]
pub struct u72 {
    a: u32,
    b: u32,
}

#[derive(Debug)]
pub struct u96 {
    a: u64,
    b: u64,
}

#[derive(Debug)]
pub struct u192 {
    a: u128,
    b: u128,
}

// signed -12

#[derive(Debug)]
pub struct i12 {
    a: i8,
    b: i8,
}

#[derive(Debug)]
pub struct i24 {
    a: i8,
    b: i8,
    c: i8,
}

#[derive(Debug)]
pub struct i48 {
    a: i16,
    b: i16,
}

#[derive(Debug)]
pub struct i72 {
    a: i32,
    b: i32,
}

#[derive(Debug)]
pub struct i96 {
    a: i64,
    b: i64,
}

#[derive(Debug)]
pub struct i192 {
    a: i128,
    b: i128,
}

// signed and unsigned -12 custom

#[derive(Debug)]
pub struct i12u12 {
    a: i12,
    b: u12,
}

#[derive(Debug)]
pub struct i12u24 {
    a: i12,
    b: u24,
}

// signed and unsigned -3 custom

#[derive(Debug)]
pub struct i3 {
    a: i8,
}

#[derive(Debug)]
pub struct u3 {
    a: u8,
}

#[derive(Debug)]
pub struct i6 {
    a: i8,
}

#[derive(Debug)]
pub struct u6 {
    a: u8,
}

#[derive(Debug)]
pub struct i9 {
    a: i8,
}

#[derive(Debug)]
pub struct u9 {
    a: u8,
}

#[derive(Debug)]
pub struct i18 {
    a: i16,
    b: i8,
}

#[derive(Debug)]
pub struct u18 {
    a: u16,
    b: u8,
}

#[derive(Debug)]
pub struct i27 {
    a: i16,
    b: i16,
    c: i8,
}

#[derive(Debug)]
pub struct u27 {
    a: u16,
    b: u16,
    c: u8,
}

#[derive(Debug)]
pub struct i36 {
    a: i16,
    b: i16,
    c: i16,
    d: i8,
}

#[derive(Debug)]
pub struct u36 {
    a: u16,
    b: u16,
    c: u16,
    d: u8,
}

// signed and unsigned misc custom

#[derive(Debug)]
pub struct u8x2 {
    a: u8,
    b: u8,
}

#[derive(Debug)]
pub struct i8x2 {
    a: i8,
    b: i8,
}

#[derive(Debug)]
pub struct u16x2 {
    a: u16,
    b: u16,
}

#[derive(Debug)]
pub struct i16x2 {
    a: i16,
    b: i16,
}

#[derive(Debug)]
pub struct u32x2 {
    a: u32,
    b: u32,
}

#[derive(Debug)]
pub struct i32x2 {
    a: i32,
    b: i32,
}

#[derive(Debug)]
pub struct u64x2 {
    a: u64,
    b: u64,
}

#[derive(Debug)]
pub struct i64x2 {
    a: i64,
    b: i64,
}

#[derive(Debug)]
pub struct u128x2 {
    a: u128,
    b: u128,
}

#[derive(Debug)]
pub struct i128x2 {
    a: i128,
    b: i128,
}

#[derive(Debug)]
pub struct ISx2 {
    a: isize,
    b: isize
}

#[derive(Debug)]
pub struct USx2 {
    a: usize,
    b: usize
}

#[derive(Debug)]
pub struct Bx2 {
    a: bool,
    b: bool
}

#[derive(Debug)]
pub struct F32x2 {
    a: f32,
    b: f32
}

#[derive(Debug)]
pub struct F64x2 {
    a: f64,
    b: f64
}

#[derive(Debug)]
pub struct Cx2 {
    a: char,
    b: char
}

#[derive(Debug)]
pub struct u8x3 {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Debug)]
pub struct i8x3 {
    a: i8,
    b: i8,
    c: i8,
}

#[derive(Debug)]
pub struct u16x3 {
    a: u16,
    b: u16,
    c: u16,
}

#[derive(Debug)]
pub struct i16x3 {
    a: i16,
    b: i16,
    c: i16,
}

#[derive(Debug)]
pub struct u32x3 {
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Debug)]
pub struct i32x3 {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug)]
pub struct u64x3 {
    a: u64,
    b: u64,
    c: u64,
}

pub struct i64x3 {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug)]
pub struct u128x3 {
    a: u128,
    b: u128,
    c: u128,
}

#[derive(Debug)]
pub struct i128x3 {
    a: i128,
    b: i128,
    c: i128,
}

#[derive(Debug)]
pub struct ISx3 {
    a: isize,
    b: isize,
    c: isize
}

#[derive(Debug)]
pub struct USx3 {
    a: usize,
    b: usize,
    c: usize
}

#[derive(Debug)]
pub struct Bx3 {
    a: bool,
    b: bool,
    c: bool
}

#[derive(Debug)]
pub struct F32x3 {
    a: f32,
    b: f32,
    c: f32
}

#[derive(Debug)]
pub struct F64x3 {
    a: f64,
    b: f64,
    c: f64
}



// impls

impl u1 {
    pub fn new(a: u8) -> u1 { u1 { a } }
}

impl i1 {
    pub fn new(a: i8) -> i1 { i1 { a } }
}

impl u2 {
    pub fn new(a: u8) -> u2 { u2 { a } }
}

impl i2 {
    pub fn new(a: i8) -> i2 { i2 { a }}
}

impl u4 {
    pub fn new(a: u8) -> u4 { u4 { a } }
}

impl i4 {
    pub fn new(a: i8) -> i4 { i4 { a } }
}


// -12
impl u12 {
    pub fn new(a: u8, b: u8) -> u12 {
        u12 { a, b }
    }
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
    pub fn new(a: i8, b: i8) -> i12 {
        i12 { a, b }
    }

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
    pub fn new(a: u8, b: u8, c: u8) -> u24 {
        u24 { a, b, c }
    }

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
    pub fn new(a: i8, b: i8, c: i8) -> i24 {
        i24 { a, b, c }
    }

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
    pub fn new(a: u16, b: u16) -> u48 {
        u48 { a, b }
    }

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
    pub fn new(a: i16, b: i16) -> i48 {
        i48 { a, b }
    }

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
    pub fn new(a: u32, b: u32) -> u72 {
        u72 { a, b }
    }

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
    pub fn new(a: i32, b: i32) -> i72 {
        i72 { a, b }
    }

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
    pub fn new(a: u64, b: u64) -> u96 {
        u96 { a, b }
    }

    fn get_bytes(&self) -> (u64, u64) {
        (self.a, self.b)
    }
    //pub fn from_bytes(a: u64, b: u64) -> u192 {
        //((a as u192) << 64) | (b as u192)
    //}
    pub fn to_bytes(&self) -> (u64, u64) {
        self.get_bytes()
    }
}

impl i96 {
    pub fn new(a: i64, b: i64) -> i96 {
        i96 { a, b }
    }

    fn get_bytes(&self) -> (i64, i64) {
        (self.a, self.b)
    }
    //pub fn from_bytes(a: i64, b: i64) -> i192 {
        //((a as i192) << 64) | (b as i192)
    //}
    pub fn to_bytes(&self) -> (i64, i64) {
        self.get_bytes()
    }
}

impl u192 {
    pub fn new(a: u128, b: u128) -> u192 {
        u192 { a, b }
    }

    fn get_bytes(&self) -> (u128, u128) {
        (self.a, self.b)
    }
    //pub fn from_bytes(a: u128, b: u128) -> u192 {
    //    ((a as u192) << 128) | (b as u192)
    //}
    pub fn to_bytes(&self) -> (u128, u128) {
        self.get_bytes()
    }
}

impl i192 {
    pub fn new(a: i128, b: i128) -> i192 {
        i192 { a, b }
    }

    fn get_bytes(&self) -> (i128, i128) {
        (self.a, self.b)
    }
    //pub fn from_bytes(a: i128, b: i128) -> i192 {
    //    ((a as i192) << 128) | (b as i192)
    //}
    pub fn to_bytes(&self) -> (i128, i128) {
        self.get_bytes()
    }
}

// -3

impl u3 {
    pub fn new(a: u8) -> u3 { u3 { a } }
}

impl i3 {
    pub fn new(a: i8) -> i3 { i3 { a } }
}

impl u6 {
    pub fn new(a: u8) -> u6 { u6 { a } }
}

impl i6 {
    pub fn new(a: i8) -> i6 { i6 { a } }
}

impl u9 {
    pub fn new(a: u8) -> u9 { u9 { a } }
}

impl i9 {
    pub fn new(a: i8) -> i9 { i9 { a } }
}

impl u18 {
    pub fn new(a: u16, b: u8) -> u18 { u18 { a, b } }
}

impl i18 {
    pub fn new(a: i16, b: i8) -> i18 { i18 { a, b } }
}

impl u27 {
    pub fn new(a: u16, b: u16, c: u8) -> u27 { u27 { a, b, c } }
}

impl i27 {
    pub fn new(a: i16, b: i16, c: i8) -> i27 { i27 { a, b, c } }
}

impl u36 {
    pub fn new(a: u16, b: u16, c: u16, d: u8) -> u36 { u36 { a, b, c, d } }
}

impl i36 {
    pub fn new(a: i16, b: i16, c: i16, d: i8) -> i36 { i36 { a, b, c, d } }
}