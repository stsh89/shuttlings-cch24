pub struct Milk {
    liters: f32,
    pints: f32,
    gallons: f32,
}

pub enum MilkVolume {
    Liters(f32),
    Gallons(f32),
    Pints(f32),
}

impl Milk {
    pub fn gallons(&self) -> f32 {
        self.gallons
    }

    pub fn liters(&self) -> f32 {
        self.liters
    }

    pub fn new(volume: MilkVolume) -> Self {
        Self {
            liters: volume.liters(),
            gallons: volume.gallons(),
            pints: volume.pints(),
        }
    }

    pub fn pints(&self) -> f32 {
        self.pints
    }
}

impl MilkVolume {
    fn gallons(&self) -> f32 {
        match self {
            MilkVolume::Liters(liters) => liters * 0.264172,
            MilkVolume::Gallons(gallons) => *gallons,
            MilkVolume::Pints(pints) => pints * 0.125,
        }
    }

    fn liters(&self) -> f32 {
        match self {
            MilkVolume::Liters(liters) => *liters,
            MilkVolume::Gallons(gallons) => gallons * 3.785412,
            MilkVolume::Pints(pints) => pints * 0.56826125,
        }
    }

    fn pints(&self) -> f32 {
        match self {
            MilkVolume::Liters(liters) => liters * 1.75975,
            MilkVolume::Gallons(gallons) => gallons * 128.0,
            MilkVolume::Pints(pints) => *pints,
        }
    }
}
