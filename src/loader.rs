use std::fs::File;
use std::io::{ BufReader, Read };

type LoaderResult<T> = std::io::Result<T>;

trait ELFParslet {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, class: Option<ELFClass>) -> LoaderResult<Self> where Self: Sized;
}

const ELF_MAGIC_BYTES: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];

/// Helper macro for reading bytes into a fixed size array during parsing
macro_rules! read_n_bytes {
    ($reader:expr, $num:expr) => {
        {
            let mut __bytes: [u8; $num] = [0; $num];
            $reader.read(&mut __bytes)?;
            __bytes
        }
    };
}

macro_rules! read_byte {
    ($reader:expr) => {
        {
            let mut __bytes: [u8; 1] = [0; 1];
            $reader.read(&mut __bytes)?;
            __bytes[0]
        }
    };
}

macro_rules! read_u16 {
    ($reader:expr, $data_format:expr) => {
        {
            let mut __bytes: [u8; 2] = [0; 2];
            let mut __temp: u16 = 0;
            $reader.read(&mut __bytes)?;
            unsafe {
                __temp = std::mem::transmute::<[u8; 2], u16>(__bytes);

                if $data_format == Some(ELFData::BigEndian) {
                    __temp = __temp.to_le();
                }
            }
            __temp
        }
    };
}

macro_rules! read_u32 {
    ($reader:expr, $data_format:expr) => {
        {
            let mut __bytes: [u8; 4] = [0; 4];
            let mut __temp: u32 = 0;
            $reader.read(&mut __bytes)?;
            unsafe {
                __temp = std::mem::transmute::<[u8; 4], u32>(__bytes);

                if $data_format == Some(ELFData::BigEndian) {
                    __temp = __temp.to_le();
                }
            }
            __temp
        }
    };
}

macro_rules! read_u64 {
    ($reader:expr, $data_format:expr) => {
        {
            let mut __bytes: [u8; 8] = [0; 8];
            let mut __temp: u64 = 0;
            $reader.read(&mut __bytes)?;
            unsafe {
                __temp = std::mem::transmute::<[u8; 8], u64>(__bytes);

                if $data_format == Some(ELFData::BigEndian) {
                    __temp = __temp.to_le();
                }
            }
            __temp
        }
    };
}

#[derive(Debug)]
enum ELFMagic {
    Valid,
    Invalid([u8; 4]),
}

impl ELFParslet for ELFMagic {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_n_bytes!(reader, 4) {
            ELF_MAGIC_BYTES => Ok(ELFMagic::Valid),
            b => Ok(ELFMagic::Invalid(b))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ELFClass {
    ELF32,
    ELF64,
    Invalid(u8)
}

impl ELFParslet for ELFClass {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_byte!(reader) {
            0x01 => Ok(ELFClass::ELF32),
            0x02 => Ok(ELFClass::ELF64),
            b => Ok(ELFClass::Invalid(b))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ELFData {
    LittleEndian,
    BigEndian,
    Invalid(u8)
}

impl ELFParslet for ELFData {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_byte!(reader) {
            0x01 => Ok(ELFData::LittleEndian),
            0x02 => Ok(ELFData::BigEndian),
            b => Ok(ELFData::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFIdentVersion {
    Current,
    Invalid(u8)
}

impl ELFParslet for ELFIdentVersion {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_byte!(reader) {
            0x01 => Ok(ELFIdentVersion::Current), // ELF only has one version, version one. Nonetheless we parse it as "current"
            b => Ok(ELFIdentVersion::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFOsAbi {
    UNIXSystemV,
    Invalid(u8)
}

impl ELFParslet for ELFOsAbi {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_byte!(reader) {
            0x00 => Ok(ELFOsAbi::UNIXSystemV),
            b => Ok(ELFOsAbi::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFAbiVersion {
    Unspecified,
    Version(u8),
}

impl ELFParslet for ELFAbiVersion {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_byte!(reader) {
            0x00 => Ok(ELFAbiVersion::Unspecified),
            b => Ok(ELFAbiVersion::Version(b))
        }
    }
}

#[derive(Debug)]
struct ELFIdent {
    magic: ELFMagic,
    class: ELFClass,
    data: ELFData,
    version: ELFIdentVersion,
    os_abi: ELFOsAbi,
    abi_ver: ELFAbiVersion,
}

impl ELFParslet for ELFIdent {
    fn parse(reader: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        let parsed = ELFIdent {
            magic: ELFMagic::parse(reader, None, None)?,
            class: ELFClass::parse(reader, None, None)?,
            data: ELFData::parse(reader, None, None)?,
            version: ELFIdentVersion::parse(reader, None, None)?,
            os_abi: ELFOsAbi::parse(reader, None, None)?,
            abi_ver: ELFAbiVersion::parse(reader, None, None)?,
        };

        // The end of the ident is composed of empty padding bytes, skip over them
        read_n_bytes!(reader, 7);

        Ok(parsed)
    } 
}

#[derive(Debug)]
enum ELFType {
    None,
    Relocatable,
    Executable,
    Shared,
    Core,
    LoProc,
    HiProc,
    Invalid(u16),
}

impl ELFParslet for ELFType {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_u16!(reader, format) {
            0x0000 => Ok(ELFType::None),
            0x0001 => Ok(ELFType::Relocatable),
            0x0002 => Ok(ELFType::Executable),
            0x0003 => Ok(ELFType::Shared),
            0x0004 => Ok(ELFType::Core),
            0xFF00 => Ok(ELFType::LoProc),
            0xFFFF => Ok(ELFType::HiProc),
            b => Ok(ELFType::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFMachine {
    None,
    AtmelAVR,
    AMD64,
    ARM,
    ST200,
    RISCV,
    Invalid(u16),
}

impl ELFParslet for ELFMachine {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_u16!(reader, format) {
            0x0000 => Ok(ELFMachine::None),
            0x0028 => Ok(ELFMachine::ARM),
            0x0053 => Ok(ELFMachine::AtmelAVR),
            0x003E => Ok(ELFMachine::AMD64),
            0x0064 => Ok(ELFMachine::ST200),
            0x00F3 => Ok(ELFMachine::RISCV),
            b => Ok(ELFMachine::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFVersion {
    Current,
    Invalid(u32)
}

impl ELFParslet for ELFVersion {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_u32!(reader, format) {
            0x01 => Ok(ELFVersion::Current),
            b => Ok(ELFVersion::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFFlags {
    Flags(u32)
}

impl ELFParslet for ELFFlags {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        match read_u32!(reader, format) {
            v => Ok(ELFFlags::Flags(v)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ELFAddress {
    ELF32Addr(u32),
    ELF64Addr(u64)
}

impl ELFParslet for ELFAddress {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, class: Option<ELFClass>) -> LoaderResult<Self> {
        match class.unwrap() {
            ELFClass::ELF32 => {
                Ok(ELFAddress::ELF32Addr(read_u32!(reader, format)))
            },
            ELFClass::ELF64 => {
                Ok(ELFAddress::ELF64Addr(read_u64!(reader, format)))
            },
            ELFClass::Invalid(e) => {
                panic!("Attempted to parse ELF address with an invalid ELF class: {:?}", e);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ELFSize(u16);

impl ELFParslet for ELFSize {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        Ok(ELFSize(read_u16!(reader, format)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ELFWord(u32);

impl ELFParslet for ELFWord {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        Ok(ELFWord(read_u32!(reader, format)))
    }
}

#[derive(Debug)]
struct ELFHeader {
    ident: ELFIdent,
    ty: ELFType,
    machine: ELFMachine,
    version: ELFVersion,
    entry: ELFAddress,
    phoff: ELFAddress,
    shoff: ELFAddress,
    flags: ELFFlags,
    ehsize: ELFSize,
    phentsize: ELFSize,
    phnum: ELFSize,
    shentsize: ELFSize,
    shnum: ELFSize,
    shstrndx: ELFSize,
}

impl ELFParslet for ELFHeader {
    fn parse(reader: &mut dyn Read, format: Option<ELFData>, class: Option<ELFClass>) -> LoaderResult<Self> {
        let ident = ELFIdent::parse(reader, format, class)?;

        let format = Some(ident.data);
        let class = Some(ident.class);

        let header = ELFHeader {
            ident: ident,
            ty: ELFType::parse(reader, format, class)?,
            machine: ELFMachine::parse(reader, format, class)?,
            version: ELFVersion::parse(reader, format, class)?,
            entry: ELFAddress::parse(reader, format, class)?,
            phoff: ELFAddress::parse(reader, format, class)?,
            shoff: ELFAddress::parse(reader, format, class)?,
            flags: ELFFlags::parse(reader, format, class)?,
            ehsize: ELFSize::parse(reader, format, class)?,
            phentsize: ELFSize::parse(reader, format, class)?,
            phnum: ELFSize::parse(reader, format, class)?,
            shentsize: ELFSize::parse(reader, format, class)?,
            shnum: ELFSize::parse(reader, format, class)?,
            shstrndx: ELFSize::parse(reader, format, class)?,
        };

        Ok(header)
    } 
}

#[derive(Debug)]
struct ELFProgramHeader {
}

impl ELFParslet for ELFProgramHeader {
    fn parse(_: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum ELFSectionHeaderType {
    NULL,
    ProgramData,
    SymbolTable,
    StringTable,
    RelocationWithAddends,
    SymbolHashTable,
    DynamicInfo,
    Note,
    NoBits,
    Relocation,
    ShLib,
    DynamicSymbolTable,
    InitArray,
    FiniArray,
    PreInitArray,
    Group,
    ExtendedSectionIndices,
}

#[derive(Debug)]
struct ELFSectionHeader {
    name: ELFWord,

}

impl ELFParslet for ELFSectionHeader {
    fn parse(_: &mut dyn Read, _: Option<ELFData>, _: Option<ELFClass>) -> LoaderResult<Self> {
        unimplemented!()
    }
}

/**
 * ELF
 * 
 * Represents an ELF (Executable and Linkable Format) file.
 * 
 * The Executable and Linkable Format is a common standard file format for executable files, object code,
 * shared libraries, and core dumps.
 * 
 * This type is responsible for loading, parsing, and modifying ELF files, and is used by the ARM
 * program loader to construct an executable image.
 *  
 */
#[derive(Debug)]
struct ELF {
    header: ELFHeader,
    section_headers: Vec<ELFSectionHeader>,
    program_headers: Vec<ELFProgramHeader>,
}

impl ELF {
    pub fn parse(reader: &mut dyn Read) -> LoaderResult<ELF> {
        let header = ELFHeader::parse(reader, None, None)?;

        let _format = Some(header.ident.data);
        let _class = Some(header.ident.class);

        let parsed = ELF {
            header: header,
            section_headers: Vec::new(),
            program_headers: Vec::new(),
        };

        Ok(parsed)
    }
}

#[derive(Debug)]
pub struct ProgramLoader {
}

impl ProgramLoader {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> LoaderResult<ProgramLoader> where P: std::fmt::Debug {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        println!("Loading {:?} ...", path);
        let elf = ELF::parse(&mut reader);
        println!("{:#?}", elf.unwrap());

        Ok(ProgramLoader{ })
    }
}
