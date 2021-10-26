#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-metadata-2021-07-01-preview")]
pub mod package_metadata_2021_07_01_preview;
#[cfg(all(feature = "package-metadata-2021-07-01-preview", not(feature = "no-default-version")))]
pub use package_metadata_2021_07_01_preview::{models, operations};
#[cfg(feature = "package-artifacts-composite-v1")]
pub mod package_artifacts_composite_v1;
#[cfg(all(feature = "package-artifacts-composite-v1", not(feature = "no-default-version")))]
pub use package_artifacts_composite_v1::{models, operations};
#[cfg(feature = "package-artifacts-2021-06-01-preview")]
pub mod package_artifacts_2021_06_01_preview;
#[cfg(all(feature = "package-artifacts-2021-06-01-preview", not(feature = "no-default-version")))]
pub use package_artifacts_2021_06_01_preview::{models, operations};
#[cfg(feature = "package-vnet-2021-06-01-preview")]
pub mod package_vnet_2021_06_01_preview;
#[cfg(all(feature = "package-vnet-2021-06-01-preview", not(feature = "no-default-version")))]
pub use package_vnet_2021_06_01_preview::{models, operations};
#[cfg(feature = "package-kql-script-2021-06-preview")]
pub mod package_kql_script_2021_06_preview;
#[cfg(all(feature = "package-kql-script-2021-06-preview", not(feature = "no-default-version")))]
pub use package_kql_script_2021_06_preview::{models, operations};
#[cfg(feature = "package-artifacts-2020-12-01")]
pub mod package_artifacts_2020_12_01;
#[cfg(all(feature = "package-artifacts-2020-12-01", not(feature = "no-default-version")))]
pub use package_artifacts_2020_12_01::{models, operations};
#[cfg(feature = "package-monitoring-2020-12-01")]
pub mod package_monitoring_2020_12_01;
#[cfg(all(feature = "package-monitoring-2020-12-01", not(feature = "no-default-version")))]
pub use package_monitoring_2020_12_01::{models, operations};
#[cfg(feature = "package-access-control-2020-12-01")]
pub mod package_access_control_2020_12_01;
#[cfg(all(feature = "package-access-control-2020-12-01", not(feature = "no-default-version")))]
pub use package_access_control_2020_12_01::{models, operations};
#[cfg(feature = "package-vnet-2020-12-01")]
pub mod package_vnet_2020_12_01;
#[cfg(all(feature = "package-vnet-2020-12-01", not(feature = "no-default-version")))]
pub use package_vnet_2020_12_01::{models, operations};
#[cfg(feature = "package-spark-2020-12-01")]
pub mod package_spark_2020_12_01;
#[cfg(all(feature = "package-spark-2020-12-01", not(feature = "no-default-version")))]
pub use package_spark_2020_12_01::{models, operations};
#[cfg(feature = "package-spark-2019-11-01-preview")]
pub mod package_spark_2019_11_01_preview;
#[cfg(all(feature = "package-spark-2019-11-01-preview", not(feature = "no-default-version")))]
pub use package_spark_2019_11_01_preview::{models, operations};
#[cfg(feature = "package-artifacts-2019-06-01-preview")]
pub mod package_artifacts_2019_06_01_preview;
#[cfg(all(feature = "package-artifacts-2019-06-01-preview", not(feature = "no-default-version")))]
pub use package_artifacts_2019_06_01_preview::{models, operations};
#[cfg(feature = "package-access-control-2020-02-01-preview")]
pub mod package_access_control_2020_02_01_preview;
#[cfg(all(feature = "package-access-control-2020-02-01-preview", not(feature = "no-default-version")))]
pub use package_access_control_2020_02_01_preview::{models, operations};
#[cfg(feature = "package-access-control-2020-08-01-preview")]
pub mod package_access_control_2020_08_01_preview;
#[cfg(all(feature = "package-access-control-2020-08-01-preview", not(feature = "no-default-version")))]
pub use package_access_control_2020_08_01_preview::{models, operations};
#[cfg(feature = "package-vnet-2019-06-01-preview")]
pub mod package_vnet_2019_06_01_preview;
#[cfg(all(feature = "package-vnet-2019-06-01-preview", not(feature = "no-default-version")))]
pub use package_vnet_2019_06_01_preview::{models, operations};
#[cfg(feature = "package-monitoring-2019-11-01-preview")]
pub mod package_monitoring_2019_11_01_preview;
use azure_core::setters;
#[cfg(all(feature = "package-monitoring-2019-11-01-preview", not(feature = "no-default-version")))]
pub use package_monitoring_2019_11_01_preview::{models, operations};
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or_else(|| "https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self
                .token_credential_resource
                .unwrap_or_else(|| "https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}
