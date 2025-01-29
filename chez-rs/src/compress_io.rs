use crate::cffi::{
    COMPRESS_FORMAT_BITS, COMPRESS_GZIP, COMPRESS_HIGH, COMPRESS_LOW, COMPRESS_LZ4, COMPRESS_MAX,
    COMPRESS_MEDIUM, COMPRESS_MIN,
};
use crate::cffi::{INT, glzFile, iptr};
use crate::cstr;
use crate::wrapper::{error1, integer};
use libz_sys::{Z_BEST_COMPRESSION, Z_BEST_SPEED};

// LZ4HC constants
const LZ4HC_CLEVEL_MIN: INT = 3;
const LZ4HC_CLEVEL_DEFAULT: INT = 9;
const LZ4HC_CLEVEL_OPT_MIN: INT = 10;
const LZ4HC_CLEVEL_MAX: INT = 12;

// the value of LZ4_OUTPUT_PORT_IN_BUFFER_SIZE was determined
// through experimentation on an intel linux server and an intel
// osx laptop.  smaller sizes result in significantly worse compression
// of object files, and larger sizes don't have much beneficial effect.
// don't increase the output-port in-buffer size unless you're sure
// it reduces object-file size or reduces compression time
// significantly.  don't decrease it unless you're sure it doesn't
// increase object-file size significantly.  one buffer of size
// LZ4_OUTPUT_PORT_IN_BUFFER_SIZE is allocated per lz4-compressed
// output port.  another buffer of a closely related size is allocated
// per thread.
const LZ4_OUTPUT_PORT_IN_BUFFER_SIZE: usize = (1 << 18);

// the values we choose for LZ4_INPUT_PORT_IN_BUFFER_SIZE and
// LZ4_INPUT_PORT_OUT_BUFFER_SIZE don't seem to make much difference
// in decompression speed, so we keep them fairly small.  one buffer
// of size LZ4_INPUT_PORT_IN_BUFFER_SIZE and one buffer of size
// LZ4_INPUT_PORT_OUT_BUFFER_SIZE are allocated per lz4-compressed
// input port.
const LZ4_INPUT_PORT_IN_BUFFER_SIZE: usize = (1 << 12);
const LZ4_INPUT_PORT_OUT_BUFFER_SIZE: usize = (1 << 14);

#[derive(Debug)]
#[repr(C)]
struct lz4File_out {
    preferences: lz4_sys::LZ4FPreferences,
    fd: INT,
    out_buffer_size: INT,
    in_pos: INT,
    err: INT,
    stream_pos: usize,
    in_buffer: [::core::ffi::c_char; LZ4_OUTPUT_PORT_IN_BUFFER_SIZE],
}

#[derive(Debug)]
#[repr(C)]
struct lz4File_in {
    fd: INT,
    dctx: lz4_sys::LZ4FDecompressionContext,
    in_pos: INT,
    in_len: INT,
    out_pos: INT,
    out_len: INT,
    frame_ended: INT,
    err: INT,
    stream_pos: usize,
    in_buffer: [::core::ffi::c_char; LZ4_INPUT_PORT_IN_BUFFER_SIZE],
    out_buffer: [::core::ffi::c_char; LZ4_INPUT_PORT_OUT_BUFFER_SIZE],
}

#[derive(Debug)]
#[repr(C)]
struct sized_buffer {
    size: INT,
    buffer: [::core::ffi::c_char; 0],
}

#[unsafe(no_mangle)]
extern "C" fn S_zlib_compress_level(compress_level: INT) -> INT {
    match compress_level as u32 {
        COMPRESS_MIN | COMPRESS_LOW => Z_BEST_SPEED,
        COMPRESS_MEDIUM => (Z_BEST_SPEED + Z_BEST_COMPRESSION) / 2,
        COMPRESS_HIGH => (Z_BEST_SPEED + (3 * Z_BEST_COMPRESSION)) / 4,
        COMPRESS_MAX => Z_BEST_COMPRESSION,
        _ => {
            error1(
                "S_zlib_compress_level",
                "unexpected compress level ~s",
                integer(compress_level as iptr),
            );
            0
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn glzdopen_output_gz(fd: INT, compress_level: INT) -> glzFile {
    use crate::cffi::{glzFile_r, glzFile_r__bindgen_ty_1};
    use libz_sys::{Z_DEFAULT_STRATEGY, gzdopen, gzsetparams};

    let as_append = if cfg!(target_os = "windows") {
        0
    } else {
        unsafe { libc::fcntl(fd, libc::F_GETFL) & libc::O_APPEND }
    };
    let mode = if as_append != 0 { "ab" } else { "wb" };

    let gz = unsafe { gzdopen(fd, cstr!(mode)) };
    if gz.is_null() {
        return ::core::ptr::null_mut();
    }

    let level = S_zlib_compress_level(compress_level);
    unsafe { gzsetparams(gz, level, Z_DEFAULT_STRATEGY) };

    let gz = gz as *mut crate::cffi::gzFile_s;
    let glz = glzFile_r {
        fd,
        inputp: 0,
        format: COMPRESS_GZIP as i32,
        __bindgen_anon_1: glzFile_r__bindgen_ty_1 { gz },
    };

    Box::into_raw(Box::new(glz))
}

#[unsafe(no_mangle)]
extern "C" fn S_lz4_compress_level(compress_level: INT) -> INT {
    match compress_level as u32 {
        COMPRESS_MIN | COMPRESS_LOW => 1,
        COMPRESS_MEDIUM => LZ4HC_CLEVEL_MIN,
        COMPRESS_HIGH => (LZ4HC_CLEVEL_MIN + LZ4HC_CLEVEL_MAX) / 2,
        COMPRESS_MAX => LZ4HC_CLEVEL_MAX,
        _ => {
            error1(
                "S_lz4_compress_level",
                "unexpected compress level ~s",
                integer(compress_level as iptr),
            );
            0
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn glzdopen_output_lz4(fd: INT, compress_level: INT) -> glzFile {
    use crate::cffi::{glzFile_r, glzFile_r__bindgen_ty_1};
    use lz4_sys::{
        BlockChecksum, BlockMode, BlockSize, ContentChecksum, FrameType, LZ4F_compressBound,
        LZ4FFrameInfo, LZ4FPreferences,
    };

    let level = S_lz4_compress_level(compress_level);
    let preferences = LZ4FPreferences {
        frame_info: LZ4FFrameInfo {
            block_size_id: BlockSize::Default,
            block_mode: BlockMode::Linked,
            content_checksum_flag: ContentChecksum::NoChecksum,
            frame_type: FrameType::Frame,
            content_size: 0,
            dict_id: 0,
            block_checksum_flag: BlockChecksum::NoBlockChecksum,
        },
        compression_level: level as u32,
        auto_flush: 1,
        favor_dec_speed: 0,
        reserved: [0; 3],
    };
    const LZ4F_HEADER_SIZE_MAX: usize = 19;
    let out_buffer_size =
        unsafe { LZ4F_compressBound(LZ4_OUTPUT_PORT_IN_BUFFER_SIZE, &preferences) };
    let out_buffer_size = out_buffer_size + LZ4F_HEADER_SIZE_MAX;
    let out_buffer_size = out_buffer_size as i32;
    let lz4_out = lz4File_out {
        preferences,
        fd,
        out_buffer_size,
        in_pos: 0,
        err: 0,
        stream_pos: 0,
        in_buffer: [0; LZ4_OUTPUT_PORT_IN_BUFFER_SIZE],
    };

    let lz4_out = Box::into_raw(Box::new(lz4_out)) as *mut crate::cffi::lz4File_out_r;

    let glz = glzFile_r {
        fd,
        inputp: 0,
        format: COMPRESS_GZIP as i32,
        __bindgen_anon_1: glzFile_r__bindgen_ty_1 { lz4_out },
    };

    Box::into_raw(Box::new(glz))
}
