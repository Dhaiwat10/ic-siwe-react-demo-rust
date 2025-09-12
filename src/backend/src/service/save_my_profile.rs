use crate::{user_profile::UserProfile, IC_SIWE_PROVIDER, USER_PROFILES};
use ic_cdk::{call::Call, update};
use serde_bytes::ByteBuf;

pub async fn get_address() -> Result<String, String> {
    let ic_siwe_provider = IC_SIWE_PROVIDER.with_borrow(|p| *p.as_ref().unwrap());
    let user_principal = ByteBuf::from(ic_cdk::api::msg_caller().as_slice());
    Call::unbounded_wait(ic_siwe_provider, "get_address")
        .with_arg(&user_principal)
        .await
        .expect("Failed to get response")
        .candid::<Result<String, String>>()
        .expect("Failed to decode Candid")
}

#[update]
async fn save_my_profile(name: String, avatar_url: String) -> Result<UserProfile, String> {
    // Get the address of the caller from the siwe provider canister, return error if it fails. A failure
    // here means that the caller is not authenticated using the siwe provider. This might happen if the
    // caller uses an anonymous principal or has authenticated using a different identity provider.
    let address = get_address().await?;

    // If user has an address and thus is authenticated, create a profile and save it.
    let profile = UserProfile {
        address,
        name,
        avatar_url,
    };

    USER_PROFILES.with(|p| {
        let mut profiles = p.borrow_mut();
        profiles.insert(ic_cdk::api::msg_caller().to_string(), profile.clone());
    });

    Ok(profile)
}
