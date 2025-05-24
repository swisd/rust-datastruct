// datastruct

use std::process::ExitCode;

const CLK: u32 = 1000000;
const PI: f64 = std::f64::consts::PI;
const E: f64 = std::f64::consts::E;
const TAU: f64 = 6.2831853071795864769252867665590057683943387987502116419498891846156328125724179972560696506842341359;
const NAN: f64 = 0.0 / 0.0;
const INF: f64 = 1.0 / 0.0;
const NULL: u8 = 0x00;
const TRUE: bool = true;
const FALSE: bool = false;

const BYTES: usize = 64;

const ELLIPSIS: &str = "...";
const EMPTY: &str = "";
const SPACE: &str = " ";
const TAB: &str = "\t";
const NEWLINE: &str = "\n";
const CR: &str = "\r";
const LF: &str = "\n";
const CRLF: &str = "\r\n";
const NULL_CHAR: &str = "\0";
const NULL_BYTE: &str = "\0";
const NULL_WORD: &str = "\0\0";
const NULL_DWORD: &str = "\0\0\0\0";
const NULL_QWORD: &str = "\0\0\0\0\0\0\0\0";
const NULL_FLOAT: &str = "\0\0\0\0";
const NULL_DOUBLE: &str = "\0\0\0\0\0\0\0\0";
const NULL_BOOL: &str = "\0";

type BYTE = u8;
type WORD = u16;
type DWORD = u32;
type QWORD = u64;
type DOUBLE = f64;
type FLOAT = f32;
type BOOL = bool;
type CHAR = char;
type SHORT = i16;
type INT = i32;
type LONG = i64;
type UCHAR = u8;
type USHORT = u16;
type UINT = u32;
type ULONG = u64;
type SCHAR = i8;
type ProgramCall = u8;
type Addr8 = u8;
type Addr16 = u16;
type Addr32 = u32;
type Addr64 = u64;
type NIBBLE = u8;
type BYTEARRAY = [u8; 16];
type WORDARRAY = [u16; 8];
type DWORDARRAY = [u32; 4];
type QWORDARRAY = [u64; 2];
type FLOATARRAY = [f32; 4];
type DOUBLEARRAY = [f64; 2];
type BOOLARRAY = [bool; 8];
type CHARARRAY = [char; 16];
type SHORTARRAY = [i16; 8];
type INTARRAY = [i32; 4];
type LONGARRAY = [i64; 2];
type UCHARARRAY = [u8; 16];
type USHORTARRAY = [u16; 8];
type UINTARRAY = [u32; 4];
type ULONGARRAY = [u64; 2];
type SCHARARRAY = [i8; 16];
type BYTEARRAY2 = [u8; 32];
type WORDARRAY2 = [u16; 16];
type DWORDARRAY2 = [u32; 8];
type QWORDARRAY2 = [u64; 4];
type FLOATARRAY2 = [f32; 8];
type DOUBLEARRAY2 = [f64; 4];
type BOOLARRAY2 = [bool; 16];
type CHARARRAY2 = [char; 32];
type SHORTARRAY2 = [i16; 16];
type INTARRAY2 = [i32; 8];
type LONGARRAY2 = [i64; 4];
type INT2 = [i32; 2];
type INT4 = [i32; 4];
type INT8 = [i32; 8];
type INT16 = [i32; 16];
type INT32 = [i32; 32];
type INT64 = [i32; 64];
type INT128 = [i32; 128];
type UINT2 = [u32; 2];
type UINT4 = [u32; 4];
type UINT8 = [u32; 8];
type UINT16 = [u32; 16];
type UINT32 = [u32; 32];
type UINT64 = [u32; 64];
type UINT128 = [u32; 128];
type FLOAT2 = [f32; 2];
type FLOAT4 = [f32; 4];
type FLOAT8 = [f32; 8];
type FLOAT16 = [f32; 16];
type FLOAT32 = [f32; 32];
type FLOAT64 = [f32; 64];
type FLOAT128 = [f32; 128];
type DOUBLE2 = [f64; 2];
type DOUBLE4 = [f64; 4];
type DOUBLE8 = [f64; 8];
type DOUBLE16 = [f64; 16];
type DOUBLE32 = [f64; 32];
type DOUBLE64 = [f64; 64];
type DOUBLE128 = [f64; 128];
type BOOL2 = [bool; 2];
type BOOL4 = [bool; 4];
type BOOL8 = [bool; 8];
type BOOL16 = [bool; 16];
type BOOL32 = [bool; 32];
type BOOL64 = [bool; 64];
type BOOL128 = [bool; 128];
type CHAR2 = [char; 2];
type CHAR4 = [char; 4];
type CHAR8 = [char; 8];
type CHAR16 = [char; 16];
type CHAR32 = [char; 32];
type SHORT2 = [i16; 2];
type SHORT4 = [i16; 4];
type SHORT8 = [i16; 8];
type SHORT16 = [i16; 16];
type INT2ARRAY2 = [i32; 4];
type INT4ARRAY2 = [i32; 8];
type INT8ARRAY2 = [i32; 16];
type INT16ARRAY2 = [i32; 32];
type INT32ARRAY2 = [i32; 64];
type INT64ARRAY2 = [i32; 128];
type INT128ARRAY2 = [i32; 256];
type UINT2ARRAY2 = [u32; 4];
type UINT4ARRAY2 = [u32; 8];
type UINT8ARRAY2 = [u32; 16];
type UINT16ARRAY2 = [u32; 32];
type UINT32ARRAY2 = [u32; 64];
type UINT64ARRAY2 = [u32; 128];
type UINT128ARRAY2 = [u32; 256];
type FLOAT2ARRAY2 = [f32; 4];
type FLOAT4ARRAY2 = [f32; 8];
type FLOAT8ARRAY2 = [f32; 16];
type FLOAT16ARRAY2 = [f32; 32];
type FLOAT32ARRAY2 = [f32; 64];
type FLOAT64ARRAY2 = [f32; 128];
type FLOAT128ARRAY2 = [f32; 256];
type DOUBLE2ARRAY2 = [f64; 4];
type DOUBLE4ARRAY2 = [f64; 8];
type DOUBLE8ARRAY2 = [f64; 16];
type DOUBLE16ARRAY2 = [f64; 32];
type DOUBLE32ARRAY2 = [f64; 64];
type DOUBLE64ARRAY2 = [f64; 128];
type DOUBLE128ARRAY2 = [f64; 256];
type BOOL2ARRAY2 = [bool; 4];
type BOOL4ARRAY2 = [bool; 8];
type BOOL8ARRAY2 = [bool; 16];
type BOOL16ARRAY2 = [bool; 32];
type BOOL32ARRAY2 = [bool; 64];
type BOOL64ARRAY2 = [bool; 128];
type BOOL128ARRAY2 = [bool; 256];
type CHAR2ARRAY2 = [char; 4];
type CHAR4ARRAY2 = [char; 8];
type CHAR8ARRAY2 = [char; 16];
type CHAR16ARRAY2 = [char; 32];
type SHORT2ARRAY2 = [i16; 4];
type SHORT4ARRAY2 = [i16; 8];
type SHORT8ARRAY2 = [i16; 16];
type SHORT16ARRAY2 = [i16; 32];


// Time

static TID: u32 = 0;
static TID_MAX: u32 = 0xFFFFFFFF;
static TID_MIN: u32 = 0;
static ABSTIME: u64 = 0;
static RELTIME: u64 = 0;
static TIME: u64 = 0;
static TINTERVAL: u64 = 0;

// Network

type IPV4 = u32;
type IPV6 = u64;
type PORT = u16;
type IPADDR = String;
type MACADDR = String;

static IPV4_: IPV4 = 0;
static IPV6_: IPV6 = 0;
static IPV4_MAX: IPV4 = 0xFFFFFFFF;
static IPV6_MAX: IPV6 = 0xFFFFFFFFFFFFFFFF;
static IPV4_MIN: IPV4 = 0;
static IPV6_MIN: IPV6 = 0;
static PORT_: PORT = 0;
static PORT_MAX: PORT = 0xFFFF;

static HOST: String = String::new();
static IPADDR: String = String::new();
static MACADDR: String = String::new();

static LOCALHOST: String = String::new();
static LOCALHOST_IPV4: IPV4 = 0x7F000001; // should be 127.0.0.1
static LOCALHOST_IPV6: IPV6 = 1; // should be ::1: or 0:0:0:0:0:0:0:1
static LOCALHOST_PORT: PORT = 80; // default is 80

// PC

trait Peripheral
{
    fn doIO(&mut self, addr: Addr64, val: u16) -> u16;
    fn doHighIO(&mut self, addr: Addr64, val: u16) -> u16;
}

struct RIG {
    mem: [Addr64; 0xF000000000000000],
    debugflags: INT,
    slots: [Option<Box<dyn Peripheral>>; 16],
    nreads: u16, // counts # of reads for noise() fn
}

// Bytecode

type Interrrupt = u16;



// GL stuff

type PtrDiff_t = i32;
type Enum = DWORD;
type Boolean = UCHAR;


static COLOR_BUFFER_BIT: Enum = 0x00004000;
static DEPTH_BUFFER_BIT: Enum = 0x00000100;
static STENCIL_BUFFER_BIT: Enum = 0x00000400;
static COLOR_BUFFER_BIT_MASK: Enum = 0x00004000;
static DEPTH_BUFFER_BIT_MASK: Enum = 0x00000100;
static STENCIL_BUFFER_BIT_MASK: Enum = 0x00000400;
static COLOR_CLEAR_VALUE: Enum = 0x00000000;
static DEPTH_CLEAR_VALUE: f64 = 1.0;
static STENCIL_CLEAR_VALUE: Enum = 0;
static COLOR_WRITE_MASK: Enum = 0x0000000F;
static POINTS: Enum = 0x0000;
static LINES: Enum = 0x0001;
static LINE_LOOP: Enum = 0x0002;
static LINE_STRIP: Enum = 0x0003;
static TRIANGLES: Enum = 0x0004;
static TRIANGLE_STRIP: Enum = 0x0005;
static TRIANGLE_FAN: Enum = 0x0006;
static NEVER: Enum = 0x0200;
static LESS: Enum = 0x0201;
static EQUAL: Enum = 0x0202;
static LEQUAL: Enum = 0x0203;
static GREATER: Enum = 0x0204;
static NOTEQUAL: Enum = 0x0205;
static GEQUAL: Enum = 0x0206;
static ALWAYS: Enum = 0x0207;
static SRC_ALPHA: Enum = 0x0302;
static QUADS: Enum = 0x0007;
static QUAD_STRIP: Enum = 0x0008;
static POLYGON: Enum = 0x0009;
static FRONT: Enum = 0x0404;
static BACK: Enum = 0x0405;
static FRONT_AND_BACK: Enum = 0x0408;
static CULL_FACE: Enum = 0x0B44;
static BLEND: Enum = 0x0BE2;
static DITHER: Enum = 0x0BD0;
static STENCIL_TEST: Enum = 0x0B90;
static DEPTH_TEST: Enum = 0x0B71;
static ZERO: f64 = 0.0;
static ONE: f64 = 1.0;
static SRC_COLOR: Enum = 0x0300;
static BMP: Enum = 0x1A00;

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


// Types

type Pointer = *mut u8;
type Size = usize;
type Index = isize;
type Char = char;
type Bool = bool;
type Float = f64;
type Int = i64;
type UInt = u64;
type Str = String;
type Vect = Vec<u8>;


// structures

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

pub (crate) struct Warning {
    code: i32,
    message: String,
}

struct Object {
    name: String,
    data: String,
    loc: Addr16,
    size: Size,
    type_: Enum,
    parent: Option<Box<Object>>,
    children: Vec<Box<Object>>,
    next: Option<Box<Object>>,
    prev: Option<Box<Object>>,
    first: Option<Box<Object>>,
    last: Option<Box<Object>>,
    flags: Enum,
}

struct Param {
    name: String,
    data: Object,
}

struct Scene {
    name: String,
    params: Vec<Param>,
}

struct Polygon {
    points: Vec<(f64, f64)>,
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


// Errors
impl Error {
    pub(crate) fn new(code: i32, message: String) -> Error {
        Error { code: code, message: message }
    }
    pub(crate) fn print(&self) {
        println!("Error: {} :: {}", self.code, self.message);
    }
    pub fn error(&self) {
        println!("Error: {} :: {}", self.code, self.message);
    }
}

// Error types
type Exception = Error;
type ArithmeticError = Error;
type AssertionError = Error;
type AttributeError = Error;
type WindowsError = Error;
type OSError = Error;
type IOError = Error;
type EnvironmentError = Error;
type BlockingIOError = Error;
type ConnectionError = Error;
type BrokenPipeError = Error;
type BufferError = Error;
type ChildProcessError = Error;
type ConnectionAbortedError = Error;
type ConnectionRefusedError = Error;
type ConnectionResetError = Error;
type EOFError = Error;
type FileExistsError = Error;
type FileNotFoundError = Error;
type FloatingPointError = ArithmeticError;
type SyntaxError = Error;
type LookupError = Error;
type IndexError = LookupError;
type InterruptedError = OSError;
type IsADirectoryError = OSError;
type KeyError = LookupError;
type MemoryError = Error;
type NameError = Error;
type NotADirectoryError = OSError;
type RuntimeError = Error;
type NotImplementedError = RuntimeError;
type OverflowError = ArithmeticError;
type PermissionError = OSError;
type ProcessLookupError = OSError;
type RecursionError = Error;
type ReferenceError = Error;
type SystemError = Error;
type TabError = SyntaxError;
type TimeoutError = OSError;
type TypeError = Error;
type UnboundLocalError = NameError;
type ValueError = Error;
type UnicodeError = ValueError;
type UnicodeDecodeError = UnicodeError;
type UnicodeEncodeError = UnicodeError;
type UnicodeTranslateError = UnicodeError;
type ZeroDivisionError = ArithmeticError;



type KeyboardInterrupt = Interrrupt;


// Warning

impl Warning {
    fn new(code: i32, message: String) -> Warning {
        Warning { code: code, message: message }
    }
    fn print(&self) {
        println!("Warning: {} :: {}", self.code, self.message);
    }
    fn warning(&self) {
        println!("Warning: {} :: {}", self.code, self.message);
    }
}

type Warning_ = Warning;
type BytesWarning = Warning;
type DeprecationWarning = Warning;
type EncodingWarning = Warning;
type FutureWarning = Warning;
type ResourceWarning = Warning;
type RuntimeWarning = Warning;
type SyntaxWarning = Warning;
type UnicodeWarning = Warning;
type UserWarning = Warning;



// More stuff

impl Object {
    fn new(name: String, data: String, loc: Addr16, size: Size, type_: Enum, parent: Option<Box<Object>>, children: Vec<Box<Object>>, next: Option<Box<Object>>, prev: Option<Box<Object>>, first: Option<Box<Object>>, last: Option<Box<Object>>, flags: Enum) -> Object {
        Object { name: name, data: data, loc: loc, size: size, type_: type_, parent: parent, children: children, next: next, prev: prev, first: first, last: last, flags: flags }
    }
    fn repr(&self) {
        println!("<Object {} at {}", self.name, self.loc);
    }
}

impl Param {
    fn new(name: String, data: Object) -> Param {
        Param { name: name, data: data }
    }
}

impl Scene {
    fn new(name: String, params: Vec<Param>) -> Scene {
        Scene { name: name, params: params }
    }
}

impl Polygon {
    fn new(points: Vec<(f64, f64)>) -> Polygon {
        Polygon { points: points }
    }

    fn num_points(&self) -> usize {
        self.points.len()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }

    fn remove_point(&mut self, index: usize) {
        self.points.remove(index);
    }
}

// Extra classes and stuff

