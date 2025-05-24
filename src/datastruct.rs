// datastruct

const CLK: u32 = 1000000;
const PI: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;
const E: f64 = 2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274;
const TAU: f64 = 6.2831853071795864769252867665590057683943387987502116419498891846156328125724179972560696506842341359;
const NAN: f64 = 0.0 / 0.0;
const INF: f64 = 1.0 / 0.0;


// Zero all int and float types

const INT_I8_0: i8 = 0;
const INT_I16_0: i16 = 0;
const INT_I32_0: i32 = 0;
const INT_I64_0: i64 = 0;
const INT_I128_0: i128 = 0;
const INT_U8_0: u8 = 0;
const INT_U16_0: u16 = 0;
const INT_U32_0: u32 = 0;
const INT_U64_0: u64 = 0;
const INT_U128_0: u128 = 0;
const FLOAT_F32_0: f32 = 0.0;
const FLOAT_F64_0: f64 = 0.0;
const ISIZE_ISIZE_0: isize = 0;
const ISIZE_USIZE_0: usize = 0;


// Empty string and other data types

const STR_0: &str = "";
const BOOL_0: bool = false;
const BOOL_1: bool = true; // Will have it there anyway
const CHAR_0: char = '\0';


const HEX_0: u8 = 0x00;
const HEX_255: u8 = 0xFF;


const U8_BYTE_42: u8 = b'*';


const UTF8_0: char = '\u{0}';
const UTF8_10000: char = '\u{10000}';
const UTF8_10FFFF: char = '\u{10FFFF}';

// MAX and MIN of all types

const INT_I8_MAX: i8 = i8::MAX;
const INT_I16_MAX: i16 = i16::MAX;
const INT_I32_MAX: i32 = i32::MAX;
const INT_I64_MAX: i64 = i64::MAX;
const INT_I128_MAX: i128 = i128::MAX;
const INT_U8_MAX: u8 = u8::MAX;
const INT_U16_MAX: u16 = u16::MAX;
const INT_U32_MAX: u32 = u32::MAX;
const INT_U64_MAX: u64 = u64::MAX;
const INT_U128_MAX: u128 = u128::MAX;
const FLOAT_F32_MAX: f32 = f32::MAX;
const FLOAT_F64_MAX: f64 = f64::MAX;
const ISIZE_ISIZE_MAX: isize = isize::MAX;
const USIZE_USIZE_MAX: usize = usize::MAX;

const INT_I8_MIN: i8 = i8::MIN;
const INT_I16_MIN: i16 = i16::MIN;
const INT_I32_MIN: i32 = i32::MIN;
const INT_I64_MIN: i64 = i64::MIN;
const INT_I128_MIN: i128 = i128::MIN;
const INT_U8_MIN: u8 = u8::MIN;
const INT_U16_MIN: u16 = u16::MIN;
const INT_U32_MIN: u32 = u32::MIN;
const INT_U64_MIN: u64 = u64::MIN;
const INT_U128_MIN: u128 = u128::MIN;
const FLOAT_F32_MIN: f32 = f32::MIN;
const FLOAT_F64_MIN: f64 = f64::MIN;
const ISIZE_ISIZE_MIN: isize = isize::MIN;
const USIZE_USIZE_MIN: usize = usize::MIN;

fn defs() {
    let mut idx: i32 = 0;
    let mut ln: i32 = 0;
    let mut col: i32 = 0;
    let mut data: Vec<u8> = Vec::new();
    let mut data_len: usize = 0;
    let mut data_cap: usize = 0;
    let mut data_ptr: *mut u8 = std::ptr::null_mut();
    let mut data_slice: &[u8] = &[];
    let mut char0: char = '*';
    let mut error: bool = false;
    let mut error_code: i32 = 0;
    let mut error_message: String = String::new();
    let mut error_message_len: usize = 0;
    let mut error_message_cap: usize = 0;
    let mut error_message_ptr: *mut u8 = std::ptr::null_mut();
    let mut error_message_slice: &[u8] = &[];
    let mut filepath: String = String::new();
    let mut filepath_len: usize = 0;
    let mut filepath_cap: usize = 0;
    let mut filepath_ptr: *mut u8 = std::ptr::null_mut();
    let mut filepath_slice: &[u8] = &[];
    let mut frame: String = String::new();
    let mut frame_len: usize = 0;
    let mut frame_cap: usize = 0;
    let mut frame_ptr: *mut u8 = std::ptr::null_mut();
    let mut frame_slice: &[u8] = &[];
}










struct Point {
    x: f64,
    y: f64,
}

struct Rect {
    p1: Point,
    p2: Point,
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

struct Vec1 {
    x: f64,
}

struct Vec2 {
    x: f64,
    y: f64,
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

struct Uniform {
    pos: Vec2,
    scale: Vec2,
    rot: f64,
}
struct Linear {
    pos: Vec2,
}

struct Vector {
    pos: Vec2,
    scale: Vec2,
    rot: f64,
}

struct Matrix {
    pos: Vec2,
    scale: Vec2,
}

struct Scalar {
    pos: Vec2,
    scale: Vec2,
}

struct Mat2 {
    m: [[f64; 2]; 2],
}

struct Mat3 {
    m: [[f64; 3]; 3],
}

struct Mat4 {
    m: [[f64; 4]; 4],
}

struct Frame {
    frame: String,
    pos: Vec2,
    width: f64,
    height: f64,
    time: f64,
    data: Vec<u8>,
    mat: Mat4,
    rect: Rect,
    pair: Pair,
}

pub(crate) struct Error {
    code: i32,
    message: String,
}









// Implementation block
impl Point {
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }

    fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Rect {
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        // `abs` is a `f64` method that returns the absolute value of the caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    
    fn new(first: i32, second: i32) -> Pair {
        Pair(Box::new(first), Box::new(second))
    }
    
    fn sum(&self) -> i32 {
        *self.0 + *self.1
    }
    
    fn product(&self) -> i32 {
        *self.0 * *self.1
    }
        
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}



impl Uniform {
    fn new(pos: Vec2, scale: Vec2, rot: f64) -> Uniform {
        Uniform { pos: pos, scale: scale, rot: rot }
    }
}

impl Linear {
    fn new(pos: Vec2) -> Linear {
        Linear { pos: pos }
    }
}

impl Vector {
    fn new(pos: Vec2, scale: Vec2, rot: f64) -> Vector {
        Vector { pos: pos, scale: scale, rot: rot }
    }
}

impl Matrix {
    fn new(pos: Vec2, scale: Vec2) -> Matrix {
        Matrix { pos: pos, scale: scale }
    }
}

impl Scalar {
    fn new(pos: Vec2, scale: Vec2) -> Scalar {
        Scalar { pos: pos, scale: scale }
    }
}

impl Vec1 {
    fn new(x: f64) -> Vec1 {
        Vec1 { x: x }
    }
    fn distance(&self, other: &Vec1) -> f64 {
        ((self.x - other.x) * (self.x - other.x)).sqrt()
    }
    fn transform(&self, mat: &Mat4) -> () {}
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    fn distance(&self, other: &Vec2) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }
    fn transform(&self, mat: &Mat4) -> () {
        let Vec2 { x, y } = *self;
        let Mat4 { m } = *mat;
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    fn distance(&self, other: &Vec3) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y) + (self.z - other.z) * (self.z - other.z)).sqrt()
    }
    fn transform(&self, mat: &Mat4) -> () {
        let Vec3 { x, y, z } = *self;
        let Mat4 { m } = *mat;
    }
}

impl Vec4 {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 { x: x, y: y, z: z, w: w }
    }
    fn distance(&self, other: &Vec4) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y) + (self.z - other.z) * (self.z - other.z) + (self.w - other.w) * (self.w - other.w)).sqrt()
    }
    fn transform(&self, mat: &Mat4) -> () {
        let Vec4 { x, y, z, w } = *self;
        let Mat4 { m } = *mat;
    }
}

impl Mat2 {
    fn identity() -> Mat2 {
        Mat2 { m: [[1.0, 0.0], [0.0, 1.0]] }
    }
    fn new(m: [[f64; 2]; 2]) -> Mat2 {
        Mat2 { m: m }
    }
    fn shift(&self, x: f64, y: f64) -> Mat2 {
        Mat2 { m: [[1.0, 0.0], [0.0, 1.0]] }
    }
}

impl Mat3 {
    fn identity() -> Mat3 {
        Mat3 { m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] }
    }
    fn new(m: [[f64; 3]; 3]) -> Mat3 {
        Mat3 { m: m }
    }
    fn shift(&self, x: f64, y: f64, z: f64) -> Mat3 {
        Mat3 { m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] }
    }
}

impl Mat4 {
    fn identity() -> Mat4 {
        Mat4 { m: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }
    fn new(m: [[f64; 4]; 4]) -> Mat4 {
        Mat4 { m: m }
    }
    fn shift(&self, x: f64, y: f64, z: f64, w: f64) -> Mat4 {
        Mat4 { m: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }
}

impl Frame {
    fn new(frame: String, pos: Vec2, width: f64, height: f64, time: f64, data: Vec<u8>, mat: Mat4, rect: Rect, pair: Pair) -> Frame {
        Frame { frame: frame, pos: pos, width: width, height: height, time: time, data: data, mat: mat, rect: rect, pair: pair }
    }
    fn append(&mut self, mut frame: Frame) {
        self.data.append(&mut frame.data);
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn destroy(self, frame: Frame) {
        println!("Destroying Frame {}", frame.frame);
    }
}

impl Error {
    pub(crate) fn new(code: i32, message: String) -> Error {
        Error { code: code, message: message }
    }
    pub(crate) fn print(&self) {
        println!("Error: {} :: {}", self.code, self.message);
    }
}

fn main(){
    Error::print(&Error::new(255, "Unspecified Error".parse().unwrap()))
}
