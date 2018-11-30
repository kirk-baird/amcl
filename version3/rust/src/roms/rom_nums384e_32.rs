/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use nums384e::big::NLEN;
use arch::Chunk;
use std;
use modtype::ModType;// Base Bits= 29

// nums384 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFFEC3, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x7F,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x0, 0x4448000, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const MCONST: Chunk = 0x13D;

// nums384e Curve
pub const CURVE_COF_I: isize = 4;
pub const CURVE_A: isize = 1;
pub const CURVE_B_I: isize = -11556;
pub const CURVE_COF: [Chunk; NLEN] = [
    0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_B: [Chunk; NLEN] = [
    0x1FFFD19F, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x7F,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x6A3897D, 0x5CEE627, 0xD721E48, 0x8AAB556, 0x1E1CF61E, 0xD0E5A35, 0x1FFF891C, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1F,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xC206BDE, 0x6AA0723, 0x116504D4, 0x52562CA, 0x163406FF, 0x1FD47998, 0x10015D8F, 0x8DCB7C9,
    0x15B30BF4, 0x14D72AED, 0x102DA884, 0xB524CD9, 0x1B111FB4, 0x30,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x10729392, 0xC681F0F, 0x1B123727, 0x561F28D, 0x1964B007, 0xC7BFB22, 0x1D5A0C3E, 0xE9E284B,
    0x1716AD82, 0x11D886E, 0x1CE2C69, 0x134DDD61, 0x983E67B, 0x41,
];

pub const MODBYTES: usize = 48;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 384;
pub const MOD8: usize = 3;
pub const MODTYPE: ModType = ModType::PSEUDO_MERSENNE;
pub const SH: usize = std::cmp::min(14, BASEBITS*(1+((8*MODBYTES-1)/BASEBITS))-MODBITS);

