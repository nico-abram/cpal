#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub(crate) mod alsa;
#[cfg(all(windows, feature = "asio"))]
pub(crate) mod asio;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) mod coreaudio;
#[cfg(any(target_os = "emscripten", all(target_arch="wasm32", target_os = "unknown", feature = "wasm-unknown-web")))]
pub(crate) mod emscripten;
pub(crate) mod null;
#[cfg(windows)]
pub(crate) mod wasapi;
