#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<DnaNuc>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<RnaNuc>);

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
enum DnaNuc {
    G,
    C,
    T,
    A,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
enum RnaNuc {
    C,
    G,
    A,
    U,
}

impl TryFrom<char> for DnaNuc {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'G' => Ok(DnaNuc::G),
            'C' => Ok(DnaNuc::C),
            'T' => Ok(DnaNuc::T),
            'A' => Ok(DnaNuc::A),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for RnaNuc {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'C' => Ok(RnaNuc::C),
            'G' => Ok(RnaNuc::G),
            'A' => Ok(RnaNuc::A),
            'U' => Ok(RnaNuc::U),
            _ => Err(()),
        }
    }
}

impl From<DnaNuc> for RnaNuc {
    fn from(value: DnaNuc) -> RnaNuc {
        match value {
            DnaNuc::G => RnaNuc::C,
            DnaNuc::C => RnaNuc::G,
            DnaNuc::T => RnaNuc::A,
            DnaNuc::A => RnaNuc::U,
        }
    }
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucs = dna
            .chars()
            .enumerate()
            .map(|(i, c)| DnaNuc::try_from(c).map_err(|_| i))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Dna(nucs))
    }

    pub fn into_rna(self) -> Rna {
        Rna (self.0.into_iter().map(RnaNuc::from).collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucs = rna
            .chars()
            .enumerate()
            .map(|(i, c)| RnaNuc::try_from(c).map_err(|_| i))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Rna(nucs))
    }
}
