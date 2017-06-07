use de::*;
use icons;
use serde::de::{self, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap as Map;
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;
use themes::{self, Theme};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub blocks: Map<String, Value>,
    #[serde(default = "icons::default", deserialize_with = "deserialize_icons")]
    pub icons: Map<String, String>,
    #[serde(default = "themes::default", deserialize_with = "deserialize_themes")]
    pub theme: Theme,
}

fn deserialize_icons<'de, D>(deserializer: D) -> Result<Map<String, String>, D::Error>
where
    D: Deserializer<'de>
{
    map_type!(Icons, String, String;
              s => Ok(Icons(icons::get_icons(s).ok_or_else(|| "cannot find specified icons")?)));

    deserializer.deserialize_any(MapType::<Icons, String, String>(PhantomData, PhantomData, PhantomData))
}

fn deserialize_themes<'de, D>(deserializer: D) -> Result<Theme, D::Error>
where
    D: Deserializer<'de>
{
    map_type!(ThemeIntermediary, String, String;
              s => Ok(ThemeIntermediary(themes::get_theme(s).ok_or_else(|| "cannot find specified theme")?.owned_map())));

    let intermediary: Map<String, String> = deserializer.deserialize_any(MapType::<ThemeIntermediary, String, String>(PhantomData, PhantomData, PhantomData))?;

    Deserialize::deserialize(de::value::MapDeserializer::new(intermediary.into_iter()))
}
