//! NeoForge's `neoforge:add_features` biome modifiers, read from content
//! mods (`data/<ns>/neoforge/biome_modifier/*.json`): carried NeoForge mods
//! add their ores and plants to biomes this way (Mekanism's osmium, tin,
//! uranium...). The features join each named biome's list for the step, in
//! the worldgen loader's `worldgen/biomes.json` bundle, as NeoForge appends
//! them to the biome's generation settings. Other modifier types (remove
//! features, spawns, carvers) are skipped with a warning.

use super::J;

/// `GenerationStep.Decoration`, in vanilla's order: a biome's `features`
/// holds one list per step at these indexes.
const STEPS: [&str; 11] = [
    "raw_generation",
    "lakes",
    "local_modifications",
    "underground_structures",
    "surface_structures",
    "strongholds",
    "underground_ores",
    "underground_decoration",
    "fluid_springs",
    "vegetal_decoration",
    "top_layer_modification",
];

/// Whether `file` is a biome modifier of a content mod.
pub(super) fn is_modifier(file: &str) -> bool {
    file.strip_prefix("data/")
        .and_then(|r| r.split_once("/neoforge/biome_modifier/"))
        .is_some_and(|(_, path)| path.ends_with(".json"))
}

/// The biome ids a `biomes` holder set names: an id, `#tag`, or a list of
/// either. `tags` is the worldgen loader's biome tag bundle (tag id → the
/// tag file's text); `known` the biomes it has. Unknown ids and tags are
/// skipped, as a tag entry naming an unknown id is.
fn biomes(value: &J, tags: &J, known: &dyn Fn(&str) -> bool, out: &mut Vec<String>, depth: u8) {
    if depth > 16 {
        return;
    }
    match value {
        J::Str(s) => match s.strip_prefix('#') {
            Some(tag) => {
                let Some(J::Str(text)) = tags.get(tag) else {
                    return;
                };
                let Ok(doc) = serde_json::from_str::<J>(text) else {
                    return;
                };
                if let Some(values) = doc.get("values") {
                    biomes(values, tags, known, out, depth + 1);
                }
            }
            None => {
                if known(s) && !out.contains(s) {
                    out.push(s.clone());
                }
            }
        },
        J::Arr(items) => {
            for item in items {
                biomes(item, tags, known, out, depth + 1);
            }
        }
        // A tag entry may be {"id": "...", "required": false}.
        J::Obj(_) => {
            if let Some(id) = value.get("id") {
                biomes(id, tags, known, out, depth + 1);
            }
        }
        _ => {}
    }
}

/// Applies `modifiers` ((file, text), in mod and file order) to the biome
/// bundle `bundle` (biome id → the biome file's text). Returns warnings.
pub(super) fn apply(
    bundle: &mut J,
    tags: &J,
    modifiers: &[(String, String)],
) -> super::Result<Vec<String>> {
    let mut warnings = Vec::new();
    let ids: Vec<String> = match &*bundle {
        J::Obj(o) => o.iter().map(|(k, _)| k.clone()).collect(),
        _ => return Ok(warnings),
    };
    let known = |id: &str| ids.iter().any(|k| k == id);
    for (file, text) in modifiers {
        let doc: J = serde_json::from_str(text).map_err(|e| format!("{file}: {e}"))?;
        match doc.get("type") {
            Some(J::Str(t)) if t == "neoforge:add_features" => {}
            Some(J::Str(t)) if t == "neoforge:none" => continue,
            other => {
                warnings.push(format!(
                    "{file}: biome modifier {other:?} is not supported; skipped"
                ));
                continue;
            }
        }
        let Some(J::Str(step)) = doc.get("step") else {
            return Err(format!("{file}: no step").into());
        };
        let Some(index) = STEPS.iter().position(|s| s == step) else {
            return Err(format!("{file}: unknown step {step}").into());
        };
        let mut features = Vec::new();
        match doc.get("features") {
            Some(J::Str(s)) => features.push(J::Str(s.clone())),
            Some(J::Arr(a)) => features.extend(a.iter().cloned()),
            _ => return Err(format!("{file}: features is not an id or a list of ids").into()),
        }
        if features
            .iter()
            .any(|f| matches!(f, J::Str(s) if s.starts_with('#')))
        {
            warnings.push(format!("{file}: feature tags are not supported; skipped"));
            continue;
        }
        let mut targets = Vec::new();
        if let Some(value) = doc.get("biomes") {
            biomes(value, tags, &known, &mut targets, 0);
        }
        for biome in targets {
            let Some(J::Str(text)) = bundle.get(&biome) else {
                continue;
            };
            let mut b: J = serde_json::from_str(text).map_err(|e| format!("{biome}: {e}"))?;
            let Some(J::Arr(steps)) = b.get_mut("features") else {
                continue;
            };
            while steps.len() <= index {
                steps.push(J::Arr(Vec::new()));
            }
            if let J::Arr(list) = &mut steps[index] {
                for f in &features {
                    if !list.contains(f) {
                        list.push(f.clone());
                    }
                }
            }
            bundle.set(&biome, J::Str(serde_json::to_string(&b)?));
        }
    }
    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn j(text: &str) -> J {
        serde_json::from_str(text).unwrap()
    }

    fn features(bundle: &J, biome: &str) -> serde_json::Value {
        let Some(J::Str(text)) = bundle.get(biome) else {
            panic!("{biome} missing");
        };
        serde_json::from_str::<serde_json::Value>(text).unwrap()["features"].clone()
    }

    #[test]
    fn add_features_joins_the_step_of_each_biome_a_tag_names() {
        let plains = r#"{"temperature": 0.8, "features": [[], [], [], [], [], [], ["minecraft:ore_iron_upper"]]}"#;
        let desert = r#"{"features": [["minecraft:lake_lava_underground"]]}"#;
        let end = r#"{"features": []}"#;
        let mut bundle = J::Obj(vec![
            ("minecraft:plains".into(), J::Str(plains.into())),
            ("minecraft:desert".into(), J::Str(desert.into())),
            ("minecraft:the_end".into(), J::Str(end.into())),
        ]);
        let tags = j(r##"{
            "mekanism:spawn_ores": "{\"values\": [\"#minecraft:is_overworld\", \"othermod:glade\"]}",
            "minecraft:is_overworld": "{\"values\": [\"minecraft:plains\", {\"id\": \"minecraft:desert\", \"required\": false}]}"
        }"##);
        let modifiers = vec![
            (
                "data/mekanism/neoforge/biome_modifier/osmium.json".into(),
                r##"{"type": "neoforge:add_features", "biomes": "#mekanism:spawn_ores",
                     "features": ["mekanism:ore_osmium_upper", "mekanism:ore_osmium_small"],
                     "step": "underground_ores"}"##
                    .into(),
            ),
            (
                "data/mekanism/neoforge/biome_modifier/spawns.json".into(),
                r#"{"type": "neoforge:add_spawns", "biomes": "minecraft:plains", "spawners": []}"#
                    .into(),
            ),
        ];
        let warnings = apply(&mut bundle, &tags, &modifiers).unwrap();
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert_eq!(
            features(&bundle, "minecraft:plains")[6],
            serde_json::json!([
                "minecraft:ore_iron_upper",
                "mekanism:ore_osmium_upper",
                "mekanism:ore_osmium_small"
            ])
        );
        // A shorter list grows to the step; earlier steps stay as they were.
        let desert = features(&bundle, "minecraft:desert");
        assert_eq!(
            desert[0],
            serde_json::json!(["minecraft:lake_lava_underground"])
        );
        assert_eq!(
            desert[6],
            serde_json::json!(["mekanism:ore_osmium_upper", "mekanism:ore_osmium_small"])
        );
        // A biome the tag does not name is left alone, other keys too.
        assert_eq!(
            features(&bundle, "minecraft:the_end"),
            serde_json::json!([])
        );
        let Some(J::Str(text)) = bundle.get("minecraft:plains") else {
            panic!()
        };
        assert!(text.contains("\"temperature\":0.8"), "{text}");
    }

    #[test]
    fn applying_twice_adds_a_feature_once_and_bad_steps_are_errors() {
        let mut bundle = J::Obj(vec![(
            "minecraft:plains".into(),
            J::Str(r#"{"features": []}"#.into()),
        )]);
        let tags = J::Obj(Vec::new());
        let m = vec![(
            "data/m/neoforge/biome_modifier/a.json".to_owned(),
            r#"{"type": "neoforge:add_features", "biomes": ["minecraft:plains"], "features": "m:f", "step": "lakes"}"#.to_owned(),
        )];
        apply(&mut bundle, &tags, &m).unwrap();
        apply(&mut bundle, &tags, &m).unwrap();
        assert_eq!(
            features(&bundle, "minecraft:plains")[1],
            serde_json::json!(["m:f"])
        );
        let bad = vec![(
            "data/m/neoforge/biome_modifier/b.json".to_owned(),
            r#"{"type": "neoforge:add_features", "biomes": [], "features": [], "step": "sideways"}"#.to_owned(),
        )];
        assert!(apply(&mut bundle, &tags, &bad).is_err());
        assert!(is_modifier("data/m/neoforge/biome_modifier/a.json"));
        assert!(!is_modifier("data/m/neoforge/data_maps/a.json"));
        // 26.3 keeps configured features in worldgen/feature/; older packs in configured_feature/.
        for file in [
            "data/m/worldgen/feature/ore.json",
            "data/m/worldgen/configured_feature/ore.json",
        ] {
            assert_eq!(
                super::super::worldgen_entry(file),
                Some(("worldgen/features.json", "m:ore".to_owned()))
            );
        }
    }
}
