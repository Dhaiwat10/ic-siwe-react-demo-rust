use ic_cdk::query;

use crate::{user_profile::UserProfile, USER_PROFILES};

#[query]
fn list_profiles() -> Result<Vec<(String, UserProfile)>, String> {
    let profiles: Vec<(String, UserProfile)> = USER_PROFILES.with_borrow(|p| {
        p.iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect()
    });
    Ok(profiles)
}
