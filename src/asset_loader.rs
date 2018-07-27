//! Module to load assets (i.e. tilemaps, textures) from the asset definition files

use gfx_device_gl::Resources;
use gfx::handle::ShaderResourceView;
use renderer::TextureKey;
use gfx_device_gl::Factory;
use renderer::atlas::*;
use serde_yaml;
use std::collections::BTreeMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum AssetDefinition {
    Tex {
        name: String,
        filename: String,
    },
    Anim {
        name: String,
        filename: String,
        frame_set: String,
        frame_w: u32,
        frame_h: u32,
    },
    AnimIcon {
        name: String,
        anim: String,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
    Tileset {
        name: String,
        filename: String,
        frame_w: u32,
        frame_h: u32,
    },
    BitmapFont {
        name: String,
        filename: String,
        glyph_w: u32,
        glyph_h: u32,
        char_map: BTreeMap<char, [u32; 2]>,
    },
}

impl AssetDefinition {
    fn name(&self) -> String {
        match self {
            AssetDefinition::Tex {
                name, ..
            } => name.clone(),
            AssetDefinition::Anim {
                name, ..
            } => name.clone(),
            AssetDefinition::AnimIcon {
                name, ..
            } => name.clone(),
            AssetDefinition::Tileset {
                name, ..
            } => name.clone(),
            AssetDefinition::BitmapFont {
                name, ..
            } => name.clone(),
        }
    }
}

/// Load all the game assets from the asset descriptions. Just panics if shit is fucked.
pub fn load_assets(_factory: &mut Factory) -> (TextureAtlas<TextureKey>, ShaderResourceView<Resources, [f32; 4]>) {
    let mut _builder = AtlasBuilder::<TextureKey>::new(512, 512);
    // Load all filenames & add them to one big asset list.
    let asset_list =
        // First, get all paths
        fs::read_dir("res/asset-definitions").unwrap().map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                panic!("Directories not allowed in asset-definitions directory. A \
                        flat-file structure must be used.");
            } else {
                path
            }
        }).flat_map(|path| {
            let mut f = fs::File::open(path).unwrap();
            let definitions : Vec<AssetDefinition> =
                serde_yaml::from_reader(&mut f).unwrap();
            definitions.into_iter()
        });

    // Assign 'ids' to asset definitions (implicit IDs with index into vec)
    let asset_map : Vec<AssetDefinition> = asset_list.collect();

    // Keep the strings in a map of strings to IDs.
    let mut asset_id_map = BTreeMap::new();
    for (ix, def) in asset_map.iter().enumerate() {
        asset_id_map.insert(def.name(), ix);
    }

    println!("{:?}", asset_id_map);
    println!("{:?}", asset_map);
    unimplemented!();
}
