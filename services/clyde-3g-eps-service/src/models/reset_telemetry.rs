//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use clyde_3g_eps_api::ResetTelemetry::Data as ResetTelemetryData;
use clyde_3g_eps_api::ResetTelemetry::Type as ResetTelemetryType;

#[derive(GraphQLObject)]
pub struct Data {
    pub motherboard: i32,
    pub daughterboard: Option<i32>,
}

impl Into<Data> for ResetTelemetryData {
    fn into(self) -> Data {
        Data {
            motherboard: i32::from(self.motherboard),
            daughterboard: self.daughterboard.map(|d| i32::from(d)),
        }
    }
}

#[derive(GraphQLEnum)]
pub enum Type {
    BrownOut,
    AutomaticSoftware,
    Manual,
    Watchdog,
}

impl Into<ResetTelemetryType> for Type {
    fn into(self) -> ResetTelemetryType {
        match self {
            Type::BrownOut => ResetTelemetryType::BrownOut,
            Type::AutomaticSoftware => ResetTelemetryType::AutomaticSoftware,
            Type::Manual => ResetTelemetryType::Manual,
            Type::Watchdog => ResetTelemetryType::Watchdog,
        }
    }
}
