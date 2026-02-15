// Jackson Coxson

use std::ffi::CStr;
use std::ptr::null_mut;

use idevice::{
    IdeviceError, IdeviceService, provider::IdeviceProvider,
    services::simulate_location::LocationSimulationService,
};

use crate::{
    IdeviceFfiError, IdeviceHandle, ffi_err, provider::IdeviceProviderHandle, run_sync_local,
};

pub struct LocationSimulationServiceHandle(pub LocationSimulationService);

/// Automatically creates and connects to Location Simulation, returning a client handle
///
/// # Arguments
/// * [`provider`] - An IdeviceProvider
/// * [`client`] - On success, will be set to point to a newly allocated LocationSimulationService handle
///
/// # Returns
/// An IdeviceFfiError on error, null on success
///
/// # Safety
/// `provider` must be a valid pointer to a handle allocated by this library
/// `client` must be a valid, non-null pointer to a location where the handle will be stored
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simulate_location_connect(
    provider: *mut IdeviceProviderHandle,
    client: *mut *mut LocationSimulationServiceHandle,
) -> *mut IdeviceFfiError {
    if provider.is_null() || client.is_null() {
        tracing::error!("Null pointer provided");
        return ffi_err!(IdeviceError::FfiInvalidArg);
    }

    let res: Result<LocationSimulationService, IdeviceError> = run_sync_local(async move {
        let provider_ref: &dyn IdeviceProvider = unsafe { &*(*provider).0 };
        LocationSimulationService::connect(provider_ref).await
    });

    match res {
        Ok(r) => {
            let boxed = Box::new(LocationSimulationServiceHandle(r));
            unsafe { *client = Box::into_raw(boxed) };
            null_mut()
        }
        Err(e) => ffi_err!(e),
    }
}

/// Creates a new Location Simulation client from an existing socket
///
/// # Arguments
/// * [`socket`] - An IdeviceSocket handle
/// * [`client`] - On success, will be set to point to a newly allocated LocationSimulationService handle
///
/// # Returns
/// An IdeviceFfiError on error, null on success
///
/// # Safety
/// `socket` must be a valid pointer to a handle allocated by this library. The socket is consumed,
/// and may not be used again.
/// `client` must be a valid, non-null pointer to a location where the handle will be stored
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simulate_location_new(
    socket: *mut IdeviceHandle,
    client: *mut *mut LocationSimulationServiceHandle,
) -> *mut IdeviceFfiError {
    if socket.is_null() || client.is_null() {
        return ffi_err!(IdeviceError::FfiInvalidArg);
    }
    let socket = unsafe { Box::from_raw(socket) }.0;
    let r = LocationSimulationService::new(socket);
    let boxed = Box::new(LocationSimulationServiceHandle(r));
    unsafe { *client = Box::into_raw(boxed) };
    null_mut()
}

/// Clears the simulated location on the device
///
/// # Arguments
/// * `client` - A valid LocationSimulationService handle
///
/// # Returns
/// An IdeviceFfiError on error, null on success
///
/// # Safety
/// `client` must be a valid pointer to a handle allocated by this library
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simulate_location_clear(
    client: *mut LocationSimulationServiceHandle,
) -> *mut IdeviceFfiError {
    if client.is_null() {
        return ffi_err!(IdeviceError::FfiInvalidArg);
    }
    let res: Result<(), IdeviceError> = run_sync_local(async move {
        let client_ref = unsafe { &mut (*client).0 };
        client_ref.clear().await
    });
    match res {
        Ok(_) => null_mut(),
        Err(e) => ffi_err!(e),
    }
}

/// Sets a simulated location on the device
///
/// # Arguments
/// * `client` - A valid LocationSimulationService handle
/// * `latitude` - The latitude as a C string (e.g. "37.3349")
/// * `longitude` - The longitude as a C string (e.g. "-122.0090")
///
/// # Returns
/// An IdeviceFfiError on error, null on success
///
/// # Safety
/// `client` must be a valid pointer to a handle allocated by this library
/// `latitude` and `longitude` must be valid C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simulate_location_set(
    client: *mut LocationSimulationServiceHandle,
    latitude: *const libc::c_char,
    longitude: *const libc::c_char,
) -> *mut IdeviceFfiError {
    if client.is_null() || latitude.is_null() || longitude.is_null() {
        return ffi_err!(IdeviceError::FfiInvalidArg);
    }

    let lat = match unsafe { CStr::from_ptr(latitude) }.to_str() {
        Ok(s) => s,
        Err(_) => return ffi_err!(IdeviceError::FfiInvalidArg),
    };
    let lng = match unsafe { CStr::from_ptr(longitude) }.to_str() {
        Ok(s) => s,
        Err(_) => return ffi_err!(IdeviceError::FfiInvalidArg),
    };

    let res: Result<(), IdeviceError> = run_sync_local(async move {
        let client_ref = unsafe { &mut (*client).0 };
        client_ref.set(lat, lng).await
    });
    match res {
        Ok(_) => null_mut(),
        Err(e) => ffi_err!(e),
    }
}

/// Frees a LocationSimulationService handle
///
/// # Arguments
/// * [`handle`] - The handle to free
///
/// # Safety
/// `handle` must be a valid pointer to the handle that was allocated by this library,
/// or NULL (in which case this function does nothing)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simulate_location_client_free(
    handle: *mut LocationSimulationServiceHandle,
) {
    if !handle.is_null() {
        tracing::debug!("Freeing location_simulation_client");
        let _ = unsafe { Box::from_raw(handle) };
    }
}
