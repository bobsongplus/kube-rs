//! Type information structs for API discovery
use crate::{gvk::GroupVersionKind, resource::Resource};

/// Information about a Kubernetes API resource
///
/// Enough information to use it like a `Resource` by passing it to the dynamic `Api`
/// constructors like `Api::all_with` and `Api::namespaced_with`.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ApiResource {
    /// Resource group, empty for core group.
    pub group: String,
    /// group version
    pub version: String,
    /// apiVersion of the resource (v1 for core group,
    /// groupName/groupVersions for other).
    pub api_version: String,
    /// Singular PascalCase name of the resource
    pub kind: String,
    /// Plural name of the resource
    pub plural: String,
}

impl ApiResource {
    /// Creates an ApiResource by type-erasing a Resource
    pub fn erase<K: Resource>(dt: &K::DynamicType) -> Self {
        ApiResource {
            group: K::group(dt).to_string(),
            version: K::version(dt).to_string(),
            api_version: K::api_version(dt).to_string(),
            kind: K::kind(dt).to_string(),
            plural: K::plural(dt).to_string(),
        }
    }

    /// Creates an ApiResource from group, version, kind and plural name.
    pub fn from_gvk_with_plural(gvk: &GroupVersionKind, plural: &str) -> Self {
        ApiResource {
            api_version: gvk.api_version(),
            group: gvk.group.clone(),
            version: gvk.version.clone(),
            kind: gvk.kind.clone(),
            plural: plural.to_string(),
        }
    }

    /// Creates an ApiResource from group, version and kind.
    ///
    /// # Warning
    /// This function will **guess** the resource plural name.
    /// Usually, this is ok, but for CRDs with complex pluralisations it can fail.
    /// If you are getting your values from `kube_derive` use the generated method for giving you an [`ApiResource`].
    /// Otherwise consider using [`ApiResource::from_gvk_with_plural`](crate::discovery::ApiResource::from_gvk_with_plural)
    /// to explicitly set the plural, or run api discovery on it via `kube::discovery`.
    pub fn from_gvk(gvk: &GroupVersionKind) -> Self {
        ApiResource::from_gvk_with_plural(gvk, &crate::resource::to_plural(&gvk.kind.to_ascii_lowercase()))
    }
}


/// Resource scope
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Scope {
    /// Objects are global
    Cluster,
    /// Each object lives in namespace.
    Namespaced,
}

/// Rbac verbs for ApiCapabilities
pub mod verbs {
    /// Create a resource
    pub const CREATE: &str = "create";
    /// Get single resource
    pub const GET: &str = "get";
    /// List objects
    pub const LIST: &str = "list";
    /// Watch for objects changes
    pub const WATCH: &str = "watch";
    /// Delete single object
    pub const DELETE: &str = "delete";
    /// Delete multiple objects at once
    pub const DELETE_COLLECTION: &str = "deletecollection";
    /// Update an object
    pub const UPDATE: &str = "update";
    /// Patch an object
    pub const PATCH: &str = "patch";
}

/// Contains the capabilities of an API resource
#[derive(Debug, Clone)]
pub struct ApiCapabilities {
    /// Scope of the resource
    pub scope: Scope,
    /// Available subresources.
    ///
    /// Please note that returned ApiResources are not standalone resources.
    /// Their name will be of form `subresource_name`, not `resource_name/subresource_name`.
    /// To work with subresources, use `Request` methods for now.
    pub subresources: Vec<(ApiResource, ApiCapabilities)>,
    /// Supported operations on this resource
    pub operations: Vec<String>,
}

impl ApiCapabilities {
    /// Checks that given verb is supported on this resource.
    pub fn supports_operation(&self, operation: &str) -> bool {
        self.operations.iter().any(|op| op == operation)
    }
}
