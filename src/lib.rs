use std::{error, fmt, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum StatLine {
    CpuAggregate(CpuMetric),
    Cpu(u32, CpuMetric),
}

impl std::str::FromStr for StatLine {
    type Err = StatLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let line_type = split.next().unwrap();
        match line_type {
            "cpu" => Ok(StatLine::CpuAggregate(CpuMetric::build(split)?)),
            other => {
                let index = other
                    .parse::<CpuPrefix>()
                    .map_err(|_| StatLineParseError::UnkownLineType(other.to_string()))?
                    .0;
                Ok(StatLine::Cpu(index, CpuMetric::build(split)?))
            }
        }
    }
}

#[derive(Debug)]
pub enum StatLineParseError {
    UnkownLineType(String),
    CpuMetricError(CpuMetricParseError),
}

impl fmt::Display for StatLineParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatLineParseError::UnkownLineType(line_type) => {
                write!(f, "unknown line type: {}", line_type)
            }
            StatLineParseError::CpuMetricError(err) => write!(f, "cpu metric error: {}", err),
        }
    }
}

impl std::error::Error for StatLineParseError {}

impl From<CpuMetricParseError> for StatLineParseError {
    fn from(value: CpuMetricParseError) -> Self {
        StatLineParseError::CpuMetricError(value)
    }
}

struct CpuPrefix(u32);

impl std::str::FromStr for CpuPrefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index: u32 = s.strip_prefix("cpu").ok_or(())?.parse().map_err(|_| ())?;
        Ok(CpuPrefix(index))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CpuMetric {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
}

impl CpuMetric {
    fn consume<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<u64, CpuMetricParseError> {
        Ok(iter
            .next()
            .ok_or(CpuMetricParseError::MissingField)?
            .parse::<u64>()?)
    }

    fn build<'a>(
        mut iter: impl Iterator<Item = &'a str>,
    ) -> Result<CpuMetric, CpuMetricParseError> {
        let user = Self::consume(&mut iter)?;
        let nice = Self::consume(&mut iter)?;
        let system = Self::consume(&mut iter)?;
        let idle = Self::consume(&mut iter)?;
        let iowait = Self::consume(&mut iter)?;
        let irq = Self::consume(&mut iter)?;
        let softirq = Self::consume(&mut iter)?;
        Ok(CpuMetric {
            user,
            nice,
            system,
            idle,
            iowait,
            irq,
            softirq,
        })
    }
}

#[derive(Debug)]
pub enum CpuMetricParseError {
    MissingField,
    ParseValueError(ParseIntError),
}

impl fmt::Display for CpuMetricParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuMetricParseError::MissingField => write!(f, "missing field for cpu metric"),
            CpuMetricParseError::ParseValueError(err) => {
                write!(f, "error parsing cpu metric field: {}", err)
            }
        }
    }
}

impl error::Error for CpuMetricParseError {}

impl From<ParseIntError> for CpuMetricParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseValueError(value)
    }
}

#[derive(Debug)]
pub enum MemoryLine {
    Free(u64),
    Total(u64),
}

impl MemoryLine {
    fn parse_value(prefix: &'static str, v: Option<&str>) -> Result<u64, MemoryLineParseError> {
        Ok(v.ok_or(MemoryLineParseError::MissingValue(prefix))?
            .strip_suffix("kB")
            .ok_or(MemoryLineParseError::MissingUnitSuffix(prefix))?
            .trim()
            .parse::<u64>()
            .map_err(|err| MemoryLineParseError::ValueParseError(prefix, err))?)
    }
}

impl std::str::FromStr for MemoryLine {
    type Err = MemoryLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let prefix = parts.next().ok_or(MemoryLineParseError::NoPrefix)?;
        match prefix {
            "MemTotal" => Ok(MemoryLine::Total(Self::parse_value(
                "MemTotal",
                parts.next(),
            )?)),
            "MemFree" => Ok(MemoryLine::Free(Self::parse_value(
                "MemFree",
                parts.next(),
            )?)),
            prefix => Err(MemoryLineParseError::UnkownPrefix(prefix.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum MemoryLineParseError {
    NoPrefix,
    UnkownPrefix(String),
    MissingValue(&'static str),
    ValueParseError(&'static str, ParseIntError),
    MissingUnitSuffix(&'static str),
}

impl std::fmt::Display for MemoryLineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemoryLineParseError::NoPrefix => write!(f, "line with no prefix"),
            MemoryLineParseError::UnkownPrefix(prefix) => write!(f, "unkown prefix: {}", prefix),
            MemoryLineParseError::MissingValue(key) => write!(f, "no value for {}", key),
            MemoryLineParseError::ValueParseError(key, err) => {
                write!(f, "could not parse value for {}: {}", key, err)
            }
            MemoryLineParseError::MissingUnitSuffix(key) => write!(f, "no unit for {}", key),
        }
    }
}

impl std::error::Error for MemoryLineParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_aggregate_line() {
        assert_eq!(
            "cpu  2255 34 2290 22625563 6290 127 456"
                .parse::<StatLine>()
                .unwrap(),
            StatLine::CpuAggregate(CpuMetric {
                user: 2255,
                nice: 34,
                system: 2290,
                idle: 22625563,
                iowait: 6290,
                irq: 127,
                softirq: 456
            })
        );
        assert_eq!(
            "cpu  2251 31 2291 22625561 6291 121 451"
                .parse::<StatLine>()
                .unwrap(),
            StatLine::CpuAggregate(CpuMetric {
                user: 2251,
                nice: 31,
                system: 2291,
                idle: 22625561,
                iowait: 6291,
                irq: 121,
                softirq: 451
            })
        );
    }

    #[test]
    fn cpu_line() {
        assert_eq!(
            "cpu0 2255 34 2290 22625563 6290 127 456"
                .parse::<StatLine>()
                .unwrap(),
            StatLine::Cpu(
                0,
                CpuMetric {
                    user: 2255,
                    nice: 34,
                    system: 2290,
                    idle: 22625563,
                    iowait: 6290,
                    irq: 127,
                    softirq: 456,
                }
            )
        );
        assert_eq!(
            "cpu1 2255 34 2290 22625563 6290 127 456"
                .parse::<StatLine>()
                .unwrap(),
            StatLine::Cpu(
                1,
                CpuMetric {
                    user: 2255,
                    nice: 34,
                    system: 2290,
                    idle: 22625563,
                    iowait: 6290,
                    irq: 127,
                    softirq: 456,
                }
            )
        );
    }

    #[test]
    fn cpu_line_missing_number() {
        "cpu1 2255 34 2290 22625563 6290 127"
            .parse::<StatLine>()
            .unwrap_err();
    }

    #[test]
    fn cpu_line_not_a_number() {
        "cpu1 2255 34 2290 22625563 62A0 127 456"
            .parse::<StatLine>()
            .unwrap_err();
    }

    #[test]
    fn cpu_bad_line() {
        "cp1 2255 34 2290 22625563 6290 127 456"
            .parse::<StatLine>()
            .unwrap_err();
    }

    #[test]
    fn parse_cpu_prefix() {
        assert_eq!("cpu1".parse::<CpuPrefix>().unwrap().0, 1);
    }

    #[test]
    fn parse_cpu_prefix_bad_prefix() {
        assert!("cp1".parse::<CpuPrefix>().is_err());
    }

    #[test]
    fn parse_cpu_bad_num() {
        assert!("cpuA".parse::<CpuPrefix>().is_err());
    }

    #[test]
    fn parse_double_whitespace() {
        let v = "cpu  840062 638 134704 413417328 84836 0 8422 0 0 0"
            .parse::<StatLine>()
            .unwrap();
        if let StatLine::CpuAggregate(v) = v {
            assert_eq!(v.user, 840062);
        } else {
            panic!("bad line variation")
        }
    }

    #[test]
    fn parse_memory_total() {
        let v = "MemTotal:        8114112 kB".parse::<MemoryLine>().unwrap();
        match v {
            MemoryLine::Total(v) => assert_eq!(v, 8114112),
            _ => panic!("bad variant"),
        }
    }

    #[test]
    fn parse_value_err() {
        assert!("MemTotal:        81AA112 kB".parse::<MemoryLine>().is_err());
    }

    #[test]
    fn parse_bad_suffix_err() {
        assert!("MemTotal:        8114112 B".parse::<MemoryLine>().is_err());
    }

    #[test]
    fn parse_memory_free() {
        let v = "MemFree:        8114112 kB".parse::<MemoryLine>().unwrap();
        match v {
            MemoryLine::Free(v) => assert_eq!(v, 8114112),
            _ => panic!("bad variant"),
        }
    }
}
