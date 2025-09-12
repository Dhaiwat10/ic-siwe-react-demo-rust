mod service;
mod user_profile;

use candid::{CandidType, Principal};
use ic_cdk::{init, post_upgrade};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde::Deserialize;
use std::cell::RefCell;
use user_profile::UserProfile;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USER_PROFILES: RefCell<StableBTreeMap<String, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static IC_SIWE_PROVIDER: RefCell<Option<Principal>> = const { RefCell::new(None) };
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct SettingsInput {
    pub ic_siwe_provider: String,
}

fn save_settings(settings: SettingsInput) {
    IC_SIWE_PROVIDER.with_borrow_mut(|ic_siwe_provider| {
        let principal = Principal::from_text(settings.ic_siwe_provider).unwrap();
        *ic_siwe_provider = Some(principal);
    })
}

#[init]
fn init(settings: SettingsInput) {
    save_settings(settings);
}

#[post_upgrade]
fn post_upgrade(settings: SettingsInput) {
    save_settings(settings);
}
