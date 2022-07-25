use crate::adsize::*;
use dioxus::prelude::Props;

#[derive(Props, PartialEq, Clone)]
pub struct AdSlot {
    pub id: String,
    pub sizes: Vec<AdSize>,
    pub ad_unit : String,
    pub ad_path: String,
}

impl AdSlot {
    pub fn new(id: &str, ad_path: &str, ad_unit: &str, sizes: Vec<AdSize>) -> Self {
        Self {
            ad_path: ad_path.to_owned(),
            ad_unit: ad_unit.to_owned(),
            id: id.to_owned(),
            sizes
        }
    }

    pub fn ad_unit_path(&self) -> String {
        format!("/{}/{}/{}", self.id, self.ad_path, self.ad_unit)
    }

    pub fn push_script(&self, id: &str) -> String {
        format!("\
        dgoogleads.push({{\
            path: \"{}\",\
            id: '{}',\
            sizes: [{}],\
        }});",
        self.ad_unit_path(),
        id,
        self.sizes.iter().map(|x| x.to_script()).collect::<Vec<String>>().join(",")
        )
    }
}