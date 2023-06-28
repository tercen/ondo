use crate::db::reference::{
    effect::{AccessEffect, Effect, Effects, MetaEffect},
    ColumnValueEffect, DatabaseServerStoredEffect, DomainStoredEffect, IndexValueEffect,
    TableStoredEffect, TableValueEffect,
};
use std::collections::HashSet;

pub(crate) fn optimize_delete_cf_effects(effects: Effects) -> Effects {
    let mut delete_cf_names = HashSet::new();
    let mut delete_cf_encountered = false;

    let optimized_effects = effects
        .into_iter()
        .rev()
        .filter_map(|effect| match effect.clone() {
            Effect::Meta(MetaEffect::DeleteCf(cf_name)) => {
                delete_cf_encountered = true;
                delete_cf_names.insert(cf_name.clone());
                None
            }
            Effect::Access(access_effect) if delete_cf_encountered => match access_effect {
                AccessEffect::DatabaseServerStoredEffect(DatabaseServerStoredEffect::Put(
                    cf_name,
                    _,
                    _,
                ))
                | AccessEffect::DatabaseServerStoredEffect(DatabaseServerStoredEffect::Delete(
                    cf_name,
                    _,
                ))
                | AccessEffect::DomainStoredEffect(DomainStoredEffect::Put(cf_name, _, _))
                | AccessEffect::DomainStoredEffect(DomainStoredEffect::Delete(cf_name, _))
                | AccessEffect::TableStoredEffect(TableStoredEffect::Put(cf_name, _, _))
                | AccessEffect::TableStoredEffect(TableStoredEffect::Delete(cf_name, _))
                | AccessEffect::IndexValueEffect(IndexValueEffect::Put(cf_name, _, _))
                | AccessEffect::IndexValueEffect(IndexValueEffect::Delete(cf_name, _))
                | AccessEffect::TableValueEffect(TableValueEffect::Put(cf_name, _, _))
                | AccessEffect::TableValueEffect(TableValueEffect::Delete(cf_name, _))
                | AccessEffect::ColumnValueEffect(ColumnValueEffect::Put(cf_name, _, _))
                | AccessEffect::ColumnValueEffect(ColumnValueEffect::Delete(cf_name, _)) => {
                    if !delete_cf_names.contains(&cf_name) {
                        Some(effect)
                    } else {
                        None
                    }
                }
            },
            _ => Some(effect),
        })
        .collect::<Vec<_>>();

    optimized_effects.into_iter().rev().collect()
}
