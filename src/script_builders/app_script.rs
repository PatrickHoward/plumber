extern crate serde;

use crate::{input::query_input_or_default, script_builders::InteractiveNew};

use serde::{Deserialize, Serialize};

pub struct AppScript {
    app_id: String,
    desc: String,
    build_output: String,
    content_root: String,
    set_live_branch: String,
    // TODO: Add depots
}

impl AppScript {
    pub fn from_dto(dto: appbuild) -> Self {
        AppScript {
            app_id: dto.AppId,
            desc: dto.Desc,
            build_output: dto.BuildOutput,
            content_root: dto.ContentRoot,
            set_live_branch: dto.SetLive,
        }
    }
}

impl InteractiveNew for AppScript {
    fn interactive_new(id: String) -> Self {
        let app_id = id;
        let description = query_input_or_default("Build Description", "");
        let set_live_branch = query_input_or_default("Set this build live to", "");
        let build_output = query_input_or_default("Output build logs to", "");
        let content_root = query_input_or_default("Set build root to", "");

        AppScript {
            app_id,
            desc: description,
            build_output,
            content_root,
            set_live_branch,
        }
    }
}

// This feels like a redunant mapping, but I use a DTO here so that the output matches the
// app script exactly.
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct appbuild {
    AppId: String,
    Desc: String,
    BuildOutput: String,
    ContentRoot: String,
    SetLive: String,
}

impl appbuild {
    pub fn to_string(&self) -> String {
        vdf_serde::to_string(self).unwrap()
    }
}
