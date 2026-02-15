#pragma once
#include <idevice++/bindings.hpp>
#include <idevice++/ffi.hpp>
#include <idevice++/provider.hpp>
#include <memory>
#include <string>

namespace IdeviceFFI {

using LocationSimulationPtr =
    std::unique_ptr<LocationSimulationServiceHandle,
                    FnDeleter<LocationSimulationServiceHandle, simulate_location_client_free>>;

class LocationSimulation {
  public:
    // Factory: connect via Provider
    static Result<LocationSimulation, FfiError> connect(Provider& provider);

    // Factory: wrap an existing Idevice socket (consumes it on success)
    static Result<LocationSimulation, FfiError> from_socket(Idevice&& socket);

    // Ops
    Result<void, FfiError> clear();
    Result<void, FfiError> set(const std::string& latitude, const std::string& longitude);

    // RAII / moves
    ~LocationSimulation() noexcept                                       = default;
    LocationSimulation(LocationSimulation&&) noexcept                    = default;
    LocationSimulation& operator=(LocationSimulation&&) noexcept         = default;
    LocationSimulation(const LocationSimulation&)                        = delete;
    LocationSimulation&             operator=(const LocationSimulation&) = delete;

    LocationSimulationServiceHandle* raw() const noexcept { return handle_.get(); }
    static LocationSimulation        adopt(LocationSimulationServiceHandle* h) noexcept {
        return LocationSimulation(h);
    }

  private:
    explicit LocationSimulation(LocationSimulationServiceHandle* h) noexcept : handle_(h) {}
    LocationSimulationPtr handle_{};
};

} // namespace IdeviceFFI
