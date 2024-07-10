use serde_json::Value;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Root {
    pub items: Items,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Items {
    #[serde(rename = "shopcategories")]
    pub shop_categories: ShopCategories,
    #[serde(rename = "hideoutitem")]
    pub hideout_item: Value,
    #[serde(rename = "trackingitem")]
    pub tracking_item: Vec<Value>,
    #[serde(rename = "trashitem")]
    pub trash_item: Vec<Value>,
    #[serde(rename = "farmableitem")]
    pub farmable_item: Vec<Value>,
    #[serde(rename = "simpleitem")]
    pub simple_item: Vec<Value>,
    #[serde(rename = "consumableitem")]
    pub consumable_item: Vec<Value>,
    #[serde(rename = "consumablefrominventoryitem")]
    pub consumable_from_inventory_item: Vec<Value>,
    #[serde(rename = "equipmentitem")]
    pub equipment_item: Vec<Value>,
    #[serde(rename = "weapon")]
    pub weapon: Vec<Value>,
    #[serde(rename = "mount")]
    pub mount: Vec<Value>,
    #[serde(rename = "furnitureitem")]
    pub furniture_item: Vec<Value>,
    #[serde(rename = "mountskin")]
    pub mount_skin: Vec<Value>,
    #[serde(rename = "journalitem")]
    pub journal_item: Vec<Value>,
    #[serde(rename = "labourercontract")]
    pub labourer_contract: Vec<Value>,
    #[serde(rename = "transformationweapon")]
    pub transformation_weapon: Vec<Value>,
    #[serde(rename = "crystalleagueitem")]
    pub crystal_league_item: Vec<Value>,
    #[serde(rename = "killtrophy")]
    pub kill_trophy: Value,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ShopCategories {
    #[serde(rename = "shopcategory")]
    pub shop_categories: Vec<ShopCategory>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ShopCategory {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "shopsubcategory")]
    pub shop_sub_category: Vec<ShopSubCategory>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ShopSubCategory {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
}
