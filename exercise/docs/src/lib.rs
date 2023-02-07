//! A pumpkin is a vernacular term for mature winter squash of species and varieties in the genus Cucurbita that has culinary and cultural significance but no agreed upon botanical or scientific meaning. The term pumpkin is sometimes used interchangeably with "squash" or "winter squash", and is commonly used for cultivars of Cucurbita argyrosperma, Cucurbita ficifolia, Cucurbita maxima, Cucurbita moschata, and Cucurbita pepo.
//! ![a photo of a pumpkin](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)

/// Big orange thing
///
/// # Recipes
/// Recipes will be coming soon.
pub struct Pumpkin {
    /// `roundness` is a percentage
    pub roundness: f32,
    /// `orangeness` is a number between 8 and 27
    pub orangeness: i32,
}

impl Pumpkin {
    /// If you smash the pumpkin, it will be gone. Then it can't be used for
    /// pie.😭
    pub fn smash(self) {}
}

/// `BURNT_ORANGE` is used for the orangeness field in the [Pumpkin] struct
pub const BURNT_ORANGE: i32 = 13;

// Challenge: Find the option to pass to `cargo doc` so that documentation for this private item
// gets generated as well.  Hint: `cargo doc -h` will show you all the relevant options.

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
