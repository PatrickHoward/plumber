extern crate serde;

use crate::{
    script_builders::InteractiveNew,
    input::query_input_or_default
};

use serde::{Serialize, Deserialize};

pub struct AppScript {
    app_id: String,
    desc: String,
    build_output: String,
    content_root: String,
    set_live_branch: String,
    // TODO: Add depots
}

impl AppScript {
    pub fn from_dto(dto: AppScriptDto) -> Self {
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
            set_live_branch
        }
    }
}

// This may be a redundant mapping, but I'll make a dto object for now.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AppScriptDto {
    AppId: String,
    Desc: String,
    BuildOutput: String,
    ContentRoot: String,
    SetLive: String,
}