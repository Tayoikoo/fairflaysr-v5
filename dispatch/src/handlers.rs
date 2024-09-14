use crate::config::*;
use axum::extract::{Path, Query};
use prost::Message;
use proto::{Gateserver, GlobalDispatchData, ServerData};
use serde::Deserialize;

pub const QUERY_DISPATCH_PATH: &str = "/query_dispatch";
pub const QUERY_GATEWAY_PATH: &str = "/query_gateway/:region_name";

#[tracing::instrument]
pub async fn query_dispatch() -> String {
    let rsp = GlobalDispatchData {
        retcode: 0,
        server_list: CONFIGURATION
            .game_servers
            .iter()
            .map(|(_, c)| ServerData {
                name: c.name.clone(),
                title: c.title.clone(),
                env_type: c.env_type.clone(),
                dispatch_url: c.dispatch_url.clone(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };

    rbase64::encode(&rsp.encode_to_vec())
}

#[derive(Deserialize, Debug)]
pub struct QueryGatewayParameters {
    pub version: String,
}

#[tracing::instrument]
pub async fn query_gateway(
    Path(region_name): Path<String>,
    parameters: Query<QueryGatewayParameters>,
) -> String {
    let rsp = if let Some(server_config) = CONFIGURATION.game_servers.get(&region_name) {
        if let Some(version_config) = CONFIGURATION.versions.get(&parameters.version) {
            Gateserver {
                ip: server_config.gateserver_ip.clone(),
                port: server_config.gateserver_port as u32,
                asset_bundle_url: version_config.asset_bundle_url.clone(),
                ex_resource_url: version_config.ex_resource_url.clone(),
                lua_url: version_config.lua_url.clone(),
                lua_version: version_config.lua_version.clone(),
                //ifix_version: String::from("0"),
                nnglebkcmla: true,
                apjecjmgakc: true,
                nphdimjokni: true,
                cnaklgmdlpe: true,
                laablmnklld: true,
                ofjaknedmdm: true,
                ebkbnekcohi: true,
                unk1: true,
                unk2: true,
                unk3: true,
                unk4: true,
                unk5: true,
                unk6: true,
                unk7: true,
                use_tcp: server_config.gateserver_protocol == GatewayProtocolType::Tcp,
                ..Default::default()
            }
        } else {
            Gateserver {
                retcode: 9,
                msg: format!("forbidden version: {} or invalid bind", parameters.version),
                ..Default::default()
            }
        }
    } else {
        Gateserver {
            retcode: 9,
            msg: format!("server config for {region_name} not found"),
            ..Default::default()
        }
    };

    rbase64::encode(&rsp.encode_to_vec())
}