use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::{ BufReader, Read, Seek, SeekFrom };

type LoaderResult<T> = std::io::Result<T>;

trait ELFParslet {
    fn parse_bytes(_: &mut dyn Read) -> LoaderResult<Self> where Self: Sized;
}

#[derive(Debug)]
pub struct ProgramLoader {
}

const ELF_MAGIC_BYTES: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];

/// Helper macro for reading bytes into a fixed size array during parsing
macro_rules! read_bytes {
    ($reader:expr, $num:expr) => {
        {
            let mut __bytes: [u8; $num] = [0; $num];
            $reader.read(&mut __bytes)?;
            __bytes
        }
    };
}

#[derive(Debug)]
enum ELFMagic {
    Valid,
    Invalid([u8; 4]),
}

impl ELFParslet for ELFMagic {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 4) {
            ELF_MAGIC_BYTES => Ok(ELFMagic::Valid),
            b => Ok(ELFMagic::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFClass {
    ELF32,
    ELF64,
    Invalid([u8; 1])
}

impl ELFParslet for ELFClass {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 1) {
            [0x01] => Ok(ELFClass::ELF32),
            [0x02] => Ok(ELFClass::ELF64),
            b => Ok(ELFClass::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFData {
    LittleEndian,
    BigEndian,
    Invalid([u8; 1])
}

impl ELFParslet for ELFData {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 1) {
            [0x01] => Ok(ELFData::LittleEndian),
            [0x02] => Ok(ELFData::BigEndian),
            b => Ok(ELFData::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFVersion {
    Current,
    Invalid([u8; 1])
}

impl ELFParslet for ELFVersion {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 1) {
            [0x01] => Ok(ELFVersion::Current), // ELF only has one version, version one. Nonetheless we parse it as "current"
            b => Ok(ELFVersion::Invalid(b))
        }
    }
}

#[derive(Debug)]
enum ELFOsAbi {
    UNIXSystemV,
    Invalid([u8; 1])
}

impl ELFParslet for ELFOsAbi {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 1) {
            [0x00] => Ok(ELFOsAbi::UNIXSystemV),
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
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 1) {
            [0x00] => Ok(ELFAbiVersion::Unspecified),
            b => Ok(ELFAbiVersion::Version(b[0]))
        }
    }
}

#[derive(Debug)]
enum ELFType {
    Executable,
    Invalid([u8; 2]),
}

impl ELFParslet for ELFType {
    fn parse_bytes(reader: &mut dyn Read) -> LoaderResult<Self> {
        match read_bytes!(reader, 2) {
            [0x02, 0x00] => Ok(ELFType::Executable),
            b => Ok(ELFType::Invalid(b))
        }
    }
}

#[derive(Debug)]
struct ELFIdent {
    magic: ELFMagic,
    class: ELFClass,
    data: ELFData,
    version: ELFVersion,
    os_abi: ELFOsAbi,
    abi_ver: ELFAbiVersion,
}

impl ELFIdent {
    pub fn parse(reader: &mut dyn Read) -> LoaderResult<ELFIdent> {
        let parsed = ELFIdent {
            magic: ELFMagic::parse_bytes(reader)?,
            class: ELFClass::parse_bytes(reader)?,
            data: ELFData::parse_bytes(reader)?,
            version: ELFVersion::parse_bytes(reader)?,
            os_abi: ELFOsAbi::parse_bytes(reader)?,
            abi_ver: ELFAbiVersion::parse_bytes(reader)?,
        };

        // The end of the ident is composed of empty padding bytes, skip over them
        read_bytes!(reader, 7);

        Ok(parsed)
    } 
}

#[derive(Debug)]
struct ELF {
    ident: ELFIdent,
    etype: ELFType,
}

impl ELF {
    pub fn parse(reader: &mut dyn Read) -> LoaderResult<ELF> {
        let parsed = ELF {
            ident: ELFIdent::parse(reader)?,
            etype: ELFType::parse_bytes(reader)?,
        };

        Ok(parsed)
    } 
}

impl ProgramLoader {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> LoaderResult<ProgramLoader> where P: std::fmt::Debug {
        let file = File::open(&path)?;
        let meta = file.metadata()?;
        let mut reader = BufReader::new(file);

        println!("Loading {:?} ...", path);
        let elf = ELF::parse(&mut reader);
        println!("{:#?}", elf.unwrap());

        Ok(ProgramLoader{ })
    }
}
