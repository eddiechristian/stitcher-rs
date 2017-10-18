// Copyright © 2016 IronNet Cybersecurity, Inc. All rights reserved.
//
// Unauthorized copying of this file, via any medium, is strictly prohibited.
//
// PROPRIETARY AND CONFIDENTIAL
//

use common::util::log;



#[derive(Debug, Clone, RustcDecodable)]
pub struct Log {
    pub update_freq: Option<u64>,
    pub app: Option<log::AppLogConfig>,
}

impl Log {
    pub fn update_freq(&self) -> u64 {
        self.update_freq.unwrap_or(1000)
    }
}

impl Default for Log {
    fn default() -> Log {
        Log {
            update_freq: None,
            app: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigThreads {
    event_workers: Vec<u8>,
    flow_workers: Vec<u8>,
    event_recv: u8,
    appliance_ironflow: u8,
    appliance_sensor_stat: u8,
    appliance_suri_alerts: u8
}

impl Default for ConfigThreads {
    fn default() -> ConfigThreads {
        ConfigThreads {
            event_workers: vec![0,2],
            flow_workers: vec![1,3],
            event_recv: 0,
            appliance_ironflow: 0,
            appliance_sensor_stat: 0,
            appliance_suri_alerts: 0
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigApplianceService {
    address: String,
    user_name: String,
    password: String,
    appliance_id: u16,
    queue_tx_id: u16,
    flow_topic: String,
    alert_topic: String
}

impl Default for ConfigApplianceService {
    fn default() -> ConfigApplianceService {
        ConfigApplianceService {
            address: String::from("10.40.15.31:9444"),
            user_name: String::from("admin@iron.net"),
            password: String::from("v-|ji,Rnh%hLMIh|F(tP:@b&@nCx-0:"),
            appliance_id: 0,
            queue_tx_id: 1,
            flow_topic: String::from("ProtoFlows"),
            alert_topic: String::from("SuricataAlerts")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    uds: String,
    threads: ConfigThreads ,
    appliance_service: ConfigApplianceService
}

impl Default for Config {
    fn default() -> Config {
        Config {
            uds: String::from("/run/stitcher/suricata0.sock"),
            threads: ConfigThreads::default(),
            appliance_service: ConfigApplianceService::default()
        }
    }
}
