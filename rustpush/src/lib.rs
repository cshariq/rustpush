
mod activation;
mod aps;
mod util;
mod imessage;
mod mmcs;
mod error;
mod auth;
mod ids;
pub mod sharedstreams;
pub mod findmy;
pub mod facetime;
pub mod cloudkit;
#[cfg(feature = "macos-support")]
mod macos;

#[cfg(feature = "mock-absinthe")]
mod macos {
    // Mock implementation of the macos module
    // Include the mock implementations of the open_absinthe functions and structs used in macos.rs
    use crate::mocks::open_absinthe::nac::{HardwareConfig, ValidationCtx};
    use crate::mocks::open_absinthe::AbsintheError;
    use crate::{DebugMeta, OSConfig, RegisterMeta, PushError};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use plist::Dictionary;
    use async_trait::async_trait;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MacOSConfig {
        pub inner: HardwareConfig,
    
        // software
        pub version: String,
        pub protocol_version: u32,
        pub device_id: String,
        pub icloud_ua: String,
        pub aoskit_version: String,
    }
    
    #[async_trait]
    impl OSConfig for MacOSConfig{
        fn build_activation_info(&self, _csr: Vec<u8>) -> crate::activation::ActivationInfo {
             crate::activation::ActivationInfo {
                activation_randomness: "".to_string(),
                activation_state: "Unactivated".to_string(),
                build_version: self.inner.os_build_num.clone(),
                device_cert_request: Data::new(vec![]),
                device_class: "MacOS".to_string(),
                product_type: self.inner.product_name.clone(),
                product_version: self.version.clone(),
                serial_number: self.inner.platform_serial_number.clone(),
                unique_device_id: self.device_id.clone().to_uppercase(),
            }
        }
        fn get_normal_ua(&self, _item: &str) -> String {
            "".to_string()
        }
        fn get_aoskit_version(&self) -> String {
            self.aoskit_version.clone()
        }
        fn get_mme_clientinfo(&self, _for_item: &str) -> String {
             format!("<{}> <macOS;{};{}> <{}>", self.inner.product_name, self.version, "".to_string(), "".to_string())
        }
        fn get_version_ua(&self) -> String {
            format!("[macOS,{},{},{}]", self.version, "".to_string(), self.inner.product_name)
        }
        fn get_activation_device(&self) -> String {
            "MacOS".to_string()
        }
        fn get_device_uuid(&self) -> String {
            self.device_id.clone()
        }
        fn get_device_name(&self) -> String {
            format!("Mac-{}", self.inner.platform_serial_number)
        }
        async fn generate_validation_data(&self) -> Result<Vec<u8>, PushError> {
            Ok(vec![])
        }
        fn get_protocol_version(&self) -> u32 {
            self.protocol_version
        }
        fn get_register_meta(&self) -> RegisterMeta {
             RegisterMeta {
                hardware_version: self.inner.product_name.clone(),
                os_version: format!("macOS,{},{}", self.version, "".to_string()),
                software_version: self.inner.os_build_num.clone(),
            }
        }
        fn get_debug_meta(&self) -> DebugMeta {
             DebugMeta {
                user_version: self.version.clone(),
                hardware_version: self.inner.product_name.clone(),
                serial_number: self.inner.platform_serial_number.clone(),
            }
        }
         fn get_gsa_hardware_headers(&self) -> HashMap<String, String> {
            [
                ("X-Apple-I-MLB", "".as_str()),
                ("X-Apple-I-ROM", "".as_str()),
                ("X-Apple-I-SRL-NO", "".as_str()),
            ].into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
        }
        fn get_serial_number(&self) -> String {
            self.inner.platform_serial_number.clone()
        }
        fn get_login_url(&self) -> &'static str{
            ""
        }
    
        fn get_private_data(&self) -> Dictionary {
            Dictionary::new()
        }
    }
}

#[cfg(feature = "macOS")]
mod macos;

mod relay;

pub mod mmcsp {
    include!(concat!(env!("OUT_DIR"), "/mmcsp.rs"));
}

use std::collections::HashMap;
use std::fmt::Debug;

use activation::ActivationInfo;
pub use aps::{APSConnectionResource, APSConnection, APSMessage, APSState};
use async_trait::async_trait;
pub use mmcs::{FileContainer, prepare_put};
pub use omnisette::AnisetteProvider;
use icloud_auth::LoginClientInfo;
pub use imessage::messages::{UpdateProfileMessage, UpdateProfileSharingMessage, MessageInst, ShareProfileMessage, SharedPoster, ScheduleMode, PermanentDeleteMessage, OperatedChat, DeleteTarget, MoveToRecycleBinMessage, TextFormat, TextEffect, TextFlags, LinkMeta, LPLinkMetadata, ReactMessageType, ErrorMessage, Reaction, UnsendMessage, EditMessage, UpdateExtensionMessage, PartExtension, ReactMessage, ChangeParticipantMessage, LPImageMetadata, RichLinkImageAttachmentSubstitute, LPIconMetadata, AttachmentType, ExtensionApp, BalloonLayout, Balloon, ConversationData, Message, MessageType, Attachment, NormalMessage, RenameMessage, IconChangeMessage, MessageParts, MessagePart, MMCSFile, IndexedMessagePart};
pub use imessage::aps_client::{IMClient, MADRID_SERVICE};
use openssl::conf;
use util::encode_hex;
pub use util::{NSArrayClass, ResourceState, NSDictionaryClass, NSURL, NSArray, ResourceFailure};
pub use ids::user::{IDSUser, register, IDSUserIdentity, IDSNGMIdentity, PrivateDeviceInfo, SupportAlert, SupportAction};
pub use ids::identity_manager::{SendJob, MessageTarget, IdentityManager};
pub use auth::{authenticate_apple, login_apple_delegates, authenticate_phone, AuthPhone, LoginDelegate};
pub use error::PushError;
#[cfg(feature = "macOS")]
pub use macos::MacOSConfig;
#[cfg(feature = "macOS")]
pub use open_absinthe::nac::HardwareConfig;
pub use cloudkit_proto;
pub use cloudkit_derive;
pub use imessage::name_photo_sharing;

use plist::Dictionary;
pub use relay::RelayConfig;
pub use util::get_gateways_for_mccmnc;


pub struct RegisterMeta {
    pub hardware_version: String,
    pub os_version: String,
    pub software_version: String,
}

pub struct DebugMeta {
    pub user_version: String,
    pub hardware_version: String,
    pub serial_number: String,
}

#[async_trait]
pub trait OSConfig: Sync + Send {
    fn build_activation_info(&self, csr: Vec<u8>) -> ActivationInfo;
    fn get_activation_device(&self) -> String;
    async fn generate_validation_data(&self) -> Result<Vec<u8>, PushError>;
    fn get_protocol_version(&self) -> u32;
    fn get_register_meta(&self) -> RegisterMeta;
    fn get_normal_ua(&self, item: &str) -> String;
    fn get_mme_clientinfo(&self, for_item: &str) -> String;
    fn get_version_ua(&self) -> String;
    fn get_device_name(&self) -> String;
    fn get_device_uuid(&self) -> String;
    fn get_private_data(&self) -> Dictionary;
    fn get_debug_meta(&self) -> DebugMeta;
    fn get_login_url(&self) -> &'static str;
    fn get_serial_number(&self) -> String;
    fn get_gsa_hardware_headers(&self) -> HashMap<String, String>;
    fn get_aoskit_version(&self) -> String;
    fn get_udid(&self) -> String { "55A1CFBF5BB56AD1159BD2CB7D6FF546E48EAAE4BF16188A07B1FB9C83138CA2".to_string() }

    fn get_adi_mme_info(&self, for_item: &str) -> String {
        self.get_mme_clientinfo(for_item)
    }

    fn get_gsa_config(&self, push: &APSState) -> LoginClientInfo {
        LoginClientInfo {
            ak_context_type: "imessage".to_string(),
            client_app_name: "Messages".to_string(),
            client_bundle_id: "com.apple.MobileSMS".to_string(),
            mme_client_info_akd: self.get_adi_mme_info("com.apple.AuthKit/1 (com.apple.akd/1.0)"),
            mme_client_info: self.get_adi_mme_info("com.apple.AuthKit/1 (com.apple.MobileSMS/1262.500.151.1.2)"),
            akd_user_agent: "akd/1.0 CFNetwork/1494.0.7 Darwin/23.4.0".to_string(),
            browser_user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko)".to_string(),
            hardware_headers: self.get_gsa_hardware_headers(),
            push_token: push.token.map(|i| encode_hex(&i).to_uppercase()),
            update_account_bundle_id: self.get_adi_mme_info("com.apple.AppleAccount/1.0 (com.apple.systempreferences.AppleIDSettings/1)"),
        }
    }
}

extern crate pretty_env_logger;
extern crate log;