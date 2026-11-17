//! Every DBC schema this crate builds, in one list, so the schemas can be dumped without the file
//! that produces each one. Calling the builders is the only true source: many fill their fields
//! from loops, `match` arms on constants and `format!` names, so nothing shorter would stay right.
//!
//! One entry per module-level `<name>_schema()` builder (the plain `schema()` ones included), in
//! lib.rs module-declaration order and source order inside a module. The key is the module path,
//! plus the builder's own name when the module has more than one. Add an entry here when a schema
//! function is added: nothing else links the two.
//!
//! The argument-taking builders (`band_schema`, `id_name_schema`, `all_u32_schema`, the three
//! `n_u32_schema`s, `emotes::schema`, `gm_survey::schema`, `text_filter_lists::schema`) are
//! listed last, called with the literals their loaders pass. A call site that takes its field
//! count from the file has no fixed schema and is not here: that is the `id_name_schema` in
//! `dbc`'s test, which sizes its table from the fixture it builds.

use benilla_dbc::{FieldType, Schema};

/// Every schema builder in this crate, by module (and builder when a module has several).
pub fn all() -> Vec<(&'static str, Schema)> {
    vec![
        (
            "characters::geosets::char_hair_geosets",
            crate::characters::char_hair_geosets_schema(),
        ),
        (
            "characters::geosets::helmet_vis",
            crate::characters::helmet_vis_schema(),
        ),
        (
            "characters::geosets::char_facial_hair",
            crate::characters::char_facial_hair_schema(),
        ),
        (
            "characters::sections",
            crate::characters::char_sections_schema(),
        ),
        (
            "camera_shakes::camera_shakes",
            crate::camera_shakes::camera_shakes_schema(),
        ),
        (
            "camera_shakes::spell_effect_camera_shakes",
            crate::camera_shakes::spell_effect_camera_shakes_schema(),
        ),
        (
            "cinematics::sequences",
            crate::cinematics::sequences_schema(),
        ),
        ("cinematics::cameras", crate::cinematics::cameras_schema()),
        (
            "creatures::creature_model_data",
            crate::creatures::creature_model_data_schema(),
        ),
        (
            "creatures::creature_display_info",
            crate::creatures::creature_display_info_schema(),
        ),
        (
            "creatures::creature_display_info_extra",
            crate::creatures::creature_display_info_extra_schema(),
        ),
        (
            "unit_blood::unit_blood_levels",
            crate::unit_blood::unit_blood_levels_schema(),
        ),
        (
            "unit_blood::unit_blood",
            crate::unit_blood::unit_blood_schema(),
        ),
        ("itemsets", crate::itemsets::item_set_schema()),
        ("lfg_dungeons", crate::lfg_dungeons::schema()),
        ("skill_tiers", crate::skill_tiers::schema()),
        (
            "gm_ticket_category",
            crate::gm_ticket_category::gm_ticket_category_schema(),
        ),
        (
            "cfg_categories",
            crate::cfg_categories::cfg_categories_schema(),
        ),
        ("cfg_configs", crate::cfg_configs::cfg_configs_schema()),
        (
            "itembagfamily",
            crate::itembagfamily::item_bag_family_schema(),
        ),
        ("itemclass", crate::itemclass::item_class_schema()),
        (
            "auction_house",
            crate::auction_house::auction_house_schema(),
        ),
        (
            "itemsubclass::item_sub_class",
            crate::itemsubclass::item_sub_class_schema(),
        ),
        (
            "itemsubclass::item_sub_class_mask",
            crate::itemsubclass::item_sub_class_mask_schema(),
        ),
        (
            "factions::faction_template",
            crate::factions::faction_template_schema(),
        ),
        ("factions::faction", crate::factions::faction_schema()),
        (
            "factions::faction_group",
            crate::factions::faction_group_schema(),
        ),
        ("factions::chr_races", crate::factions::chr_races_schema()),
        (
            "creature_types",
            crate::creature_types::creature_type_schema(),
        ),
        ("languages::languages", crate::languages::languages_schema()),
        ("languages::chr_races", crate::languages::chr_races_schema()),
        (
            "languages::language_words",
            crate::languages::language_words_schema(),
        ),
        (
            "creature_families::family",
            crate::creature_families::family_schema(),
        ),
        (
            "creature_families::food",
            crate::creature_families::food_schema(),
        ),
        ("creature_spell_data", crate::creature_spell_data::schema()),
        ("resistances", crate::resistances::schema()),
        ("gameobjects", crate::gameobjects::schema()),
        ("durability::costs", crate::durability::costs_schema()),
        (
            "durability::qualities",
            crate::durability::qualities_schema(),
        ),
        ("exhaustion", crate::exhaustion::exhaustion_schema()),
        (
            "bank_bag_slot_prices",
            crate::bank_bag_slot_prices::schema(),
        ),
        ("stable_slot_prices", crate::stable_slot_prices::schema()),
        ("lock", crate::lock::schema()),
        ("lock_type", crate::lock_type::schema()),
        ("chr_races", crate::chr_races::schema()),
        ("chr_classes", crate::chr_classes::schema()),
        (
            "pet_stats::personality",
            crate::pet_stats::personality_schema(),
        ),
        ("pet_stats::loyalty", crate::pet_stats::loyalty_schema()),
        ("items", crate::items::item_display_info_schema()),
        (
            "item_sounds",
            crate::item_sounds::item_group_sounds_schema(),
        ),
        (
            "item_visuals::item_visuals",
            crate::item_visuals::item_visuals_schema(),
        ),
        (
            "item_visuals::item_visual_effects",
            crate::item_visuals::item_visual_effects_schema(),
        ),
        (
            "item_visuals::spell_item_enchantment",
            crate::item_visuals::spell_item_enchantment_schema(),
        ),
        (
            "item_random_properties",
            crate::item_random_properties::item_random_properties_schema(),
        ),
        (
            "ground_effects::texture",
            crate::ground_effects::texture_schema(),
        ),
        (
            "ground_effects::doodad",
            crate::ground_effects::doodad_schema(),
        ),
        ("light::light", crate::light::light_schema()),
        ("light::light_params", crate::light::light_params_schema()),
        ("loading_screen", crate::loading_screen::schema()),
        ("maps", crate::maps::map_schema()),
        ("anim_data", crate::anim_data::schema()),
        ("area_trigger", crate::area_trigger::area_trigger_schema()),
        ("area_sound::area", crate::area_sound::area_schema()),
        (
            "area_sound::zone_music",
            crate::area_sound::zone_music_schema(),
        ),
        ("area_sound::intro", crate::area_sound::intro_schema()),
        ("area_sound::ambience", crate::area_sound::ambience_schema()),
        ("creature_sound::csd", crate::creature_sound::csd_schema()),
        ("creature_sound::cdi", crate::creature_sound::cdi_schema()),
        ("creature_sound::cmd", crate::creature_sound::cmd_schema()),
        (
            "npc_greeting::npcsounds",
            crate::npc_greeting::npcsounds_schema(),
        ),
        ("npc_greeting::cdi", crate::npc_greeting::cdi_schema()),
        (
            "emote_text::emotes_text",
            crate::emote_text::emotes_text_schema(),
        ),
        (
            "emote_text::emotes_text_data",
            crate::emote_text::emotes_text_data_schema(),
        ),
        (
            "environmental_damage",
            crate::environmental_damage::schema(),
        ),
        (
            "sound_entries",
            crate::sound_entries::sound_entries_schema(),
        ),
        ("sound_provider", crate::sound_provider::schema()),
        ("sound_water", crate::sound_water::schema()),
        ("weapon_impact", crate::weapon_impact::schema()),
        ("sheathe", crate::sheathe::schema()),
        ("weapon_swing", crate::weapon_swing::schema()),
        ("material", crate::material::schema()),
        ("wmo_area", crate::wmo_area::schema()),
        ("spells", crate::spells::spell_schema()),
        (
            "skill_lines::skill_line",
            crate::skill_lines::skill_line_schema(),
        ),
        (
            "skill_lines::skill_line_ability",
            crate::skill_lines::skill_line_ability_schema(),
        ),
        (
            "skill_lines::skill_line_category",
            crate::skill_lines::skill_line_category_schema(),
        ),
        (
            "skill_lines::skill_race_class_info",
            crate::skill_lines::skill_race_class_info_schema(),
        ),
        ("spell_focus", crate::spell_focus::schema()),
        ("spell_mechanic", crate::spell_mechanic::schema()),
        ("talents::talent", crate::talents::talent_schema()),
        ("talents::talent_tab", crate::talents::talent_tab_schema()),
        (
            "spell_visual::effect_name",
            crate::spell_visual::effect_name_schema(),
        ),
        ("spell_visual::kit", crate::spell_visual::kit_schema()),
        (
            "spell_visual::chain_effects",
            crate::spell_visual::chain_effects::chain_effects_schema(),
        ),
        ("world_map_area", crate::world_map_area::schema()),
        ("world_map_overlay", crate::world_map_overlay::schema()),
        ("world_map_continent", crate::world_map_continent::schema()),
        ("area_poi", crate::area_poi::schema()),
        ("world_state_ui", crate::world_state_ui::schema()),
        ("area_table", crate::area_table::schema()),
        ("chat_channels", crate::chat_channels::schema()),
        ("server_messages", crate::server_messages::schema()),
        ("game_tips", crate::game_tips::schema()),
        ("taxi", crate::taxi::schema()),
        ("elevators", crate::elevators::schema()),
        ("taxi_nodes", crate::taxi_nodes::schema()),
        ("taxi_path", crate::taxi_path::schema()),
        // Argument-taking builders, one entry per constant call site.
        (
            "light::tables::int_band",
            crate::light::band_schema("LightIntBand", FieldType::UInt32),
        ),
        (
            "light::tables::float_band",
            crate::light::band_schema("LightFloatBand", FieldType::Float32),
        ),
        (
            "dbc::id_name::quest_info",
            crate::dbc::id_name_schema("QuestInfo", 1, 10),
        ),
        (
            "dbc::id_name::area_table",
            crate::dbc::id_name_schema("AreaTable", 11, 25),
        ),
        (
            "dbc::id_name::quest_sort",
            crate::dbc::id_name_schema("QuestSort", 1, 10),
        ),
        (
            "characters::customization::char_sections",
            crate::characters::all_u32_schema("CharSections", 10),
        ),
        (
            "characters::customization::char_facial_hair_styles",
            crate::characters::all_u32_schema("CharacterFacialHairStyles", 9),
        ),
        (
            "footsteps::ground_effect_texture",
            crate::footsteps::n_u32_schema("GroundEffectTexture", 7, &[]),
        ),
        (
            "footsteps::terrain_type",
            crate::footsteps::n_u32_schema("TerrainType", 6, &[1]),
        ),
        (
            "footsteps::footstep_terrain_lookup",
            crate::footsteps::n_u32_schema("FootstepTerrainLookup", 5, &[]),
        ),
        (
            "footsteps::footprint_textures",
            crate::footsteps::n_u32_schema("FootprintTextures", 2, &[1]),
        ),
        (
            "death_thud::terrain_type_sounds",
            crate::death_thud::n_u32_schema("TerrainTypeSounds", 1),
        ),
        (
            "death_thud::death_thud_lookups",
            crate::death_thud::n_u32_schema("DeathThudLookups", 5),
        ),
        (
            "spell_visual::spell_visual",
            crate::spell_visual::n_u32_schema("SpellVisual", crate::spell_visual::SPELL_VISUAL_FIELDS),
        ),
        (
            "emotes::emotes_text",
            crate::emotes::schema("EmotesText", 19, &[1]),
        ),
        (
            "emotes::emotes_text_sound",
            crate::emotes::schema("EmotesTextSound", 5, &[]),
        ),
        ("emotes::emotes", crate::emotes::schema("Emotes", 7, &[1])),
        (
            "gm_survey::current_survey",
            crate::gm_survey::schema("GMSurveyCurrentSurvey", 0, 1),
        ),
        (
            "gm_survey::surveys",
            crate::gm_survey::schema("GMSurveySurveys", 0, 10),
        ),
        (
            "gm_survey::questions",
            crate::gm_survey::schema("GMSurveyQuestions", 8, 0),
        ),
        (
            "text_filter_lists::chat_profanity",
            crate::text_filter_lists::schema("ChatProfanity"),
        ),
        (
            "text_filter_lists::spam_messages",
            crate::text_filter_lists::schema("SpamMessages"),
        ),
        (
            "text_filter_lists::names_profanity",
            crate::text_filter_lists::schema("NamesProfanity"),
        ),
        (
            "text_filter_lists::names_reserved",
            crate::text_filter_lists::schema("NamesReserved"),
        ),
    ]
}
