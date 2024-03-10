#[derive(serde::Deserialize)]
pub struct Localization {
    #[serde(rename = "LocalizedNames")]
    pub localized_names: Option<LocalizedName>,
    #[serde(rename = "LocalizedDescriptions")]
    pub localized_descriptions: Option<LocalizedDescription>,
    #[serde(rename = "UniqueName")]
    pub item: String,
    #[serde(rename = "Index")]
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct LocalizedName {
    #[serde(rename = "EN-US")]
    pub en_us: Option<String>,
    #[serde(rename = "DE-DE")]
    pub de_de: Option<String>,
    #[serde(rename = "FR-FR")]
    pub fr_fr: Option<String>,
    #[serde(rename = "RU-RU")]
    pub ru_ru: Option<String>,
    #[serde(rename = "PL-PL")]
    pub pl_pl: Option<String>,
    #[serde(rename = "ES-ES")]
    pub es_es: Option<String>,
    #[serde(rename = "PT-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "IT-IT")]
    pub it_it: Option<String>,
    #[serde(rename = "ZH-CN")]
    pub zh_cn: Option<String>,
    #[serde(rename = "KO-KR")]
    pub ko_kr: Option<String>,
    #[serde(rename = "JA-JP")]
    pub ja_jp: Option<String>,
    #[serde(rename = "ZH-TW")]
    pub zh_tw: Option<String>,
    #[serde(rename = "ID-ID")]
    pub id_id: Option<String>,
    #[serde(rename = "TR-TR")]
    pub tr_tr: Option<String>,
    #[serde(rename = "AR-SA")]
    pub ar_sa: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LocalizedDescription {
    #[serde(rename = "EN-US")]
    pub en_us: Option<String>,
    #[serde(rename = "DE-DE")]
    pub de_de: Option<String>,
    #[serde(rename = "FR-FR")]
    pub fr_fr: Option<String>,
    #[serde(rename = "RU-RU")]
    pub ru_ru: Option<String>,
    #[serde(rename = "PL-PL")]
    pub pl_pl: Option<String>,
    #[serde(rename = "ES-ES")]
    pub es_es: Option<String>,
    #[serde(rename = "PT-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "IT-IT")]
    pub it_it: Option<String>,
    #[serde(rename = "ZH-CN")]
    pub zh_cn: Option<String>,
    #[serde(rename = "KO-KR")]
    pub ko_kr: Option<String>,
    #[serde(rename = "JA-JP")]
    pub ja_jp: Option<String>,
    #[serde(rename = "ZH-TW")]
    pub zh_tw: Option<String>,
    #[serde(rename = "ID-ID")]
    pub id_id: Option<String>,
    #[serde(rename = "TR-TR")]
    pub tr_tr: Option<String>,
    #[serde(rename = "AR-SA")]
    pub ar_sa: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Location {
    #[serde(rename = "Index")]
    pub id: String,
    #[serde(rename = "UniqueName")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Root {
    pub items: Items,
}

#[derive(Debug, serde::Deserialize)]
pub struct Items {
    #[serde(rename = "shopcategories")]
    pub shop_categories: ShopCategories,
    #[serde(rename = "hideoutitem")]
    pub hideout_item: Item,
    #[serde(rename = "trackingitem")]
    pub tracking_item: Vec<Item>,
    #[serde(rename = "farmableitem")]
    pub farmable_item: Vec<Item>,
    #[serde(rename = "simpleitem")]
    pub simple_item: Vec<Item>,
    #[serde(rename = "consumableitem")]
    pub consumable_item: Vec<Item>,
    #[serde(rename = "consumablefrominventoryitem")]
    pub consumable_from_inventory_item: Vec<Item>,
    #[serde(rename = "equipmentitem")]
    pub equipment_item: Vec<Item>,
    #[serde(rename = "weapon")]
    pub weapon: Vec<Item>,
    #[serde(rename = "mount")]
    pub mount: Vec<Item>,
    #[serde(rename = "furnitureitem")]
    pub furniture_item: Vec<Item>,
    #[serde(rename = "mountskin")]
    pub mount_skin: Vec<Item>,
    #[serde(rename = "journalitem")]
    pub journal_item: Vec<Item>,
    #[serde(rename = "labourercontract")]
    pub labourer_contract: Vec<Item>,
    #[serde(rename = "transformationweapon")]
    pub transformation_weapon: Vec<Item>,
    #[serde(rename = "crystalleagueitem")]
    pub crystal_league_item: Vec<Item>,
    #[serde(rename = "killtrophy")]
    pub kill_trophy: Item,
}

#[derive(Debug, serde::Deserialize)]
pub struct ShopCategories {
    #[serde(rename = "shopcategory")]
    pub shop_category: Vec<ShopCategory>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ShopCategory {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "shopsubcategory")]
    pub shop_sub_category: Vec<ShopSubCategory>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ShopSubCategory {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "@uniquename")]
    pub unique_name: String,
    #[serde(rename = "@shopcategory")]
    pub shop_category: Option<String>,
    #[serde(rename = "@shopsubcategory1")]
    pub shop_sub_category: Option<String>,
    #[serde(rename = "@weight")]
    pub weight: Option<String>,
    #[serde(rename = "@tier")]
    pub tier: Option<String>,

    #[serde(default)]
    #[serde(rename = "craftingrequirements")]
    #[serde(deserialize_with = "deserialize_crafting_requirements")]
    pub crafting_requirements: Option<Vec<CraftingRequirements>>,
    #[serde(rename = "enchantments")]
    pub enchantments: Option<Enchantments>,
}

fn deserialize_crafting_requirements<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<CraftingRequirements>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;

    match value {
        Some(serde_json::Value::Array(arr)) => {
            serde_json::from_value(serde_json::Value::Array(arr)).map_err(serde::de::Error::custom)
        }
        Some(serde_json::Value::Object(obj)) => {
            let single_object_as_array = vec![serde_json::Value::Object(obj)];
            serde_json::from_value(serde_json::Value::Array(single_object_as_array))
                .map_err(serde::de::Error::custom)
        }
        _ => Ok(None),
    }
}

fn deserialize_crafting_resource<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<CraftResource>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;

    match value {
        Some(serde_json::Value::Array(arr)) => {
            serde_json::from_value(serde_json::Value::Array(arr)).map_err(serde::de::Error::custom)
        }
        Some(serde_json::Value::Object(obj)) => {
            let single_object_as_array = vec![serde_json::Value::Object(obj)];
            serde_json::from_value(serde_json::Value::Array(single_object_as_array))
                .map_err(serde::de::Error::custom)
        }
        _ => Ok(None),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftingRequirements {
    #[serde(rename = "@silver")]
    pub silver: Option<String>,
    #[serde(rename = "@time")]
    pub time: Option<String>,
    #[serde(rename = "@amount")]
    pub amount: Option<String>,
    #[serde(rename = "@craftingfocus")]
    pub crafting_focus: Option<String>,
    #[serde(rename = "@swaptransaction")]
    pub swap_transaction: Option<String>,
    #[serde(default)]
    #[serde(rename = "craftresource")]
    #[serde(deserialize_with = "deserialize_crafting_resource")]
    pub craft_resource: Option<Vec<CraftResource>>,
    #[serde(rename = "playerfactionstanding")]
    pub player_faction_standing: Option<PlayerFactionStanding>,
    #[serde(rename = "mistcitystanding")]
    pub mist_city_standing: Option<MistCityStanding>,
    #[serde(rename = "currency")]
    pub currency: Option<Currency>,
    #[serde(rename = "@amountcrafted")]
    pub amount_crafted: Option<String>,
    #[serde(rename = "@salvageitemfactor")]
    pub salvage_item_factor: Option<String>,
    #[serde(rename = "@returnproductnotresource")]
    pub return_product_not_resource: Option<String>,
    #[serde(rename = "@forcesinglecraft")]
    pub force_single_craft: Option<String>,
    #[serde(rename = "@gold")]
    pub gold: Option<String>,
    #[serde(rename = "@craftsingleperdefault")]
    pub craft_single_per_default: Option<String>,
    #[serde(rename = "@compensategold")]
    pub compensate_gold: Option<String>,
    #[serde(rename = "@salvagesilverfactor")]
    pub salvages_silver_factor: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftResource {
    #[serde(rename = "@uniquename")]
    pub unique_name: String,
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "@maxreturnamount")]
    pub max_return_amount: Option<String>,
    #[serde(rename = "@enchantmentlevel")]
    pub enchantment_level: Option<String>,
    #[serde(rename = "@preservequality")]
    pub preserve_quality: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PlayerFactionStanding {
    #[serde(rename = "@faction")]
    pub faction: String,
    #[serde(rename = "@minstanding")]
    pub min_standing: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct MistCityStanding {
    #[serde(rename = "@minstanding")]
    pub min_standing: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Currency {
    #[serde(rename = "@uniquename")]
    pub unqiue_name: String,
    #[serde(rename = "@amount")]
    pub amount: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Enchantments {
    #[serde(rename = "enchantment")]
    pub enchantment: Vec<Enchantment>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Enchantment {
    #[serde(rename = "@enchantmentlevel")]
    pub enchantment_level: String,
    #[serde(rename = "@weight")]
    pub weight: Option<String>,

    #[serde(default)]
    #[serde(rename = "craftingrequirements")]
    #[serde(deserialize_with = "deserialize_crafting_requirements")]
    pub crafting_requirements: Option<Vec<CraftingRequirements>>,
    #[serde(rename = "upgraderequirements")]
    pub upgrade_requirements: Option<UpgradeRequirements>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpgradeRequirements {
    #[serde(rename = "upgraderesource")]
    pub upgrade_resource: UpgradeResource,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpgradeResource {
    #[serde(rename = "@uniquename")]
    pub unique_name: String,
    #[serde(rename = "@count")]
    pub count: String,
}
