// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Spanner* crate version *5.0.5+20240618*, where *20240618* is the exact revision of the *spanner:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.5*.
//! 
//! Everything else about the *Spanner* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/spanner/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/spanner1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Spanner) ... 
//! 
//! * projects
//!  * [*instance config operations list*](api::ProjectInstanceConfigOperationListCall), [*instance configs create*](api::ProjectInstanceConfigCreateCall), [*instance configs delete*](api::ProjectInstanceConfigDeleteCall), [*instance configs get*](api::ProjectInstanceConfigGetCall), [*instance configs list*](api::ProjectInstanceConfigListCall), [*instance configs operations cancel*](api::ProjectInstanceConfigOperationCancelCall), [*instance configs operations delete*](api::ProjectInstanceConfigOperationDeleteCall), [*instance configs operations get*](api::ProjectInstanceConfigOperationGetCall), [*instance configs operations list*](api::ProjectInstanceConfigOperationListCall1), [*instance configs patch*](api::ProjectInstanceConfigPatchCall), [*instance configs ssd caches operations cancel*](api::ProjectInstanceConfigSsdCachOperationCancelCall), [*instance configs ssd caches operations delete*](api::ProjectInstanceConfigSsdCachOperationDeleteCall), [*instance configs ssd caches operations get*](api::ProjectInstanceConfigSsdCachOperationGetCall), [*instance configs ssd caches operations list*](api::ProjectInstanceConfigSsdCachOperationListCall), [*instances backup operations list*](api::ProjectInstanceBackupOperationListCall), [*instances backups copy*](api::ProjectInstanceBackupCopyCall), [*instances backups create*](api::ProjectInstanceBackupCreateCall), [*instances backups delete*](api::ProjectInstanceBackupDeleteCall), [*instances backups get*](api::ProjectInstanceBackupGetCall), [*instances backups get iam policy*](api::ProjectInstanceBackupGetIamPolicyCall), [*instances backups list*](api::ProjectInstanceBackupListCall), [*instances backups operations cancel*](api::ProjectInstanceBackupOperationCancelCall), [*instances backups operations delete*](api::ProjectInstanceBackupOperationDeleteCall), [*instances backups operations get*](api::ProjectInstanceBackupOperationGetCall), [*instances backups operations list*](api::ProjectInstanceBackupOperationListCall1), [*instances backups patch*](api::ProjectInstanceBackupPatchCall), [*instances backups set iam policy*](api::ProjectInstanceBackupSetIamPolicyCall), [*instances backups test iam permissions*](api::ProjectInstanceBackupTestIamPermissionCall), [*instances create*](api::ProjectInstanceCreateCall), [*instances database operations list*](api::ProjectInstanceDatabaseOperationListCall), [*instances databases changequorum*](api::ProjectInstanceDatabaseChangequorumCall), [*instances databases create*](api::ProjectInstanceDatabaseCreateCall), [*instances databases database roles list*](api::ProjectInstanceDatabaseDatabaseRoleListCall), [*instances databases database roles test iam permissions*](api::ProjectInstanceDatabaseDatabaseRoleTestIamPermissionCall), [*instances databases drop database*](api::ProjectInstanceDatabaseDropDatabaseCall), [*instances databases get*](api::ProjectInstanceDatabaseGetCall), [*instances databases get ddl*](api::ProjectInstanceDatabaseGetDdlCall), [*instances databases get iam policy*](api::ProjectInstanceDatabaseGetIamPolicyCall), [*instances databases get scans*](api::ProjectInstanceDatabaseGetScanCall), [*instances databases list*](api::ProjectInstanceDatabaseListCall), [*instances databases operations cancel*](api::ProjectInstanceDatabaseOperationCancelCall), [*instances databases operations delete*](api::ProjectInstanceDatabaseOperationDeleteCall), [*instances databases operations get*](api::ProjectInstanceDatabaseOperationGetCall), [*instances databases operations list*](api::ProjectInstanceDatabaseOperationListCall1), [*instances databases patch*](api::ProjectInstanceDatabasePatchCall), [*instances databases restore*](api::ProjectInstanceDatabaseRestoreCall), [*instances databases sessions batch create*](api::ProjectInstanceDatabaseSessionBatchCreateCall), [*instances databases sessions batch write*](api::ProjectInstanceDatabaseSessionBatchWriteCall), [*instances databases sessions begin transaction*](api::ProjectInstanceDatabaseSessionBeginTransactionCall), [*instances databases sessions commit*](api::ProjectInstanceDatabaseSessionCommitCall), [*instances databases sessions create*](api::ProjectInstanceDatabaseSessionCreateCall), [*instances databases sessions delete*](api::ProjectInstanceDatabaseSessionDeleteCall), [*instances databases sessions execute batch dml*](api::ProjectInstanceDatabaseSessionExecuteBatchDmlCall), [*instances databases sessions execute sql*](api::ProjectInstanceDatabaseSessionExecuteSqlCall), [*instances databases sessions execute streaming sql*](api::ProjectInstanceDatabaseSessionExecuteStreamingSqlCall), [*instances databases sessions get*](api::ProjectInstanceDatabaseSessionGetCall), [*instances databases sessions list*](api::ProjectInstanceDatabaseSessionListCall), [*instances databases sessions partition query*](api::ProjectInstanceDatabaseSessionPartitionQueryCall), [*instances databases sessions partition read*](api::ProjectInstanceDatabaseSessionPartitionReadCall), [*instances databases sessions read*](api::ProjectInstanceDatabaseSessionReadCall), [*instances databases sessions rollback*](api::ProjectInstanceDatabaseSessionRollbackCall), [*instances databases sessions streaming read*](api::ProjectInstanceDatabaseSessionStreamingReadCall), [*instances databases set iam policy*](api::ProjectInstanceDatabaseSetIamPolicyCall), [*instances databases test iam permissions*](api::ProjectInstanceDatabaseTestIamPermissionCall), [*instances databases update ddl*](api::ProjectInstanceDatabaseUpdateDdlCall), [*instances delete*](api::ProjectInstanceDeleteCall), [*instances get*](api::ProjectInstanceGetCall), [*instances get iam policy*](api::ProjectInstanceGetIamPolicyCall), [*instances instance partition operations list*](api::ProjectInstanceInstancePartitionOperationListCall), [*instances instance partitions create*](api::ProjectInstanceInstancePartitionCreateCall), [*instances instance partitions delete*](api::ProjectInstanceInstancePartitionDeleteCall), [*instances instance partitions get*](api::ProjectInstanceInstancePartitionGetCall), [*instances instance partitions list*](api::ProjectInstanceInstancePartitionListCall), [*instances instance partitions operations cancel*](api::ProjectInstanceInstancePartitionOperationCancelCall), [*instances instance partitions operations delete*](api::ProjectInstanceInstancePartitionOperationDeleteCall), [*instances instance partitions operations get*](api::ProjectInstanceInstancePartitionOperationGetCall), [*instances instance partitions operations list*](api::ProjectInstanceInstancePartitionOperationListCall1), [*instances instance partitions patch*](api::ProjectInstanceInstancePartitionPatchCall), [*instances list*](api::ProjectInstanceListCall), [*instances move*](api::ProjectInstanceMoveCall), [*instances operations cancel*](api::ProjectInstanceOperationCancelCall), [*instances operations delete*](api::ProjectInstanceOperationDeleteCall), [*instances operations get*](api::ProjectInstanceOperationGetCall), [*instances operations list*](api::ProjectInstanceOperationListCall), [*instances patch*](api::ProjectInstancePatchCall), [*instances set iam policy*](api::ProjectInstanceSetIamPolicyCall) and [*instances test iam permissions*](api::ProjectInstanceTestIamPermissionCall)
//! * [scans](api::Scan)
//!  * [*list*](api::ScanListCall)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Spanner)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.projects().instance_configs_operations_get(...).doit().await
//! let r = hub.projects().instance_configs_ssd_caches_operations_get(...).doit().await
//! let r = hub.projects().instance_configs_create(...).doit().await
//! let r = hub.projects().instance_configs_patch(...).doit().await
//! let r = hub.projects().instances_backups_operations_get(...).doit().await
//! let r = hub.projects().instances_backups_copy(...).doit().await
//! let r = hub.projects().instances_backups_create(...).doit().await
//! let r = hub.projects().instances_databases_operations_get(...).doit().await
//! let r = hub.projects().instances_databases_changequorum(...).doit().await
//! let r = hub.projects().instances_databases_create(...).doit().await
//! let r = hub.projects().instances_databases_patch(...).doit().await
//! let r = hub.projects().instances_databases_restore(...).doit().await
//! let r = hub.projects().instances_databases_update_ddl(...).doit().await
//! let r = hub.projects().instances_instance_partitions_operations_get(...).doit().await
//! let r = hub.projects().instances_instance_partitions_create(...).doit().await
//! let r = hub.projects().instances_instance_partitions_patch(...).doit().await
//! let r = hub.projects().instances_operations_get(...).doit().await
//! let r = hub.projects().instances_create(...).doit().await
//! let r = hub.projects().instances_move(...).doit().await
//! let r = hub.projects().instances_patch(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-spanner1 = "*"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_spanner1 as spanner1;
//! use spanner1::api::Backup;
//! use spanner1::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use spanner1::{Spanner, oauth2, hyper, hyper_rustls, chrono, FieldMask};
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Spanner::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Backup::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().instances_backups_create(req, "parent")
//!              .add_encryption_config_kms_key_names("sanctus")
//!              .encryption_config_kms_key_name("sed")
//!              .encryption_config_encryption_type("amet.")
//!              .backup_id("takimata")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! ## Cargo Features
//! 
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//! 
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

// Re-export the hyper and hyper_rustls crate, they are required to build the hub
pub use hyper;
pub use hyper_rustls;
pub extern crate google_apis_common as client;
pub use client::chrono;
pub mod api;

// Re-export the hub type and some basic client structs
pub use api::Spanner;
pub use client::{Result, Error, Delegate, FieldMask};

// Re-export the yup_oauth2 crate, that is required to call some methods of the hub and the client
#[cfg(feature = "yup-oauth2")]
pub use client::oauth2;