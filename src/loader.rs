use std::fs::File;
use std::io::{ BufReader, Read };

type LoaderResult<T> = std::io::Result<T>;

trait ELFIdentParslet {
    fn parse_bytes(_: &mut dyn Read) -> LoaderResult<Self> where Self: Sized;
}

trait ELFParslet {
    fn parse_bytes(_: &mut dyn Read, format: ELFData, class: ELFClass) -> LoaderResult<Self> where Self: Sized;
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

                if $data_format == ELFData::BigEndian {
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

                if $data_format == ELFData::BigEndian {
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

                if $data_format == ELFData::BigEndian {
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

impl ELFIdentParslet for ELFMagic {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
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

impl ELFIdentParslet for ELFClass {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
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

impl ELFIdentParslet for ELFData {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
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

impl ELFIdentParslet for ELFIdentVersion {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
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

impl ELFIdentParslet for ELFOsAbi {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
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

impl ELFIdentParslet for ELFAbiVersion {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
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

impl ELFIdent {
    pub fn parse(reader: &mut dyn Read) -> LoaderResult<(ELFData, ELFClass, ELFIdent)> {
        let parsed = ELFIdent {
            magic: ELFMagic::parse_bytes(reader)?,
            class: ELFClass::parse_bytes(reader)?,
            data: ELFData::parse_bytes(reader)?,
            version: ELFIdentVersion::parse_bytes(reader)?,
            os_abi: ELFOsAbi::parse_bytes(reader)?,
            abi_ver: ELFAbiVersion::parse_bytes(reader)?,
        };

        // The end of the ident is composed of empty padding bytes, skip over them
        read_n_bytes!(reader, 7);

        Ok((parsed.data, parsed.class, parsed))
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
    fn parse_bytes(reader: &mut dyn Read, format: ELFData, _class: ELFClass) -> LoaderResult<Self> {
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
    fn parse_bytes(reader: &mut dyn Read, format: ELFData, _class: ELFClass) -> LoaderResult<Self> {
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
    fn parse_bytes(reader: &mut dyn Read, format: ELFData, _class: ELFClass) -> LoaderResult<Self> {
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
    fn parse_bytes(reader: &mut dyn Read, format: ELFData, _class: ELFClass) -> LoaderResult<Self> {
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
    fn parse_bytes(reader: &mut dyn Read, format: ELFData, class: ELFClass) -> LoaderResult<Self> {
        
        match class {
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
    fn parse_bytes(reader: &mut dyn Read, format: ELFData, class: ELFClass) -> LoaderResult<Self> {
        Ok(ELFSize(read_u16!(reader, format)))
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

impl ELFHeader {
    pub fn parse(reader: &mut dyn Read) -> LoaderResult<(ELFData, ELFClass, ELFHeader)> {
        let (format, class, ident) = ELFIdent::parse(reader)?;

        let parsed = ELFHeader {
            ident: ident,
            ty: ELFType::parse_bytes(reader, format, class)?,
            machine: ELFMachine::parse_bytes(reader, format, class)?,
            version: ELFVersion::parse_bytes(reader, format, class)?,
            entry: ELFAddress::parse_bytes(reader, format, class)?,
            phoff: ELFAddress::parse_bytes(reader, format, class)?,
            shoff: ELFAddress::parse_bytes(reader, format, class)?,
            flags: ELFFlags::parse_bytes(reader, format, class)?,
            ehsize: ELFSize::parse_bytes(reader, format, class)?,
            phentsize: ELFSize::parse_bytes(reader, format, class)?,
            phnum: ELFSize::parse_bytes(reader, format, class)?,
            shentsize: ELFSize::parse_bytes(reader, format, class)?,
            shnum: ELFSize::parse_bytes(reader, format, class)?,
            shstrndx: ELFSize::parse_bytes(reader, format, class)?,
        };

        Ok((format, class, parsed))
    } 
}


#[derive(Debug)]
struct ELF {
    header: ELFHeader,
}

impl ELF {
    pub fn parse(reader: &mut dyn Read) -> LoaderResult<ELF> {
        let (format, class, header) = ELFHeader::parse(reader)?;

        let parsed = ELF {
            header: header,
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
