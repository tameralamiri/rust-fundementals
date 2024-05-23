#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    BarossaValley,
}

impl WineRegions {
    fn is_supported(&self) -> bool {
        match self {
            WineRegions::Rioja => true,
            _ => false,
        }
    }

    fn popularity(&self) -> String {
        match self {
            WineRegions::Bordeaux => "High".to_string(),
            WineRegions::Burgundy => "Medium".to_string(),
            WineRegions::Champagne => "High".to_string(),
            WineRegions::Tuscany => "Low".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Champagne"),
        region: WineRegions::Champagne,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

    println!("Popularity of Bordeaux: {}", WineRegions::Bordeaux.popularity());
    println!("Popularity of {}: {}", wine3.name, wine3.region.popularity());
}
