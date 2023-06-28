use crate::db::reference::effect::{Effect, Effects};

pub(crate) fn split_effects(effects: Effects) -> (Effects, Effects) {
    let mut meta_effects = Vec::new();
    let mut access_effects = Vec::new();

    for effect in effects {
        match effect {
            Effect::Meta(meta_effect) => meta_effects.push(Effect::Meta(meta_effect)),
            Effect::Access(access_effect) => access_effects.push(Effect::Access(access_effect)),
        }
    }

    (meta_effects, access_effects)
}
