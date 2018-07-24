use renderer::TextureKey;

pub enum Helmet {
    BronzeHelmet
}

impl Helmet {
    pub fn get_anim_key(&self) -> TextureKey {
        match self {
            Helmet::BronzeHelmet => TextureKey::BronzeHelmetAnim,
        }
    }
}

pub enum Body {
}

impl Body {
    pub fn get_anim_key(&self) -> TextureKey {
        match self {
            _ => unimplemented!()
        }
    }
}

pub enum Weapon {
}

pub enum Ring {
}
