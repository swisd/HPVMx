use crate::x4::ops::{__ROL4__, __ROR4__, __PAIR64__, HIDWORD, LODWORD};


/*__int64 __fastcall MixState_MersenneMod3529_AddHighDword(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v3; // r8d
unsigned __int64 v4; // rax
__int64 v5; // rax
__int64 result; // rax

v3 = a3 - 765773044;
v4 = 3529194252u * ((3529194252u * __PAIR64__(v3 + a2[1], v3 + a2[1])) >> 32);
v5 = 3529194252LL
* (2 * HIDWORD(v4)
- (((int)v4 >> 31) & 0x7FFFFFFF)
+ (_DWORD)v4
- 0x7FFFFFFF
+ (((2 * HIDWORD(v4) - (((int)v4 >> 31) & 0x7FFFFFFF) + (int)v4 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFFu));
a2[1] = v5 + HIDWORD(v5);
result = (unsigned int)(v5 + HIDWORD(v5) + v3);
*a2 += result;
return result;
}*/
fn mix_state_mersenne_mod3529_add_high_dword(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // int v3; // r8d
    let v3 = a3 as i32 - 765773044;
    // unsigned __int64 v4; // rax
    let v4 = 3529194252u64
        * (3529194252u64 * __PAIR64__(v3 as u32 + a2[1], v3 as u32 + a2[1])) >> 32;
    // __int64 v5; // rax
    let v5 = 3529194252i64
        * (2 * /*((v4 >> 32) as i32) as i64*/ HIDWORD!(v4) as i64
        - ((((v4 as i32) >> 31) & 0x7FFFFFFF) as i64)
        + (v4 as i32) as i64
        - 0x7FFFFFFF
        + ((((2 * ((v4 >> 32) as i32) as i64
        - ((((v4 as i32) >> 31) & 0x7FFFFFFF) as i64)
        + (v4 as i32) as i64
        - 0x7FFFFFFF)
        >> 31) as i32
        & 0x7FFFFFFF) as i64));
    a2[1] = (v5 + ((v5 >> 32) as i64)) as u32;
    // __int64 result; // rax
    let result = (v5 + ((v5 >> 32) as i64) + v3 as i64) as u32;
    a2[0] = a2[0].wrapping_add(result);
    result as i64
}

/*__int64 __fastcall MixState_Ror27_HighMul1446(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v4; // r8d
unsigned int v5; // eax
int v6; // edx
__int64 result; // rax

v4 = a3 + 1446708061;
v5 = __ROR4__(1446708061 * ((1446708061 * __PAIR64__(v4 + a2[1], v4 + a2[1])) >> 32), 27);
v6 = (1446708061 * __PAIR64__(v5, v5)) >> 32;
a2[1] = v6;
result = (unsigned int)(v6 + v4);
*a2 += result;
return result;
}*/
fn mix_state_ror27_high_mul1446(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // int v4; // r8d
    // unsigned int v5; // eax
    // int v6; // edx
    // __int64 result; // rax

    let v4 = a3 as i32 + 1446708061;
    let v5 = __ROR4__(1446708061u32
        .wrapping_mul(
            (((v4 as u64 + a2[1] as u64) as u64).wrapping_mul(1446708061u64) >> 32) as u32,
        ),27) as u32;
    let v6 = ((1446708061u64 * __PAIR64__(v5, v5)) >> 32) as i32;
    a2[1] = v6 as u32;
    let result = (v6 + v4) as u32 as i64;
    a2[0] = a2[0].wrapping_add(result as u32);
    result
}

/*__int64 __fastcall MixState_DualRor19_3_Rol20339(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v3; // r8d
int v4; // eax
int v5; // ecx
__int64 result; // rax

v3 = a3 - 1052177357;
v4 = __ROR4__(-1052177357 * ((3242789939u * __PAIR64__(v3 + a2[1], v3 + a2[1])) >> 32), 19);
v5 = __ROR4__(v4, 3) ^ __ROL4__(20339 * v4, 3);
a2[1] = v5;
result = (unsigned int)(v5 + v3);
*a2 += result;
return result;
}*/
fn mix_state_dual_ror19_3_rol20339(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // int v3; // r8d
    let v3 = a3 as i32 - 1052177357;
    // int v4; // eax
    let v4 = __ROR4__((-1052177357i32 as i64)
        .wrapping_mul(((3242789939u32 as u64 * ((v3 as u64 + a2[1] as u64) as i64 as u64)) >> 32) as i64),19) as i32;
    // int v5; // ecx
    let v5 = __ROR4__(v4,3) ^ __ROL4__((20339 * v4),3);
    a2[1] = v5 as u32;
    // __int64 result; // rax
    let result = (v5 + v3) as u32 as i64;
    a2[0] = a2[0].wrapping_add(result as u32);
    result
}

/*__int64 __fastcall MixState_MersenneMod2123_RorRol2403(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v4; // r9d
__int64 v5; // rax
int v6; // r8d
__int64 v7; // rax
int v8; // edx
__int64 result; // rax

v4 = a3 + 2123974043;
v5 = 2123974043LL * (unsigned int)(v4 + a2[1]);
v6 = 2 * HIDWORD(v5) - (((2123974043 * (v4 + a2[1])) >> 31) & 0x7FFFFFFF);
v7 = 2123974043LL * (v6 + (_DWORD)v5 - 0x7FFFFFFF + (((v6 + (int)v5 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFFu));
v8 = 2 * HIDWORD(v7)
- (((int)v7 >> 31) & 0x7FFFFFFF)
+ v7
- 0x7FFFFFFF
+ (((2 * HIDWORD(v7) - (((int)v7 >> 31) & 0x7FFFFFFF) + (int)v7 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFF);
LODWORD(v7) = __ROR4__(v8, 11) ^ __ROL4__(2403 * v8, 11);
a2[1] = v7;
result = (unsigned int)(v4 + v7);
*a2 += result;
return result;
}*/
fn mix_state_mersenne_mod2123_ror_rol2403(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // v4: a3 + 2123974043
    let v4 = a3 as i32 + 2123974043;

    // v5: 2123974043LL * (unsigned int)(v4 + a2[1])
    let v5 = 2123974043i64 * (v4 as u32 + a2[1]) as i64;

    // v6: 2 * HIDWORD(v5) - (((2123974043 * (v4 + a2[1])) >> 31) & 0x7FFFFFFF)
    let v6 = (2 * ((v5 >> 32) as i32)) - (((v5 as u32 as i32) >> 31) & 0x7FFFFFFF);

    // v7: 2123974043LL * (v6 + (_DWORD)v5 - 0x7FFFFFFF + (((v6 + (int)v5 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFFu))
    let v7 = 2123974043i64
        * ((v6 + (v5 as u32) as i32 - 0x7FFFFFFF
        + (((v6 + (v5 as u32) as i32 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFF)) as i64);

    // v8: 2 * HIDWORD(v7)
    //  - (((int)v7 >> 31) & 0x7FFFFFFF)
    //  + v7
    //  - 0x7FFFFFFF
    //  + (((2 * HIDWORD(v7) - (((int)v7 >> 31) & 0x7FFFFFFF) + (int)v7 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFF)
    let v8 = (2 * HIDWORD!(v7))
        - (((v7 as u32 as i32) >> 31) & 0x7FFFFFFF)
        + (v7 as i32)
        - 0x7FFFFFFF
        + (((((2 * HIDWORD!(v7))
        - (((v7 as u32 as i32) >> 31) & 0x7FFFFFFF)
        + (v7 as i32)
        - 0x7FFFFFFF)
        >> 31)
        & 0x7FFFFFFF));

    // LODWORD(v7): __ROR4__(v8, 11) ^ __ROL4__(2403 * v8, 11)
    let ror4_v8_11 = __ROR4__(v8,11);
    let rol4_2403_v8_11 = __ROR4__((2403 * v8),11);
    let v7 = ror4_v8_11 ^ rol4_2403_v8_11;

    a2[1] = v7 as u32;

    // result: (unsigned int)(v4 + v7)
    let result = (v4 + v7) as u32 as i64;

    // *a2 += result
    a2[0] = a2[0].wrapping_add(result as u32);

    result
}

/*__int64 __fastcall MixState_MersenneRor2(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v4; // r9d
__int64 v5; // rax
int v6; // edx
__int64 result; // rax

v4 = a3 + 1897147385;
v5 = 1897147385LL * (unsigned int)(v4 + a2[1]);
LODWORD(v5) = __ROR4__(
1897147385
* (2 * HIDWORD(v5)
- (((int)v5 >> 31) & 0x7FFFFFFF)
+ v5
- 0x7FFFFFFF
+ (((2 * HIDWORD(v5) - (((int)v5 >> 31) & 0x7FFFFFFF) + (int)v5 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFF)),
2);
v6 = (1897147385 * __PAIR64__(v5, v5)) >> 32;
a2[1] = v6;
result = (unsigned int)(v6 + v4);
*a2 += result;
return result;
}*/
fn mix_state_mersenne_ror2(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // int v4; // r9d
    // __int64 v5; // rax
    // int v6; // edx
    // __int64 result; // rax

    let v4 = a3 as i32 + 1897147385i32;
    let mut v5 = 1897147385_i64.wrapping_mul((v4 + a2[1] as i32) as u32 as i64);
    v5 = __ROR4__((1897147385_i64.wrapping_mul(
        2 * ((v5 >> 32) as i64)
            - ((((v5 as i64) >> 31) & 0x7FFFFFFF) as i64)
            + v5
            - 0x7FFFFFFF
            + (((2 * ((v5 >> 32) as i64)
            - ((((v5 as i64) >> 31) & 0x7FFFFFFF) as i64)
            + (v5 as i64)
            - 0x7FFFFFFF)
            >> 31)
            & 0x7FFFFFFF) as i64,
    ) as u32),2) as i64;

    let v6 = (1897147385_u64.wrapping_mul(__PAIR64__(v5 as u32, v5 as u32)) >> 32) as i32;
    a2[1] = v6 as u32;
    let result = (v6 + v4) as u32 as i64;
    a2[0] = a2[0].wrapping_add(result as u32);
    result
}

/*__int64 __fastcall MixState_XorMul52700(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v4; // r9d
int v5; // r8d
int v6; // edx
__int64 result; // rax

v4 = a3 + 0x6AED6C90;
v5 = (v4 + a2[1]) ^ (52700 * (v4 + a2[1]));
v6 = (0x6AED6C90 * __PAIR64__((0x6AED6C90 * __PAIR64__(v5, v5)) >> 32, (0x6AED6C90 * __PAIR64__(v5, v5)) >> 32)) >> 32;
a2[1] = v6;
result = (unsigned int)(v6 + v4);
*a2 += result;
return result;
}*/
fn mix_state_xor_mul_52700(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    let v4: i32; // r9d
    let v5: i32; // r8d
    let v6: i32; // edx
    let result: i64; // rax

    v4 = a3 as i32 + 0x6AED6C90;
    v5 = (v4 + a2[1] as i32) ^ (52700 * (v4 + a2[1] as i32));
    v6 = ((0x6AED6C90 as i64
        * (((0x6AED6C90 as i64 * __PAIR64__(v5 as u32, v5 as u32) as i64) >> 32) as u64 as i64))
        >> 32) as i32;
    a2[1] = v6 as u32;
    result = (v6 as u32).wrapping_add(v4 as u32) as i64;
    a2[0] = a2[0].wrapping_add(result as u32);
    result
}

/*__int64 __fastcall MixState_MersenneMod3127(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v3; // r8d
__int64 v4; // rax
__int64 v5; // rax
__int64 result; // rax

v3 = a3 - 1167106736;
v4 = 3127860560LL * (unsigned int)(-1167106736 * (v3 + a2[1]));
v5 = 3127860560LL
* (2 * HIDWORD(v4)
- (((int)v4 >> 31) & 0x7FFFFFFF)
+ (_DWORD)v4
- 0x7FFFFFFF
+ (((2 * HIDWORD(v4) - (((int)v4 >> 31) & 0x7FFFFFFF) + (int)v4 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFFu));
LODWORD(v5) = 2 * HIDWORD(v5)
- (((int)v5 >> 31) & 0x7FFFFFFF)
+ v5
- 0x7FFFFFFF
+ (((2 * HIDWORD(v5) - (((int)v5 >> 31) & 0x7FFFFFFF) + (int)v5 - 0x7FFFFFFF) >> 31) & 0x7FFFFFFF);
a2[1] = v5;
result = (unsigned int)(v3 + v5);
*a2 += result;
return result;
}*/
fn mix_state_mersenne_mod3127(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // int v3; // r8d
    // __int64 v4; // rax
    // __int64 v5; // rax
    // __int64 result; // rax

    let v3 = a3 as i32 - 1167106736;
    let v4 = 3127860560i64 * (-1167106736i32 * (v3 + a2[1] as i32)) as u32 as i64;
    let mut v5 = 3127860560i64
        * ((2 * ((v4 >> 32) as i32) as i64
        - (((v4 as i32 >> 31) & 0x7FFFFFFF) as i64)
        + (v4 as i32) as i64
        - 0x7FFFFFFF as i64
        + (((2 * ((v4 >> 32) as i32) as i64
        - (((v4 as i32 >> 31) & 0x7FFFFFFF) as i64)
        + (v4 as i32) as i64
        - 0x7FFFFFFF as i64)
        >> 31) as i32 & 0x7FFFFFFF) as i64));
    let v5_lo = ((2 * ((v5 >> 32) as i32) as i64
        - (((v5 as i32 >> 31) & 0x7FFFFFFF) as i64)
        + (v5 as i32) as i64
        - 0x7FFFFFFF as i64
        + (((2 * ((v5 >> 32) as i32) as i64
        - (((v5 as i32 >> 31) & 0x7FFFFFFF) as i64)
        + (v5 as i32) as i64
        - 0x7FFFFFFF as i64)
        >> 31) as i32 & 0x7FFFFFFF) as i64)) as i32;
    v5 = v5_lo as i64;

    a2[1] = v5 as u32;

    let result = (v3 + v5 as i32) as u32;
    a2[0] = a2[0].wrapping_add(result);
    result as i64
}

/*__int64 __fastcall MixState_DualRor13_HighMul2203(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v4; // r8d
unsigned int v5; // eax
int v6; // edx
__int64 result; // rax

v4 = a3 + 22037131;
v5 = __ROR4__(22037131 * __ROR4__(22037131 * (v4 + a2[1]), 13), 13);
v6 = (22037131 * __PAIR64__(v5, v5)) >> 32;
a2[1] = v6;
result = (unsigned int)(v6 + v4);
*a2 += result;
return result;
}*/
fn mix_state_dual_ror13_high_mul2203(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // v4: r8d
    let v4 = a3 as i32 + 22037131;
    // v5: eax
    let v5 = __ROR4__(22037131u32.wrapping_mul(__ROR4__(22037131u32.wrapping_mul((v4 + a2[1] as i32) as u32), 13)), 13);
    // v6: edx
    let v6 = ((22037131u64.wrapping_mul(__PAIR64__(v5, v5)) >> 32) as i32);
    a2[1] = v6 as u32;
    let result = (v6 + v4) as u32;
    a2[0] = a2[0].wrapping_add(result);
    result as i64
}

/*__int64 __fastcall MixState_DualRor18_RorRol14_23570(__int64 a1, _DWORD *a2, unsigned __int8 a3)
{
int v3; // r8d
int v4; // ecx
int v5; // eax
__int64 result; // rax

v3 = a3 + 1428895070;
v4 = __ROR4__(1428895070 * __ROR4__(1428895070 * (v3 + a2[1]), 18), 18);
v5 = __ROR4__(v4, 14) ^ __ROL4__(23570 * v4, 14);
a2[1] = v5;
result = (unsigned int)(v3 + v5);
*a2 += result;
return result;
}*/
fn mix_state_dual_ror18_rorrol14_23570(a1: i64, a2: &mut [u32; 2], a3: u8) -> i64 {
    // v3 = a3 + 1428895070;
    let v3 = a3 as i32 + 1428895070;
    // v4 = __ROR4__(1428895070 * __ROR4__(1428895070 * (v3 + a2[1]), 18), 18);
    let v4 = __ROR4__((1428895070u32.wrapping_mul(__ROR4__(1428895070u32.wrapping_mul((v3 + a2[1] as i32) as u32), 18))), 18);
    // v5 = __ROR4__(v4, 14) ^ __ROL4__(23570 * v4, 14);
    let v5 = __ROR4__(v4, 14) ^ __ROL4__(23570 * v4, 14);
    a2[1] = v5;
    // result = (unsigned int)(v3 + v5);
    let result = (v3 + v5 as i32) as u32 as i64;
    // *a2 += result;
    a2[0] = a2[0].wrapping_add(result as u32);
    return result;
}

/*__int64 __fastcall MixState_RorRol28850(__int64 ctxRef, _DWORD *pStateBuffer, unsigned __int8 salt)
{
int seed; // r8d
int v5; // edx
int v6; // eax
__int64 result; // rax

seed = salt + 0x35D271FE;
v5 = (0x35D271FE
* __PAIR64__(
(0x35D271FE * __PAIR64__(seed + pStateBuffer[1], seed + pStateBuffer[1])) >> 32,
(0x35D271FE * __PAIR64__(seed + pStateBuffer[1], seed + pStateBuffer[1])) >> 32)) >> 32;
v6 = __ROR4__(v5, 14) ^ __ROL4__(0x70B2 * v5, 14);
pStateBuffer[1] = v6;
result = (unsigned int)(seed + v6);
*pStateBuffer += result;
return result;
}*/
fn mix_state_rorrol28850(ctx_ref: i64, p_state_buffer: &mut [u32], salt: u8) -> i64 {
    let seed; // r8d
    let v5; // edx
    let v6; // eax
    let result; // rax

    seed = salt as i32 + 0x35D271FE;
    v5 = ((0x35D271FE_u64
        * (((0x35D271FE_u64 * __PAIR64__((seed as u64 + p_state_buffer[1] as u64) as u32, (seed as u64 + p_state_buffer[1] as u64) as u32))
        >> 32)
        << 32)) >> 32) as i32;
    v6 = __ROR4__(v5, 14) ^ __ROL4__(0x70B2 * v5, 14);
    p_state_buffer[1] = v6 as u32;
    result = (seed as u64 + v6 as u64) as i64;
    p_state_buffer[0] = p_state_buffer[0].wrapping_add(result as u32);
    result
}