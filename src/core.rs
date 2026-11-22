//! Plant tables and replanting payment: pure logic, independent of the game API.

pub const MOD_ID: &str = "right_click_crops";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plant {
    Bamboo,
    Beetroot,
    Cactus,
    Carrots,
    Cocoa,
    Kelp,
    Melon,
    NetherWart,
    PitcherCrop,
    Potatoes,
    Pumpkin,
    SugarCane,
    Torchflower,
    TwistingVines,
    WeepingVines,
    Wheat,
}

impl Plant {
    pub const ALL: [Self; 16] = [
        Self::Bamboo,
        Self::Beetroot,
        Self::Cactus,
        Self::Carrots,
        Self::Cocoa,
        Self::Kelp,
        Self::Melon,
        Self::NetherWart,
        Self::PitcherCrop,
        Self::Potatoes,
        Self::Pumpkin,
        Self::SugarCane,
        Self::Torchflower,
        Self::TwistingVines,
        Self::WeepingVines,
        Self::Wheat,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Bamboo => "bamboo",
            Self::Beetroot => "beetroot",
            Self::Cactus => "cactus",
            Self::Carrots => "carrots",
            Self::Cocoa => "cocoa",
            Self::Kelp => "kelp",
            Self::Melon => "melon",
            Self::NetherWart => "nether_wart",
            Self::PitcherCrop => "pitcher_crop",
            Self::Potatoes => "potatoes",
            Self::Pumpkin => "pumpkin",
            Self::SugarCane => "sugar_cane",
            Self::Torchflower => "torchflower",
            Self::TwistingVines => "twisting_vines",
            Self::WeepingVines => "weeping_vines",
            Self::Wheat => "wheat",
        }
    }

    // The English label; the game shows it from assets/right_click_crops/lang/en_us.json.
    #[cfg_attr(not(test), allow(dead_code))]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Bamboo => "Bamboo",
            Self::Beetroot => "Beetroot",
            Self::Cactus => "Cactus",
            Self::Carrots => "Carrots",
            Self::Cocoa => "Cocoa",
            Self::Kelp => "Kelp",
            Self::Melon => "Melon",
            Self::NetherWart => "Nether wart",
            Self::PitcherCrop => "Pitcher crop",
            Self::Potatoes => "Potatoes",
            Self::Pumpkin => "Pumpkin",
            Self::SugarCane => "Sugar cane",
            Self::Torchflower => "Torchflower",
            Self::TwistingVines => "Twisting vines",
            Self::WeepingVines => "Weeping vines",
            Self::Wheat => "Wheat",
        }
    }

    /// The translation key of the plant's setting label (`assets/right_click_crops/lang/`).
    pub fn label_key(self) -> String {
        format!("{MOD_ID}.setting.{}", self.key())
    }

    pub fn from_block(block: &str) -> Option<Self> {
        match block {
            "minecraft:bamboo" => Some(Self::Bamboo),
            "minecraft:beetroots" => Some(Self::Beetroot),
            "minecraft:cactus" => Some(Self::Cactus),
            "minecraft:carrots" => Some(Self::Carrots),
            "minecraft:cocoa" => Some(Self::Cocoa),
            "minecraft:kelp" | "minecraft:kelp_plant" => Some(Self::Kelp),
            "minecraft:melon" => Some(Self::Melon),
            "minecraft:nether_wart" => Some(Self::NetherWart),
            "minecraft:pitcher_crop" => Some(Self::PitcherCrop),
            "minecraft:potatoes" => Some(Self::Potatoes),
            "minecraft:pumpkin" => Some(Self::Pumpkin),
            "minecraft:sugar_cane" => Some(Self::SugarCane),
            "minecraft:torchflower" => Some(Self::Torchflower),
            "minecraft:twisting_vines" | "minecraft:twisting_vines_plant" => Some(Self::TwistingVines),
            "minecraft:weeping_vines" | "minecraft:weeping_vines_plant" => Some(Self::WeepingVines),
            "minecraft:wheat" => Some(Self::Wheat),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crop {
    pub mature_age: Option<&'static str>,
    pub planting_item: &'static str,
    // Cocoa uses the game's direction-sensitive survival check instead.
    pub support_below: Option<&'static str>,
    pub initial_block: Option<&'static str>,
    pub two_blocks: bool,
}

// Explicit profiles, not every block with an age property.
pub fn crop_profile(block: &str) -> Option<Crop> {
    let (mature_age, planting_item, support_below, initial_block, two_blocks) = match block {
        "minecraft:wheat" => (Some("7"), "minecraft:wheat_seeds", Some("minecraft:farmland"), None, false),
        "minecraft:carrots" => (Some("7"), "minecraft:carrot", Some("minecraft:farmland"), None, false),
        "minecraft:potatoes" => (Some("7"), "minecraft:potato", Some("minecraft:farmland"), None, false),
        "minecraft:beetroots" => (Some("3"), "minecraft:beetroot_seeds", Some("minecraft:farmland"), None, false),
        "minecraft:nether_wart" => (Some("3"), "minecraft:nether_wart", Some("minecraft:soul_sand"), None, false),
        "minecraft:cocoa" => (Some("2"), "minecraft:cocoa_beans", None, None, false),
        "minecraft:torchflower" => (None, "minecraft:torchflower_seeds", Some("minecraft:farmland"), Some("minecraft:torchflower_crop"), false),
        "minecraft:pitcher_crop" => (Some("4"), "minecraft:pitcher_pod", Some("minecraft:farmland"), None, true),
        _ => return None,
    };
    Some(Crop { mature_age, planting_item, support_below, initial_block, two_blocks })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerticalPlant {
    pub head: &'static str,
    pub body: &'static str,
    pub growth_y: i32,
}

pub fn vertical_profile(block: &str) -> Option<VerticalPlant> {
    let profile = match block {
        "minecraft:sugar_cane" => VerticalPlant { head: "minecraft:sugar_cane", body: "minecraft:sugar_cane", growth_y: 1 },
        "minecraft:cactus" => VerticalPlant { head: "minecraft:cactus", body: "minecraft:cactus", growth_y: 1 },
        "minecraft:bamboo" => VerticalPlant { head: "minecraft:bamboo", body: "minecraft:bamboo", growth_y: 1 },
        "minecraft:kelp" | "minecraft:kelp_plant" => VerticalPlant { head: "minecraft:kelp", body: "minecraft:kelp_plant", growth_y: 1 },
        "minecraft:twisting_vines" | "minecraft:twisting_vines_plant" => VerticalPlant { head: "minecraft:twisting_vines", body: "minecraft:twisting_vines_plant", growth_y: 1 },
        "minecraft:weeping_vines" | "minecraft:weeping_vines_plant" => VerticalPlant { head: "minecraft:weeping_vines", body: "minecraft:weeping_vines_plant", growth_y: -1 },
        _ => return None,
    };
    Some(profile)
}

pub fn same_vertical_plant(block: &str, profile: VerticalPlant) -> bool {
    block == profile.head || block == profile.body
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StemFruit {
    pub fruit: &'static str,
    pub attached_stem: &'static str,
    pub stem: &'static str,
}

pub fn stem_fruit_profile(block: &str) -> Option<StemFruit> {
    match block {
        "minecraft:pumpkin" => Some(StemFruit {
            fruit: "minecraft:pumpkin",
            attached_stem: "minecraft:attached_pumpkin_stem",
            stem: "minecraft:pumpkin_stem",
        }),
        "minecraft:melon" => Some(StemFruit {
            fruit: "minecraft:melon",
            attached_stem: "minecraft:attached_melon_stem",
            stem: "minecraft:melon_stem",
        }),
        _ => None,
    }
}


/// Blocks in this block tag are harvested as ordinary crops when their `age` is at its maximum.
/// It ships holding `#minecraft:crops`, where mods put their crops; a data pack can add more.
pub const HARVESTABLE_TAG: &str = "right_click_crops:harvestable";
/// Blocks in this tag are never harvested by the tag rule (stems and other members of
/// `#minecraft:crops` that are not "harvest and replant" crops).
pub const EXCLUDED_TAG: &str = "right_click_crops:not_harvestable";
/// The setting that switches the tag rule on and off.
pub const MODDED_SETTING: &str = "modded_crops";
#[cfg_attr(not(test), allow(dead_code))]
pub const MODDED_LABEL: &str = "Modded crops";

/// A crop found by the tag rule: the age it replants at and the age that counts as mature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgeRange {
    pub young: String,
    pub mature: String,
}

/// The rule for a block outside the explicit profiles. `tagged` and `excluded` are the tag reads
/// (None when tags are not loaded yet), `properties` the block's state definition and `age` the
/// clicked state's current age. A mature crop gives its age range; anything else is left alone.
///
/// The maximum comes from the block's own definition, so a mod's crop with age 0..3 or 0..7 needs
/// no table entry. Two-block crops (a `half` property) are refused: harvesting one half would
/// leave the other floating.
pub fn tag_crop(
    tagged: Option<bool>,
    excluded: Option<bool>,
    properties: &[(&str, &[String])],
    age: Option<&str>,
) -> Option<AgeRange> {
    if tagged != Some(true) || excluded != Some(false) {
        return None;
    }
    if properties.iter().any(|(name, _)| *name == "half") {
        return None;
    }
    let (_, values) = properties.iter().find(|(name, _)| *name == "age")?;
    let ages: Vec<(u32, &String)> = values.iter().filter_map(|v| Some((v.parse().ok()?, v))).collect();
    if ages.len() < 2 || ages.len() != values.len() {
        return None;
    }
    let young = ages.iter().min_by_key(|(n, _)| *n)?.1.clone();
    let mature = ages.iter().max_by_key(|(n, _)| *n)?.1.clone();
    (age == Some(mature.as_str())).then_some(AgeRange { young, mature })
}

pub trait SeedStack {
    fn item(&self) -> &str;
    fn count(&self) -> u32;
    // Used by `spend_one`, the reference model the payment tests check.
    #[cfg_attr(not(test), allow(dead_code))]
    fn count_mut(&mut self) -> &mut u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Payment {
    Loot(usize),
    Inventory(usize),
    None,
}

pub fn is_seed(stack: &impl SeedStack, planting_item: &str) -> bool {
    stack.item() == planting_item && stack.count() > 0
}

pub fn select_payment<'a, S: SeedStack + 'a>(
    planting_item: &str,
    loot: &[S],
    inventory: impl IntoIterator<Item = (usize, &'a S)>,
) -> Payment {
    if let Some(index) = loot.iter().position(|s| is_seed(s, planting_item)) {
        Payment::Loot(index)
    } else if let Some((slot, _)) = inventory.into_iter().find(|(_, stack)| is_seed(*stack, planting_item)) {
        Payment::Inventory(slot)
    } else {
        Payment::None
    }
}

// Called only after the world transition succeeds. Selection guarantees a seed.
#[cfg_attr(not(test), allow(dead_code))]
pub fn spend_one<S: SeedStack>(slot: &mut Option<S>, planting_item: &str) {
    if let Some(stack) = slot.as_mut().filter(|s| is_seed(*s, planting_item)) {
        *stack.count_mut() -= 1;
        if stack.count() == 0 {
            *slot = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: &str = "minecraft:wheat_seeds";

    #[derive(Clone, Debug, PartialEq)]
    struct Stack(&'static str, u32);
    impl SeedStack for Stack {
        fn item(&self) -> &str { self.0 }
        fn count(&self) -> u32 { self.1 }
        fn count_mut(&mut self) -> &mut u32 { &mut self.1 }
    }

    fn seeds(count: u32) -> Stack { Stack(SEED, count) }

    #[test]
    fn loot_precedes_inventory_and_charges_exactly_one() {
        let loot = vec![Stack("minecraft:wheat", 1), seeds(3), seeds(2)];
        let inventory = [seeds(10)];
        assert_eq!(select_payment(SEED, &loot, inventory.iter().enumerate()), Payment::Loot(1));
        let mut paid = Some(loot[1].clone());
        spend_one(&mut paid, SEED);
        assert_eq!(paid, Some(seeds(2)));
        assert_eq!(inventory[0], seeds(10));
    }

    #[test]
    fn one_loot_seed_leaves_no_ground_seed() {
        let mut slot = Some(seeds(1));
        spend_one(&mut slot, SEED);
        assert_eq!(slot, None);
    }

    #[test]
    fn inventory_fallback_keeps_all_loot() {
        let loot = [Stack("minecraft:wheat", 1)];
        let mut inventory = [Some(seeds(2))];
        assert_eq!(select_payment(SEED, &loot, inventory.iter().enumerate().filter_map(|(i,s)| s.as_ref().map(|s| (i,s)))), Payment::Inventory(0));
        spend_one(&mut inventory[0], SEED);
        assert_eq!(inventory, [Some(seeds(1))]);
        assert_eq!(loot, [Stack("minecraft:wheat", 1)]);
    }

    #[test]
    fn empty_loot_can_use_the_last_offhand_seed() {
        let mut offhand = Some(seeds(1));
        assert_eq!(select_payment(SEED, &[], [(40, offhand.as_ref().unwrap())]), Payment::Inventory(40));
        spend_one(&mut offhand, SEED);
        assert_eq!(offhand, None);
    }

    #[test]
    fn wrong_items_and_zero_counts_do_not_pay() {
        let loot = [Stack("minecraft:beetroot_seeds", 8), seeds(0)];
        let inventory = [Stack("minecraft:wheat", 1), seeds(0)];
        assert_eq!(select_payment(SEED, &loot, inventory.iter().enumerate()), Payment::None);
        assert_eq!(select_payment::<Stack>(SEED, &[], []), Payment::None);
    }

    #[test]
    fn selection_and_cancelled_commit_have_no_item_effects() {
        let loot = [Stack("minecraft:wheat", 1)];
        let inventory = [seeds(1)];
        let before = (loot.clone(), inventory.clone());
        let _planned = select_payment(SEED, &loot, inventory.iter().enumerate());
        // An adapter that cannot commit must not call spend_one or spawn loot.
        assert_eq!((loot, inventory), before);
    }

    #[test]
    fn deterministic_first_positive_seed_stack() {
        let inventory = [(8, seeds(0)), (12, seeds(2)), (40, seeds(5))];
        assert_eq!(select_payment(SEED, &[], inventory.iter().map(|(i,s)| (*i,s))), Payment::Inventory(12));
    }

    #[test]
    fn spending_never_underflows_or_charges_wrong_items() {
        for stack in [seeds(0), Stack("minecraft:beetroot_seeds", 1)] {
            let mut slot = Some(stack.clone());
            spend_one(&mut slot, SEED);
            assert_eq!(slot, Some(stack));
        }
    }

    #[test]
    fn field_profiles_have_exact_maturity_and_planting_items() {
        for (block, age, item, support, initial, tall) in [
            ("minecraft:wheat", Some("7"), SEED, Some("minecraft:farmland"), None, false),
            ("minecraft:carrots", Some("7"), "minecraft:carrot", Some("minecraft:farmland"), None, false),
            ("minecraft:potatoes", Some("7"), "minecraft:potato", Some("minecraft:farmland"), None, false),
            ("minecraft:beetroots", Some("3"), "minecraft:beetroot_seeds", Some("minecraft:farmland"), None, false),
            ("minecraft:nether_wart", Some("3"), "minecraft:nether_wart", Some("minecraft:soul_sand"), None, false),
            ("minecraft:cocoa", Some("2"), "minecraft:cocoa_beans", None, None, false),
            ("minecraft:torchflower", None, "minecraft:torchflower_seeds", Some("minecraft:farmland"), Some("minecraft:torchflower_crop"), false),
            ("minecraft:pitcher_crop", Some("4"), "minecraft:pitcher_pod", Some("minecraft:farmland"), None, true),
        ] {
            assert_eq!(crop_profile(block), Some(Crop { mature_age: age, planting_item: item, support_below: support, initial_block: initial, two_blocks: tall }));
        }
        for block in ["minecraft:torchflower_crop", "minecraft:pitcher_plant", "minecraft:sweet_berry_bush", "minecraft:air"] {
            assert_eq!(crop_profile(block), None);
        }
    }

    #[test]
    fn every_profile_pays_only_its_own_item_and_prefers_loot() {
        let items = [SEED, "minecraft:carrot", "minecraft:potato", "minecraft:beetroot_seeds", "minecraft:nether_wart", "minecraft:cocoa_beans", "minecraft:torchflower_seeds", "minecraft:pitcher_pod"];
        for item in items {
            let loot = [Stack(item, 2)];
            let inventory = [Stack(item, 4)];
            assert_eq!(select_payment(item, &loot, inventory.iter().enumerate()), Payment::Loot(0));
            assert_eq!(select_payment(item, &[], inventory.iter().enumerate()), Payment::Inventory(0));
            let mut last = Some(Stack(item, 1));
            spend_one(&mut last, item);
            assert_eq!(last, None);
            for wrong in items.into_iter().chain(["minecraft:poisonous_potato", "minecraft:beetroot"]) {
                if wrong == item { continue; }
                let wrong_loot = [Stack(wrong, 3), Stack(item, 0)];
                assert_eq!(select_payment(item, &wrong_loot, [(40, &wrong_loot[0])]), Payment::None);
                let mut slot = Some(Stack(wrong, 3));
                spend_one(&mut slot, item);
                assert_eq!(slot, Some(Stack(wrong, 3)));
            }
        }
    }

    #[test]
    fn vertical_profiles_are_explicit_directed_and_need_no_planting_payment() {
        for name in ["minecraft:sugar_cane", "minecraft:cactus", "minecraft:bamboo"] {
            let profile = vertical_profile(name).unwrap();
            assert_eq!(profile, VerticalPlant { head: name, body: name, growth_y: 1 });
            assert_eq!(crop_profile(name), None);
        }
        for (head, body, growth_y) in [
            ("minecraft:kelp", "minecraft:kelp_plant", 1),
            ("minecraft:twisting_vines", "minecraft:twisting_vines_plant", 1),
            ("minecraft:weeping_vines", "minecraft:weeping_vines_plant", -1),
        ] {
            let expected = VerticalPlant { head, body, growth_y };
            assert_eq!(vertical_profile(head), Some(expected));
            assert_eq!(vertical_profile(body), Some(expected));
            assert!(same_vertical_plant(head, expected));
            assert!(same_vertical_plant(body, expected));
        }
        for name in ["minecraft:bamboo_sapling", "minecraft:cave_vines", "minecraft:vine"] {
            assert_eq!(vertical_profile(name), None);
        }
    }

    #[test]
    fn stem_fruits_are_explicit_and_decorations_are_excluded() {
        assert_eq!(stem_fruit_profile("minecraft:pumpkin"), Some(StemFruit {
            fruit: "minecraft:pumpkin",
            attached_stem: "minecraft:attached_pumpkin_stem",
            stem: "minecraft:pumpkin_stem",
        }));
        assert_eq!(stem_fruit_profile("minecraft:melon"), Some(StemFruit {
            fruit: "minecraft:melon",
            attached_stem: "minecraft:attached_melon_stem",
            stem: "minecraft:melon_stem",
        }));
        for name in ["minecraft:carved_pumpkin", "minecraft:jack_o_lantern", "minecraft:pumpkin_stem"] {
            assert_eq!(stem_fruit_profile(name), None);
        }
    }

    #[test]
    fn plant_settings_are_alphabetical_and_cover_only_mod_targets() {
        assert_eq!(Plant::ALL.map(Plant::label), [
            "Bamboo", "Beetroot", "Cactus", "Carrots", "Cocoa", "Kelp", "Melon",
            "Nether wart", "Pitcher crop", "Potatoes", "Pumpkin", "Sugar cane",
            "Torchflower", "Twisting vines", "Weeping vines", "Wheat",
        ]);
        assert_eq!(Plant::from_block("minecraft:kelp_plant"), Some(Plant::Kelp));
        assert_eq!(Plant::from_block("minecraft:weeping_vines_plant"), Some(Plant::WeepingVines));
        for ordinary in ["minecraft:sweet_berry_bush", "minecraft:cave_vines", "minecraft:cave_vines_plant"] {
            assert_eq!(Plant::from_block(ordinary), None);
        }
    }

    fn ages(n: u32) -> Vec<String> {
        (0..=n).map(|a| a.to_string()).collect()
    }

    #[test]
    fn tag_rule_harvests_only_tagged_mature_single_block_crops() {
        let seven = ages(7);
        let three = ages(3);
        let crop: [(&str, &[String]); 1] = [("age", &seven)];
        let small: [(&str, &[String]); 1] = [("age", &three)];
        let mature = AgeRange { young: "0".into(), mature: "7".into() };
        assert_eq!(tag_crop(Some(true), Some(false), &crop, Some("7")), Some(mature));
        assert_eq!(
            tag_crop(Some(true), Some(false), &small, Some("3")),
            Some(AgeRange { young: "0".into(), mature: "3".into() })
        );
        for age in [None, Some("0"), Some("6")] {
            assert_eq!(tag_crop(Some(true), Some(false), &crop, age), None);
        }
        assert_eq!(tag_crop(Some(true), Some(false), &small, Some("7")), None);
    }

    #[test]
    fn tag_rule_needs_the_tag_no_exclusion_and_loaded_tags() {
        let seven = ages(7);
        let crop: [(&str, &[String]); 1] = [("age", &seven)];
        for (tagged, excluded) in [
            (Some(false), Some(false)),
            (None, Some(false)),
            (Some(true), Some(true)),
            (Some(true), None),
        ] {
            assert_eq!(tag_crop(tagged, excluded, &crop, Some("7")), None);
        }
    }

    #[test]
    fn tag_rule_refuses_two_block_crops_and_odd_ages() {
        let seven = ages(7);
        let halves = vec!["lower".to_string(), "upper".to_string()];
        let tall: [(&str, &[String]); 2] = [("age", &seven), ("half", &halves)];
        assert_eq!(tag_crop(Some(true), Some(false), &tall, Some("7")), None);
        let none: [(&str, &[String]); 0] = [];
        assert_eq!(tag_crop(Some(true), Some(false), &none, None), None);
        let one = ages(0);
        let single: [(&str, &[String]); 1] = [("age", &one)];
        assert_eq!(tag_crop(Some(true), Some(false), &single, Some("0")), None);
        let named = vec!["young".to_string(), "old".to_string()];
        let words: [(&str, &[String]); 1] = [("age", &named)];
        assert_eq!(tag_crop(Some(true), Some(false), &words, Some("old")), None);
    }

    #[test]
    fn tag_rule_reads_ages_by_number_not_by_order() {
        let shuffled: Vec<String> = ["3", "10", "0", "2"].map(String::from).to_vec();
        let crop: [(&str, &[String]); 1] = [("age", &shuffled)];
        assert_eq!(
            tag_crop(Some(true), Some(false), &crop, Some("10")),
            Some(AgeRange { young: "0".into(), mature: "10".into() })
        );
    }

    #[test]
    fn shipped_tags_cover_mod_crops_and_exclude_non_replant_vanilla_members() {
        let harvestable = include_str!("../data/right_click_crops/tags/block/harvestable.json");
        assert!(harvestable.contains("\"#minecraft:crops\""));
        let excluded = include_str!("../data/right_click_crops/tags/block/not_harvestable.json");
        for block in ["minecraft:melon_stem", "minecraft:pumpkin_stem", "minecraft:torchflower_crop"] {
            assert!(excluded.contains(&format!("\"{block}\"")), "{block} not excluded");
            assert_eq!(Plant::from_block(block), None);
        }
        let lang = include_str!("../assets/right_click_crops/lang/en_us.json");
        assert!(lang.contains(&format!("\"{MOD_ID}.setting.{MODDED_SETTING}\": \"{MODDED_LABEL}\"")));
    }

    #[test]
    fn every_plant_setting_has_an_english_label() {
        let lang = include_str!("../assets/right_click_crops/lang/en_us.json");
        for plant in Plant::ALL {
            let entry = format!("\"{}\": \"{}\"", plant.label_key(), plant.label());
            assert!(lang.contains(&entry), "missing {entry}");
        }
    }
}
