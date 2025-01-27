#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

// ########
// HUB ###
// ######

/// Central instance to access all FirebaseRemoteConfig related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebaseremoteconfig1 as firebaseremoteconfig1;
/// use firebaseremoteconfig1::api::RemoteConfig;
/// use firebaseremoteconfig1::{Result, Error};
/// # async fn dox() {
/// use firebaseremoteconfig1::{FirebaseRemoteConfig, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = FirebaseRemoteConfig::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RemoteConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().update_remote_config(req, "project")
///              .validate_only(true)
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct FirebaseRemoteConfig<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for FirebaseRemoteConfig<C> {}

impl<'a, C> FirebaseRemoteConfig<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> FirebaseRemoteConfig<C> {
        FirebaseRemoteConfig {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://firebaseremoteconfig.googleapis.com/".to_string(),
            _root_url: "https://firebaseremoteconfig.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C> {
        ProjectMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://firebaseremoteconfig.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://firebaseremoteconfig.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// While default_value and conditional_values are each optional, at least one of
/// the two is required - otherwise, the parameter is meaningless (and an
/// exception will be thrown by the validation logic).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RemoteConfigParameter {
    /// Optional - value to set the parameter to, when none of the named conditions
    /// evaluate to <code>true</code>.
    #[serde(rename = "defaultValue")]
    pub default_value: Option<RemoteConfigParameterValue>,
    /// Optional.
    /// A description for this Parameter. Length must be less than or equal to
    /// 100 characters (or more precisely, unicode code points, which is defined in
    /// java/com/google/wireless/android/config/ConstsExporter.java).
    /// A description may contain any Unicode characters
    pub description: Option<String>,
    /// Optional - a map of (condition_name, value). The condition_name of the
    /// highest priority (the one listed first in the conditions array) determines
    /// the value of this parameter.
    #[serde(rename = "conditionalValues")]
    pub conditional_values: Option<HashMap<String, RemoteConfigParameterValue>>,
}

impl common::Part for RemoteConfigParameter {}

/// A single RemoteConfig Condition.  A list of these (because order matters) are
/// part of a single RemoteConfig template.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RemoteConfigCondition {
    /// Optional.
    /// The display (tag) color of this condition. This serves as part of a tag
    /// (in the future, we may add tag text as well as tag color, but that is not
    /// yet implemented in the UI).
    /// This value has no affect on the semantics of the delivered config and it
    /// is ignored by the backend, except for passing it through write/read
    /// requests.
    /// Not having this value or having the "CONDITION_DISPLAY_COLOR_UNSPECIFIED"
    /// value (0) have the same meaning:  Let the UI choose any valid color when
    /// displaying the condition.
    #[serde(rename = "tagColor")]
    pub tag_color: Option<String>,
    /// Required.
    /// A non empty and unique name of this condition.
    pub name: Option<String>,
    /// Optional.
    /// A description for this Condition. Length must be less than or equal to
    /// 100 characters (or more precisely, unicode code points, which is defined in
    /// java/com/google/wireless/android/config/ConstsExporter.java).
    /// A description may contain any Unicode characters
    pub description: Option<String>,
    /// Required.
    pub expression: Option<String>,
}

impl common::Part for RemoteConfigCondition {}

/// A RemoteConfigParameter's "value" (either the default value, or the value
/// associated with a condition name) is either a string, or the
/// "use_in_app_default" indicator (which means to leave out the parameter from
/// the returned <key, value> map that is the output of the parameter fetch).
/// We represent the "use_in_app_default" as a bool, but (when using the boolean
/// instead of the string) it should always be <code>true</code>.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RemoteConfigParameterValue {
    /// the string to set the parameter to
    pub value: Option<String>,
    /// if true, omit the parameter from the map of fetched parameter values
    #[serde(rename = "useInAppDefault")]
    pub use_in_app_default: Option<bool>,
}

impl common::Part for RemoteConfigParameterValue {}

/// *
///
/// The RemoteConfig consists of a list of conditions (which can be
/// thought of as named “if” statements) and a map of parameters (parameter key
/// to a structure containing an optional default value, as well as a optional
/// submap of (condition name to value when that condition is true).
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get remote config projects](ProjectGetRemoteConfigCall) (response)
/// * [update remote config projects](ProjectUpdateRemoteConfigCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RemoteConfig {
    /// Map of parameter keys to their optional default values and optional submap
    /// of (condition name : value). Order doesn't affect semantics, and so is
    /// sorted by the server. The 'key' values of the params must be unique.
    pub parameters: Option<HashMap<String, RemoteConfigParameter>>,
    /// The list of named conditions. The order *does* affect the semantics.
    /// The condition_name values of these entries must be unique.
    ///
    /// The resolved value of a config parameter P is determined as follow:
    /// * Let Y be the set of values from the submap of P that refer to conditions
    ///   that evaluate to <code>true</code>.
    /// * If Y is non empty, the value is taken from the specific submap in Y whose
    ///   condition_name is the earliest in this condition list.
    /// * Else, if P has a default value option (condition_name is empty) then
    ///   the value is taken from that option.
    /// * Else, parameter P has no value and is omitted from the config result.
    ///
    /// Example: parameter key "p1", default value "v1", submap specified as
    /// {"c1": v2, "c2": v3} where "c1" and "c2" are names of conditions in the
    /// condition list (where "c1" in this example appears before "c2").  The
    /// value of p1 would be v2 as long as c1 is true.  Otherwise, if c2 is true,
    /// p1 would evaluate to v3, and if c1 and c2 are both false, p1 would evaluate
    /// to v1.  If no default value was specified, and c1 and c2 were both false,
    /// no value for p1 would be generated.
    pub conditions: Option<Vec<RemoteConfigCondition>>,
}

impl common::RequestValue for RemoteConfig {}
impl common::ResponseResult for RemoteConfig {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`FirebaseRemoteConfig`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebaseremoteconfig1 as firebaseremoteconfig1;
///
/// # async fn dox() {
/// use firebaseremoteconfig1::{FirebaseRemoteConfig, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = FirebaseRemoteConfig::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_remote_config(...)` and `update_remote_config(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C>
where
    C: 'a,
{
    hub: &'a FirebaseRemoteConfig<C>,
}

impl<'a, C> common::MethodsBuilder for ProjectMethods<'a, C> {}

impl<'a, C> ProjectMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Get the latest version Remote Configuration for a project.
    /// Returns the RemoteConfig as the payload, and also the eTag as a
    /// response header.
    ///
    /// # Arguments
    ///
    /// * `project` - The GMP project identifier. Required.
    ///               See note at the beginning of this file regarding project ids.
    pub fn get_remote_config(&self, project: &str) -> ProjectGetRemoteConfigCall<'a, C> {
        ProjectGetRemoteConfigCall {
            hub: self.hub,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Update a RemoteConfig. We treat this as an always-existing
    /// resource (when it is not found in our data store, we treat it as version
    /// 0, a template with zero conditions and zero parameters). Hence there are
    /// no Create or Delete operations. Returns the updated template when
    /// successful (and the updated eTag as a response header), or an error if
    /// things go wrong.
    /// Possible error messages:
    /// * VALIDATION_ERROR (HTTP status 400) with additional details if the
    /// template being passed in can not be validated.
    /// * AUTHENTICATION_ERROR (HTTP status 401) if the request can not be
    /// authenticate (e.g. no access token, or invalid access token).
    /// * AUTHORIZATION_ERROR (HTTP status 403) if the request can not be
    /// authorized (e.g. the user has no access to the specified project id).
    /// * VERSION_MISMATCH (HTTP status 412) when trying to update when the
    /// expected eTag (passed in via the "If-match" header) is not specified, or
    /// is specified but does does not match the current eTag.
    /// * Internal error (HTTP status 500) for Database problems or other internal
    /// errors.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The GMP project identifier. Required.
    ///               See note at the beginning of this file regarding project ids.
    pub fn update_remote_config(
        &self,
        request: RemoteConfig,
        project: &str,
    ) -> ProjectUpdateRemoteConfigCall<'a, C> {
        ProjectUpdateRemoteConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Get the latest version Remote Configuration for a project.
/// Returns the RemoteConfig as the payload, and also the eTag as a
/// response header.
///
/// A builder for the *getRemoteConfig* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_firebaseremoteconfig1 as firebaseremoteconfig1;
/// # async fn dox() {
/// # use firebaseremoteconfig1::{FirebaseRemoteConfig, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = FirebaseRemoteConfig::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_remote_config("project")
///              .doit().await;
/// # }
/// ```
pub struct ProjectGetRemoteConfigCall<'a, C>
where
    C: 'a,
{
    hub: &'a FirebaseRemoteConfig<C>,
    _project: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ProjectGetRemoteConfigCall<'a, C> {}

impl<'a, C> ProjectGetRemoteConfigCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, RemoteConfig)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "firebaseremoteconfig.projects.getRemoteConfig",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "project"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("project", self._project);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+project}/remoteConfig";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+project}", "project")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The GMP project identifier. Required.
    /// See note at the beginning of this file regarding project ids.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> ProjectGetRemoteConfigCall<'a, C> {
        self._project = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectGetRemoteConfigCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetRemoteConfigCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Update a RemoteConfig. We treat this as an always-existing
/// resource (when it is not found in our data store, we treat it as version
/// 0, a template with zero conditions and zero parameters). Hence there are
/// no Create or Delete operations. Returns the updated template when
/// successful (and the updated eTag as a response header), or an error if
/// things go wrong.
/// Possible error messages:
/// * VALIDATION_ERROR (HTTP status 400) with additional details if the
/// template being passed in can not be validated.
/// * AUTHENTICATION_ERROR (HTTP status 401) if the request can not be
/// authenticate (e.g. no access token, or invalid access token).
/// * AUTHORIZATION_ERROR (HTTP status 403) if the request can not be
/// authorized (e.g. the user has no access to the specified project id).
/// * VERSION_MISMATCH (HTTP status 412) when trying to update when the
/// expected eTag (passed in via the "If-match" header) is not specified, or
/// is specified but does does not match the current eTag.
/// * Internal error (HTTP status 500) for Database problems or other internal
/// errors.
///
/// A builder for the *updateRemoteConfig* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_firebaseremoteconfig1 as firebaseremoteconfig1;
/// use firebaseremoteconfig1::api::RemoteConfig;
/// # async fn dox() {
/// # use firebaseremoteconfig1::{FirebaseRemoteConfig, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = FirebaseRemoteConfig::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RemoteConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().update_remote_config(req, "project")
///              .validate_only(true)
///              .doit().await;
/// # }
/// ```
pub struct ProjectUpdateRemoteConfigCall<'a, C>
where
    C: 'a,
{
    hub: &'a FirebaseRemoteConfig<C>,
    _request: RemoteConfig,
    _project: String,
    _validate_only: Option<bool>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ProjectUpdateRemoteConfigCall<'a, C> {}

impl<'a, C> ProjectUpdateRemoteConfigCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, RemoteConfig)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "firebaseremoteconfig.projects.updateRemoteConfig",
            http_method: hyper::Method::PUT,
        });

        for &field in ["alt", "project", "validateOnly"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("project", self._project);
        if let Some(value) = self._validate_only.as_ref() {
            params.push("validateOnly", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+project}/remoteConfig";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+project}", "project")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: RemoteConfig) -> ProjectUpdateRemoteConfigCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The GMP project identifier. Required.
    /// See note at the beginning of this file regarding project ids.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> ProjectUpdateRemoteConfigCall<'a, C> {
        self._project = new_value.to_string();
        self
    }
    /// Optional. Defaults to <code>false</code> (UpdateRemoteConfig call should
    /// update the backend if there are no validation/interal errors). May be set
    /// to <code>true</code> to indicate that, should no validation errors occur,
    /// the call should return a "200 OK" instead of performing the update. Note
    /// that other error messages (500 Internal Error, 412 Version Mismatch, etc)
    /// may still result after flipping to <code>false</code>, even if getting a
    /// "200 OK" when calling with <code>true</code>.
    ///
    /// Sets the *validate only* query property to the given value.
    pub fn validate_only(mut self, new_value: bool) -> ProjectUpdateRemoteConfigCall<'a, C> {
        self._validate_only = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectUpdateRemoteConfigCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectUpdateRemoteConfigCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}
