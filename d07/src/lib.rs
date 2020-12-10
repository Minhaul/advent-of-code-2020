#[derive(Debug)]
pub struct BagHoldings {
    pub bag: String,
    pub contains: Option<Vec<(u32, String)>>,
}

impl BagHoldings {
    pub fn new() -> BagHoldings {
        BagHoldings {
            bag: String::from(""),
            contains: None,
        }
    }
}

