use renderer::*;
use comp::ANIM_SPRITE_NO_LOOP;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimDataDef {
    anim_num: usize,
    num_frames: usize,
    frame_time: f32,
    flags: Vec<String>,
}

impl AnimDataDef {
    /// Create a list of item details from this 
    fn link_assets(&self) -> AnimData {
        AnimData {
            anim_num: self.anim_num,
            num_frames: self.num_frames,
            frame_time: self.frame_time,
            flags: self.get_flags(),
        }
    }

    /// Converts the flag list to a u8
    fn get_flags(&self) -> u8 {
        let mut total = 0;
        for f in &self.flags {
            match f.as_ref() {
                "ANIM_SPRITE_NO_LOOP" => total |= ANIM_SPRITE_NO_LOOP,
                f => panic!("Unknown flag {}", f),
            }
        }
        total
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InWorldGfxDef {
    tex_key: String,
    width: f32,
    height: f32,
    anim_data: Option<AnimDataDef>,
}

impl InWorldGfxDef {
    /// Create a list of item details from this 
    fn link_assets(&self) -> InWorldGfx {
        InWorldGfx {
            tex_key: get_asset_by_name(&self.tex_key),
            width: self.width,
            height: self.width,
            anim_data: self.anim_data.as_ref().map(|ad| ad.link_assets()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentDataDef {
    equipment_type: EquipmentType,
    anim_key: Option<String>,
}

impl EquipmentDataDef {
    /// Create a list of item details from this
    fn link_assets(&self) -> EquipmentData {
        EquipmentData {
            equipment_type: self.equipment_type,
            anim_key: self.anim_key.as_ref().map(|s| get_asset_by_name(&s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemDetailsDef {
    in_world_gfx: InWorldGfxDef,
    icon: String,
    equipment_data: Option<EquipmentDataDef>,
    stacks: bool,
}

impl ItemDetailsDef {
    /// Create a list of item details from this definition, by linking in the
    /// texture assets and doing any other pre-processing
    pub fn link_assets(&self, name: String) -> ItemDetails {
        ItemDetails {
            in_world_gfx: self.in_world_gfx.link_assets(),
            icon: get_asset_by_name(&self.icon),
            equipment_data: self.equipment_data.as_ref().map(|edd| edd.link_assets()),
            stacks: self.stacks,
            name: name,
        }
    }
}

/// Load item defs from files. Returns a map of item names to details.
pub fn load_defs() -> BTreeMap<String, ItemDetailsDef> {
    // Load all filenames & add them to one big asset list.
    let mut items : BTreeMap<String, ItemDetailsDef> = BTreeMap::new();
    // First, get all paths
    fs::read_dir("res/item-types").unwrap().map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            panic!("Directories not allowed in item-types directory. A \
                    flat-file structure must be used.");
        } else {
            path
        }
    }).for_each(|path| {
        let mut f = fs::File::open(path).unwrap();
        let mut definitions : BTreeMap<String, ItemDetailsDef> =
            serde_yaml::from_reader(&mut f).unwrap();
        items.append(&mut definitions);
    });

    items
}
