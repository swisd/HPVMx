#![allow(unsafe_code, dead_code, non_camel_case_types, non_snake_case, unused)]



// Types

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::f64::math::sqrt;
use core::sync::atomic::Atomic;
use crate::message;

pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type QWORD = u64;
pub type DOUBLE = f64;
pub type FLOAT = f32;
pub type BOOL = bool;
pub type CHAR = char;
pub type SHORT = i16;
pub type INT = i32;
pub type LONG = i64;
pub type UCHAR = u8;
pub type USHORT = u16;
pub type UINT = u32;
pub type ULONG = u64;
pub type SCHAR = i8;
pub type ProgramCall = u8;
pub type ADDR8 = u8;
pub type ADDR16 = u16;
pub type ADDR32 = u32;
pub type ADDR64 = u64;
pub type POINTER = *mut u8;
pub type SIZE = usize;
pub type INDEX = isize;
pub type STR = String;
pub type VECTOR = Vec<u8>;
pub type VECTOR2 = Vec<u16>;
pub type VECTOR4 = Vec<u32>;
pub type VECTOR8 = Vec<u64>;
pub type VECTOR16 = Vec<u128>;
pub type VECTOR32 = Vec<u32>;
pub type VECTOR64 = Vec<u64>;
pub type VECTOR128 = Vec<u128>;
pub type ARRAY = [u8; 16];
pub type ARRAY2 = [u16; 8];
pub type ARRAY4 = [u32; 4];
pub type ARRAY8 = [u64; 2];
pub type ARRAY16 = [u128; 1];
pub type ARRAY32 = [u32; 2];
pub type ARRAY64 = [u64; 4];
pub type ARRAY128 = [u128; 2];
pub type ARRAY256 = [u128; 4];
pub type ARRAY512 = [u128; 8];
// pub // type SET = HashSet<u8>;
// pub // type SET2 = HashSet<u16>;
// pub // type SET4 = HashSet<u32>;
// pub // type SET8 = HashSet<u64>;
// pub // type SET16 = HashSet<u128>;
// pub // type SET32 = HashSet<u32>;
// pub // type SET64 = HashSet<u64>;
// pub // type SET128 = HashSet<u128>;
// pub // type SET256 = HashSet<u128>;
// pub // type SET512 = HashSet<u128>;
pub type TUPLE = (u8, u16, u32, u64, u128);
pub type TUPLE2 = (u16, u32, u64, u128);
pub type TUPLE4 = (u32, u64, u128);
pub type TUPLE8 = (u64, u128);
pub type TUPLE16 = (u128,);
pub type TUPLE32 = (u32, u64, u128);
pub type TUPLE64 = (u64, u128);
pub type TUPLE128 = (u128, u128);
pub type TUPLE256 = (u128, u128, u128);
pub type TUPLE512 = (u128, u128, u128, u128);
pub type HWADDR = [u8; 6];
pub type MEMADDR = [u8; 16];
pub type PCIEADDR = [u8; 16];
pub type VADDR = [u8; 16]; // Virtual address
pub type PADDR = [u8; 16]; // Physical address
pub type ADDR128 = u128;
pub type ADDR256 = u128;
pub type ADDR512 = u128;
pub type ADDR1024 = u128;
pub type ADDR2048 = u128;
pub type ADDR4096 = u128;
pub type ADDR8192 = u128;
pub type ENUM = u32;
pub type BIT = u8;
pub type BIT2 = u8;
pub type NIBBLE = u8;
pub type BYTEARRAY = [u8; 16];
pub type WORDARRAY = [u16; 8];
pub type DWORDARRAY = [u32; 4];
pub type QWORDARRAY = [u64; 2];
pub type FLOATARRAY = [f32; 4];
pub type DOUBLEARRAY = [f64; 2];
pub type BOOLARRAY = [bool; 8];
pub type CHARARRAY = [char; 16];
pub type SHORTARRAY = [i16; 8];
pub type INTARRAY = [i32; 4];
pub type LONGARRAY = [i64; 2];
pub type UCHARARRAY = [u8; 16];
pub type USHORTARRAY = [u16; 8];
pub type UINTARRAY = [u32; 4];
pub type ULONGARRAY = [u64; 2];
pub type SCHARARRAY = [i8; 16];
pub type BYTEARRAY2 = [u8; 32];
pub type WORDARRAY2 = [u16; 16];
pub type DWORDARRAY2 = [u32; 8];
pub type QWORDARRAY2 = [u64; 4];
pub type FLOATARRAY2 = [f32; 8];
pub type DOUBLEARRAY2 = [f64; 4];
pub type BOOLARRAY2 = [bool; 16];
pub type CHARARRAY2 = [char; 32];
pub type SHORTARRAY2 = [i16; 16];
pub type INTARRAY2 = [i32; 8];
pub type LONGARRAY2 = [i64; 4];
pub type INT2 = [i32; 2];
pub type INT4 = [i32; 4];
pub type INT8 = [i32; 8];
pub type INT16 = [i32; 16];
pub type INT32 = [i32; 32];
pub type INT64 = [i32; 64];
pub type INT128 = [i32; 128];
pub type UINT2 = [u32; 2];
pub type UINT4 = [u32; 4];
pub type UINT8 = [u32; 8];
pub type UINT16 = [u32; 16];
pub type UINT32 = [u32; 32];
pub type UINT64 = [u32; 64];
pub type UINT128 = [u32; 128];
pub type FLOAT2 = [f32; 2];
pub type FLOAT4 = [f32; 4];
pub type FLOAT8 = [f32; 8];
pub type FLOAT16 = [f32; 16];
pub type FLOAT32 = [f32; 32];
pub type FLOAT64 = [f32; 64];
pub type FLOAT128 = [f32; 128];
pub type DOUBLE2 = [f64; 2];
pub type DOUBLE4 = [f64; 4];
pub type DOUBLE8 = [f64; 8];
pub type DOUBLE16 = [f64; 16];
pub type DOUBLE32 = [f64; 32];
pub type DOUBLE64 = [f64; 64];
pub type DOUBLE128 = [f64; 128];
pub type BOOL2 = [bool; 2];
pub type BOOL4 = [bool; 4];
pub type BOOL8 = [bool; 8];
pub type BOOL16 = [bool; 16];
pub type BOOL32 = [bool; 32];
pub type BOOL64 = [bool; 64];
pub type BOOL128 = [bool; 128];
pub type CHAR2 = [char; 2];
pub type CHAR4 = [char; 4];
pub type CHAR8 = [char; 8];
pub type CHAR16 = [char; 16];
pub type CHAR32 = [char; 32];
pub type SHORT2 = [i16; 2];
pub type SHORT4 = [i16; 4];
pub type SHORT8 = [i16; 8];
pub type SHORT16 = [i16; 16];
pub type INT2ARRAY2 = [i32; 4];
pub type INT4ARRAY2 = [i32; 8];
pub type INT8ARRAY2 = [i32; 16];
pub type INT16ARRAY2 = [i32; 32];
pub type INT32ARRAY2 = [i32; 64];
pub type INT64ARRAY2 = [i32; 128];
pub type INT128ARRAY2 = [i32; 256];
pub type UINT2ARRAY2 = [u32; 4];
pub type UINT4ARRAY2 = [u32; 8];
pub type UINT8ARRAY2 = [u32; 16];
pub type UINT16ARRAY2 = [u32; 32];
pub type UINT32ARRAY2 = [u32; 64];
pub type UINT64ARRAY2 = [u32; 128];
pub type UINT128ARRAY2 = [u32; 256];
pub type FLOAT2ARRAY2 = [f32; 4];
pub type FLOAT4ARRAY2 = [f32; 8];
pub type FLOAT8ARRAY2 = [f32; 16];
pub type FLOAT16ARRAY2 = [f32; 32];
pub type FLOAT32ARRAY2 = [f32; 64];
pub type FLOAT64ARRAY2 = [f32; 128];
pub type FLOAT128ARRAY2 = [f32; 256];
pub type DOUBLE2ARRAY2 = [f64; 4];
pub type DOUBLE4ARRAY2 = [f64; 8];
pub type DOUBLE8ARRAY2 = [f64; 16];
pub type DOUBLE16ARRAY2 = [f64; 32];
pub type DOUBLE32ARRAY2 = [f64; 64];
pub type DOUBLE64ARRAY2 = [f64; 128];
pub type DOUBLE128ARRAY2 = [f64; 256];
pub type BOOL2ARRAY2 = [bool; 4];
pub type BOOL4ARRAY2 = [bool; 8];
pub type BOOL8ARRAY2 = [bool; 16];
pub type BOOL16ARRAY2 = [bool; 32];
pub type BOOL32ARRAY2 = [bool; 64];
pub type BOOL64ARRAY2 = [bool; 128];
pub type BOOL128ARRAY2 = [bool; 256];
pub type CHAR2ARRAY2 = [char; 4];
pub type CHAR4ARRAY2 = [char; 8];
pub type CHAR8ARRAY2 = [char; 16];
pub type CHAR16ARRAY2 = [char; 32];
pub type SHORT2ARRAY2 = [i16; 4];
pub type SHORT4ARRAY2 = [i16; 8];
pub type SHORT8ARRAY2 = [i16; 16];
pub type SHORT16ARRAY2 = [i16; 32];



// Time

pub static TID: u32 = 0;
pub static TID_MAX: u32 = 0xFFFFFFFF;
pub static TID_MIN: u32 = 0;
pub static ABSTIME: u64 = 0;
pub static RELTIME: u64 = 0;
pub static TIME: u64 = 0;
pub static TINTERVAL: u64 = 0;

// Network

pub type IPV4 = u32;
pub type IPV6 = u64;
pub type PORT = u16;
pub type IPADDR = String;
pub type MACADDR = String;

pub static IPV4_: IPV4 = 0;
pub static IPV6_: IPV6 = 0;
pub static IPV4_MAX: IPV4 = 0xFFFFFFFF;
pub static IPV6_MAX: IPV6 = 0xFFFFFFFFFFFFFFFF;
pub static IPV4_MIN: IPV4 = 0;
pub static IPV6_MIN: IPV6 = 0;
pub static PORT_: PORT = 0;
pub static PORT_MAX: PORT = 0xFFFF;

pub static HOST: String = String::new();
pub static IPADDR: String = String::new();
pub static MACADDR: String = String::new();

pub static LOCALHOST: String = String::new();
pub static LOCALHOST_IPV4: IPV4 = 0x7F000001; // should be 127.0.0.1
pub static LOCALHOST_IPV6: IPV6 = 1; // should be ::1: or 0:0:0:0:0:0:0:1
pub static LOCALHOST_PORT: PORT = 80; // default is 80

// PC

pub trait Peripheral
{
    fn doIO(&mut self, addr: ADDR64, val: u16) -> u16;
    fn doHighIO(&mut self, addr: ADDR64, val: u16) -> u16;
}

pub struct RIG {
    mem: [ADDR64; 0xF000000000000000],
    debugflags: INT,
    slots: [Option<Box<dyn Peripheral>>; 16],
    nreads: u16, // counts # of reads for noise() fn
}

// Bytecode

pub type Interrrupt = u16;
pub type Buffer = u16;
pub type Poly = u16;
pub type DMI = u8;

// GL stuff

pub type PtrDiffT = i32;
pub type Enum = DWORD;
pub type Boolean = UCHAR;


pub static COLOR_BUFFER_BIT: Enum = 0x00004000;
pub static DEPTH_BUFFER_BIT: Enum = 0x00000100;
pub static STENCIL_BUFFER_BIT: Enum = 0x00000400;
pub static COLOR_BUFFER_BIT_MASK: Enum = 0x00004000;
pub static DEPTH_BUFFER_BIT_MASK: Enum = 0x00000100;
pub static STENCIL_BUFFER_BIT_MASK: Enum = 0x00000400;
pub static COLOR_CLEAR_VALUE: Enum = 0x00000000;
pub static DEPTH_CLEAR_VALUE: f64 = 1.0;
pub static STENCIL_CLEAR_VALUE: Enum = 0;
pub static COLOR_WRITE_MASK: Enum = 0x0000000F;
pub static POINTS: Enum = 0x0000;
pub static LINES: Enum = 0x0001;
pub static LINE_LOOP: Enum = 0x0002;
pub static LINE_STRIP: Enum = 0x0003;
pub static TRIANGLES: Enum = 0x0004;
pub static TRIANGLE_STRIP: Enum = 0x0005;
pub static TRIANGLE_FAN: Enum = 0x0006;
pub static NEVER: Enum = 0x0200;
pub static LESS: Enum = 0x0201;
pub static EQUAL: Enum = 0x0202;
pub static LEQUAL: Enum = 0x0203;
pub static GREATER: Enum = 0x0204;
pub static NOTEQUAL: Enum = 0x0205;
pub static GEQUAL: Enum = 0x0206;
pub static ALWAYS: Enum = 0x0207;
pub static SRC_ALPHA: Enum = 0x0302;
pub static QUADS: Enum = 0x0007;
pub static QUAD_STRIP: Enum = 0x0008;
pub static POLYGON: Enum = 0x0009;
pub static FRONT: Enum = 0x0404;
pub static BACK: Enum = 0x0405;
pub static LEFT: Enum = 0x0406;
pub static RIGHT: Enum = 0x0407;
pub static CCW: Enum = 0x0901;
pub static CW: Enum = 0x0900;
pub static LINE_WIDTH: Enum = 0x0B21;
pub static CULL_FACE_MODE: Enum = 0x0B45;
pub static CULL_FACE_MODE_FRONT: Enum = 0x0B46;
pub static CULL_FACE_MODE_BACK: Enum = 0x0B47;
pub static CULL_FACE_MODE_FRONT_AND_BACK: Enum = 0x0B48;
pub static FRONT_AND_BACK: Enum = 0x0408;
pub static CULL_FACE: Enum = 0x0B44;
pub static BLEND: Enum = 0x0BE2;
pub static DITHER: Enum = 0x0BD0;
pub static STENCIL_TEST: Enum = 0x0B90;
pub static DEPTH_TEST: Enum = 0x0B71;
pub static ZERO: f64 = 0.0;
pub static ONE: f64 = 1.0;
pub static SRC_COLOR: Enum = 0x0300;
pub static BMP: Enum = 0x1A00;
pub static BMP_RGB: Enum = 0x1A00;
pub static BMP_RGBA: Enum = 0x1A01;
pub static BMP_INDEXED: Enum = 0x1A02;
pub static BMP_RGB_ALPHA: Enum = 0x1A03;
pub static BMP_RGBA_ALPHA: Enum = 0x1A04;
pub static BMP_LUMINANCE: Enum = 0x1A06;
pub static BMP_LUMINANCE_ALPHA: Enum = 0x1A07;
pub static BMP_ALPHA: Enum = 0x1A08;

// Zero all int and float types

pub const INT_I8_0: i8 = 0;
pub const INT_I16_0: i16 = 0;
pub const INT_I32_0: i32 = 0;
pub const INT_I64_0: i64 = 0;
pub const INT_I128_0: i128 = 0;
pub const INT_U8_0: u8 = 0;
pub const INT_U16_0: u16 = 0;
pub const INT_U32_0: u32 = 0;
pub const INT_U64_0: u64 = 0;
pub const INT_U128_0: u128 = 0;
pub const FLOAT_F32_0: f32 = 0.0;
pub const FLOAT_F64_0: f64 = 0.0;
pub const ISIZE_ISIZE_0: isize = 0;
pub const ISIZE_USIZE_0: usize = 0;


// Empty string and other data types

pub const STR_0: &str = "";
pub const BOOL_0: bool = false;
pub const BOOL_1: bool = true; // Will have it there anyway
pub const CHAR_0: char = '\0';


pub const HEX_0: u8 = 0x00;
pub const HEX_255: u8 = 0xFF;


pub const U8_BYTE_42: u8 = b'*';


pub const UTF8_0: char = '\u{0}';
pub const UTF8_10000: char = '\u{10000}';
pub const UTF8_10FFFF: char = '\u{10FFFF}';

// Maximum value of every tpe defiend in Types

pub const BYTE_MAX: BYTE = 0xFF;
pub const WORD_MAX: WORD = 0xFFFF;
pub const DWORD_MAX: DWORD = 0xFFFFFFFF;
pub const QWORD_MAX: QWORD = 0xFFFFFFFFFFFFFFFF;
pub const DOUBLE_MAX: DOUBLE = DOUBLE::MAX;
pub const FLOAT_MAX: FLOAT = FLOAT::MAX;
pub const BOOL_MAX: BOOL = true;
pub const CHAR_MAX: CHAR = CHAR::MAX;
pub const SHORT_MAX: SHORT = SHORT::MAX;
pub const INT_MAX: INT = INT::MAX;
pub const LONG_MAX: LONG = LONG::MAX;
pub const UCHAR_MAX: UCHAR = UCHAR::MAX;
pub const USHORT_MAX: USHORT = USHORT::MAX;
pub const UINT_MAX: UINT = UINT::MAX;
pub const ULONG_MAX: ULONG = ULONG::MAX;
pub const SCHAR_MAX: SCHAR = SCHAR::MAX;
pub const PROGRAM_CALL_MAX: ProgramCall = ProgramCall::MAX;
pub const ADDR8_MAX: ADDR8 = ADDR8::MAX;
pub const ADDR16_MAX: ADDR16 = ADDR16::MAX;
pub const ADDR32_MAX: ADDR32 = ADDR32::MAX;
pub const ADDR64_MAX: ADDR64 = ADDR64::MAX;
pub const NIBBLE_MAX: NIBBLE = NIBBLE::MAX;


// Minimum value of every tpe defiend in Types
pub const BYTE_MIN: BYTE = 0x00;
pub const WORD_MIN: WORD = 0x0000;
pub const DWORD_MIN: DWORD = 0x00000000;
pub const QWORD_MIN: QWORD = 0x0000000000000000;
pub const DOUBLE_MIN: DOUBLE = DOUBLE::MIN;
pub const FLOAT_MIN: FLOAT = FLOAT::MIN;
pub const BOOL_MIN: BOOL = false;
pub const CHAR_MIN: CHAR = CHAR::MIN;
pub const SHORT_MIN: SHORT = SHORT::MIN;
pub const INT_MIN: INT = INT::MIN;
pub const LONG_MIN: LONG = LONG::MIN;
pub const UCHAR_MIN: UCHAR = UCHAR::MIN;
pub const USHORT_MIN: USHORT = USHORT::MIN;
pub const UINT_MIN: UINT = UINT::MIN;
pub const ULONG_MIN: ULONG = ULONG::MIN;
pub const SCHAR_MIN: SCHAR = SCHAR::MIN;
pub const PROGRAM_CALL_MIN: ProgramCall = ProgramCall::MIN;
pub const ADDR8_MIN: ADDR8 = ADDR8::MIN;
pub const ADDR16_MIN: ADDR16 = ADDR16::MIN;
pub const ADDR32_MIN: ADDR32 = ADDR32::MIN;
pub const ADDR64_MIN: ADDR64 = ADDR64::MIN;
pub const NIBBLE_MIN: NIBBLE = NIBBLE::MIN;



// Definitions Function

pub fn defs() -> (i32, i32, i32, Vec<u8>, usize, usize, &'static [u8], char, bool, i32, String, usize, &'static [u8], usize, String, usize, &'static [u8], usize, String, usize, usize, &'static [u8]) {
    let mut idx: i32 = 0;
    let mut ln: i32 = 0;
    let mut col: i32 = 0;
    let mut data: Vec<u8> = Vec::new();
    let mut data_len: usize = 0;
    let mut data_cap: usize = 0;
    //let mut data_ptr: *mut u8 = std::ptr::null_mut();
    let mut data_slice: &[u8] = &[];
    let mut char0: char = '*';
    let mut error: bool = false;
    let mut error_code: i32 = 0;
    let mut error_message: String = String::new();
    let mut error_message_len: usize = 0;
    let mut error_message_cap: usize = 0;
    //let mut error_message_ptr: *mut u8 = std::ptr::null_mut();
    let mut error_message_slice: &[u8] = &[];
    let mut filepath: String = String::new();
    let mut filepath_len: usize = 0;
    let mut filepath_cap: usize = 0;
    //let mut filepath_ptr: *mut u8 = std::ptr::null_mut();
    let mut filepath_slice: &[u8] = &[];
    let mut frame: String = String::new();
    let mut frame_len: usize = 0;
    let mut frame_cap: usize = 0;
    //let mut frame_ptr: *mut u8 = std::ptr::null_mut();
    let mut frame_slice: &[u8] = &[];
    return (idx, ln, col, data, data_len, data_cap, data_slice, char0, error, error_code, error_message, error_message_len, error_message_slice, error_message_cap, filepath, filepath_len, filepath_slice, filepath_cap, frame, frame_len, frame_cap, frame_slice)
}


// General Types

pub type Pointer = *mut u8;
pub type Size = usize;
pub type Index = isize;
pub type Char = char;
pub type Bool = bool;
pub type Float = f64;
pub type Int = i64;
pub type UInt = u64;
pub type Str = String;
pub type Vect = Vec<u8>;


// structures

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Rect {
    p1: Point,
    p2: Point,
}

// `Pair` owns resources: two heap allocated integers
pub struct Pair(Box<i32>, Box<i32>);

pub struct Vec1 {
    x: f64,
}

pub struct Vec2 {
    x: f64,
    y: f64,
}

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub struct Uniform {
    pos: Vec2,
    scale: Vec2,
    rot: f64,
}
pub struct Linear {
    pos: Vec2,
}

pub struct Vector {
    pos: Vec2,
    scale: Vec2,
    rot: f64,
}

pub struct Matrix {
    pos: Vec2,
    scale: Vec2,
}

pub struct Scalar {
    pos: Vec2,
    scale: Vec2,
}

pub struct Mat2 {
    m: [[f64; 2]; 2],
}

pub struct Mat3 {
    m: [[f64; 3]; 3],
}

pub struct Mat4 {
    m: [[f64; 4]; 4],
}

pub struct Frame {
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

pub struct Error {
    code: i32,
    message: String,
}

pub struct Warning {
    code: i32,
    message: String,
}

pub struct Object {
    name: String,
    data: String,
    loc: ADDR16,
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

pub struct Param {
    name: String,
    data: Object,
}

pub struct Scene {
    name: String,
    params: Vec<Param>,
}

pub struct Polygon {
    points: Vec<(Vertex, Vertex, Vertex)>,
}


pub struct Vertex {
    pos: Vec2,
    tex: Vec2,
    col: Vec3,
    norm: Vec3,
}

pub struct VecOp; // for creating any vec, mat or arr (takes no arguments)


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
        sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
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

        message!("\n", "Destroying Pair({}, {})", first, second);

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
        sqrt((self.x - other.x) * (self.x - other.x))
    }
    fn transform(&self, mat: &Mat4) -> () { // need a transform equation

    }
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    fn distance(&self, other: &Vec2) -> f64 {
        sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
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
        sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y) + (self.z - other.z) * (self.z - other.z))
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
        sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y) + (self.z - other.z) * (self.z - other.z) + (self.w - other.w) * (self.w - other.w))
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
        message!("\n", "Destroying Frame {}", frame.frame);
    }
}


// Errors
impl Error {
    pub(crate) fn new(code: i32, message: String) -> Error {
        Error { code: code, message: message }
    }
    pub(crate) fn print(&self) {
        message!("\n", "Error: {} :: {}", self.code, self.message);
    }
    pub fn error(&self) {
        message!("\n", "[Error] {} :: {}", self.code, self.message);
    }
    pub fn cerror(&self) {
        message!("\n", "[Error {}] {}", self.code, self.message)
    }
}

// Error types
pub type Exception = Error;
pub type ArithmeticError = Error;
pub type AssertionError = Error;
pub type AttributeError = Error;
pub type WindowsError = Error;
pub type OSError = Error;
pub type IOError = Error;
pub type EnvironmentError = Error;
pub type BlockingIOError = Error;
pub type ConnectionError = Error;
pub type BrokenPipeError = Error;
pub type BufferError = Error;
pub type ChildProcessError = Error;
pub type ConnectionAbortedError = Error;
pub type ConnectionRefusedError = Error;
pub type ConnectionResetError = Error;
pub type EOFError = Error;
pub type FileExistsError = Error;
pub type FileNotFoundError = Error;
pub type FloatingPointError = ArithmeticError;
pub type SyntaxError = Error;
pub type LookupError = Error;
pub type IndexError = LookupError;
pub type InterruptedError = OSError;
pub type IsADirectoryError = OSError;
pub type KeyError = LookupError;
pub type MemoryError = Error;
pub type NameError = Error;
pub type NotADirectoryError = OSError;
pub type RuntimeError = Error;
pub type NotImplementedError = RuntimeError;
pub type OverflowError = ArithmeticError;
pub type PermissionError = OSError;
pub type ProcessLookupError = OSError;
pub type RecursionError = Error;
pub type ReferenceError = Error;
pub type SystemError = Error;
pub type TabError = SyntaxError;
pub type TimeoutError = OSError;
pub type TypeError = Error;
pub type UnboundLocalError = NameError;
pub type ValueError = Error;
pub type UnicodeError = ValueError;
pub type UnicodeDecodeError = UnicodeError;
pub type UnicodeEncodeError = UnicodeError;
pub type UnicodeTranslateError = UnicodeError;
pub type ZeroDivisionError = ArithmeticError;


pub type KeyboardInterrupt = Interrrupt;


// Warning

impl Warning {
    fn new(code: i32, message: String) -> Warning {
        Warning { code: code, message: message }
    }
    fn print(&self) {
        message!("\n", "Warning: {} :: {}", self.code, self.message);
    }
    fn warning(&self) {
        message!("\n", "Warning: {} :: {}", self.code, self.message);
    }
}

pub type Warning_ = Warning;
pub type BytesWarning = Warning;
pub type DeprecationWarning = Warning;
pub type EncodingWarning = Warning;
pub type FutureWarning = Warning;
pub type ResourceWarning = Warning;
pub type RuntimeWarning = Warning;
pub type SyntaxWarning = Warning;
pub type UnicodeWarning = Warning;
pub type UserWarning = Warning;


// More stuff

impl Object {
    fn new(name: String, data: String, loc: ADDR16, size: Size, type_: Enum, parent: Option<Box<Object>>, children: Vec<Box<Object>>, next: Option<Box<Object>>, prev: Option<Box<Object>>, first: Option<Box<Object>>, last: Option<Box<Object>>, flags: Enum) -> Object {
        Object { name: name, data: data, loc: loc, size: size, type_: type_, parent: parent, children: children, next: next, prev: prev, first: first, last: last, flags: flags }
    }
    fn repr(&self) {
        message!("\n", "<Object {} at {}", self.name, self.loc);
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
    fn new(points: Vec<(Vertex, Vertex, Vertex)>) -> Polygon {
        Polygon { points: points }
    }

    fn num_points(&self) -> usize {
        self.points.len()
    }

    fn add_point(&mut self, point: (Vertex, Vertex, Vertex)) {
        self.points.push(point);
    }

    fn remove_point(&mut self, index: usize) {
        self.points.remove(index);
    }
}

impl Vertex {
    fn new(pos: Vec2, tex: Vec2, col: Vec3, norm: Vec3) -> Vertex {
        Vertex { pos: pos, tex: tex, col: col, norm: norm }
    }
    fn transform(&self, mat: &Mat4) -> () {
        let Vertex { pos, tex, col, norm } = &self;
        let Mat4 { m } = *mat;
    }
    fn transform_mut(&mut self, mat: &Mat4) -> () {
        let Vertex { pos, tex, col, norm } = &self;
    }
}


// Extra classes and stuff

impl VecOp {
    fn new() -> VecOp {
        VecOp {}
    }
    fn create(&self, typ: String) {
        if typ == "Vec1" {
            Vec1::new(0.0);
        }
        if typ == "Vec2" {
            Vec2::new(0.0, 0.0);
        }
        if typ == "Vec3" {
            Vec3::new(0.0, 0.0, 0.0);
        }
        if typ == "Vec4" {
            Vec4::new(0.0, 0.0, 0.0, 0.0);
        }
        if typ == "Mat2" {
            Mat2::identity();
        }
        if typ == "Mat3" {
            Mat3::identity();
        }
        if typ == "Mat4" {
            Mat4::identity();
        }
    }
}

pub struct Serializer {
    key: USHORT,
    data: [BYTE; 8],
}

impl Serializer {
    fn new(key: USHORT, data: [BYTE; 8]) -> Serializer {
        Serializer { key: key, data }
    }
    fn serialize(&self) -> Vec<BYTE> {
        self.data.to_vec()
    }
}

pub trait Serialized {
    fn serialize(&self) -> Vec<u8>;
}


pub struct Visitor<'s> {
    serializer: &'s mut dyn Serialized,
}

impl<'s> Visitor<'s> {
    fn new(serializer: &'s mut dyn Serialized) -> Self {
        Self { serializer }
    }
}


//linux kernel types

// PCI
pub type PciClass = u32;
pub type PciDevice = u32;
pub type PciVendor = u16;


pub type PhyDeviceId = u32;
pub type PhyClass = u32;







pub mod net {
    use crate::types::net::phy::PhyDeviceId;

    pub mod phy {
        use core::str::Utf8Chunk;
        use uefi_raw::protocol::scsi::ScsiIoTargetStatus;
        use crate::consts::net::phy::Driver as OtherDriver;

        pub struct MDIODeviceID {
            phy_id: u32,
            phy_id_mask: u32
        }

        impl MDIODeviceID {
            pub fn new(phy_id: u32, phy_id_mask: u32) -> MDIODeviceID {
                MDIODeviceID { phy_id, phy_id_mask }
            }
        }
        pub struct PhyDeviceId(MDIODeviceID);
        impl PhyDeviceId {
            /// Creates a new instance with the exact match mask.
            pub const fn new_with_exact_mask(id: u32) -> Self {
                Self(MDIODeviceID {
                    phy_id: id,
                    phy_id_mask: DeviceMask::Exact.as_int()
                })
            }

            /// Creates a new instance with the model match mask.
            pub const fn new_with_model_mask(id: u32) -> Self {
                Self(MDIODeviceID {
                    phy_id: id,
                    phy_id_mask: DeviceMask::Model.as_int(),
                })
            }

            /// Creates a new instance with the vendor match mask.
            pub const fn new_with_vendor_mask(id: u32) -> Self {
                Self(MDIODeviceID {
                    phy_id: id,
                    phy_id_mask: DeviceMask::Vendor.as_int(),
                })
            }

            /// Creates a new instance with a custom match mask.
            pub const fn new_with_custom_mask(id: u32, mask: u32) -> Self {
                Self(MDIODeviceID {
                    phy_id: id,
                    phy_id_mask: DeviceMask::Custom(mask).as_int(),
                })
            }

            /// Creates a new instance from [`Driver`].
            pub const fn new_with_driver<T: Driver>() -> Self {
                T::PHY_DEVICE_ID
            }

            /// Get the MDIO device's PHY ID.
            pub const fn id(&self) -> u32 {
                self.0.phy_id
            }

            /// Get the MDIO device's match mask.
            pub const fn mask_as_int(&self) -> u32 {
                self.0.phy_id_mask
            }

            // macro use only
            #[doc(hidden)]
            pub fn mdio_device_id(&self) -> MDIODeviceID {
                MDIODeviceID::new(self.0.phy_id, self.0.phy_id_mask)
            }
        }


        #[derive(PartialEq, Eq)]
        pub enum DeviceState {
            /// PHY device and driver are not ready for anything.
            Down,
            /// PHY is ready to send and receive packets.
            Ready,
            /// PHY is up, but no polling or interrupts are done.
            Halted,
            /// PHY is up, but is in an error state.
            Error,
            /// PHY and attached device are ready to do work.
            Up,
            /// PHY is currently running.
            Running,
            /// PHY is up, but not currently plugged in.
            NoLink,
            /// PHY is performing a cable test.
            CableTest,
        }

        /// A mode of Ethernet communication.
        ///
        /// PHY drivers get duplex information from hardware and update the current state.
        pub enum DuplexMode {
            /// PHY is in full-duplex mode.
            Full,
            /// PHY is in half-duplex mode.
            Half,
            /// PHY is in unknown duplex mode.
            Unknown,
        }
        enum DeviceMask {
            Exact,
            Model,
            Vendor,
            Custom(u32),
        }

        impl DeviceMask {
            const MASK_EXACT: u32 = !0;
            const MASK_MODEL: u32 = !0 << 4;
            const MASK_VENDOR: u32 = !0 << 10;

            const fn as_int(&self) -> u32 {
                match self {
                    DeviceMask::Exact => Self::MASK_EXACT,
                    DeviceMask::Model => Self::MASK_MODEL,
                    DeviceMask::Vendor => Self::MASK_VENDOR,
                    DeviceMask::Custom(mask) => *mask,
                }
            }
        }

        pub struct esm {
            get: u8,
        }
        impl esm {
            fn get(&self) -> u8 {
                0
            }
        }

        pub struct Device {
            // Need to pull from     github.com/torvalds/linux/include/linux/phy.h
            i: u32,
            phyindex: u32,
            phy_device_id: u32,
            e: esm,
        }

        pub trait Driver {
            /// Defines certain other features this PHY supports.
            /// It is a combination of the flags in the [`flags`] module.
            const FLAGS: u32 = 0;

            /// The friendly name of this PHY type.
            const NAME: &'static crate::types::Str;

            /// This driver only works for PHYs with IDs which match this field.
            /// The default id and mask are zero.
            const PHY_DEVICE_ID: PhyDeviceId = PhyDeviceId::new_with_custom_mask(0, 0);

            /// Issues a PHY software reset.
            fn soft_reset(_dev: &mut Device) -> Result<(), ()> {
                return Err(());
            }

            /// Sets up device-specific structures during discovery.
            fn probe(_dev: &mut Device) -> Result<(), ()> {
                return Err(());
            }

            fn suspend(_dev: &mut Device) -> Result<(), ()> {
                return Err(());
            }

            fn resume(_dev: &mut Device) -> Result<(), ()> {
                return Err(());
            }
        }

        pub mod reg {
            use crate::types::net::phy::Device;

            pub trait Register {
                /// Reads a PHY register.
                fn read(&self, dev: &mut Device) -> Result<u16, ()>;

                /// Writes a PHY register.
                fn write(&self, dev: &mut Device, val: u16) -> Result<u16, ()>;

                /// Checks the link status and updates current link state.
                fn read_status(dev: &mut Device) -> Result<u16, ()>;
            }

            /// A single MDIO clause 22 register address (5 bits).
            #[derive(Copy, Clone, Debug)]
            pub struct C22(u8);

            impl C22 {
                /// Basic mode control.
                pub const BMCR: Self = C22(0x00);
                /// Basic mode status.
                pub const BMSR: Self = C22(0x01);
                /// PHY identifier 1.
                pub const PHYSID1: Self = C22(0x02);
                /// PHY identifier 2.
                pub const PHYSID2: Self = C22(0x03);
                /// Auto-negotiation advertisement.
                pub const ADVERTISE: Self = C22(0x04);
                /// Auto-negotiation link partner base page ability.
                pub const LPA: Self = C22(0x05);
                /// Auto-negotiation expansion.
                pub const EXPANSION: Self = C22(0x06);
                /// Auto-negotiation next page transmit.
                pub const NEXT_PAGE_TRANSMIT: Self = C22(0x07);
                /// Auto-negotiation link partner received next page.
                pub const LP_RECEIVED_NEXT_PAGE: Self = C22(0x08);
                /// Master-slave control.
                pub const MASTER_SLAVE_CONTROL: Self = C22(0x09);
                /// Master-slave status.
                pub const MASTER_SLAVE_STATUS: Self = C22(0x0a);
                /// PSE Control.
                pub const PSE_CONTROL: Self = C22(0x0b);
                /// PSE Status.
                pub const PSE_STATUS: Self = C22(0x0c);
                /// MMD Register control.
                pub const MMD_CONTROL: Self = C22(0x0d);
                /// MMD Register address data.
                pub const MMD_DATA: Self = C22(0x0e);
                /// Extended status.
                pub const EXTENDED_STATUS: Self = C22(0x0f);

                /// Creates a new instance of `C22` with a vendor specific register.
                pub const fn vendor_specific<const N: u8>() -> Self {
                    assert!(
            N > 0x0f && N < 0x20,
            "Vendor-specific register address must be between 16 and 31"
        );
                    C22(N)
                }
            }


            // impl Register for C22 {
            //     fn read(&self, dev: &mut Device) -> Result<u16, ()> {
            //         let phydev = dev.e.get();
            //         // SAFETY: `phydev` is pointing to a valid object by the type invariant of `Device`.
            //         // So it's just an FFI call, open code of `phy_read()` with a valid `phy_device` pointer
            //         // `phydev`.
            //         // let ret = unsafe {
            //         //     bindings::mdiobus_read((*phydev).mdio.bus, (*phydev).mdio.addr, self.0.into())
            //         // };
            //         // to_result(ret)?;
            //         // Ok(ret as u16)
            //         Ok(0)
            //     }
            //
            //     fn write(&self, dev: &mut Device, val: u16) -> Result<u16, ()> {
            //         let phydev = dev.e.get();
            //         // SAFETY: `phydev` is pointing to a valid object by the type invariant of `Device`.
            //         // So it's just an FFI call, open code of `phy_write()` with a valid `phy_device` pointer
            //         // `phydev`.
            //         // to_result(unsafe {
            //         //     bindings::mdiobus_write((*phydev).mdio.bus, (*phydev).mdio.addr, self.0.into(), val)
            //         // })
            //         Ok(0)
            //     }
            //
            //     fn read_status(dev: &mut Device) -> Result<u16, ()> {
            //         let phydev = dev.e.get();
            //         // // SAFETY: `phydev` is pointing to a valid object by the type invariant of `Self`.
            //         // // So it's just an FFI call.
            //         // let ret = unsafe { bindings::genphy_read_status(phydev) };
            //         // to_result(ret)?;
            //         // Ok(ret as u16)
            //         Ok(0)
            //     }
            // }

            /// A single MDIO clause 45 register device and address.
            #[derive(Copy, Clone, Debug)]
            pub struct Mmd(u8);

            impl Mmd {
                /// Physical Medium Attachment/Dependent.
                pub const PMAPMD: Self = Mmd(1 as u8);
                /// WAN interface sublayer.
                pub const WIS: Self = Mmd(2 as u8);
                /// Physical coding sublayer.
                pub const PCS: Self = Mmd(3 as u8);
                /// PHY Extender sublayer.
                pub const PHYXS: Self = Mmd(4 as u8);
                /// DTE Extender sublayer.
                pub const DTEXS: Self = Mmd(5 as u8);
                /// Transmission convergence.
                pub const TC: Self = Mmd(6 as u8);
                /// Auto negotiation.
                pub const AN: Self = Mmd(7 as u8);
                /// Separated PMA (1).
                pub const SEPARATED_PMA1: Self = Mmd(8);
                /// Separated PMA (2).
                pub const SEPARATED_PMA2: Self = Mmd(9);
                /// Separated PMA (3).
                pub const SEPARATED_PMA3: Self = Mmd(10);
                /// Separated PMA (4).
                pub const SEPARATED_PMA4: Self = Mmd(11);
                /// OFDM PMA/PMD.
                pub const OFDM_PMAPMD: Self = Mmd(12);
                /// Power unit.
                pub const POWER_UNIT: Self = Mmd(13);
                /// Clause 22 extension.
                pub const C22_EXT: Self = Mmd(29 as u8);
                /// Vendor specific 1.
                pub const VEND1: Self = Mmd(30 as u8);
                /// Vendor specific 2.
                pub const VEND2: Self = Mmd(31 as u8);
            }

            /// A single MDIO clause 45 register device and address.
            ///
            /// Clause 45 uses a 5-bit device address to access a specific MMD within
            /// a port, then a 16-bit register address to access a location within
            /// that device. `C45` represents this by storing a [`Mmd`] and
            /// a register number.
            pub struct C45 {
                devad: Mmd,
                regnum: u16,
            }

            impl C45 {
                /// Creates a new instance of `C45`.
                pub fn new(devad: Mmd, regnum: u16) -> Self {
                    Self { devad, regnum }
                }
            }

            // impl private::Sealed for C45 {}
            //
            // impl Register for C45 {
            //     fn read(&self, dev: &mut Device) -> Result<u16> {
            //         let phydev = dev.0.get();
            //         // SAFETY: `phydev` is pointing to a valid object by the type invariant of `Device`.
            //         // So it's just an FFI call.
            //         let ret =
            //             unsafe { bindings::phy_read_mmd(phydev, self.devad.0.into(), self.regnum.into()) };
            //         to_result(ret)?;
            //         Ok(ret as u16)
            //     }
            //
            //     fn write(&self, dev: &mut Device, val: u16) -> Result {
            //         let phydev = dev.0.get();
            //         // SAFETY: `phydev` is pointing to a valid object by the type invariant of `Device`.
            //         // So it's just an FFI call.
            //         to_result(unsafe {
            //             bindings::phy_write_mmd(phydev, self.devad.0.into(), self.regnum.into(), val)
            //         })
            //     }
            //
            //     fn read_status(dev: &mut Device) -> Result<u16> {
            //         let phydev = dev.0.get();
            //         // SAFETY: `phydev` is pointing to a valid object by the type invariant of `Self`.
            //         // So it's just an FFI call.
            //         let ret = unsafe { bindings::genphy_c45_read_status(phydev) };
            //         to_result(ret)?;
            //         Ok(ret as u16)
            //     }
            // }
        }
    }
}




pub type BoxedVec<T> = Box<Vec<T>>;
pub type BoxedResult<T, E> = Box<Result<T, E>>;
pub type BoxedOption<T> = Box<Option<T>>;
pub type BoxedBox<T> = Box<Box<T>>;

pub type VecBox<T> = Vec<Box<T>>;

pub type AtomicBox<T> = Atomic<Box<T>>;

pub type DualVec<T> = Vec<Vec<T>>;
pub type TripleVec<T> = Vec<Vec<Vec<T>>>;
pub type QuadVec<T> = Vec<Vec<Vec<Vec<T>>>>;
pub type DualBox<T> = Box<Box<T>>;
pub type TripleBox<T> = Box<Box<Box<T>>>;
pub type QuadBox<T> = Box<Box<Box<Box<T>>>>;
pub type DualOption<T> = Option<Option<T>>;
pub type TripleOption<T> = Option<Option<Option<T>>>;
pub type QuadOption<T> = Option<Option<Option<Option<T>>>>;

pub type Synchronous<T> = Vec<Box<Vec<T>>>;

// type DynBoxedVec<T> = BoxedVec<dyn T>;



pub type VersionString = String;