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

#[allow(unused_imports)]
pub(crate) use std::{
    env,
    ffi::{CStr, CString},
    io,
    fs,
    os::unix::{
        ffi::OsStrExt,
        fs::MetadataExt,
    },
    path::{Path, PathBuf},
    ptr,
    process::Command,
};

//pub(crate) use sourcetrait_twostr::*;
pub(crate) use sourcetrait_crossplat_bridge::{
    self as cross,
    //prelude::driver::*
};
pub(crate) use sourcetrait_crossplat_unix as unix;