use crate::Result;
use std::path::{Path, PathBuf};

#[cfg(feature = "build_native")]
pub fn build(out_dir: impl AsRef<Path>, jasper_dir: impl AsRef<Path>) -> Result<PathBuf> {
    use std::process::Stdio;
    let _res = std::process::Command::new("sh")
        .arg(jasper_dir.as_ref().join("build/build"))
        .arg("--static")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    let libjasper = jasper_dir
        .as_ref()
        .join("tmp_cmake/build/src/libjasper/libjasper.a");
    let output = out_dir.as_ref().join("libjasper.a");
    std::fs::copy(libjasper, &output)?;
    Ok(output)
}

#[cfg(feature = "build_cc")]
pub fn build(out_dir: impl AsRef<Path>, jasper_dir: impl AsRef<Path>) -> Result<PathBuf> {
    std::env::set_current_dir(jasper_dir.as_ref().join("src").join("libjasper"))?;

    let mut jasper = cc::Build::new();

    jasper.include(concat!(env!("CARGO_MANIFEST_DIR"), "/config"));
    jasper.include("include");
    let p1: &Path = concat!(env!("CARGO_MANIFEST_DIR"), "/config").as_ref();
    let p2 = std::env::current_dir()?.join("include");
    let p2: &Path = p2.as_ref();

    // Put the include dirs in the DEP_JASPER_INCLUDE
    let include = std::env::join_paths([p1, p2])?;
    println!("cargo:include={:?}", include);

    // jasper.define("JAS_EXPORT", None);
    // jasper.define("JAS_LOCAL", None);
    // dbg!(std::fs::read_dir(".")?.collect::<Vec<_>>());

    #[cfg(feature = "base")]
    jasper.files(BASE);

    #[cfg(feature = "bmp")]
    jasper.files(BMP);

    #[cfg(feature = "jp2")]
    jasper.files(JP2);

    #[cfg(feature = "jpc")]
    jasper.files(JPC);

    #[cfg(feature = "jpg")]
    jasper.files(JPG);
    #[cfg(feature = "jpg")]
    if let Ok(path) = std::env::var("DEP_JPEG_INCLUDE") {
        jasper.includes(std::env::split_paths(&path));
    }

    #[cfg(feature = "heic")]
    jasper.files(HEIC);

    #[cfg(feature = "mif")]
    jasper.files(MIF);

    #[cfg(feature = "pgx")]
    jasper.files(PGX);

    #[cfg(feature = "pnm")]
    jasper.files(PNM);

    #[cfg(feature = "ras")]
    jasper.files(RAS);

    jasper.compile("jasper");

    let output = out_dir.as_ref().join("libjasper.a");
    Ok(output)
}

// pub const HEADERS: &[&'static str] = &[
//     // "${CMAKE_CURRENT_BINARY_DIR}/include/jasper/jas_config.h"
//     "include/jasper/jas_cm.h",
//     "include/jasper/jas_compiler.h",
//     "include/jasper/jas_debug.h",
//     "include/jasper/jas_dll.h",
//     "include/jasper/jas_fix.h",
//     "include/jasper/jas_getopt.h",
//     "include/jasper/jas_icc.h",
//     "include/jasper/jas_image.h",
//     "include/jasper/jas_init.h",
//     "include/jasper/jas_log.h",
//     "include/jasper/jas_malloc.h",
//     "include/jasper/jas_math.h",
//     "include/jasper/jasper.h",
//     "include/jasper/jas_seq.h",
//     "include/jasper/jas_stream.h",
//     "include/jasper/jas_string.h",
//     "include/jasper/jas_thread.h",
//     "include/jasper/jas_tmr.h",
//     "include/jasper/jas_tvp.h",
//     "include/jasper/jas_types.h",
//     "include/jasper/jas_version.h",
// ];

#[cfg(feature = "base")]
pub const BASE: &[&'static str] = &[
    "base/jas_cm.c",
    "base/jas_debug.c",
    "base/jas_getopt.c",
    "base/jas_icc.c",
    "base/jas_iccdata.c",
    "base/jas_image.c",
    "base/jas_init.c",
    "base/jas_malloc.c",
    "base/jas_seq.c",
    "base/jas_stream.c",
    "base/jas_string.c",
    "base/jas_tmr.c",
    "base/jas_tvp.c",
    "base/jas_version.c",
];

#[cfg(feature = "bmp")]
pub const BMP: &[&'static str] = &["bmp/bmp_cod.c", "bmp/bmp_dec.c", "bmp/bmp_enc.c"];

#[cfg(feature = "jp2")]
pub const JP2: &[&'static str] = &["jp2/jp2_cod.c", "jp2/jp2_dec.c", "jp2/jp2_enc.c"];

#[cfg(feature = "jpc")]
pub const JPC: &[&'static str] = &[
    "jpc/jpc_bs.c",
    "jpc/jpc_cs.c",
    "jpc/jpc_cod.c",
    "jpc/jpc_dec.c",
    "jpc/jpc_enc.c",
    "jpc/jpc_math.c",
    "jpc/jpc_mct.c",
    "jpc/jpc_mqcod.c",
    "jpc/jpc_mqdec.c",
    "jpc/jpc_mqenc.c",
    "jpc/jpc_qmfb.c",
    "jpc/jpc_t1cod.c",
    "jpc/jpc_t1dec.c",
    "jpc/jpc_t1enc.c",
    "jpc/jpc_t2cod.c",
    "jpc/jpc_t2dec.c",
    "jpc/jpc_t2enc.c",
    "jpc/jpc_tagtree.c",
    "jpc/jpc_tsfb.c",
    "jpc/jpc_util.c",
];

#[cfg(feature = "jpg")]
pub const JPG: &[&'static str] = &["jpg/jpg_val.c", "jpg/jpg_dec.c", "jpg/jpg_enc.c"];

#[cfg(feature = "heic")]
pub const HEIC: &[&'static str] = &["heic/heic_val.c", "heic/heic_dec.c", "heic/heic_enc.c"];

#[cfg(feature = "mif")]
pub const MIF: &[&'static str] = &["mif/mif_cod.c"];

#[cfg(feature = "pgx")]
pub const PGX: &[&'static str] = &["pgx/pgx_cod.c", "pgx/pgx_dec.c", "pgx/pgx_enc.c"];

#[cfg(feature = "pnm")]
pub const PNM: &[&'static str] = &["pnm/pnm_cod.c", "pnm/pnm_dec.c", "pnm/pnm_enc.c"];

#[cfg(feature = "ras")]
pub const RAS: &[&'static str] = &["ras/ras_cod.c", "ras/ras_dec.c", "ras/ras_enc.c"];
