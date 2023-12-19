use modules::utility::env_utils::get_env_var;

use crate::constants;

#[derive(Debug, Clone)]
pub struct ConfigOpenApi {
    pub enabled: bool,
    pub url_path: String,
    pub swagger: ConfigOpenApiUi,
    pub scalar: ConfigOpenApiUi,
    pub rapi_doc: ConfigOpenApiUi,
    pub re_doc: ConfigOpenApiUi,
}

#[derive(Debug, Clone)]
pub struct ConfigOpenApiUi {
    pub enabled: bool,
    pub url_path: String,
}

impl Default for ConfigOpenApi {
    fn default() -> Self {
        ConfigOpenApi::new()
    }
}

impl ConfigOpenApi {
    pub fn new() -> Self {
        let default_oapi: Option<&str> = Some(constants::CYB_OAPI);
        let default_oapi_path: Option<&str> = Some(constants::CYB_OAPI_PATH);
        let default_oapi_swagger: Option<&str> = Some(constants::CYB_OAPI_SWAGGER);
        let default_oapi_swagger_path: Option<&str> = Some(constants::CYB_OAPI_SWAGGER_PATH);
        let default_oapi_scalar: Option<&str> = Some(constants::CYB_OAPI_SCALAR);
        let default_oapi_scalar_path: Option<&str> = Some(constants::CYB_OAPI_SCALAR_PATH);
        let default_oapi_rapidoc: Option<&str> = Some(constants::CYB_OAPI_RAPIDOC);
        let default_oapi_rapidoc_path: Option<&str> = Some(constants::CYB_OAPI_RAPIDOC_PATH);
        let default_oapi_redoc: Option<&str> = Some(constants::CYB_OAPI_REDOC);
        let default_oapi_redoc_path: Option<&str> = Some(constants::CYB_OAPI_REDOC_PATH);

        let enabled: bool = get_env_var("CYB_OAPI", default_oapi) == "true";
        let url_path: String = get_env_var("CYB_OAPI_PATH", default_oapi_path);

        let swagger: ConfigOpenApiUi = ConfigOpenApiUi {
            enabled: get_env_var("CYB_OAPI_SWAGGER", default_oapi_swagger) == "true",
            url_path: get_env_var("CYB_OAPI_SWAGGER_PATH", default_oapi_swagger_path),
        };
        let scalar: ConfigOpenApiUi = ConfigOpenApiUi {
            enabled: get_env_var("CYB_OAPI_SCALAR", default_oapi_scalar) == "true",
            url_path: get_env_var("CYB_OAPI_SCALAR_PATH", default_oapi_scalar_path),
        };
        let rapi_doc: ConfigOpenApiUi = ConfigOpenApiUi {
            enabled: get_env_var("CYB_OAPI_RAPIDOC", default_oapi_rapidoc) == "true",
            url_path: get_env_var("CYB_OAPI_RAPIDOC_PATH", default_oapi_rapidoc_path),
        };
        let re_doc: ConfigOpenApiUi = ConfigOpenApiUi {
            enabled: get_env_var("CYB_OAPI_REDOC", default_oapi_redoc) == "true",
            url_path: get_env_var("CYB_OAPI_REDOC_PATH", default_oapi_redoc_path),
        };

        ConfigOpenApi {
            enabled,
            url_path,
            swagger,
            scalar,
            rapi_doc,
            re_doc,
        }
    }
}
