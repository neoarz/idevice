// Jackson Coxson

#include <idevice++/bindings.hpp>
#include <idevice++/ffi.hpp>
#include <idevice++/simulate_location.hpp>
#include <idevice++/provider.hpp>

namespace IdeviceFFI {

Result<LocationSimulation, FfiError> LocationSimulation::connect(Provider& provider) {
    LocationSimulationServiceHandle* out = nullptr;
    FfiError                         e(::simulate_location_connect(provider.raw(), &out));
    if (e) {
        provider.release();
        return Err(e);
    }
    return Ok(LocationSimulation::adopt(out));
}

Result<LocationSimulation, FfiError> LocationSimulation::from_socket(Idevice&& socket) {
    LocationSimulationServiceHandle* out = nullptr;
    FfiError                         e(::simulate_location_new(socket.raw(), &out));
    if (e) {
        return Err(e);
    }
    socket.release();
    return Ok(LocationSimulation::adopt(out));
}

Result<void, FfiError> LocationSimulation::clear() {
    FfiError e(::simulate_location_clear(handle_.get()));
    if (e) {
        return Err(e);
    }
    return Ok();
}

Result<void, FfiError> LocationSimulation::set(const std::string& latitude,
                                               const std::string& longitude) {
    FfiError e(::simulate_location_set(handle_.get(), latitude.c_str(), longitude.c_str()));
    if (e) {
        return Err(e);
    }
    return Ok();
}

} // namespace IdeviceFFI
