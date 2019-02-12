// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Common types for KTM exposed at both the Nt- and Win32-layer
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, ULONG};
use um::winnt::{LARGE_INTEGER, PVOID, WCHAR};
pub type UOW = GUID;
pub type PUOW = *mut GUID;
pub type CRM_PROTOCOL_ID = GUID;
pub type PCRM_PROTOCOL_ID = *mut GUID;
pub const TRANSACTION_MANAGER_VOLATILE: ULONG = 0x00000001;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: ULONG = 0x00000000;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: ULONG = 0x00000002;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: ULONG = 0x00000004;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: ULONG = 0x00000008;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: ULONG = 0x00000010;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: ULONG = 0x00000020;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: ULONG = 0x0000003F;
pub const TRANSACTION_DO_NOT_PROMOTE: DWORD = 0x00000001;
pub const TRANSACTION_MAXIMUM_OPTION: DWORD = 0x00000001;
pub const RESOURCE_MANAGER_VOLATILE: DWORD = 0x00000001;
pub const RESOURCE_MANAGER_COMMUNICATION: DWORD = 0x00000002;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: DWORD = 0x00000003;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: DWORD = 0x00000001;
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: DWORD = 0x00000002;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: DWORD = 0x00000003;
pub const ENLISTMENT_SUPERIOR: ULONG = 0x00000001;
pub const ENLISTMENT_MAXIMUM_OPTION: ULONG = 0x00000001;
pub type NOTIFICATION_MASK = ULONG;
pub const TRANSACTION_NOTIFY_MASK: ULONG = 0x3FFFFFFF;
pub const TRANSACTION_NOTIFY_PREPREPARE: ULONG = 0x00000001;
pub const TRANSACTION_NOTIFY_PREPARE: ULONG = 0x00000002;
pub const TRANSACTION_NOTIFY_COMMIT: ULONG = 0x00000004;
pub const TRANSACTION_NOTIFY_ROLLBACK: ULONG = 0x00000008;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: ULONG = 0x00000010;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: ULONG = 0x00000020;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: ULONG = 0x00000040;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: ULONG = 0x00000080;
pub const TRANSACTION_NOTIFY_RECOVER: ULONG = 0x00000100;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: ULONG = 0x00000200;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: ULONG = 0x00000400;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: ULONG = 0x00000800;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: ULONG = 0x00001000;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: ULONG = 0x00002000;
pub const TRANSACTION_NOTIFY_INDOUBT: ULONG = 0x00004000;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: ULONG = 0x00008000;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: ULONG = 0x00010000;
pub const TRANSACTION_NOTIFY_MARSHAL: ULONG = 0x00020000;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: ULONG = 0x00040000;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: ULONG = 0x01000000;
pub const TRANSACTION_NOTIFY_TM_ONLINE: ULONG = 0x02000000;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: ULONG = 0x04000000;
pub const TRANSACTION_NOTIFY_PROMOTE: ULONG = 0x08000000;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: ULONG = 0x10000000;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: ULONG = 0x20000000;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: ULONG = 0x40000000;
pub const TRANSACTIONMANAGER_OBJECT_PATH: &'static str = "\\TransactionManager\\";
pub const TRANSACTION_OBJECT_PATH: &'static str = "\\Transaction\\";
pub const ENLISTMENT_OBJECT_PATH: &'static str = "\\Enlistment\\";
pub const RESOURCE_MANAGER_OBJECT_PATH: &'static str = "\\ResourceManager\\";
STRUCT!{struct TRANSACTION_NOTIFICATION {
    TransactionKey: PVOID,
    TransactionNotification: ULONG,
    TmVirtualClock: LARGE_INTEGER,
    ArgumentLength: ULONG,
}}
pub type PTRANSACTION_NOTIFICATION = *mut TRANSACTION_NOTIFICATION;
STRUCT!{struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    EnlistmentId: GUID,
    UOW: UOW,
}}
pub type PTRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT
    = *mut TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT;
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: ULONG = 0x1;
STRUCT!{struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    TmIdentity: GUID,
    Flags: ULONG,
}}
pub type PTRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT
    = *mut TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT;
pub type SAVEPOINT_ID = ULONG;
pub type PSAVEPOINT_ID = *mut ULONG;
STRUCT!{struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    SavepointId: SAVEPOINT_ID,
}}
pub type PTRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT
    = *mut TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT;
STRUCT!{struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    PropagationCookie: ULONG,
    UOW: GUID,
    TmIdentity: GUID,
    BufferLength: ULONG,
}}
pub type PTRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT
    = *mut TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
STRUCT!{struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    MarshalCookie: ULONG,
    UOW: GUID,
}}
pub type PTRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT
    = *mut TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT;
pub type TRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT = TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
pub type PTRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT
    = *mut TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: ULONG = 1;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: ULONG = 1;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: usize = 64;
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: usize = 64;
STRUCT!{struct KCRM_MARSHAL_HEADER {
    VersionMajor: ULONG,
    VersionMinor: ULONG,
    NumProtocols: ULONG,
    Unused: ULONG,
}}
pub type PKCRM_MARSHAL_HEADER = *mut KCRM_MARSHAL_HEADER;
pub type PRKCRM_MARSHAL_HEADER = *mut KCRM_MARSHAL_HEADER;
STRUCT!{struct KCRM_TRANSACTION_BLOB {
    UOW: UOW,
    TmIdentity: GUID,
    IsolationLevel: ULONG,
    IsolationFlags: ULONG,
    Timeout: ULONG,
    Description: [WCHAR; MAX_TRANSACTION_DESCRIPTION_LENGTH],
}}
pub type PKCRM_TRANSACTION_BLOB = *mut KCRM_TRANSACTION_BLOB;
pub type PRKCRM_TRANSACTION_BLOB = *mut KCRM_TRANSACTION_BLOB;
STRUCT!{struct KCRM_PROTOCOL_BLOB {
    ProtocolId: CRM_PROTOCOL_ID,
    StaticInfoLength: ULONG,
    TransactionIdInfoLength: ULONG,
    Unused1: ULONG,
    Unused2: ULONG,
}}
pub type PKCRM_PROTOCOL_BLOB = *mut KCRM_PROTOCOL_BLOB;
pub type PRKCRM_PROTOCOL_BLOB = *mut KCRM_PROTOCOL_BLOB;
