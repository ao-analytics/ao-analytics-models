use serde::Deserialize;

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
    pub shop_categories: Vec<ShopCategory>,
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
    #[serde(rename = "@abilitypower")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub ability_power: Option<i32>,

    #[serde(rename = "@accessrightspreset")]
    pub access_rights_preset: Option<String>,

    #[serde(rename = "@activefarmactiondurationseconds")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub active_farm_action_duration_seconds: Option<i32>,

    #[serde(rename = "@activefarmbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub active_farm_bonus: Option<f32>,

    #[serde(rename = "@activefarmcyclelengthseconds")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub active_farm_cycle_length_seconds: Option<i32>,

    #[serde(rename = "@activefarmfocuscost")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub active_farm_focus_cost: Option<i32>,

    #[serde(rename = "@activefarmmaxcycles")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub active_farm_max_cycles: Option<i32>,

    #[serde(rename = "@activespellslots")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub active_spell_slots: Option<i32>,

    #[serde(rename = "@allowfullstackusage")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub allow_full_stack_usage: Option<bool>,

    #[serde(rename = "@animationid")]
    pub animation_id: Option<String>,

    #[serde(rename = "@attackbuildingdamage")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_building_damage: Option<f32>,

    #[serde(rename = "@attackdamage")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_damage: Option<f32>,

    #[serde(rename = "@attackdamagetimefactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_damage_time_factor: Option<f32>,

    #[serde(rename = "@attackendanimgaptime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_end_anim_gap_time: Option<f32>,

    #[serde(rename = "@attackrange")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_range: Option<f32>,

    #[serde(rename = "@attackspeed")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_speed: Option<f32>,

    #[serde(rename = "@attackspeedbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_speed_bonus: Option<f32>,
    #[serde(rename = "@attacktype")]
    pub attack_type: Option<String>,

    #[serde(rename = "@attackupperbodyblendtime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_upper_body_blend_time: Option<f32>,

    #[serde(rename = "@baselootamount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub base_loot_amount: Option<f32>,
    #[serde(rename = "@beardstate")]
    pub beard_state: Option<String>,

    #[serde(rename = "@bonusccdurationvsmobs")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub bonus_cc_duration_vs_mobs: Option<f32>,

    #[serde(rename = "@bonusccdurationvsplayers")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub bonus_cc_duration_vs_players: Option<f32>,

    #[serde(rename = "@bonusdefensevsmobs")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub bonus_defense_vs_mobs: Option<f32>,

    #[serde(rename = "@bonusdefensevsplayers")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub bonus_defense_vs_players: Option<f32>,

    #[serde(rename = "@canbeovercharged")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub can_be_overcharged: Option<bool>,

    #[serde(rename = "@canbestoredinbattlevault")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub can_be_stored_in_battle_vault: Option<bool>,

    #[serde(rename = "@canuseinfactionwarfare")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub can_use_in_faction_warfare: Option<bool>,

    #[serde(rename = "@canuseingvg")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub can_use_in_gvg: Option<bool>,

    #[serde(rename = "@canusetownportal")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub can_use_town_portal: Option<bool>,

    #[serde(rename = "@combatspecachievement")]
    pub combat_spec_achievement: Option<String>,

    #[serde(rename = "@consumespell")]
    pub consume_spell: Option<String>,

    #[serde(rename = "@craftingcategory")]
    pub crafting_category: Option<String>,

    #[serde(rename = "@crowdcontrolresistance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub crowd_control_resistance: Option<i32>,

    #[serde(rename = "@customizewithguildlogo")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub customize_with_guild_logo: Option<bool>,

    #[serde(rename = "@decayitem")]
    pub decay_item: Option<String>,

    #[serde(rename = "@decayondatetime")]
    pub decay_on_date_time: Option<String>,

    #[serde(rename = "@descriptionlocatag")]
    pub description_loca_tag: Option<String>,

    #[serde(rename = "@descvariable0")]
    pub desc_variable0: Option<String>,

    #[serde(rename = "@descvariable1")]
    pub desc_variable1: Option<String>,

    #[serde(rename = "@despawneffect")]
    pub despawn_effect: Option<String>,

    #[serde(rename = "@despawneffectscaling")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub despawn_effect_scaling: Option<i32>,

    #[serde(rename = "@destinycraftfamefactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub destiny_craft_fame_factor: Option<f32>,

    #[serde(rename = "@destroyable")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub destroyable: Option<bool>,

    #[serde(rename = "@dismountbuff")]
    pub dismount_buff: Option<String>,

    #[serde(rename = "@dismounttime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub dismount_time: Option<i32>,

    #[serde(rename = "@dontgivefameoncraft")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub dont_give_fame_on_craft: Option<bool>,

    #[serde(rename = "@droponpvpknockdown")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub drop_on_pvp_knockdown: Option<bool>,

    #[serde(rename = "@dummyitempower")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub dummy_item_power: Option<i32>,

    #[serde(rename = "@durability")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability: Option<f32>,

    #[serde(rename = "@durabilityloss_attack")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_attack: Option<f32>,

    #[serde(rename = "@durabilityloss_findtrack")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_find_track: Option<f32>,

    #[serde(rename = "@durabilityloss_inspecttrack")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_inspect_track: Option<f32>,

    #[serde(rename = "@durabilityloss_mounting")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_mounting: Option<f32>,

    #[serde(rename = "@durabilityloss_receivedattack")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_received_attack: Option<f32>,

    #[serde(rename = "@durabilityloss_receivedattack_mounted")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_received_attack_mounted: Option<f32>,

    #[serde(rename = "@durabilityloss_receivedspell")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_received_spell: Option<f32>,

    #[serde(rename = "@durabilityloss_receivedspell_mounted")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_received_spell_mounted: Option<f32>,

    #[serde(rename = "@durabilityloss_spelluse")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_spell_use: Option<f32>,

    #[serde(rename = "@durabilitylossperdayfactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_per_day_factor: Option<f32>,

    #[serde(rename = "@durabilitylossperusefactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability_loss_per_use_factor: Option<f32>,

    #[serde(rename = "@enchantmentlevel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub enchantment_level: Option<i32>,

    #[serde(rename = "@energycostreduction")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub energy_cost_reduction: Option<f32>,

    #[serde(rename = "@energymax")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub energy_max: Option<i32>,

    #[serde(rename = "@energyregenerationbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub energy_regeneration_bonus: Option<f32>,

    #[serde(rename = "@estimatedmarketvalueoverride")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub estimated_market_value_override: Option<i32>,

    #[serde(rename = "@facestate")]
    pub face_state: Option<String>,

    #[serde(rename = "@famevalue")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fame_value: Option<f32>,

    #[serde(rename = "@fasttravelfactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fast_travel_factor: Option<i32>,

    #[serde(rename = "@findtrackspell")]
    pub find_track_spell: Option<String>,

    #[serde(rename = "@fishing")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fishing: Option<bool>,

    #[serde(rename = "@fishingfame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fishing_fame: Option<f32>,

    #[serde(rename = "@fishingminigamesetting")]
    pub fishing_minigame_setting: Option<String>,

    #[serde(rename = "@fishingspeed")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fishing_speed: Option<f32>,

    #[serde(rename = "@focusfireprotectionpenetration")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub focus_fire_protection_penetration: Option<f32>,

    #[serde(rename = "@foodcategory")]
    pub food_category: Option<String>,

    #[serde(rename = "@forceddismountbuff")]
    pub forced_dismount_buff: Option<String>,

    #[serde(rename = "@forceddismountcooldown")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub forced_dismount_cooldown: Option<f32>,

    #[serde(rename = "@forceddismountspellcooldown")]
    pub forced_dismount_spell_cooldown: Option<String>,

    #[serde(rename = "@fulldismountcooldown")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub full_dismount_cooldown: Option<i32>,

    #[serde(rename = "@fxbonename")]
    pub fx_bone_name: Option<String>,

    #[serde(rename = "@fxboneoffset")]
    pub fx_bone_offset: Option<String>,

    #[serde(rename = "@halfmountedbuff")]
    pub half_mounted_buff: Option<String>,

    #[serde(rename = "@halfmountprefaboverride")]
    pub half_mount_prefab_override: Option<String>,

    #[serde(rename = "@halfmountrange")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub half_mount_range: Option<i32>,

    #[serde(rename = "@healbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub heal_bonus: Option<f32>,

    #[serde(rename = "@healmodifier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub heal_modifier: Option<f32>,

    #[serde(rename = "@hidedecayoverlay")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub hide_decay_overlay: Option<bool>,

    #[serde(rename = "@hidefromplayeroncontext")]
    pub hide_from_player_on_context: Option<String>,

    #[serde(rename = "@hitpointsmax")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub hitpoints_max: Option<i32>,

    #[serde(rename = "@hitpointsregenerationbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub hitpoints_regeneration_bonus: Option<f32>,

    #[serde(rename = "@hostiledismountbuff")]
    pub hostile_dismount_buff: Option<String>,

    #[serde(rename = "@itempower")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub item_power: Option<i32>,

    #[serde(rename = "@itempowerprogressiontype")]
    pub item_power_progression_type: Option<String>,

    #[serde(rename = "@itemvalue")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub item_value: Option<f32>,

    #[serde(rename = "@kind")]
    pub kind: Option<String>,

    #[serde(rename = "@labourerfurnituretype")]
    pub labourer_furniture_type: Option<String>,

    #[serde(rename = "@labourerhappiness")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub labourer_happiness: Option<i32>,

    #[serde(rename = "@labourersaffected")]
    pub labourers_affected: Option<String>,

    #[serde(rename = "@labourersperfurnitureitem")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub labourers_per_furniture_item: Option<i32>,

    #[serde(rename = "@logconsumption")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub log_consumption: Option<bool>,

    #[serde(rename = "@longhostiledismountbuff")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub long_hostile_dismount_buff: Option<String>,

    #[serde(rename = "@magicattackdamagebonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub magic_attack_damage_bonus: Option<f32>,

    #[serde(rename = "@magiccasttimereduction")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub magic_cast_time_reduction: Option<f32>,

    #[serde(rename = "@magiccooldownreduction")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub magic_cooldown_reduction: Option<f32>,

    #[serde(rename = "@magicresistance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub magic_resistance: Option<f32>,

    #[serde(rename = "@magicspelldamagebonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub magic_spell_damage_bonus: Option<f32>,

    #[serde(rename = "@mainhandanimationtype")]
    pub mainhand_animation_type: Option<String>,

    #[serde(rename = "@mainhandanimationtype_attack")]
    pub mainhand_animation_type_attack: Option<String>,

    #[serde(rename = "@mainhandanimationtype_repairbuilding")]
    pub mainhand_animation_type_repair_building: Option<String>,

    #[serde(rename = "@masterymodifier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub mastery_modifier: Option<f32>,

    #[serde(rename = "@maxfame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub max_fame: Option<i32>,

    #[serde(rename = "@maxload")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub max_load: Option<i32>,

    #[serde(rename = "@maxqualitylevel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub max_quality_level: Option<i32>,

    #[serde(rename = "@maxstacksize")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub max_stack_size: Option<i32>,

    #[serde(rename = "@mesh")]
    pub mesh: Option<String>,

    #[serde(rename = "@mindistance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub min_distance: Option<i32>,

    #[serde(rename = "@mindistanceintunnel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub min_distance_in_tunnel: Option<i32>,

    #[serde(rename = "@mountcategory")]
    pub mount_category: Option<String>,

    #[serde(rename = "@mountedbuff")]
    pub mounted_buff: Option<String>,

    #[serde(rename = "@mounthitpointsmax")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub mount_hitpoints_max: Option<i32>,

    #[serde(rename = "@mounthitpointsregeneration")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub mount_hitpoints_regeneration: Option<f32>,

    #[serde(rename = "@mounttime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub mount_time: Option<f32>,

    #[serde(rename = "@movespeed")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub move_speed: Option<i32>,

    #[serde(rename = "@movespeedbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub move_speed_bonus: Option<i32>,

    #[serde(rename = "@namelocatag")]
    pub name_loca_tag: Option<String>,

    #[serde(rename = "@nametagoffset")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub name_tag_offset: Option<f32>,

    #[serde(rename = "@nutrition")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub nutrition: Option<i32>,

    #[serde(rename = "@offhandanimationtype")]
    pub offhand_animation_type: Option<String>,

    #[serde(rename = "@omitmesh")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub omit_mesh: Option<bool>,

    #[serde(rename = "@passivespellslots")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub passive_spell_slots: Option<i32>,

    #[serde(rename = "@physicalarmor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub physical_armor: Option<i32>,

    #[serde(rename = "@physicalattackdamagebonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub physical_attack_damage_bonus: Option<f32>,

    #[serde(rename = "@physicalspelldamagebonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub physical_spell_damage_bonus: Option<f32>,

    #[serde(rename = "@pickupable")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub pickupable: Option<bool>,

    #[serde(rename = "@placeableindoors")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub placeable_indoors: Option<bool>,

    #[serde(rename = "@placeableindungeons")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub placeable_in_dungeons: Option<bool>,

    #[serde(rename = "@placeableinexpeditions")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub placeable_in_expeditions: Option<bool>,

    #[serde(rename = "@placeableonlyonislands")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub placeable_only_on_islands: Option<bool>,

    #[serde(rename = "@placeableoutdoors")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub placeable_outdoors: Option<bool>,

    #[serde(rename = "@placefame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub place_fame: Option<i32>,

    #[serde(rename = "@placementduration")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub placement_duration: Option<i32>,

    #[serde(rename = "@prefabname")]
    pub prefab_name: Option<String>,

    #[serde(rename = "@prefabscale")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub prefab_scale: Option<f32>,

    #[serde(rename = "@prefabscaling")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub prefab_scaling: Option<f32>,

    #[serde(rename = "@primetimedurationminutes")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub prime_time_duration_minutes: Option<i32>,

    #[serde(rename = "@remountdistance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub remount_distance: Option<i32>,

    #[serde(rename = "@remounttime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub remount_time: Option<f32>,

    #[serde(rename = "@repairbuildingdamage")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub repair_building_damage: Option<i32>,

    #[serde(rename = "@requiredaccesslevel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub required_access_level: Option<i32>,

    #[serde(rename = "@residencyslots")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub residency_slots: Option<i32>,

    #[serde(rename = "@resourcetype")]
    pub resource_type: Option<String>,

    #[serde(rename = "@resourcevalue")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub resource_value: Option<i32>,

    #[serde(rename = "@salvageable")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub salvageable: Option<bool>,

    #[serde(rename = "@scalemodifier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub scale_modifier: Option<f32>,

    #[serde(rename = "@shopcategory")]
    pub shop_category: Option<String>,

    #[serde(rename = "@shopsubcategory1")]
    pub shop_sub_category: Option<String>,

    #[serde(rename = "@showinmarketplace")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub show_in_marketplace: Option<bool>,

    #[serde(rename = "@skincolormodifier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub skin_color_modifier: Option<i32>,

    #[serde(rename = "@skincount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub skin_count: Option<i32>,

    #[serde(rename = "@slottype")]
    pub slot_type: Option<String>,

    #[serde(rename = "@threatbonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub threat_bonus: Option<f32>,

    #[serde(rename = "@tier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub tier: Option<i32>,

    #[serde(rename = "@tile")]
    pub tile: Option<String>,

    #[serde(rename = "@trackingfamebonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub tracking_fame_bonus: Option<f32>,

    #[serde(rename = "@trackingtimereduction")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub tracking_time_reduction: Option<f32>,

    #[serde(rename = "@tradable")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub tradable: Option<bool>,

    #[serde(rename = "@transformation")]
    pub transformation: Option<String>,

    #[serde(rename = "@triggershealingsickness")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub triggers_healing_sickness: Option<bool>,

    #[serde(rename = "@twohanded")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub two_handed: Option<bool>,

    #[serde(rename = "@uicraftsoundfinish")]
    pub ui_craft_sound_finish: Option<String>,

    #[serde(rename = "@uicraftsoundstart")]
    pub ui_craft_sound_start: Option<String>,

    #[serde(rename = "@uisprite")]
    pub ui_sprite: Option<String>,

    #[serde(rename = "@uispriteoverlay1")]
    pub ui_sprite_overlay1: Option<String>,

    #[serde(rename = "@uispriteoverlay2")]
    pub ui_sprite_overlay2: Option<String>,

    #[serde(rename = "@unequipincombat")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub unequip_in_combat: Option<bool>,

    #[serde(rename = "@uniquename")]
    pub unique_name: String,

    #[serde(rename = "@unlockedtocraft")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub unlocked_to_craft: Option<bool>,

    #[serde(rename = "@unlockedtoequip")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub unlocked_to_equip: Option<bool>,

    #[serde(rename = "@unlockedtoplace")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub unlocked_to_place: Option<bool>,

    #[serde(rename = "@unlockedtouse")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub unlocked_to_use: Option<bool>,

    #[serde(rename = "@vfxAddonKeyword")]
    pub vfx_addon_keyword: Option<String>,

    #[serde(rename = "@weight")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub weight: Option<f32>,

    #[serde(rename = "AssetVfxPreset")]
    pub asset_vfx_preset: Option<AssetVfxPreset>,

    #[serde(rename = "AudioInfo")]
    pub audio_info: Option<AudioInfo>,

    #[serde(rename = "FootStepVfxPreset")]
    pub foot_step_vfx_preset: Option<FootStepVfxPreset>,

    #[serde(rename = "SocketPreset")]
    pub socket_preset: Option<SocketPreset>,

    #[serde(rename = "attackvariations")]
    pub attack_variations: Option<AttackVariations>,

    #[serde(rename = "attackvfx")]
    pub attack_vfx: Option<AttackVfx>,

    #[serde(rename = "canharvest")]
    pub can_harvest: Option<CanHarvest>,

    #[serde(rename = "cheatprovider")]
    pub cheat_provider: Option<CheatProvider>, // always null

    #[serde(rename = "consumption")]
    pub consumption: Option<Consumption>,

    #[serde(rename = "container")]
    pub container: Option<Container>,

    #[serde(rename = "craftingrequirements")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
    pub crafting_requirements: Option<Vec<CraftingRequirements>>,

    #[serde(rename = "craftingspelllist")]
    pub crafting_spell_list: Option<CraftingSpellList>,

    #[serde(rename = "enchantments")]
    pub enchantments: Option<Enchantments>,

    #[serde(rename = "famefillingmissions")]
    pub fame_filling_missions: Option<FameFillingMissions>,

    #[serde(rename = "grownitem")]
    pub grown_item: Option<GrownItem>,

    #[serde(rename = "harvest")]
    pub harvest: Option<Harvest>,

    #[serde(rename = "lootlist")]
    pub loot_list: Option<LootList>,

    #[serde(rename = "mountspelllist")]
    pub mount_spell_list: Option<MountSpellList>,

    #[serde(rename = "products")]
    pub products: Option<Products>,

    #[serde(rename = "projectile")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
    pub projectiles: Option<Vec<Projectile>>,

    #[serde(rename = "repairkit")]
    pub repair_kit: Option<RepairKit>,

    #[serde(rename = "replaceondeath")]
    pub replace_on_death: Option<ReplaceOnDeath>,
}

// (c) aichingert 2024
fn deserialize_option_vec_or_val<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    Ok(match Option::deserialize(deserializer)? {
        Some(serde_json::Value::Array(arr)) => Some(
            arr.into_iter()
                .map(|e| serde_json::from_value(e))
                .collect::<Result<Vec<T>, serde_json::Error>>()
                .map_err(serde::de::Error::custom)?,
        ),
        Some(serde_json::Value::Object(obj)) => {
            Some(vec![serde_json::from_value(serde_json::Value::Object(obj))
                .map_err(serde::de::Error::custom)?])
        }
        _ => None,
    })
}

fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    T: std::fmt::Debug,
{
    Ok(match Option::deserialize(deserializer)? {
        Some(serde_json::Value::String(s)) => {
            Some(s.parse::<T>().map_err(serde::de::Error::custom)?)
        }
        _ => None,
    })
}

#[derive(Debug, serde::Deserialize)]
pub struct AssetVfxPreset {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AudioInfo {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct FootStepVfxPreset {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct SocketPreset {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AttackVariations {
    #[serde(rename = "attackchaintime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_chain_time: Option<f32>,

    #[serde(rename = "attack")]
    pub attacks: Vec<Attack>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Attack {
    #[serde(rename = "@attackdamagefactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_damage_factor: Option<f32>,

    #[serde(rename = "@attackdamagetimefactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_damage_time_factor: Option<f32>,

    #[serde(rename = "@attackspeedfactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub attack_speed_factor: Option<f32>,

    #[serde(rename = "@attacktype")]
    pub attack_type: Option<String>,

    #[serde(rename = "attackvfx")]
    pub attack_vfx: Option<Vec<AttackVfx>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AttackVfx {
    #[serde(rename = "@prefab")]
    pub prefab: Option<String>,

    #[serde(rename = "@target")]
    pub target: Option<String>,
    /*
    pub target: Option<String>,
    #[serde(rename = "@constraintpreset")]
    pub contraint_preset: Option<String>,
    #[serde(rename = "@socket")]
    pub socket: Option<String>,
    #[serde(rename = "@visibility")]
    pub visibility: Option<String>,
    */
}

#[derive(Debug, serde::Deserialize)]
pub struct CanHarvest {
    #[serde(rename = "@resourcetype")]
    pub resource_type: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CheatProvider {}

#[derive(Debug, serde::Deserialize)]
pub struct Consumption {
    #[serde(rename = "food")]
    pub food: Option<Food>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Food {
    #[serde(rename = "@hungryoverheadicon")]
    pub hungry_overhead_icon: Option<String>,

    #[serde(rename = "@lossbeforehungry")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub loss_before_hungry: Option<i32>,

    #[serde(rename = "@nutrisionmax")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub nutrition_max: Option<i32>,

    #[serde(rename = "@secondspernutrition")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub seconds_per_nutrition: Option<f32>,

    #[serde(rename = "acceptedfood")]
    pub accepted_food: Option<AcceptedFood>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AcceptedFood {
    #[serde(rename = "@favourite")]
    pub favourite: Option<String>,

    #[serde(rename = "@favouritebonus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub favourite_bonus: Option<i32>,

    #[serde(rename = "@foodcategory")]
    pub food_category: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Container {
    #[serde(rename = "@capacity")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub capacity: Option<i32>,

    #[serde(rename = "@weightlimit")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub weight_limit: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftingRequirements {
    #[serde(rename = "@amountcrafted")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub amount_crafted: Option<i32>,

    #[serde(rename = "@compensatedgold")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub compensated_gold: Option<i32>,

    #[serde(rename = "@craftingfocus")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub crafting_focus: Option<i32>,

    #[serde(rename = "@craftsingleperdefault")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub craft_single_per_default: Option<bool>,

    #[serde(rename = "@forcesinglecraft")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub force_single_craft: Option<bool>,

    #[serde(rename = "@gold")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub gold: Option<i32>,

    #[serde(rename = "@returnproductnotresource")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub return_product_not_resource: Option<bool>,

    #[serde(rename = "@salvageitemfactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub salvage_item_factor: Option<f32>,

    #[serde(rename = "@salvagesilverfactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub salvage_silver_factor: Option<i32>,

    #[serde(rename = "@silver")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub silver: Option<i32>,

    #[serde(rename = "@swaptransaction")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub swap_transaction: Option<bool>,

    #[serde(rename = "@time")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub time: Option<f32>,

    #[serde(rename = "craftresource")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
    pub craft_resource: Option<Vec<CraftResource>>,

    #[serde(rename = "currency")]
    pub currency: Option<Currency>,

    #[serde(rename = "playerfactionstanding")]
    pub player_faction_standing: Option<PlayerFactionStanding>,

    #[serde(rename = "mistcitystanding")]
    pub mist_city_standing: Option<MistCityStanding>,

    #[serde(rename = "vanityunlocked")]
    pub vanity_unlocked: Option<VanityUnlocked>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftResource {
    #[serde(rename = "@count")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub count: Option<i32>,

    #[serde(rename = "@enchantmentlevel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub enchantment_level: Option<String>,

    #[serde(rename = "@maxreturnamount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub max_return_amount: Option<i32>,

    #[serde(rename = "@preservequality")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub preserve_quality: Option<bool>,

    #[serde(rename = "@uniquename")]
    pub unique_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Currency {
    #[serde(rename = "@uniquename")]
    pub unqiue_name: String,

    #[serde(rename = "@amount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub amount: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MistCityStanding {
    #[serde(rename = "@minstanding")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub min_standing: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PlayerFactionStanding {
    #[serde(rename = "@faction")]
    pub faction: Option<String>,

    #[serde(rename = "@minstanding")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub min_standing: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct VanityUnlocked {
    #[serde(rename = "@type")]
    pub vanity_type: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftingSpellList {
    #[serde(rename = "craftspell")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
    pub craft_spells: Option<Vec<CraftSpell>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftSpell {
    #[serde(rename = "@slots")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub slots: Option<i32>,

    #[serde(rename = "@tag")]
    pub tag: Option<String>,

    #[serde(rename = "@uniquename")]
    pub unique_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Enchantments {
    #[serde(rename = "enchantment")]
    pub enchantments: Vec<Enchantment>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Enchantment {
    #[serde(rename = "@abilitypower")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub ability_power: Option<i32>,

    #[serde(rename = "@consumespell")]
    pub consume_spell: Option<String>,

    #[serde(rename = "@dummyitempower")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub dummy_item_power: Option<i32>,

    #[serde(rename = "@durability")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub durability: Option<f32>,

    #[serde(rename = "@enchantmentlevel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub enchantment_level: Option<i32>,

    #[serde(rename = "@itempower")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub item_power: Option<i32>,

    #[serde(rename = "@nutrition")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub nutrition: Option<i32>,

    #[serde(rename = "@weight")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub weight: Option<f32>,

    #[serde(rename = "craftingrequirements")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
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
    pub unique_name: Option<String>,

    #[serde(rename = "@count")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub count: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FameFillingMissions {
    #[serde(rename = "craftitemfame")]
    pub craft_item_fame: Option<CraftItemFame>,

    #[serde(rename = "fameearned")]
    pub fame_earned: Option<FameEarned>,

    #[serde(rename = "fishingfame")]
    pub fishing_fame: Option<FishingFame>,

    #[serde(rename = "gatherfame")]
    pub gather_fame: Option<GatherFame>,

    #[serde(rename = "killmobfame")]
    pub kill_mod_fame: Option<KillMobFame>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CraftItemFame {
    #[serde(rename = "@mintier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fame: Option<i32>,

    #[serde(rename = "@value")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub value: Option<i32>,

    #[serde(rename = "validitem")]
    pub valid_items: Option<Vec<ValidItem>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ValidItem {
    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FameEarned {
    #[serde(rename = "@value")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub value: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FishingFame {
    #[serde(rename = "@value")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub value: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GatherFame {
    #[serde(rename = "@mintier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fame: Option<i32>,

    #[serde(rename = "@value")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub value: Option<i32>,

    #[serde(rename = "validitem")]
    pub valid_items: Option<Vec<ValidItem>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct KillMobFame {
    #[serde(rename = "@mintier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub min_tier: Option<i32>,

    #[serde(rename = "@value")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub value: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GrownItem {
    #[serde(rename = "@fame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fame: Option<i32>,

    #[serde(rename = "@growtime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub grow_time: Option<i32>,

    #[serde(rename = "@uniquename")]
    pub unique_name: Option<String>,

    #[serde(rename = "offspring")]
    pub offspring: Option<Offspring>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Offspring {
    #[serde(rename = "@amount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub amount: Option<i32>,

    #[serde(rename = "@chance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub chance: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Harvest {
    #[serde(rename = "@fame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fame: Option<i32>,

    #[serde(rename = "@growtime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub grow_time: Option<i32>,

    #[serde(rename = "@lootchance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub loot_chance: Option<f32>,

    #[serde(rename = "@lootlist")]
    pub loot_list: Option<String>,

    #[serde(rename = "seed")]
    pub seed: Option<Seed>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Seed {
    #[serde(rename = "@amount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub amount: Option<i32>,

    #[serde(rename = "@chance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub chance: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LootList {
    #[serde(rename = "loot")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
    pub loot: Option<Vec<Loot>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Loot {
    #[serde(rename = "@itemamount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub item_amount: Option<i32>,

    #[serde(rename = "@itemenchantmentlevel")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub item_enchantment_level: Option<i32>,

    #[serde(rename = "@itemname")]
    pub item_name: Option<String>,

    #[serde(rename = "@labourerfame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub labourer_fame: Option<i32>,

    #[serde(rename = "@silveramount")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub silver_amount: Option<i32>,

    #[serde(rename = "@weight")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub weight: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MountSpellList {
    #[serde(rename = "mountspell")]
    #[serde(default, deserialize_with = "deserialize_option_vec_or_val")]
    pub mount_spells: Option<Vec<MountSpell>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MountSpell {
    #[serde(rename = "@spellslot")]
    pub spell_slot: Option<String>,

    #[serde(rename = "@uniquename")]
    pub unique_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Products {
    #[serde(rename = "product")]
    pub product: Option<Product>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Product {
    #[serde(rename = "@actionname")]
    pub action_name: Option<String>,

    #[serde(rename = "@fame")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fame: Option<i32>,

    #[serde(rename = "@lootchance")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub loot_chance: Option<f32>,

    #[serde(rename = "@lootlist")]
    pub loot_list: Option<String>,

    #[serde(rename = "@productiontime")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub production_time: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Projectile {
    #[serde(rename = "@flyspeed")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub fly_speed: Option<i32>,

    #[serde(rename = "@hitsocket")]
    pub hit_socket: Option<String>,

    #[serde(rename = "@prefab")]
    pub prefab: Option<String>,

    #[serde(rename = "@startsocket")]
    pub startsocket: Option<String>,

    #[serde(rename = "@timeoffset")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub time_offset: Option<f32>,

    #[serde(rename = "AudioInfo")]
    pub audio_info: Option<AudioInfo>,

    #[serde(rename = "impactvfx")]
    pub impact_vfx: Option<ImpactVfx>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ImpactVfx {
    #[serde(rename = "@impactsocket")]
    pub impact_socket: Option<String>,

    #[serde(rename = "@prefab")]
    pub prefab: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RepairKit {
    #[serde(rename = "@maxtier")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub max_tier: Option<i32>,

    #[serde(rename = "@repaircostfactor")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub repair_cost_factor: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReplaceOnDeath {
    #[serde(rename = "replacementitem")]
    pub replacement_item: ReplacementItem,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReplacementItem {
    #[serde(rename = "@count")]
    #[serde(default, deserialize_with = "deserialize_from_string")]
    pub count: Option<i32>,

    #[serde(rename = "@uniquename")]
    pub unique_name: Option<String>,
}
