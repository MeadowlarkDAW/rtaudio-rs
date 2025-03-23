#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[doc = " \\typedef typedef unsigned long rtaudio_format_t;\n\\brief RtAudio data format type.\n\n- \\e RTAUDIO_FORMAT_SINT8:   8-bit signed integer.\n- \\e RTAUDIO_FORMAT_SINT16:  16-bit signed integer.\n- \\e RTAUDIO_FORMAT_SINT24:  24-bit signed integer.\n- \\e RTAUDIO_FORMAT_SINT32:  32-bit signed integer.\n- \\e RTAUDIO_FORMAT_FLOAT32: Normalized between plus/minus 1.0.\n- \\e RTAUDIO_FORMAT_FLOAT64: Normalized between plus/minus 1.0.\n\nSee \\ref RtAudioFormat."]
pub type rtaudio_format_t = c_ulong;
pub const RTAUDIO_FORMAT_SINT8: rtaudio_format_t = 1;
pub const RTAUDIO_FORMAT_SINT16: rtaudio_format_t = 2;
pub const RTAUDIO_FORMAT_SINT24: rtaudio_format_t = 4;
pub const RTAUDIO_FORMAT_SINT32: rtaudio_format_t = 8;
pub const RTAUDIO_FORMAT_FLOAT32: rtaudio_format_t = 16;
pub const RTAUDIO_FORMAT_FLOAT64: rtaudio_format_t = 32;

#[doc = " \\typedef typedef unsigned long rtaudio_stream_flags_t;\n\\brief RtAudio stream option flags.\n\nThe following flags can be OR'ed together to allow a client to\nmake changes to the default stream behavior:\n\n- \\e RTAUDIO_FLAGS_NONINTERLEAVED:   Use non-interleaved buffers (default = interleaved).\n- \\e RTAUDIO_FLAGS_MINIMIZE_LATENCY: Attempt to set stream parameters for lowest possible latency.\n- \\e RTAUDIO_FLAGS_HOG_DEVICE:       Attempt grab device for exclusive use.\n- \\e RTAUDIO_FLAGS_ALSA_USE_DEFAULT: Use the \"default\" PCM device (ALSA only).\n- \\e RTAUDIO_FLAGS_JACK_DONT_CONNECT: Do not automatically connect ports (JACK only).\n\nSee \\ref RtAudioStreamFlags."]
pub type rtaudio_stream_flags_t = c_uint;
pub const RTAUDIO_FLAGS_NONINTERLEAVED: rtaudio_stream_flags_t = 1;
pub const RTAUDIO_FLAGS_MINIMIZE_LATENCY: rtaudio_stream_flags_t = 2;
pub const RTAUDIO_FLAGS_HOG_DEVICE: rtaudio_stream_flags_t = 4;
pub const RTAUDIO_FLAGS_SCHEDULE_REALTIME: rtaudio_stream_flags_t = 8;
pub const RTAUDIO_FLAGS_ALSA_USE_DEFAULT: rtaudio_stream_flags_t = 16;
pub const RTAUDIO_FLAGS_JACK_DONT_CONNECT: rtaudio_stream_flags_t = 32;

#[doc = " \\typedef typedef unsigned long rtaudio_stream_status_t;\n\\brief RtAudio stream status (over- or underflow) flags.\n\nNotification of a stream over- or underflow is indicated by a\nnon-zero stream \\c status argument in the RtAudioCallback function.\nThe stream status can be one of the following two options,\ndepending on whether the stream is open for output and/or input:\n\n- \\e RTAUDIO_STATUS_INPUT_OVERFLOW:   Input data was discarded because of an overflow condition at the driver.\n- \\e RTAUDIO_STATUS_OUTPUT_UNDERFLOW: The output buffer ran low, likely producing a break in the output sound.\n\nSee \\ref RtAudioStreamStatus."]
pub type rtaudio_stream_status_t = c_uint;
pub const RTAUDIO_STATUS_INPUT_OVERFLOW: rtaudio_stream_status_t = 1;
pub const RTAUDIO_STATUS_OUTPUT_UNDERFLOW: rtaudio_stream_status_t = 2;

pub const NUM_SAMPLE_RATES: usize = 16;
pub const MAX_NAME_LENGTH: usize = 512;

#[doc = "! RtAudio callback function prototype.\n*!\nAll RtAudio clients must create a function of this type to read\nand/or write data from/to the audio stream.  When the underlying\naudio system is ready for new input or output data, this function\nwill be invoked.\n\nSee \\ref RtAudioCallback.\n*/"]
pub type rtaudio_cb_t = ::std::option::Option<
    unsafe extern "C" fn(
        out: *mut c_void,
        in_: *mut c_void,
        nFrames: c_uint,
        stream_time: f64,
        status: rtaudio_stream_status_t,
        userdata: *mut c_void,
    ) -> c_int,
>;

#[doc = " \\brief Error codes for RtAudio.\n\nSee \\ref RtAudioError."]
pub type rtaudio_error_t = c_int;
#[doc = "< No error."]
pub const RTAUDIO_ERROR_NONE: rtaudio_error_t = 0;
#[doc = "< A non-critical error."]
pub const RTAUDIO_ERROR_WARNING: rtaudio_error_t = 1;
#[doc = "< An unspecified error type."]
pub const RTAUDIO_ERROR_UNKNOWN: rtaudio_error_t = 2;
#[doc = "< No devices found on system."]
pub const RTAUDIO_ERROR_NO_DEVICES_FOUND: rtaudio_error_t = 3;
#[doc = "< An invalid device ID was specified."]
pub const RTAUDIO_ERROR_INVALID_DEVICE: rtaudio_error_t = 4;
#[doc = "< A device in use was disconnected."]
pub const RTAUDIO_ERROR_DEVICE_DISCONNECT: rtaudio_error_t = 5;
#[doc = "< An error occurred during memory allocation."]
pub const RTAUDIO_ERROR_MEMORY_ERROR: rtaudio_error_t = 6;
#[doc = "< An invalid parameter was specified to a function."]
pub const RTAUDIO_ERROR_INVALID_PARAMETER: rtaudio_error_t = 7;
#[doc = "< The function was called incorrectly."]
pub const RTAUDIO_ERROR_INVALID_USE: rtaudio_error_t = 8;
#[doc = "< A system driver error occurred."]
pub const RTAUDIO_ERROR_DRIVER_ERROR: rtaudio_error_t = 9;
#[doc = "< A system error occurred."]
pub const RTAUDIO_ERROR_SYSTEM_ERROR: rtaudio_error_t = 10;
#[doc = "< A thread error occurred."]
pub const RTAUDIO_ERROR_THREAD_ERROR: rtaudio_error_t = 11;

#[doc = "! RtAudio error callback function prototype.\n*!\n\\param err Type of error.\n\\param msg Error description.\n\nSee \\ref RtAudioErrorCallback.\n*/"]
pub type rtaudio_error_cb_t =
    ::std::option::Option<unsafe extern "C" fn(err: rtaudio_error_t, msg: *const c_char)>;

#[doc = "! Audio API specifier.  See \\ref RtAudio::Api."]
pub type rtaudio_api_t = c_int;
#[doc = "< Search for a working compiled API."]
pub const RTAUDIO_API_UNSPECIFIED: rtaudio_api_t = 0;
#[doc = "< Macintosh OS-X Core Audio API."]
pub const RTAUDIO_API_MACOSX_CORE: rtaudio_api_t = 1;
#[doc = "< The Advanced Linux Sound Architecture API."]
pub const RTAUDIO_API_LINUX_ALSA: rtaudio_api_t = 2;
#[doc = "< The Jack Low-Latency Audio Server API."]
pub const RTAUDIO_API_UNIX_JACK: rtaudio_api_t = 3;
#[doc = "< The Linux PulseAudio API."]
pub const RTAUDIO_API_LINUX_PULSE: rtaudio_api_t = 4;
#[doc = "< The Linux Open Sound System API."]
pub const RTAUDIO_API_LINUX_OSS: rtaudio_api_t = 5;
#[doc = "< The Steinberg Audio Stream I/O API."]
pub const RTAUDIO_API_WINDOWS_ASIO: rtaudio_api_t = 6;
#[doc = "< The Microsoft WASAPI API."]
pub const RTAUDIO_API_WINDOWS_WASAPI: rtaudio_api_t = 7;
#[doc = "< The Microsoft DirectSound API."]
pub const RTAUDIO_API_WINDOWS_DS: rtaudio_api_t = 8;
#[doc = "< A compilable but non-functional API."]
pub const RTAUDIO_API_DUMMY: rtaudio_api_t = 9;
#[doc = "< Number of values in this enum."]
pub const RTAUDIO_API_NUM: rtaudio_api_t = 10;

#[doc = "! The public device information structure for returning queried values.\n! See \\ref RtAudio::DeviceInfo."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtaudio_device_info {
    pub id: c_uint,
    pub output_channels: c_uint,
    pub input_channels: c_uint,
    pub duplex_channels: c_uint,
    pub is_default_output: c_int,
    pub is_default_input: c_int,
    pub native_formats: rtaudio_format_t,
    pub preferred_sample_rate: c_uint,
    pub sample_rates: [c_uint; NUM_SAMPLE_RATES],
    pub name: [c_char; MAX_NAME_LENGTH],
}
#[doc = "! The public device information structure for returning queried values.\n! See \\ref RtAudio::DeviceInfo."]
pub type rtaudio_device_info_t = rtaudio_device_info;

#[doc = "! The structure for specifying input or output stream parameters.\n! See \\ref RtAudio::StreamParameters."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtaudio_stream_parameters {
    pub device_id: c_uint,
    pub num_channels: c_uint,
    pub first_channel: c_uint,
}
#[doc = "! The structure for specifying input or output stream parameters.\n! See \\ref RtAudio::StreamParameters."]
pub type rtaudio_stream_parameters_t = rtaudio_stream_parameters;

#[doc = "! The structure for specifying stream options.\n! See \\ref RtAudio::StreamOptions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtaudio_stream_options {
    pub flags: rtaudio_stream_flags_t,
    pub num_buffers: c_uint,
    pub priority: c_int,
    pub name: [c_char; MAX_NAME_LENGTH],
}
#[doc = "! The structure for specifying stream options.\n! See \\ref RtAudio::StreamOptions."]
pub type rtaudio_stream_options_t = rtaudio_stream_options;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtaudio {
    _unused: [u8; 0],
}
pub type rtaudio_t = *mut rtaudio;

extern "C" {
    #[doc = "! Determine the current RtAudio version.  See \\ref RtAudio::getVersion()."]
    pub fn rtaudio_version() -> *const c_char;

    #[doc = "! Determine the number of available compiled audio APIs, the length\n! of the array returned by rtaudio_compiled_api().  See \\ref\n! RtAudio::getCompiledApi()."]
    pub fn rtaudio_get_num_compiled_apis() -> c_uint;

    #[doc = "! Return an array of rtaudio_api_t compiled into this instance of\n! RtAudio.  This array is static (do not free it) and has the length\n! returned by rtaudio_get_num_compiled_apis().  See \\ref\n! RtAudio::getCompiledApi()."]
    pub fn rtaudio_compiled_api() -> *const rtaudio_api_t;

    #[doc = "! Return the name of a specified rtaudio_api_t.  This string can be\n! used to look up an API by rtaudio_compiled_api_by_name().  See\n! \\ref RtAudio::getApiName()."]
    pub fn rtaudio_api_name(api: rtaudio_api_t) -> *const c_char;

    #[doc = "! Return the display name of a specified rtaudio_api_t.  See \\ref\n! RtAudio::getApiDisplayName()."]
    pub fn rtaudio_api_display_name(api: rtaudio_api_t) -> *const c_char;

    #[doc = "! Return the rtaudio_api_t having the given name.  See \\ref\n! RtAudio::getCompiledApiByName()."]
    pub fn rtaudio_compiled_api_by_name(name: *const c_char) -> rtaudio_api_t;

    pub fn rtaudio_error(audio: rtaudio_t) -> *const c_char;

    pub fn rtaudio_error_type(audio: rtaudio_t) -> rtaudio_error_t;

    #[doc = "! Create an instance of struct rtaudio."]
    pub fn rtaudio_create(api: rtaudio_api_t) -> rtaudio_t;

    #[doc = "! Free an instance of struct rtaudio."]
    pub fn rtaudio_destroy(audio: rtaudio_t);

    #[doc = "! Returns the audio API specifier for the current instance of\n! RtAudio.  See RtAudio::getCurrentApi()."]
    pub fn rtaudio_current_api(audio: rtaudio_t) -> rtaudio_api_t;

    #[doc = "! Queries for the number of audio devices available.  See \\ref\n! RtAudio::getDeviceCount()."]
    pub fn rtaudio_device_count(audio: rtaudio_t) -> c_int;

    #[doc = "! Returns the audio device ID corresponding to a given index\n! value (valid index values are between 0 and rtaudio_device_count()-1).\n! Note that a return value of 0 is invalid, which will occur if the\n! index value is out of bounds or no devices are found. See \\ref\n! RtAudio::getDeviceIds()."]
    pub fn rtaudio_get_device_id(audio: rtaudio_t, i: c_int) -> c_uint;

    #[doc = "! Return a struct rtaudio_device_info for a specified device ID.\n! See \\ref RtAudio::getDeviceInfo()."]
    pub fn rtaudio_get_device_info(audio: rtaudio_t, id: c_uint) -> rtaudio_device_info_t;

    #[doc = "! Returns the device id of the default output device.  See \\ref\n! RtAudio::getDefaultOutputDevice()."]
    pub fn rtaudio_get_default_output_device(audio: rtaudio_t) -> c_uint;

    #[doc = "! Returns the device id of the default input device.  See \\ref\n! RtAudio::getDefaultInputDevice()."]
    pub fn rtaudio_get_default_input_device(audio: rtaudio_t) -> c_uint;

    #[doc = "! Opens a stream with the specified parameters.  See \\ref RtAudio::openStream().\n! \\return an \\ref rtaudio_error."]
    pub fn rtaudio_open_stream(
        audio: rtaudio_t,
        output_params: *mut rtaudio_stream_parameters_t,
        input_params: *mut rtaudio_stream_parameters_t,
        format: rtaudio_format_t,
        sample_rate: c_uint,
        buffer_frames: *mut c_uint,
        cb: rtaudio_cb_t,
        userdata: *mut c_void,
        options: *mut rtaudio_stream_options_t,
        errcb: rtaudio_error_cb_t,
    ) -> rtaudio_error_t;

    #[doc = "! Closes a stream and frees any associated stream memory.  See \\ref RtAudio::closeStream()."]
    pub fn rtaudio_close_stream(audio: rtaudio_t);

    #[doc = "! Starts a stream.  See \\ref RtAudio::startStream()."]
    pub fn rtaudio_start_stream(audio: rtaudio_t) -> rtaudio_error_t;

    #[doc = "! Stop a stream, allowing any samples remaining in the output queue\n! to be played.  See \\ref RtAudio::stopStream()."]
    pub fn rtaudio_stop_stream(audio: rtaudio_t) -> rtaudio_error_t;

    #[doc = "! Stop a stream, discarding any samples remaining in the\n! input/output queue.  See \\ref RtAudio::abortStream()."]
    pub fn rtaudio_abort_stream(audio: rtaudio_t) -> rtaudio_error_t;

    #[doc = "! Returns 1 if a stream is open and false if not.  See \\ref RtAudio::isStreamOpen()."]
    pub fn rtaudio_is_stream_open(audio: rtaudio_t) -> c_int;

    #[doc = "! Returns 1 if a stream is running and false if it is stopped or not\n! open.  See \\ref RtAudio::isStreamRunning()."]
    pub fn rtaudio_is_stream_running(audio: rtaudio_t) -> c_int;

    #[doc = "! Returns the number of elapsed seconds since the stream was\n! started.  See \\ref RtAudio::getStreamTime()."]
    pub fn rtaudio_get_stream_time(audio: rtaudio_t) -> f64;

    #[doc = "! Set the stream time to a time in seconds greater than or equal to\n! 0.0.  See \\ref RtAudio::setStreamTime()."]
    pub fn rtaudio_set_stream_time(audio: rtaudio_t, time: f64);

    #[doc = "! Returns the internal stream latency in sample frames.  See \\ref\n! RtAudio::getStreamLatency()."]
    pub fn rtaudio_get_stream_latency(audio: rtaudio_t) -> c_long;

    #[doc = "! Returns actual sample rate in use by the stream.  See \\ref\n! RtAudio::getStreamSampleRate()."]
    pub fn rtaudio_get_stream_sample_rate(audio: rtaudio_t) -> c_uint;

    #[doc = "! Specify whether warning messages should be printed to stderr.  See\n! \\ref RtAudio::showWarnings()."]
    pub fn rtaudio_show_warnings(audio: rtaudio_t, show: c_int);
}
