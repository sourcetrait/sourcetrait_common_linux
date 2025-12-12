#[cfg(feature = "crossplat")]
pub(crate) mod crossplat {
    pub(crate) mod component {
        pub(crate) mod net {
            pub(crate) mod net;
            //pub(crate) mod nss;
        }
        pub(crate) mod cmd;
        pub(crate) mod ui;
    }
    pub(crate) mod consts;
    //pub(crate) mod error;
    //pub(crate) mod model;
}

pub use crate::{
    crossplat::{
        component::{
            net::{
                net::*,
                //nss::*,
            },
            cmd::*,
            ui::*,
        },
    },
};

pub(crate) use crate::{
    crossplat::{
        consts::*,
    },
};

pub(crate) use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

//pub(crate) use sourcetrait_twostr::*;
pub(crate) use sourcetrait_crossplat_bridge::{
    self as cross,
    //prelude::driver::*
};
pub(crate) use sourcetrait_crossplat_unix as unix;