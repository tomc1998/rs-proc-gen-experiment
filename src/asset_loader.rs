//! Module to load assets (i.e. tilemaps, textures) from the asset definition files

use gfx_device_gl::Resources;
use gfx::handle::ShaderResourceView;
use renderer::TextureKey;
use gfx_device_gl::Factory;
use renderer::atlas::*;
use renderer::ASSET_NAME_MAP;
use serde_yaml;
use std::collections::BTreeMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum FrameSetEntryType {
    Ordered,
}

#[derive(Serialize, Deserialize, Debug)]
struct FrameSetEntry {
    #[serde(rename = "type")]
    entry_type: FrameSetEntryType,
    start: usize,
    end: usize,
}

impl FrameSetEntry {
    /// Convert this to a 'frames' value, used for frame sets
    pub fn to_frames(&self) -> Frames {
        match self.entry_type {
            FrameSetEntryType::Ordered =>
                Frames::Ordered(self.start, self.end)
        }
    }
}

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
        frame_w: u16,
        frame_h: u16,
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
        tiles_x: u32,
        tiles_y: u32,
    },
    BitmapFont {
        name: String,
        filename: String,
        glyph_w: u16,
        glyph_h: u16,
        char_map: BTreeMap<char, [u16; 2]>,
    },
    FrameSet {
        name: String,
        frames: Vec<FrameSetEntry>
    }
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
            AssetDefinition::FrameSet {
                name, ..
            } => name.clone(),
        }
    }
}

/// Load all the game assets from the asset descriptions. Just panics if shit is fucked.
pub fn load_assets(factory: &mut Factory) -> (TextureAtlas<TextureKey>,
                                              ShaderResourceView<Resources, [f32; 4]>) {
    let mut builder = AtlasBuilder::<TextureKey>::new(512, 512);
    // Load all filenames & add them to one big asset list.
    let asset_list : Vec<(usize, AssetDefinition)> =
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
        }).enumerate().collect();

    // Keep the strings in a map of strings to IDs.
    {
        let mut anm = ASSET_NAME_MAP.write().unwrap();
        asset_list.iter().for_each(|(ix, def)| {
            anm.insert(def.name(), *ix);
        });
    }

    // Process all frame sets
    let mut frame_sets : BTreeMap<&String, usize> = BTreeMap::new();
    asset_list.iter().for_each(|(_, asset_def)| {
        match asset_def {
            AssetDefinition::FrameSet { name, frames } => {
                frame_sets.insert(name, builder.add_frame_set(FrameSet {
                    frames: frames.iter().map(FrameSetEntry::to_frames).collect()
                }));
            }
            _ => ()
        }
    });

    // Maps of anim names to texture keys
    let mut anims : BTreeMap<&String, TextureKey> = BTreeMap::new();

    // Loop over assets, and insert into the atlas for each asset
    asset_list.iter().for_each(|(ix, asset_def)| {
        match asset_def {
            AssetDefinition::Tex {filename, ..} => {
                builder.add_tex(*ix, filename).unwrap();
            }
            AssetDefinition::Anim {filename, frame_set, frame_w, frame_h, name} => {
                builder.add_anim_sprite(
                    filename, *ix,
                    *frame_sets.get(frame_set)
                        .expect(&format!("Couldn't find frameset {}\nFramesets: {:?}",
                                         frame_set, frame_sets)),
                    *frame_w, *frame_h).unwrap();
                anims.insert(name, *ix);
            }
            AssetDefinition::Tileset {filename, tiles_x, tiles_y, ..} => {
                builder.add_tileset(*ix, filename, *tiles_x, *tiles_y).unwrap();
            }
            AssetDefinition::BitmapFont {filename, glyph_w, glyph_h, char_map, ..} => {
                let char_map : Vec<(char, (u16, u16))> = char_map.iter().map(|(k, [x, y])| (*k, (*x, *y))).collect();
                builder.add_bitmap_font(
                    *ix, filename, &char_map[..], *glyph_w, *glyph_h).unwrap();
            }
            _ => ()
        }
    });

    // Link up animicons
    asset_list.iter().for_each(|(ix, asset_def)| {
        match asset_def {
            AssetDefinition::AnimIcon {anim, x, y, w, h, ..} => {
                builder.add_anim_icon(*ix, *anims.get(anim).unwrap(), *x, *y, *w, *h);
            }
            _ => ()
        }
    });

    builder.build(factory)
}
