fn welcome_message() {
    println!("Economy Engine - Trade game - alpha version.");
    println!("Following google comprehensive rust guide.");
}

fn goodbye_message() {
    println!("Everything went well, goodbye.");
}

fn build_version_string(array: [i32;6]) -> String {
    // let engine_version_string: &str = "Economy Engine - alpha";
    // let game_version_string: &str = "Trading Game - alpha";
    // TODO: prepare string building of version string that is returned from this function
    // return "Version string TO BE IMPLEMENTED"
    // Returning using format! macro - this creates a String class object
    return format!("{}.{}.{} Economy engine\n                  {}.{}.{} Trading game", array[0], array[1], array[2], array[3], array[4], array[5]);
}

// TODO: create world structure
// TODO: create function that creates world - parameters are stored in stuff known at compile-time
// World type (struct?) Does Rust have structs?
// Parameters types (struct?) Does Rust have structs?

// World consists of: Ports
//   Ports have: Position (2D)
//               Market (1 per port)
//               Population - integer number
#[derive(Debug)]
enum CommodityGroup {
    // Do not mix those!
    // HARD
    Agriculture,
    Marine, // Provided by oceans and seas
    Development, // Building materials
    Mining, // Provided by mining
    Forestry, // Provided by forestry (mostly wood)
    Energy,
    Luxury,

    // SERVICE
    Education, // Provides means to move up the pops and to use more sophisticated commodities
    Healthcare,
    Religion,
    Trade, // Infrastructure and means to move goods around
}

#[derive(Debug)]
enum CommodityType {
    HARD,
    SERVICE,
}

struct Commodity {
    name: Box<str>, // TODO learn about Boxes
    commodity_group: CommodityGroup,
    commodity_type: CommodityType,
}

// ver 1 of Commodities
// grain
// fish
// meat
// wine
// beer
//
// hemp
// cotton
// wool
//
// wood
// clay
// bricks
// pelts
//
// pottery
// cloth
//
// iron
// copper
// coal
// charcoal
//
// iron goods
// tools
//
// salt
// spices
//

struct World {}

struct Market {}

struct Port {}

struct City {}

struct Population {
    number: u64,
}

struct PARAMETERS {}

// TODO: create function that populates the world - parameters stored at compile-time

fn main() {
    welcome_message();

    // Array with version numbers - two version: major minor and baby numbers
    // 0, 1, 2 <--- econ engine version
    // 3, 4, 5 <--- trad game version

    let version_array: [i32; 6]; // Uninitialized array?
    version_array = [0, 0, 1, 0, 0, 1]; // Initializing with version nums

    // println!("  --> {} - built for - {}", engine_version_string, game_version_string);
    println!("Project versions: {}", build_version_string(version_array));

    // TODO: Rust doesn't have a defer, but maybe we can implement it using macros?
    goodbye_message();
}
