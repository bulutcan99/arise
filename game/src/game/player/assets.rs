use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

//TODO: simdilik burayi kullanma!
#[derive(AssetCollection, Resource)]
pub struct Asset {
    //Sprite sheet
    #[asset(key = "texture/hunter.jpg")]
    pub hunter: Handle<Image>,

    #[asset(key = "effects/slash.png")]
    pub slash_effect: Handle<Image>,

    #[asset(key = "effects/skill1.png")]
    pub skill1: Handle<Image>,

    #[asset(key = "effects/skill2.png")]
    pub skill2: Handle<Image>,

    #[asset(key = "effects/skill3.png")]
    pub skill3: Handle<Image>,

    // Ses efektleri
    #[asset(key = "audio/slash.ogg")]
    pub slash_sound: Handle<AudioSource>,

    #[asset(key = "audio/skill1.ogg")]
    pub skill1_sound: Handle<AudioSource>,

    #[asset(key = "audio/skill2.ogg")]
    pub skill2_sound: Handle<AudioSource>,

    #[asset(key = "audio/skill3.ogg")]
    pub skill3_sound: Handle<AudioSource>,
}

//TODO: karakterin sprite sheet hali eklenecek
