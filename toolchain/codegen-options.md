# `rustc` codegen options
```bash
rustc -C help

Available codegen options:

    -C                       ar=val -- this option is deprecated and does nothing
    -C               code-model=val -- choose the code model to use (`rustc --print code-models` for details)
    -C            codegen-units=val -- divide crate into N units to optimize in parallel
    -C       control-flow-guard=val -- use Windows Control Flow Guard (default: no)
    -C         debug-assertions=val -- explicitly enable the `cfg(debug_assertions)` directive
    -C                debuginfo=val -- debug info emission level (0-2, none, line-directives-only, line-tables-only, limited, or full; default: 0)
    -C default-linker-libraries=val -- allow the linker to link its default libraries (default: no)
    -C                  dlltool=val -- import library generation tool (ignored except when targeting windows-gnu)
    -C            embed-bitcode=val -- emit bitcode in rlibs (default: yes)
    -C           extra-filename=val -- extra data to put in each output filename
    -C     force-frame-pointers=val -- force use of the frame pointers
    -C      force-unwind-tables=val -- force use of unwind tables
    -C              incremental=val -- enable incremental compilation
    -C         inline-threshold=val -- set the threshold for inlining a function
    -C      instrument-coverage=val -- instrument the generated code to support LLVM source-based code coverage reports (note, the compiler build config must include `profiler = true`); implies `-C symbol-mangling-version=v0`. Optional values are:
        `=all` (implicit value)
        `=except-unused-generics`
        `=except-unused-functions`
        `=off` (default)
    -C                 link-arg=val -- a single extra argument to append to the linker invocation (can be used several times)
    -C                link-args=val -- extra arguments to append to the linker invocation (space separated)
    -C           link-dead-code=val -- keep dead code at link time (useful for code coverage) (default: no)
    -C      link-self-contained=val -- control whether to link Rust provided C objects/libraries or rely
        on C toolchain installed in the system
    -C                   linker=val -- system linker to link outputs with
    -C            linker-flavor=val -- linker flavor
    -C        linker-plugin-lto=val -- generate build artifacts that are compatible with linker-based LTO
    -C                llvm-args=val -- a list of arguments to pass to LLVM (space separated)
    -C                      lto=val -- perform LLVM link-time optimizations
    -C                 metadata=val -- metadata to mangle symbol names with
    -C    no-prepopulate-passes=val -- give an empty list of passes to the pass manager
    -C               no-redzone=val -- disable the use of the redzone
    -C           no-stack-check=val -- this option is deprecated and does nothing
    -C       no-vectorize-loops=val -- disable loop vectorization optimization passes
    -C         no-vectorize-slp=val -- disable LLVM's SLP vectorization pass
    -C                opt-level=val -- optimization level (0-3, s, or z; default: 0)
    -C          overflow-checks=val -- use overflow checks for integer arithmetic
    -C                    panic=val -- panic strategy to compile crate with
    -C                   passes=val -- a list of extra LLVM passes to run (space separated)
    -C           prefer-dynamic=val -- prefer dynamic linking to static linking (default: no)
    -C         profile-generate=val -- compile the program with profiling instrumentation
    -C              profile-use=val -- use the given `.profdata` file for profile-guided optimization
    -C         relocation-model=val -- control generation of position-independent code (PIC) (`rustc --print relocation-models` for details)
    -C                   remark=val -- print remarks for these optimization passes (space separated, or "all")
    -C                    rpath=val -- set rpath values in libs/exes (default: no)
    -C               save-temps=val -- save all temporary output files during compilation (default: no)
    -C               soft-float=val -- use soft float ABI (*eabihf targets only) (default: no)
    -C          split-debuginfo=val -- how to handle split-debuginfo, a platform-specific option
    -C                    strip=val -- tell the linker which information to strip (`none` (default), `debuginfo` or `symbols`)
    -C  symbol-mangling-version=val -- which mangling version to use for symbol names ('legacy' (default) or 'v0')
    -C               target-cpu=val -- select target processor (`rustc --print target-cpus` for details)
    -C           target-feature=val -- target specific attributes. (`rustc --print target-features` for details). This feature is unsafe.
```

<br>

## Target specific codegen options: `target-features`
```bash
rustc --print=target-features
Features supported by rustc for this target:
    aes                                - Enable AES support (FEAT_AES, FEAT_PMULL).
    bf16                               - Enable BFloat16 Extension (FEAT_BF16).
    bti                                - Enable Branch Target Identification (FEAT_BTI).
    crc                                - Enable ARMv8 CRC-32 checksum instructions (FEAT_CRC32).
    dit                                - Enable v8.4-A Data Independent Timing instructions (FEAT_DIT).
    dotprod                            - Enable dot product support (FEAT_DotProd).
    dpb                                - Enable v8.2 data Cache Clean to Point of Persistence (FEAT_DPB).
    dpb2                               - Enable v8.5 Cache Clean to Point of Deep Persistence (FEAT_DPB2).
    f32mm                              - Enable Matrix Multiply FP32 Extension (FEAT_F32MM).
    f64mm                              - Enable Matrix Multiply FP64 Extension (FEAT_F64MM).
    fcma                               - Enable v8.3-A Floating-point complex number support (FEAT_FCMA).
    fhm                                - Enable FP16 FML instructions (FEAT_FHM).
    flagm                              - Enable v8.4-A Flag Manipulation Instructions (FEAT_FlagM).
    fp16                               - Full FP16 (FEAT_FP16).
    frintts                            - Enable FRInt[32|64][Z|X] instructions that round a floating-point number to an integer (in FP format) forcing it to fit into a 32- or 64-bit int (FEAT_FRINTTS).
    i8mm                               - Enable Matrix Multiply Int8 Extension (FEAT_I8MM).
    jsconv                             - Enable v8.3-A JavaScript FP conversion instructions (FEAT_JSCVT).
    lor                                - Enables ARM v8.1 Limited Ordering Regions extension (FEAT_LOR).
    lse                                - Enable ARMv8.1 Large System Extension (LSE) atomic instructions (FEAT_LSE).
    mte                                - Enable Memory Tagging Extension (FEAT_MTE, FEAT_MTE2).
    neon                               - Enable Advanced SIMD instructions (FEAT_AdvSIMD).
    paca                               - Enable v8.3-A Pointer Authentication extension (FEAT_PAuth).
    pacg                               - Enable v8.3-A Pointer Authentication extension (FEAT_PAuth).
    pan                                - Enables ARM v8.1 Privileged Access-Never extension (FEAT_PAN).
    pmuv3                              - Enable Code Generation for ARMv8 PMUv3 Performance Monitors extension (FEAT_PMUv3).
    rand                               - Enable Random Number generation instructions (FEAT_RNG).
    ras                                - Enable ARMv8 Reliability, Availability and Serviceability Extensions (FEAT_RAS, FEAT_RASv1p1).
    rcpc                               - Enable support for RCPC extension (FEAT_LRCPC).
    rcpc2                              - Enable v8.4-A RCPC instructions with Immediate Offsets (FEAT_LRCPC2).
    rdm                                - Enable ARMv8.1 Rounding Double Multiply Add/Subtract instructions (FEAT_RDM).
    sb                                 - Enable v8.5 Speculation Barrier (FEAT_SB).
    sha2                               - Enable SHA1 and SHA256 support (FEAT_SHA1, FEAT_SHA256).
    sha3                               - Enable SHA512 and SHA3 support (FEAT_SHA3, FEAT_SHA512).
    sm4                                - Enable SM3 and SM4 support (FEAT_SM4, FEAT_SM3).
    spe                                - Enable Statistical Profiling extension (FEAT_SPE).
    ssbs                               - Enable Speculative Store Bypass Safe bit (FEAT_SSBS, FEAT_SSBS2).
    sve                                - Enable Scalable Vector Extension (SVE) instructions (FEAT_SVE).
    sve2                               - Enable Scalable Vector Extension 2 (SVE2) instructions (FEAT_SVE2).
    sve2-aes                           - Enable AES SVE2 instructions (FEAT_SVE_AES, FEAT_SVE_PMULL128).
    sve2-bitperm                       - Enable bit permutation SVE2 instructions (FEAT_SVE_BitPerm).
    sve2-sha3                          - Enable SHA3 SVE2 instructions (FEAT_SVE_SHA3).
    sve2-sm4                           - Enable SM4 SVE2 instructions (FEAT_SVE_SM4).
    tme                                - Enable Transactional Memory Extension (FEAT_TME).
    v8.1a                              - Support ARM v8.1a instructions.
    v8.2a                              - Support ARM v8.2a instructions.
    v8.3a                              - Support ARM v8.3a instructions.
    v8.4a                              - Support ARM v8.4a instructions.
    v8.5a                              - Support ARM v8.5a instructions.
    v8.6a                              - Support ARM v8.6a instructions.
    v8.7a                              - Support ARM v8.7a instructions.
    vh                                 - Enables ARM v8.1 Virtual Host extension (FEAT_VHE).
    crt-static                         - Enables C Run-time Libraries to be statically linked.

Code-generation features supported by LLVM for this target:
    CONTEXTIDREL2                      - Enable RW operand CONTEXTIDR_EL2.
    a35                                - Cortex-A35 ARM processors.
    a510                               - Cortex-A510 ARM processors.
    a53                                - Cortex-A53 ARM processors.
    a55                                - Cortex-A55 ARM processors.
    a57                                - Cortex-A57 ARM processors.
    a64fx                              - Fujitsu A64FX processors.
    a65                                - Cortex-A65 ARM processors.
    a710                               - Cortex-A710 ARM processors.
    a715                               - Cortex-A715 ARM processors.
    a72                                - Cortex-A72 ARM processors.
    a73                                - Cortex-A73 ARM processors.
    a75                                - Cortex-A75 ARM processors.
    a76                                - Cortex-A76 ARM processors.
    a77                                - Cortex-A77 ARM processors.
    a78                                - Cortex-A78 ARM processors.
    a78c                               - Cortex-A78C ARM processors.
    aggressive-fma                     - Enable Aggressive FMA for floating-point..
    all                                - Enable all instructions.
    alternate-sextload-cvt-f32-pattern - Use alternative pattern for sextload convert to f32.
    altnzcv                            - Enable alternative NZCV format for floating point comparisons (FEAT_FlagM2).
    am                                 - Enable v8.4-A Activity Monitors extension (FEAT_AMUv1).
    ampere1                            - Ampere Computing Ampere-1 processors.
    ampere1a                           - Ampere Computing Ampere-1A processors.
    amvs                               - Enable v8.6-A Activity Monitors Virtualization support (FEAT_AMUv1p1).
    apple-a10                          - Apple A10.
    apple-a11                          - Apple A11.
    apple-a12                          - Apple A12.
    apple-a13                          - Apple A13.
    apple-a14                          - Apple A14.
    apple-a15                          - Apple A15.
    apple-a16                          - Apple A16.
    apple-a7                           - Apple A7 (the CPU formerly known as Cyclone).
    apple-a7-sysreg                    - Apple A7 (the CPU formerly known as Cyclone).
    arith-bcc-fusion                   - CPU fuses arithmetic+bcc operations.
    arith-cbz-fusion                   - CPU fuses arithmetic + cbz/cbnz operations.
    ascend-store-address               - Schedule vector stores by ascending address.
    b16b16                             - Enable SVE2.1 or SME2.1 non-widening BFloat16 to BFloat16 instructions (FEAT_B16B16).
    balance-fp-ops                     - balance mix of odd and even D-registers for fp multiply(-accumulate) ops.
    brbe                               - Enable Branch Record Buffer Extension (FEAT_BRBE).
    call-saved-x10                     - Make X10 callee saved..
    call-saved-x11                     - Make X11 callee saved..
    call-saved-x12                     - Make X12 callee saved..
    call-saved-x13                     - Make X13 callee saved..
    call-saved-x14                     - Make X14 callee saved..
    call-saved-x15                     - Make X15 callee saved..
    call-saved-x18                     - Make X18 callee saved..
    call-saved-x8                      - Make X8 callee saved..
    call-saved-x9                      - Make X9 callee saved..
    carmel                             - Nvidia Carmel processors.
    ccidx                              - Enable v8.3-A Extend of the CCSIDR number of sets (FEAT_CCIDX).
    clrbhb                             - Enable Clear BHB instruction (FEAT_CLRBHB).
    cmp-bcc-fusion                     - CPU fuses cmp+bcc operations.
    cortex-r82                         - Cortex-R82 ARM processors.
    cortex-x1                          - Cortex-X1 ARM processors.
    cortex-x2                          - Cortex-X2 ARM processors.
    cortex-x3                          - Cortex-X3 ARM processors.
    crypto                             - Enable cryptographic instructions.
    cssc                               - Enable Common Short Sequence Compression (CSSC) instructions (FEAT_CSSC).
    custom-cheap-as-move               - Use custom handling of cheap instructions.
    d128                               - Enable Armv9.4-A 128-bit Page Table Descriptors, System Registers and Instructions (FEAT_D128, FEAT_LVA3, FEAT_SYSREG128, FEAT_SYSINSTR128).
    disable-latency-sched-heuristic    - Disable latency scheduling heuristic.
    ecv                                - Enable enhanced counter virtualization extension (FEAT_ECV).
    el2vmsa                            - Enable Exception Level 2 Virtual Memory System Architecture.
    el3                                - Enable Exception Level 3.
    enable-select-opt                  - Enable the select optimize pass for select loop heuristics.
    ete                                - Enable Embedded Trace Extension (FEAT_ETE).
    exynos-cheap-as-move               - Use Exynos specific handling of cheap instructions.
    exynosm3                           - Samsung Exynos-M3 processors.
    exynosm4                           - Samsung Exynos-M4 processors.
    falkor                             - Qualcomm Falkor processors.
    fgt                                - Enable fine grained virtualization traps extension (FEAT_FGT).
    fix-cortex-a53-835769              - Mitigate Cortex-A53 Erratum 835769.
    fmv                                - Enable Function Multi Versioning support..
    force-32bit-jump-tables            - Force jump table entries to be 32-bits wide except at MinSize.
    fp-armv8                           - Enable ARMv8 FP (FEAT_FP).
    fuse-address                       - CPU fuses address generation and memory operations.
    fuse-adrp-add                      - CPU fuses adrp+add operations.
    fuse-aes                           - CPU fuses AES crypto operations.
    fuse-arith-logic                   - CPU fuses arithmetic and logic operations.
    fuse-crypto-eor                    - CPU fuses AES/PMULL and EOR operations.
    fuse-csel                          - CPU fuses conditional select operations.
    fuse-literals                      - CPU fuses literal generation operations.
    harden-sls-blr                     - Harden against straight line speculation across BLR instructions.
    harden-sls-nocomdat                - Generate thunk code for SLS mitigation in the normal text section.
    harden-sls-retbr                   - Harden against straight line speculation across RET and BR instructions.
    hbc                                - Enable Armv8.8-A Hinted Conditional Branches Extension (FEAT_HBC).
    hcx                                - Enable Armv8.7-A HCRX_EL2 system register (FEAT_HCX).
    ite                                - Enable Armv9.4-A Instrumentation Extension FEAT_ITE.
    kryo                               - Qualcomm Kryo processors.
    ls64                               - Enable Armv8.7-A LD64B/ST64B Accelerator Extension (FEAT_LS64, FEAT_LS64_V, FEAT_LS64_ACCDATA).
    lse128                             - Enable Armv9.4-A 128-bit Atomic Instructions (FEAT_LSE128).
    lse2                               - Enable ARMv8.4 Large System Extension 2 (LSE2) atomicity rules (FEAT_LSE2).
    lsl-fast                           - CPU has a fastpath logical shift of up to 3 places.
    mec                                - Enable Memory Encryption Contexts Extension.
    mops                               - Enable Armv8.8-A memcpy and memset acceleration instructions (FEAT_MOPS).
    mpam                               - Enable v8.4-A Memory system Partitioning and Monitoring extension (FEAT_MPAM).
    neoverse512tvb                     - Neoverse 512-TVB ARM processors.
    neoversee1                         - Neoverse E1 ARM processors.
    neoversen1                         - Neoverse N1 ARM processors.
    neoversen2                         - Neoverse N2 ARM processors.
    neoversev1                         - Neoverse V1 ARM processors.
    neoversev2                         - Neoverse V2 ARM processors.
    nmi                                - Enable Armv8.8-A Non-maskable Interrupts (FEAT_NMI, FEAT_GICv3_NMI).
    no-bti-at-return-twice             - Don't place a BTI instruction after a return-twice.
    no-neg-immediates                  - Convert immediates and instructions to their negated or complemented equivalent when the immediate does not fit in the encoding..
    no-zcz-fp                          - Has no zero-cycle zeroing instructions for FP registers.
    nv                                 - Enable v8.4-A Nested Virtualization Enchancement (FEAT_NV, FEAT_NV2).
    outline-atomics                    - Enable out of line atomics to support LSE instructions.
    pan-rwv                            - Enable v8.2 PAN s1e1R and s1e1W Variants (FEAT_PAN2).
    predictable-select-expensive       - Prefer likely predicted branches over selects.
    predres                            - Enable v8.5a execution and data prediction invalidation instructions (FEAT_SPECRES).
    prfm-slc-target                    - Enable SLC target for PRFM instruction.
    rasv2                              - Enable ARMv8.9-A Reliability, Availability and Serviceability Extensions (FEAT_RASv2).
    rcpc3                              - Enable Armv8.9-A RCPC instructions for A64 and Advanced SIMD and floating-point instruction set (FEAT_LRCPC3).
    reserve-x1                         - Reserve X1, making it unavailable as a GPR.
    reserve-x10                        - Reserve X10, making it unavailable as a GPR.
    reserve-x11                        - Reserve X11, making it unavailable as a GPR.
    reserve-x12                        - Reserve X12, making it unavailable as a GPR.
    reserve-x13                        - Reserve X13, making it unavailable as a GPR.
    reserve-x14                        - Reserve X14, making it unavailable as a GPR.
    reserve-x15                        - Reserve X15, making it unavailable as a GPR.
    reserve-x18                        - Reserve X18, making it unavailable as a GPR.
    reserve-x2                         - Reserve X2, making it unavailable as a GPR.
    reserve-x20                        - Reserve X20, making it unavailable as a GPR.
    reserve-x21                        - Reserve X21, making it unavailable as a GPR.
    reserve-x22                        - Reserve X22, making it unavailable as a GPR.
    reserve-x23                        - Reserve X23, making it unavailable as a GPR.
    reserve-x24                        - Reserve X24, making it unavailable as a GPR.
    reserve-x25                        - Reserve X25, making it unavailable as a GPR.
    reserve-x26                        - Reserve X26, making it unavailable as a GPR.
    reserve-x27                        - Reserve X27, making it unavailable as a GPR.
    reserve-x28                        - Reserve X28, making it unavailable as a GPR.
    reserve-x3                         - Reserve X3, making it unavailable as a GPR.
    reserve-x30                        - Reserve X30, making it unavailable as a GPR.
    reserve-x4                         - Reserve X4, making it unavailable as a GPR.
    reserve-x5                         - Reserve X5, making it unavailable as a GPR.
    reserve-x6                         - Reserve X6, making it unavailable as a GPR.
    reserve-x7                         - Reserve X7, making it unavailable as a GPR.
    reserve-x9                         - Reserve X9, making it unavailable as a GPR.
    rme                                - Enable Realm Management Extension (FEAT_RME).
    saphira                            - Qualcomm Saphira processors.
    sel2                               - Enable v8.4-A Secure Exception Level 2 extension (FEAT_SEL2).
    slow-misaligned-128store           - Misaligned 128 bit stores are slow.
    slow-paired-128                    - Paired 128 bit loads and stores are slow.
    slow-strqro-store                  - STR of Q register with register offset is slow.
    sme                                - Enable Scalable Matrix Extension (SME) (FEAT_SME).
    sme-f16f16                         - Enable SME2.1 non-widening Float16 instructions (FEAT_SME_F16F16).
    sme-f64f64                         - Enable Scalable Matrix Extension (SME) F64F64 instructions (FEAT_SME_F64F64).
    sme-i16i64                         - Enable Scalable Matrix Extension (SME) I16I64 instructions (FEAT_SME_I16I64).
    sme2                               - Enable Scalable Matrix Extension 2 (SME2) instructions.
    sme2p1                             - Enable Scalable Matrix Extension 2.1 (FEAT_SME2p1) instructions.
    spe-eef                            - Enable extra register in the Statistical Profiling Extension (FEAT_SPEv1p2).
    specres2                           - Enable Speculation Restriction Instruction (FEAT_SPECRES2).
    specrestrict                       - Enable architectural speculation restriction (FEAT_CSV2_2).
    strict-align                       - Disallow all unaligned memory access.
    sve2p1                             - Enable Scalable Vector Extension 2.1 instructions.
    tagged-globals                     - Use an instruction sequence for taking the address of a global that allows a memory tag in the upper address bits.
    the                                - Enable Armv8.9-A Translation Hardening Extension (FEAT_THE).
    thunderx                           - Cavium ThunderX processors.
    thunderx2t99                       - Cavium ThunderX2 processors.
    thunderx3t110                      - Marvell ThunderX3 processors.
    thunderxt81                        - Cavium ThunderX processors.
    thunderxt83                        - Cavium ThunderX processors.
    thunderxt88                        - Cavium ThunderX processors.
    tlb-rmi                            - Enable v8.4-A TLB Range and Maintenance Instructions (FEAT_TLBIOS, FEAT_TLBIRANGE).
    tpidr-el1                          - Permit use of TPIDR_EL1 for the TLS base.
    tpidr-el2                          - Permit use of TPIDR_EL2 for the TLS base.
    tpidr-el3                          - Permit use of TPIDR_EL3 for the TLS base.
    tracev8.4                          - Enable v8.4-A Trace extension (FEAT_TRF).
    trbe                               - Enable Trace Buffer Extension (FEAT_TRBE).
    tsv110                             - HiSilicon TS-V110 processors.
    uaops                              - Enable v8.2 UAO PState (FEAT_UAO).
    use-experimental-zeroing-pseudos   - Hint to the compiler that the MOVPRFX instruction is merged with destructive operations.
    use-postra-scheduler               - Schedule again after register allocation.
    use-reciprocal-square-root         - Use the reciprocal square root approximation.
    use-scalar-inc-vl                  - Prefer inc/dec over add+cnt.
    v8.8a                              - Support ARM v8.8a instructions.
    v8.9a                              - Support ARM v8.9a instructions.
    v8a                                - Support ARM v8.0a instructions.
    v8r                                - Support ARM v8r instructions.
    v9.1a                              - Support ARM v9.1a instructions.
    v9.2a                              - Support ARM v9.2a instructions.
    v9.3a                              - Support ARM v9.3a instructions.
    v9.4a                              - Support ARM v9.4a instructions.
    v9a                                - Support ARM v9a instructions.
    wfxt                               - Enable Armv8.7-A WFET and WFIT instruction (FEAT_WFxT).
    xs                                 - Enable Armv8.7-A limited-TLB-maintenance instruction (FEAT_XS).
    zcm                                - Has zero-cycle register moves.
    zcz                                - Has zero-cycle zeroing instructions.
    zcz-fp-workaround                  - The zero-cycle floating-point zeroing instruction has a bug.
    zcz-gp                             - Has zero-cycle zeroing instructions for generic registers.

Use +feature to enable a feature, or -feature to disable it.
For example, rustc -C target-cpu=mycpu -C target-feature=+feature1,-feature2

Code-generation features cannot be used in cfg or #[target_feature],
and may be renamed or removed in a future version of LLVM or rustc.
```