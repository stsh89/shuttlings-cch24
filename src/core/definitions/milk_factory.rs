pub struct Milk {
    liters: f32,
    gallons: f32,
}

pub enum MilkVolume {
    Liters(f32),
    Gallons(f32),
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
        }
    }
}

impl MilkVolume {
    fn gallons(&self) -> f32 {
        match self {
            MilkVolume::Liters(liters) => liters * 0.264172,
            MilkVolume::Gallons(gallons) => *gallons,
        }
    }

    fn liters(&self) -> f32 {
        match self {
            MilkVolume::Liters(liters) => *liters,
            MilkVolume::Gallons(gallons) => gallons * 3.785412,
        }
    }
}
