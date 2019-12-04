//! Definition of 16 bit thumb instructions

pub const NUM_TH16_INSTRUCTIONS: usize = (::std::u16::MAX as usize) + 1;

#[derive(Clone, Copy, Debug)]
pub enum InstrThumb16 {
    AdcReg { rm: u8, rdn: u8 },
    AddImm { imm: u8, rdn: u8, rd: u8 },
    AddReg { rm: u8, rdn: u8, rd: u8 },
    AddSpImm { imm: u8, rd: u8},
    AddSpReg { rdm: u8 },
    Adr { rd: u8, imm: u8 },
    AndReg { rm: u8, rdn: u8 },
    AsrImm { imm: u8, rm: u8, rd: u8 },
    AsrReg { rm: u8, rdn: u8 },
    Branch { cond: u8, imm: u8, immx: u8  },
    BicReg { rm: u8, rdn: u8 },
    Breakpoint { imm: u8 },
    BranchLx { rm: u8 },
    BranchX { rm: u8 },
    Cbnz { imm1: u8, imm5: u8, rn: u8 },
    Cbz { imm1: u8, imm5: u8, rn: u8 },
    CmnReg { rm: u8, rn: u8 },
    CmpImm { rn: u8, imm: u8 },
    CmpReg { rm: u8, rn: u8, n: u8 },
    Cps { imm: u8, fi: u8, ff: u8 },
    EorReg { rm: u8, rdn: u8 },
    IfThen { cond: u8, mask: u8 },
    Ldm { list: u8, rn: u8 },
    LdrImm { imm: u8, rn: u8, rt: u8 },
    LdrLit { rt: u8, imm: u8 },
    LdrReg { rm: u8, rn: u8, rt: u8 },
    LdrbImm { imm: u8, rn: u8, rt: u8 },
    LdrbReg { rm: u8, rn: u8, rt: u8 },
    LdrhImm { imm: u8, rn: u8, rt: u8 },
    LdrhReg { rm: u8, rn: u8, rt: u8 },
    LdrSbReg { rm: u8, rn: u8, rt: u8 },
    LdrShReg { rm: u8, rn: u8, rt: u8 },
    LslImm { imm: u8, rm: u8, rd: u8 },
    LslReg { rm: u8, rdn: u8 },
    LsrImm { imm: u8, rm: u8, rd: u8 },
    LsrReg { rm: u8, rdn: u8 },
    MovImm { rd: u8, imm: u8 },
    MovReg { rm: u8, rd: u8, d: u8 },
    Mul { rn: u8, rdm: u8 },
    MvnReg { rm: u8, rd: u8 },
    Nop,
    OrrReg { rm: u8, rdn: u8 },
    Pop { p: u8, list: u8 },
    Push { m: u8, list: u8 },
    Rev { rm: u8, rd: u8 },
    Rev16 { rm: u8, rd: u8 },
    RevSh { rm: u8, rd: u8 },
    RorReg { rm: u8, rdn: u8 },
    RsbImm { rn: u8, rd: u8 },
    SbcReg { rm: u8, rdn: u8 },
    Sev,
    Stm { rn: u8, list: u8 },
    StrImm { imm: u8, rn: u8, rt: u8 },
    StrReg { rm: u8, rn: u8, rt: u8 },
    StrbImm { imm: u8, rn: u8, rt: u8 },
    StrbReg { rm: u8, rn: u8, rt: u8 },
    StrhImm { imm: u8, rn: u8, rt: u8 },
    StrhReg { rm: u8, rn: u8, rt: u8 },
    SubImm { imm: u8, rn: u8, rdn: u8 },
    SubReg { rm: u8, rn: u8, rd: u8 },
    SubSpImm { imm: u8 },
    Svc,
    Sxtb { rm: u8, rd: u8 },
    Sxth { rm: u8, rd: u8 },
    TstReg { rm: u8, rn: u8 },
    Udf { imm: u8 },
    Uxtb { rm: u8, rd: u8 },
    Uxth { rm: u8, rd: u8 },
    Wfe,
    Wfi,
    Yield,
    
    // thumb2 special
    Thumb2A { high: u8, low: u8 },
    Thumb2B { high: u8, low: u8 },
    Thumb2C { high: u8, low: u8 },
    
    Undefined,
}

impl InstrThumb16 {
    pub fn generate_decode_table() -> [InstrThumb16; NUM_TH16_INSTRUCTIONS] {
        define_instructions! {

            // todo: AdcImm (thumb2 only)

            instruction! {
                name: AdcReg,
                encoding: [
                    base: 0x4140,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: AddImm,
                encoding: [
                    base: 0x1C00,
                    operand: [rd, 3 << 0],
                    operand: [rdn, 3 << 3],
                    operand: [imm, 3 << 6]
                ],
                encoding: [
                    base: 0x3000,
                    operand: [imm, 8 << 0],
                    operand: [rdn, 3 << 8],
                    operand: [rd, unused]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: AddReg,
                encoding: [
                    base: 0x1800,
                    operand: [rm, 3 << 6],
                    operand: [rdn, 3 << 3],
                    operand: [rd, 3 << 0]
                ],
                encoding: [
                    base: 0x4400,
                    operand: [rm, 4 << 3],
                    operand: [rdn, 3 << 0],
                    operand: [rd, 1 << 7]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: AddSpImm,
                encoding: [
                    base: 0xA800,
                    operand: [rd, 3 << 8],
                    operand: [imm, 8 << 0]
                ],
                encoding: [
                    base: 0xB000,
                    operand: [imm, 7 << 0],
                    operand: [rd, unused]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: AddSpReg,
                encoding: [
                    base: 0x4468,
                    operand: [rdm, 3 << 0]
                ],
                encoding: [
                    base: 0x4485,
                    operand: [rdm, 4 << 3]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Adr,
                encoding: [
                    base: 0xA000,
                    operand: [rd, 3 << 8],
                    operand: [imm, 8 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: AndImm (thumb2 only)

            instruction! {
                name: AndReg,
                encoding: [
                    base: 0x4000,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: AsrImm,
                encoding: [
                    base: 0x1000,
                    operand: [imm, 5 << 6],
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: AsrReg,
                encoding: [
                    base: 0x4100,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]
            },

            instruction! {
                name: Branch,
                encoding: [
                    base: 0xD000,
                    operand: [cond, 4 << 8],
                    operand: [imm, 8 << 0],
                    operand: [immx, unused]
                ],
                encoding: [
                    base: 0xE000,
                    operand: [imm, 8 << 0],
                    operand: [immx, 3 << 8],
                    operand: [cond, unused]
                ]

                // todo: thumb2 encodings
            },

            // todo: Bfc (thumb2 only)

            // todo: Bfi (thumb2 only)

            // todo: BicImm (thumb2 only)

            instruction! {
                name: BicReg,
                encoding: [
                    base: 0x4380,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Breakpoint,
                encoding: [
                    base: 0xBE00,
                    operand: [imm, 8 << 0]
                ]
            },

            // todo: BranchL (thumb2 only)

            instruction! {
                name: BranchLx,
                encoding: [
                    base: 0x4780,
                    operand: [rm, 4 << 3]
                ]
            },

            instruction! {
                name: BranchX,
                encoding: [
                    base: 0x4700,
                    operand: [rm, 4 << 3]
                ]
            },

            instruction! {
                name: Cbnz,
                encoding: [
                    base: 0xB900,
                    operand: [imm1, 1 << 9],
                    operand: [imm5, 5 << 3],
                    operand: [rn, 3 << 0]
                ]
            },

            instruction! {
                name: Cbz,
                encoding: [
                    base: 0xB100,
                    operand: [imm1, 1 << 9],
                    operand: [imm5, 5 << 3],
                    operand: [rn, 3 << 0]
                ]
            },

            // todo: Cdp (thumb2 only)

            // todo: Cdp2 (thumb2 only)

            // todo: ClrEx (thumb2 only)

            // todo: Clz (thumb2 only)

            // todo: CmnImm (thumb2 only)

            instruction! {
                name: CmnReg,
                encoding: [
                    base: 0x42C0,
                    operand: [rm, 3 << 3],
                    operand: [rn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: CmpImm,
                encoding: [
                    base: 0x2800,
                    operand: [rn, 3 << 8],
                    operand: [imm, 8 << 0]
                ]
            },

            instruction! {
                name: CmpReg,
                encoding: [
                    base: 0x4280,
                    operand: [rm, 3 << 3],
                    operand: [rn, 3 << 0],
                    operand: [n, unused]
                ],
                encoding: [
                    base: 0x2280,
                    operand: [n, 1 << 7],
                    operand: [rm, 4 << 3],
                    operand: [rn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Cps,
                encoding: [
                    base: 0x5B30,
                    operand: [imm, 1 << 4],
                    operand: [fi, 1 << 1],
                    operand: [ff, 1 << 0]
                ]
            },

            // Cpy == Mov

            // todo: Csdb (thumb2 only)
            
            // todo: Dbg (thumb2 only)
            
            // todo: Dmb (thumb2 only)

            // todo: Dsb (thumb2 only)

            // todo: EorImm (thumb2 only)

            instruction! {
                name: EorReg,
                encoding: [
                    base: 0x4040,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Isb (thumb2 only)

            instruction! {
                name: IfThen,
                encoding: [
                    base: 0xBF00,
                    operand: [cond, 4 << 4],
                    operand: [mask, 4 << 0]
                ]
            },

            // todo: LdcImm (thumb2 only)
            
            // todo: Ldc2Imm (thumb2 only)

            // todo: LdcLit (thumb2 only)
            
            // todo: Ldc2Lit (thumb2 only)

            instruction! {
                name: Ldm, // Ldmia, Ldmfd
                encoding: [
                    base: 0xC800,
                    operand: [list, 8 << 0],
                    operand: [rn, 3 << 8]
                ]

                // todo: thumb2 encodings
            },

            // todo: Ldmdb (Ldmea) (thumb2 only)

            instruction! {
                name: LdrImm,
                encoding: [
                    base: 0x6800,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ],
                encoding: [
                    base: 0x9800,
                    operand: [rt, 3 << 8],
                    operand: [imm, 8 << 0],
                    operand: [rn, unused]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: LdrLit,
                encoding: [
                    base: 0x4800,
                    operand: [rt, 3 << 8],
                    operand: [imm, 8 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: LdrReg,
                encoding: [
                    base: 0x5800,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: LdrbImm,
                encoding: [
                    base: 0x7800,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: LdrbLit (thumb2 only)

            instruction! {
                name: LdrbReg,
                encoding: [
                    base: 0x5C00,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Ldrbt (thumb2 only)

            // todo: LdrdImm (thumb2 only)

            // todo: LdrdLit (thumb2 only)

            // todo: LdrEx (thumb2 only)

            // todo: LdrExB (thumb2 only)

            // todo: LdrExH (thumb2 only)

            instruction! {
                name: LdrhImm,
                encoding: [
                    base: 0x8800,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: LdrhLit (thumb2 only)

            instruction! {
                name: LdrhReg,
                encoding: [
                    base: 0x5A00,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]
                
                // todo: thumb2 encodings
            },

            // todo: Ldrht (thumb2 only)
            
            // todo: LdrsbImm (thumb2 only)

            // todo: LdrsbLit (thumb2 only)
            
            instruction! {
                name: LdrSbReg,
                encoding: [
                    base: 0x5600,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Ldrsbt (thumb2 only)

            // todo: LdrshImm (thumb2 only)

            // todo: LdrshLit (thumb2 only)

            instruction! {
                name: LdrShReg,
                encoding: [
                    base: 0x5E00,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Ldrsht (thumb2 only)

            // todo: Ldrt (thumb2 only)

            instruction! {
                name: LslImm,
                encoding: [
                    base: 0x0000,
                    operand: [imm, 5 << 6],
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: LslReg,
                encoding: [
                    base: 0x4080,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: LsrImm,
                encoding: [
                    base: 0x0800,
                    operand: [imm, 5 << 6],
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: LsrReg,
                encoding: [
                    base: 0x40C0,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Mcr (thumb2 only)

            // todo: Mcr2 (thumb2 only)

            // todo: Mcrr (thumb2 only)
            
            // todo: Mcrr2 (thumb2 only)

            // todo: Mla (thumb2 only)

            // todo: Mls (thumb2 only)

            instruction! {
                name: MovImm,
                encoding: [
                    base: 0x2000,
                    operand: [rd, 3 << 8],
                    operand: [imm, 8 << 0]
                ]
                
                // todo: thumb2 encodings
            },

            instruction! {
                name: MovReg,
                encoding: [
                    base: 0x4600,
                    operand: [d, 1 << 7],
                    operand: [rm, 4 << 3],
                    operand: [rd, 3 << 0]
                ]
                
                // todo: thumb2 encodings
            },

            // todo: MovTop (thumb2 only)

            // todo: Mrc (thumb2 only)
            
            // todo: Mrc2 (thumb2 only)

            // todo: Mrrc (thumb2 only)
            
            // todo: Mrrc2 (thumb2 only)

            // todo: Mrs (thumb2 only)

            // todo: Msr (thumb2 only)

            instruction! {
                name: Mul,
                encoding: [
                    base: 0x4340,
                    operand: [rn, 3 << 3],
                    operand: [rdm, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: MvnImm (thumb2 only)

            instruction! {
                name: MvnReg,
                encoding: [
                    base: 0x43C0,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Nop,
                encoding: [
                    base: 0xBF00
                ]

                // todo: thumb2 encodings
            },

            // todo: OrnImm (thumb2 only)

            // todo: OrnReg (thumb2 only)

            // todo: OrrImm (thumb2 only)

            instruction! {
                name: OrrReg,
                encoding: [
                    base: 0x4300,
                    operand: [rm, 3 << 0],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Pkhbt (thumb2 only)

            // todo: Pkhtb (thumb2 only)

            // todo: PldImm (thumb2 only)

            // todo: PldLit (thumb2 only)

            // todo: PldReg (thumb2 only)

            // todo: PliImm (thumb2 only)

            // todo: PliLit (thumb2 only)

            // todo: PliReg (thumb2 only)

            instruction! {
                name: Pop,
                encoding: [
                    base: 0xBC00,
                    operand: [p, 1 << 8],
                    operand: [list, 8 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo:: Pssbb (thumb2 only)

            instruction! {
                name: Push,
                encoding: [
                    base: 0xB400,
                    operand: [m, 1 << 8],
                    operand: [list, 8 << 0],
                ]

                // todo: thumb2 encodings
            },

            // todo: Qadd (thumb2 only)

            // todo: Qadd16 (thumb2 only)

            // todo: Qadd8 (thumb2 only)

            // todo: QasX (thumb2 only)

            // todo: QdAdd (thumb2 only)

            // todo: QdSub (thumb2 only)

            // todo: QsaX (thumb2 only)

            // todo: Qsub (thumb2 only)

            // todo: Qsub16 (thumb2 only)

            // todo: Qsub8 (thumb2 only)

            // todo: Rbit (thumb2 only)

            instruction! {
                name: Rev,
                encoding: [
                    base: 0xBA00,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Rev16,
                encoding: [
                    base: 0xBA40,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0],
                ]
            },

            instruction! {
                name: RevSh,
                encoding: [
                    base: 0xBAC0,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0],
                ]

                // todo: thumb2 encodings
            },

            // todo: RorImm (thumb2 only)

            instruction! {
                name: RorReg,
                encoding: [
                    base: 0x41C0,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]
                
                // todo: thumb2 encodings
            },

            // todo: RrX (thumb2 only)

            instruction! {
                name: RsbImm,
                encoding: [
                    base: 0x4240,
                    operand: [rn, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: RsbReg (thumb2 only)

            // todo: Sadd16 (thumb2 only)
            
            // todo: Sadd8 (thumb2 only)

            // todo: SasX (thumb2 only)

            // todo: SbcImm (thumb2 only)

            instruction! {
                name: SbcReg,
                encoding: [
                    base: 0x4180,
                    operand: [rm, 3 << 3],
                    operand: [rdn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: SbfX (thumb2 only)

            // todo: Sdiv (thumb2 only)

            // todo: Sel (thumb2 only)

            instruction! {
                name: Sev,
                encoding: [
                    base: 0xBF40
                ]

                // todo: thumb2 encodings
            },

            // todo: ShAdd16 (thumbs only)
            // todo: ShAdd8 (thumbs only)

            // todo: ShasX (thumbs only)
            // todo: ShsaX (thumbs only)

            // todo: ShSub16 (thumbs only)
            // todo: ShSub8 (thumbs only)
            // todo: ShSub8 (thumbs only)

            // todo: SmulaBb (thumb2 only)
            // todo: SmulaBt (thumb2 only)
            // todo: SmulaTb (thumb2 only)
            // todo: SmulaTt (thumb2 only)
            // todo: SmulaD (thumb2 only)
            // todo: SmulaX (thumb2 only)
            // todo: SmulaL (thumb2 only)
            // todo: SmulaLbb (thumb2 only)
            // todo: SmulaLbt (thumb2 only)
            // todo: SmulaLtb (thumb2 only)
            // todo: SmulaLtt (thumb2 only)
            // todo: SmulaLd (thumb2 only)
            // todo: SmulaLx (thumb2 only)
            // todo: SmulaWb (thumb2 only)
            // todo: SmulaWt (thumb2 only)
            // todo: SmulSd (thumb2 only)
            // todo: SmulSdx (thumb2 only)
            // todo: SmulsLd (thumb2 only)
            // todo: SmulsLx (thumb2 only)
            // todo: SmmLa (thumb2 only)
            // todo: SmmLar (thumb2 only)
            // todo: SmmLs (thumb2 only)
            // todo: SmmLsr (thumb2 only)
            // todo: SmmuL (thumb2 only)
            // todo: SmmuLr (thumb2 only)
            // todo: SmulAd (thumb2 only)
            // todo: SmulAdx (thumb2 only)
            // todo: SmulBb (thumb2 only)
            // todo: SmulBt (thumb2 only)
            // todo: SmulTb (thumb2 only)
            // todo: SmulTt (thumb2 only)
            // todo: SmulL (thumb2 only)
            // todo: SmulWb (thumb2 only)
            // todo: SmulWt (thumb2 only)
            // todo: SmulSd (thumb2 only)
            // todo: SmulSdx (thumb2 only)

            // todo: Ssat (thumb2 only)
            // todo: Ssat16 (thumb2 only)
            
            // todo: SsaX (thumb2 only)
            
            // todo: SsBb (thumb2 only)
            
            // todo: Ssub16 (thumb2 only)
            // todo: Ssub8 (thumb2 only)
            // todo: Ssub8 (thumb2 only)

            // todo: Stc (thumb2 only)
            // todo: Stc2 (thumb2 only)

            instruction! {
                name: Stm, // Stmia, Stmea
                encoding: [
                    base: 0xC000,
                    operand: [rn, 3 << 8],
                    operand: [list, 8 << 0],
                ]

                // todo: thumb2 encodings
            },

            // todo: StmDb (thumb2 only)
            // todo: StmFd (thumb2 only)

            instruction! {
                name: StrImm,
                encoding: [
                    base: 0x6000,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: StrReg,
                encoding: [
                    base: 0x5000,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: StrbImm,
                encoding: [
                    base: 0x7000,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: StrbReg,
                encoding: [
                    base: 0x5400,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: StrBt (thumb2 only)

            // todo: StrBtImm (thumb2 only)

            // todo: StrEx (thumb2 only)

            // todo: StrExb (thumb2 only)

            // todo: StrExh (thumb2 only)

            instruction! {
                name: StrhImm,
                encoding: [
                    base: 0x8000,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: StrhReg,
                encoding: [
                    base: 0x5200,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: StrHt (thumb2 only)
            // todo: Strt (thumb2 only)

            instruction! {
                name: SubImm,
                encoding: [
                    base: 0x1E00,
                    operand: [imm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rdn, 3 << 0]
                ],
                encoding: [
                    base: 0x3800,
                    operand: [rdn, 3 << 8],
                    operand: [imm, 8 << 0],
                    operand: [rn, unused]
                ]
                
                // todo: thumb2 encodings
            },

            instruction! {
                name: SubReg,
                encoding: [
                    base: 0x1A00,
                    operand: [rm, 3 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: SubSpImm,
                encoding: [
                    base: 0xB080,
                    operand: [imm, 7 << 0]
                ]
                
                // todo: thumb2 encodings
            },
            
            // todo: SubSpReg (thumb2 only)

            instruction! {
                name: Svc,
                encoding: [
                    base: 0xDF00
                ]
            },

            // todo: SxtAb (thumb2 only)
            // todo: SxtAb16 (thumb2 only)
            // todo: SxtAh (thumb2 only)

            instruction! {
                name: Sxtb,
                encoding: [
                    base: 0xB240,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Sxtb16 (thumb2 only)
            
            instruction! {
                name: Sxth,
                encoding: [
                    base: 0xB200,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0],
                ]

                // todo: thumb2 encodings
            },

            // todo: Tbb (thumb2 only)
            // todo: Tbh (thumb2 only)

            // todo: TeqImm (thumb2 only)
            // todo: TeqReg (thumb2 only)
            // todo: TstImm (thumb2 only)

            instruction! {
                name: TstReg,
                encoding: [
                    base: 0x4200,
                    operand: [rm, 3 << 3],
                    operand: [rn, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Uadd16 (thumb2 only)
            // todo: Uadd8 (thumb2 only)

            // todo: UasX (thumb2 only)

            // todo: UbFx (thumb2 only)

            instruction! {
                name: Udf,
                encoding: [
                    base: 0xDE00,
                    operand: [imm, 8 << 0],
                ]

                // todo: thumb2 encodings
            },

            // todo: Udiv (thumb2 only)

            // todo: UhAdd16 (thumb2 only)
            // todo: UhAdd8 (thumb2 only)

            // todo: UhAsX (thumb2 only)            
            // todo: UhSaX (thumb2 only)
            
            // todo: UhSub16 (thumb2 only)
            // todo: UhSub8 (thumb2 only)

            // todo: UmulAaL (thumb2 only)
            // todo: UmulAl (thumb2 only)
            // todo: UmulL (thumb2 only)

            // todo: UqAdd16 (thumb2 only)
            // todo: UqAdd8 (thumb2 only)

            // todo: UqAsX (thumb2 only)
            // todo: UqSaX (thumb2 only)
            
            // todo: UqSub16 (thumb2 only)
            // todo: UqSub8 (thumb2 only)

            // todo: Usad8 (thumb2 only)
            // todo: UsadA16 (thumb2 only)

            // todo: Usat (thumb2 only)
            // todo: Usat16 (thumb2 only)

            // todo: UsaX (thumb2 only)
            // todo: Usub16 (thumb2 only)
            // todo: Usub8 (thumb2 only)

            // todo: UxtAb (thumb2 only)
            // todo: UxtAb16 (thumb2 only)
            // todo: UxtAh (thumb2 only)
            
            instruction! {
                name: Uxtb,
                encoding: [
                    base: 0xB2C0,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },

            // todo: Uxtb16 (thumb2 only)

            instruction! {
                name: Uxth,
                encoding: [
                    base: 0xB280,
                    operand: [rm, 3 << 3],
                    operand: [rd, 3 << 0]
                ]

                // todo: thumb2 encodings
            },



            // !!!
            // todo: Optional floating point module operations
            // !!!



            instruction! {
                name: Wfe,
                encoding: [
                    base: 0xBF20
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Wfi,
                encoding: [
                    base: 0xBF30
                ]

                // todo: thumb2 encodings
            },

            instruction! {
                name: Yield,
                encoding: [
                    base: 0xBF10
                ]

                // todo: thumb2 encodings
            },

            // 32 bit thumb2 instructions
            instruction! {
                name: Thumb2A,
                encoding: [
                    base: 0xE800,
                    operand: [high, 3 << 8],
                    operand: [low, 8 << 0]
                ]
            },

            instruction! {
                name: Thumb2B,
                encoding: [
                    base: 0xF000,
                    operand: [high, 3 << 8],
                    operand: [low, 8 << 0]
                ]
            },

            instruction! {
                name: Thumb2C,
                encoding: [
                    base: 0xF800,
                    operand: [high, 3 << 8],
                    operand: [low, 8 << 0]
                ]
            }
        }
    }
}
