cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub type PCIDevice = linux::LinuxPCIDevice;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
        pub type PCIDevice = macos::MacOSPCIDevice;
    } else if #[cfg(target_os = "netbsd")] {
        pub mod netbsd;
        pub type PCIDevice = netbsd::NetBSDPCIDevice;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub type PCIDevice = windows::WindowsPCIDevice;
    } else {
        compile_error!("aparato does not support this platform, at least not yet.");
    }
}

pub trait Properties: private::PrivateProperties {
        /// This function returns a new instance of `PCIDevice` struct using the given `path`.
        ///
        /// ## Examples
        ///
        /// ```
        /// use aparato::PCIDevice;
        /// use aparato::traits::*;
        ///
        /// // PCIDevice::new() can autocomplete the path to the PCIDevice
        /// // if it isn't provided.
        ///
        /// // The following statements all point to the same device.
        /// let device_1 = PCIDevice::new("00:02.0");
        /// let device_2 = PCIDevice::new("0000:00:02.0");
        /// let device_3 = PCIDevice::new("/sys/bus/pci/devices/0000:00:02.0");
        /// ```
        fn new(path: &str) -> Self;
}

pub(crate) mod private {
    pub trait PrivateProperties {
        /// `PCIDevice::new()` calls this function to initialize the device's fields
        /// by calling several *setters*.
        ///
        /// The following are fields that `init()` sets for the caller:
        /// - `address`
        /// - `class_id`
        /// - `vendor_id`
        /// - `device_id`
        /// - `class_name`
        /// - `numa_node`
        #[doc(hidden)]
        fn init(&mut self);

        // Getters...

        /// This function returns the `PCIDevice` path.
        fn path(&self) -> std::path::PathBuf;

        /// This function returns the `PCIDevice` address.
        fn address(&self) -> String;

        /// This function returns the `PCIDevice` class ID.
        fn class_id(&self) -> Vec<u8>;

        /// This function returns the `PCIDevice` vendor ID.
        fn vendor_id(&self) -> Vec<u8>;

        /// This function returns the `PCIDevice` device ID.
        fn device_id(&self) -> Vec<u8>;

        /// This function returns the `PCIDevice` NUMA node.
        fn numa_node(&self) -> isize;

        /// This function returns the `PCIDevice` class name.
        fn class_name(&self) -> String;

        /// This function returns the `PCIDevice` vendor name.
        fn vendor_name(&self) -> String;

        /// This function returns the `PCIDevice` device name.
        fn device_name(&self) -> String;

        /// This function returns whether the `PCIDevice` is enabled.
        fn enabled(&self) -> bool;

        /// This function returns whether the `PCIDevice` is enabled.
        fn d3cold_allowed(&self) -> bool;

        /// This function returns whether the `PCIDevice` is enabled.
        fn revision(&self) -> Vec<u8>;

        /// This function returns the `PCIDevice` subsystem vendor.
        fn subsystem_name(&self) -> String;

        /// This function returns the `PCIDevice` subsystem vendor.
        fn subsystem_vendor_id(&self) -> Vec<u8>;

        /// This function returns the `PCIDevice` subsystem vendor.
        fn subsystem_device_id(&self) -> Vec<u8>;

        // Setters...

        /// Set the `path` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_path(&mut self, p: std::path::PathBuf);

        /// This function sets the `address` field of the `PCIDevice`
        #[doc(hidden)]
        fn set_address(&mut self);

        /// This function sets the `device_id` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_class_id(&mut self);

        /// This function sets the `device_id` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_vendor_id(&mut self);

        /// This function sets the `device_id` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_device_id(&mut self);

        /// This function sets the `numa_node` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_numa_node(&mut self);

        /// This function sets the `class_name` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_class_name(&mut self);

        /// This function sets the `revision` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_revision(&mut self);

        /// This function sets the `enabled` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_enabled(&mut self);

        /// This function sets the `d3cold_allowed` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_d3cold_allowed(&mut self);

        /// This function sets the `vendor_name` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_vendor_name(&mut self);

        /// This function sets the `device_name` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_device_name(&mut self);

        /// This function sets the `subsystem_vendor_id` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_subsystem_device_id(&mut self);

        /// This function sets the `subsystem_device_id` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_subsystem_vendor_id(&mut self);

        /// This function sets the `subsystem_name` field of the `PCIDevice`.
        #[doc(hidden)]
        fn set_subsystem_name(&mut self);
    }
}

pub trait Fetch {
    /// This function returns a list of available PCI devices and their information.
    fn fetch() -> Vec<PCIDevice>;

    /// This function returns a list of available PCI devices of a specific class and their information.
    fn fetch_by_class(class: crate::classes::DeviceClass) -> Vec<PCIDevice>;

    /// This function returns a list of available GPUs and their information.
    ///
    /// This essentially wraps `fetch_by_class(DeviceClass::DisplayController)`
    /// but masks unnecessary data from device and vendor names, for example: \
    /// - `TU117M [GeForce GTX 1650 Mobile / Max-Q]` becomes `GeForce GTX 1650 Mobile / Max-Q`
    /// - `NVIDIA Corporation` becomes `NVIDIA`
    fn fetch_gpus() -> Vec<PCIDevice>;
}

mod classes;
mod extra;