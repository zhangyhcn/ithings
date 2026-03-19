use anyhow::Result;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions;
use k8s_openapi::api::apps::v1 as apps;
use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams},
    error::ErrorResponse,
    core::DynamicObject,
    discovery::ApiResource,
    Client, Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct K8sClient {
    client: Arc<Client>,
}

impl K8sClient {
    pub async fn new() -> Result<Self> {
        let client = Client::try_default().await?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn from_config(config: Config) -> Result<Self> {
        let client = Client::try_from(config)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn apply_crd(&self, crd: apiextensions::CustomResourceDefinition) -> Result<apiextensions::CustomResourceDefinition> {
        let api: Api<apiextensions::CustomResourceDefinition> =
            Api::all(self.client.as_ref().clone());

        let pp = PostParams::default();

        match api.create(&pp, &crd).await {
            Ok(crd) => Ok(crd),
            Err(kube::Error::Api(ErrorResponse { code: 409, .. })) => {
                let name = crd.metadata.name.clone().unwrap();
                let patch = Patch::Apply(&crd);
                let pp = PatchParams::apply("icloud");
                let crd = api.patch(&name, &pp, &patch).await?;
                Ok(crd)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn delete_crd(&self, name: &str) -> Result<()> {
        let api: Api<apiextensions::CustomResourceDefinition> =
            Api::all(self.client.as_ref().clone());

        let dp = DeleteParams::default();
        api.delete(name, &dp).await?;
        Ok(())
    }

    pub async fn get_crd(&self, name: &str) -> Result<Option<apiextensions::CustomResourceDefinition>> {
        let api: Api<apiextensions::CustomResourceDefinition> =
            Api::all(self.client.as_ref().clone());

        match api.get(name).await {
            Ok(crd) => Ok(Some(crd)),
            Err(kube::Error::Api(ErrorResponse { code: 404, .. })) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list_crds(&self) -> Result<Vec<apiextensions::CustomResourceDefinition>> {
        let api: Api<apiextensions::CustomResourceDefinition> =
            Api::all(self.client.as_ref().clone());

        let lp = ListParams::default();
        let list = api.list(&lp).await?;
        Ok(list.items)
    }

    pub async fn apply_dynamic_resource(
        &self,
        api_resource: &ApiResource,
        namespace: Option<&str>,
        resource: DynamicObject,
    ) -> Result<DynamicObject> {
        let name = resource.metadata.name.clone()
            .ok_or_else(|| anyhow::anyhow!("Resource must have metadata.name"))?;

        let api: Api<DynamicObject> = match namespace {
            Some(ns) => Api::namespaced_with(self.client.as_ref().clone(), ns, api_resource),
            None => Api::all_with(self.client.as_ref().clone(), api_resource),
        };

        let pp = PostParams::default();

        match api.create(&pp, &resource).await {
            Ok(obj) => Ok(obj),
            Err(kube::Error::Api(ErrorResponse { code: 409, .. })) => {
                let patch = Patch::Apply(&resource);
                let pp = PatchParams::apply("icloud");
                let obj = api.patch(&name, &pp, &patch).await?;
                Ok(obj)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn delete_dynamic_resource(
        &self,
        api_resource: &ApiResource,
        namespace: Option<&str>,
        name: &str,
    ) -> Result<()> {
        let api: Api<DynamicObject> = match namespace {
            Some(ns) => Api::namespaced_with(self.client.as_ref().clone(), ns, api_resource),
            None => Api::all_with(self.client.as_ref().clone(), api_resource),
        };

        let dp = DeleteParams::default();
        api.delete(name, &dp).await?;
        Ok(())
    }

    pub async fn get_dynamic_resource(
        &self,
        api_resource: &ApiResource,
        namespace: Option<&str>,
        name: &str,
    ) -> Result<Option<DynamicObject>> {
        let api: Api<DynamicObject> = match namespace {
            Some(ns) => Api::namespaced_with(self.client.as_ref().clone(), ns, api_resource),
            None => Api::all_with(self.client.as_ref().clone(), api_resource),
        };

        match api.get(name).await {
            Ok(obj) => Ok(Some(obj)),
            Err(kube::Error::Api(ErrorResponse { code: 404, .. })) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list_dynamic_resources(
        &self,
        api_resource: &ApiResource,
        namespace: Option<&str>,
    ) -> Result<Vec<DynamicObject>> {
        let api: Api<DynamicObject> = match namespace {
            Some(ns) => Api::namespaced_with(self.client.as_ref().clone(), ns, api_resource),
            None => Api::all_with(self.client.as_ref().clone(), api_resource),
        };

        let lp = ListParams::default();
        let list = api.list(&lp).await?;
        Ok(list.items)
    }

    pub async fn apply_deployment(&self, namespace: &str, deployment: apps::Deployment) -> Result<apps::Deployment> {
        let api: Api<apps::Deployment> = Api::namespaced(self.client.as_ref().clone(), namespace);

        let pp = PostParams::default();

        match api.create(&pp, &deployment).await {
            Ok(deploy) => Ok(deploy),
            Err(kube::Error::Api(ErrorResponse { code: 409, .. })) => {
                let name = deployment.metadata.name.clone().unwrap();
                let patch = Patch::Apply(&deployment);
                let pp = PatchParams::apply("icloud");
                let deploy = api.patch(&name, &pp, &patch).await?;
                Ok(deploy)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn delete_deployment(&self, namespace: &str, name: &str) -> Result<()> {
        let api: Api<apps::Deployment> = Api::namespaced(self.client.as_ref().clone(), namespace);

        let dp = DeleteParams::default();
        api.delete(name, &dp).await?;
        Ok(())
    }

    pub async fn get_deployment(&self, namespace: &str, name: &str) -> Result<Option<apps::Deployment>> {
        let api: Api<apps::Deployment> = Api::namespaced(self.client.as_ref().clone(), namespace);

        match api.get(name).await {
            Ok(deploy) => Ok(Some(deploy)),
            Err(kube::Error::Api(ErrorResponse { code: 404, .. })) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K8sObjectMetadata {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub annotations: Option<std::collections::BTreeMap<String, String>>,
    pub labels: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K8sCRDApplyRequest {
    pub group: String,
    pub version: String,
    pub kind: String,
    pub plural: String,
    pub scope: String,
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K8sResourceApplyRequest {
    pub api_version: String,
    pub kind: String,
    pub metadata: K8sObjectMetadata,
    pub spec: serde_json::Value,
}
