//! # D-Bus interface proxy for: `org.freedesktop.systemd1.Manager`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `Interface '/org/freedesktop/systemd1' from service 'org.freedesktop.systemd1' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
#![allow(warnings)]
#![allow(clippy)]
use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
pub trait Manager {
    /// AbandonScope method
    fn abandon_scope(&self, name: &str) -> zbus::Result<()>;

    /// AddDependencyUnitFiles method
    fn add_dependency_unit_files(
        &self,
        files: &[&str],
        target: &str,
        type_: &str,
        runtime: bool,
        force: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// AttachProcessesToUnit method
    fn attach_processes_to_unit(
        &self,
        unit_name: &str,
        subcgroup: &str,
        pids: &[u32],
    ) -> zbus::Result<()>;

    /// BindMountUnit method
    fn bind_mount_unit(
        &self,
        name: &str,
        source: &str,
        destination: &str,
        read_only: bool,
        mkdir: bool,
    ) -> zbus::Result<()>;

    /// CancelJob method
    fn cancel_job(&self, id: u32) -> zbus::Result<()>;

    /// CleanUnit method
    fn clean_unit(&self, name: &str, mask: &[&str]) -> zbus::Result<()>;

    /// ClearJobs method
    fn clear_jobs(&self) -> zbus::Result<()>;

    /// DisableUnitFiles method
    fn disable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// DisableUnitFilesWithFlags method
    fn disable_unit_files_with_flags(
        &self,
        files: &[&str],
        flags: u64,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// DisableUnitFilesWithFlagsAndInstallInfo method
    fn disable_unit_files_with_flags_and_install_info(
        &self,
        files: &[&str],
        flags: u64,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// Dump method
    fn dump(&self) -> zbus::Result<String>;

    /// DumpByFileDescriptor method
    fn dump_by_file_descriptor(&self) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// DumpUnitFileDescriptorStore method
    fn dump_unit_file_descriptor_store(
        &self,
        name: &str,
    ) -> zbus::Result<Vec<(String, u32, u32, u32, u64, u32, u32, String, u32)>>;

    /// DumpUnitsMatchingPatterns method
    fn dump_units_matching_patterns(&self, patterns: &[&str]) -> zbus::Result<String>;

    /// DumpUnitsMatchingPatternsByFileDescriptor method
    fn dump_units_matching_patterns_by_file_descriptor(
        &self,
        patterns: &[&str],
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// EnableUnitFiles method
    fn enable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// EnableUnitFilesWithFlags method
    fn enable_unit_files_with_flags(
        &self,
        files: &[&str],
        flags: u64,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// EnqueueMarkedJobs method
    fn enqueue_marked_jobs(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// EnqueueUnitJob method
    #[allow(clippy::too_many_arguments)]
    fn enqueue_unit_job(
        &self,
        name: &str,
        job_type: &str,
        job_mode: &str,
    ) -> zbus::Result<(
        u32,
        zbus::zvariant::OwnedObjectPath,
        String,
        zbus::zvariant::OwnedObjectPath,
        String,
        Vec<(
            u32,
            zbus::zvariant::OwnedObjectPath,
            String,
            zbus::zvariant::OwnedObjectPath,
            String,
        )>,
    )>;

    /// Exit method
    fn exit(&self) -> zbus::Result<()>;

    /// FreezeUnit method
    fn freeze_unit(&self, name: &str) -> zbus::Result<()>;

    /// GetDefaultTarget method
    fn get_default_target(&self) -> zbus::Result<String>;

    /// GetDynamicUsers method
    fn get_dynamic_users(&self) -> zbus::Result<Vec<(u32, String)>>;

    /// GetJob method
    fn get_job(&self, id: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetJobAfter method
    fn get_job_after(
        &self,
        id: u32,
    ) -> zbus::Result<
        Vec<(
            u32,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// GetJobBefore method
    fn get_job_before(
        &self,
        id: u32,
    ) -> zbus::Result<
        Vec<(
            u32,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// GetUnit method
    fn get_unit(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUnitByControlGroup method
    fn get_unit_by_control_group(
        &self,
        cgroup: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUnitByInvocationID method
    #[zbus(name = "GetUnitByInvocationID")]
    fn get_unit_by_invocation_id(
        &self,
        invocation_id: &[u8],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUnitByPID method
    #[zbus(name = "GetUnitByPID")]
    fn get_unit_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUnitByPIDFD method
    #[zbus(name = "GetUnitByPIDFD")]
    fn get_unit_by_pidfd(
        &self,
        pidfd: zbus::zvariant::Fd<'_>,
    ) -> zbus::Result<(zbus::zvariant::OwnedObjectPath, String, Vec<u8>)>;

    /// GetUnitFileLinks method
    fn get_unit_file_links(&self, name: &str, runtime: bool) -> zbus::Result<Vec<String>>;

    /// GetUnitFileState method
    fn get_unit_file_state(&self, file: &str) -> zbus::Result<String>;

    /// GetUnitProcesses method
    fn get_unit_processes(&self, name: &str) -> zbus::Result<Vec<(String, u32, String)>>;

    /// Halt method
    fn halt(&self) -> zbus::Result<()>;

    /// KExec method
    #[zbus(name = "KExec")]
    fn kexec(&self) -> zbus::Result<()>;

    /// KillUnit method
    fn kill_unit(&self, name: &str, whom: &str, signal: i32) -> zbus::Result<()>;

    /// LinkUnitFiles method
    fn link_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// ListJobs method
    fn list_jobs(
        &self,
    ) -> zbus::Result<
        Vec<(
            u32,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// ListUnitFiles method
    fn list_unit_files(&self) -> zbus::Result<Vec<(String, String)>>;

    /// ListUnitFilesByPatterns method
    fn list_unit_files_by_patterns(
        &self,
        states: &[&str],
        patterns: &[&str],
    ) -> zbus::Result<Vec<(String, String)>>;

    /// ListUnits method
    fn list_units(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            u32,
            String,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// ListUnitsByNames method
    fn list_units_by_names(
        &self,
        names: &[&str],
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            u32,
            String,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// ListUnitsByPatterns method
    fn list_units_by_patterns(
        &self,
        states: &[&str],
        patterns: &[&str],
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            u32,
            String,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// ListUnitsFiltered method
    fn list_units_filtered(
        &self,
        states: &[&str],
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            zbus::zvariant::OwnedObjectPath,
            u32,
            String,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// LoadUnit method
    fn load_unit(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// LookupDynamicUserByName method
    fn lookup_dynamic_user_by_name(&self, name: &str) -> zbus::Result<u32>;

    /// LookupDynamicUserByUID method
    #[zbus(name = "LookupDynamicUserByUID")]
    fn lookup_dynamic_user_by_uid(&self, uid: u32) -> zbus::Result<String>;

    /// MaskUnitFiles method
    fn mask_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// MountImageUnit method
    fn mount_image_unit(
        &self,
        name: &str,
        source: &str,
        destination: &str,
        read_only: bool,
        mkdir: bool,
        options: &[&(&str, &str)],
    ) -> zbus::Result<()>;

    /// PowerOff method
    fn power_off(&self) -> zbus::Result<()>;

    /// PresetAllUnitFiles method
    fn preset_all_unit_files(
        &self,
        mode: &str,
        runtime: bool,
        force: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// PresetUnitFiles method
    fn preset_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// PresetUnitFilesWithMode method
    fn preset_unit_files_with_mode(
        &self,
        files: &[&str],
        mode: &str,
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// QueueSignalUnit method
    fn queue_signal_unit(
        &self,
        name: &str,
        whom: &str,
        signal: i32,
        value: i32,
    ) -> zbus::Result<()>;

    /// Reboot method
    fn reboot(&self) -> zbus::Result<()>;

    /// ReenableUnitFiles method
    fn reenable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// Reexecute method
    fn reexecute(&self) -> zbus::Result<()>;

    /// RefUnit method
    fn ref_unit(&self, name: &str) -> zbus::Result<()>;

    /// Reload method
    fn reload(&self) -> zbus::Result<()>;

    /// ReloadOrRestartUnit method
    fn reload_or_restart_unit(
        &self,
        name: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ReloadOrTryRestartUnit method
    fn reload_or_try_restart_unit(
        &self,
        name: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ReloadUnit method
    fn reload_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ResetFailed method
    fn reset_failed(&self) -> zbus::Result<()>;

    /// ResetFailedUnit method
    fn reset_failed_unit(&self, name: &str) -> zbus::Result<()>;

    /// RestartUnit method
    fn restart_unit(&self, name: &str, mode: &str)
        -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// RevertUnitFiles method
    fn revert_unit_files(&self, files: &[&str]) -> zbus::Result<Vec<(String, String, String)>>;

    /// SetDefaultTarget method
    fn set_default_target(
        &self,
        name: &str,
        force: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// SetEnvironment method
    fn set_environment(&self, assignments: &[&str]) -> zbus::Result<()>;

    /// SetExitCode method
    fn set_exit_code(&self, number: u8) -> zbus::Result<()>;

    /// SetShowStatus method
    fn set_show_status(&self, mode: &str) -> zbus::Result<()>;

    /// SetUnitProperties method
    fn set_unit_properties(
        &self,
        name: &str,
        runtime: bool,
        properties: &[&(&str, &zbus::zvariant::Value<'_>)],
    ) -> zbus::Result<()>;

    /// SoftReboot method
    fn soft_reboot(&self, new_root: &str) -> zbus::Result<()>;

    /// StartTransientUnit method
    #[allow(clippy::type_complexity)]
    fn start_transient_unit(
        &self,
        name: &str,
        mode: &str,
        properties: &[&(&str, &zbus::zvariant::Value<'_>)],
        aux: &[&(&str, &[&(&str, &zbus::zvariant::Value<'_>)])],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// StartUnit method
    fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// StartUnitReplace method
    fn start_unit_replace(
        &self,
        old_unit: &str,
        new_unit: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// StartUnitWithFlags method
    fn start_unit_with_flags(
        &self,
        name: &str,
        mode: &str,
        flags: u64,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// StopUnit method
    fn stop_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Subscribe method
    fn subscribe(&self) -> zbus::Result<()>;

    /// SwitchRoot method
    fn switch_root(&self, new_root: &str, init: &str) -> zbus::Result<()>;

    /// ThawUnit method
    fn thaw_unit(&self, name: &str) -> zbus::Result<()>;

    /// TryRestartUnit method
    fn try_restart_unit(
        &self,
        name: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// UnmaskUnitFiles method
    fn unmask_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;

    /// UnrefUnit method
    fn unref_unit(&self, name: &str) -> zbus::Result<()>;

    /// UnsetAndSetEnvironment method
    fn unset_and_set_environment(&self, names: &[&str], assignments: &[&str]) -> zbus::Result<()>;

    /// UnsetEnvironment method
    fn unset_environment(&self, names: &[&str]) -> zbus::Result<()>;

    /// Unsubscribe method
    fn unsubscribe(&self) -> zbus::Result<()>;

    /// JobNew signal
    #[zbus(signal)]
    fn job_new(&self, id: u32, job: zbus::zvariant::ObjectPath<'_>, unit: &str)
        -> zbus::Result<()>;

    /// JobRemoved signal
    #[zbus(signal)]
    fn job_removed(
        &self,
        id: u32,
        job: zbus::zvariant::ObjectPath<'_>,
        unit: &str,
        result: &str,
    ) -> zbus::Result<()>;

    /// Reloading signal
    #[zbus(signal)]
    fn reloading(&self, active: bool) -> zbus::Result<()>;

    /// StartupFinished signal
    #[zbus(signal)]
    fn startup_finished(
        &self,
        firmware: u64,
        loader: u64,
        kernel: u64,
        initrd: u64,
        userspace: u64,
        total: u64,
    ) -> zbus::Result<()>;

    /// UnitFilesChanged signal
    #[zbus(signal)]
    fn unit_files_changed(&self) -> zbus::Result<()>;

    /// UnitNew signal
    #[zbus(signal)]
    fn unit_new(&self, id: &str, unit: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// UnitRemoved signal
    #[zbus(signal)]
    fn unit_removed(&self, id: &str, unit: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// Architecture property
    #[zbus(property)]
    fn architecture(&self) -> zbus::Result<String>;

    /// ConfidentialVirtualization property
    #[zbus(property)]
    fn confidential_virtualization(&self) -> zbus::Result<String>;

    /// ConfirmSpawn property
    #[zbus(property)]
    fn confirm_spawn(&self) -> zbus::Result<bool>;

    /// ControlGroup property
    #[zbus(property)]
    fn control_group(&self) -> zbus::Result<String>;

    /// CtrlAltDelBurstAction property
    #[zbus(property)]
    fn ctrl_alt_del_burst_action(&self) -> zbus::Result<String>;

    /// DefaultBlockIOAccounting property
    #[zbus(property, name = "DefaultBlockIOAccounting")]
    fn default_block_ioaccounting(&self) -> zbus::Result<bool>;

    /// DefaultCPUAccounting property
    #[zbus(property, name = "DefaultCPUAccounting")]
    fn default_cpuaccounting(&self) -> zbus::Result<bool>;

    /// DefaultDeviceTimeoutUSec property
    #[zbus(property, name = "DefaultDeviceTimeoutUSec")]
    fn default_device_timeout_usec(&self) -> zbus::Result<u64>;

    /// DefaultIOAccounting property
    #[zbus(property, name = "DefaultIOAccounting")]
    fn default_ioaccounting(&self) -> zbus::Result<bool>;

    /// DefaultIPAccounting property
    #[zbus(property, name = "DefaultIPAccounting")]
    fn default_ipaccounting(&self) -> zbus::Result<bool>;

    /// DefaultLimitAS property
    #[zbus(property, name = "DefaultLimitAS")]
    fn default_limit_as(&self) -> zbus::Result<u64>;

    /// DefaultLimitASSoft property
    #[zbus(property, name = "DefaultLimitASSoft")]
    fn default_limit_assoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitCORE property
    #[zbus(property, name = "DefaultLimitCORE")]
    fn default_limit_core(&self) -> zbus::Result<u64>;

    /// DefaultLimitCORESoft property
    #[zbus(property, name = "DefaultLimitCORESoft")]
    fn default_limit_coresoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitCPU property
    #[zbus(property, name = "DefaultLimitCPU")]
    fn default_limit_cpu(&self) -> zbus::Result<u64>;

    /// DefaultLimitCPUSoft property
    #[zbus(property, name = "DefaultLimitCPUSoft")]
    fn default_limit_cpusoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitDATA property
    #[zbus(property, name = "DefaultLimitDATA")]
    fn default_limit_data(&self) -> zbus::Result<u64>;

    /// DefaultLimitDATASoft property
    #[zbus(property, name = "DefaultLimitDATASoft")]
    fn default_limit_datasoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitFSIZE property
    #[zbus(property, name = "DefaultLimitFSIZE")]
    fn default_limit_fsize(&self) -> zbus::Result<u64>;

    /// DefaultLimitFSIZESoft property
    #[zbus(property, name = "DefaultLimitFSIZESoft")]
    fn default_limit_fsizesoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitLOCKS property
    #[zbus(property, name = "DefaultLimitLOCKS")]
    fn default_limit_locks(&self) -> zbus::Result<u64>;

    /// DefaultLimitLOCKSSoft property
    #[zbus(property, name = "DefaultLimitLOCKSSoft")]
    fn default_limit_lockssoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitMEMLOCK property
    #[zbus(property, name = "DefaultLimitMEMLOCK")]
    fn default_limit_memlock(&self) -> zbus::Result<u64>;

    /// DefaultLimitMEMLOCKSoft property
    #[zbus(property, name = "DefaultLimitMEMLOCKSoft")]
    fn default_limit_memlocksoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitMSGQUEUE property
    #[zbus(property, name = "DefaultLimitMSGQUEUE")]
    fn default_limit_msgqueue(&self) -> zbus::Result<u64>;

    /// DefaultLimitMSGQUEUESoft property
    #[zbus(property, name = "DefaultLimitMSGQUEUESoft")]
    fn default_limit_msgqueuesoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitNICE property
    #[zbus(property, name = "DefaultLimitNICE")]
    fn default_limit_nice(&self) -> zbus::Result<u64>;

    /// DefaultLimitNICESoft property
    #[zbus(property, name = "DefaultLimitNICESoft")]
    fn default_limit_nicesoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitNOFILE property
    #[zbus(property, name = "DefaultLimitNOFILE")]
    fn default_limit_nofile(&self) -> zbus::Result<u64>;

    /// DefaultLimitNOFILESoft property
    #[zbus(property, name = "DefaultLimitNOFILESoft")]
    fn default_limit_nofilesoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitNPROC property
    #[zbus(property, name = "DefaultLimitNPROC")]
    fn default_limit_nproc(&self) -> zbus::Result<u64>;

    /// DefaultLimitNPROCSoft property
    #[zbus(property, name = "DefaultLimitNPROCSoft")]
    fn default_limit_nprocsoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitRSS property
    #[zbus(property, name = "DefaultLimitRSS")]
    fn default_limit_rss(&self) -> zbus::Result<u64>;

    /// DefaultLimitRSSSoft property
    #[zbus(property, name = "DefaultLimitRSSSoft")]
    fn default_limit_rsssoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitRTPRIO property
    #[zbus(property, name = "DefaultLimitRTPRIO")]
    fn default_limit_rtprio(&self) -> zbus::Result<u64>;

    /// DefaultLimitRTPRIOSoft property
    #[zbus(property, name = "DefaultLimitRTPRIOSoft")]
    fn default_limit_rtpriosoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitRTTIME property
    #[zbus(property, name = "DefaultLimitRTTIME")]
    fn default_limit_rttime(&self) -> zbus::Result<u64>;

    /// DefaultLimitRTTIMESoft property
    #[zbus(property, name = "DefaultLimitRTTIMESoft")]
    fn default_limit_rttimesoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitSIGPENDING property
    #[zbus(property, name = "DefaultLimitSIGPENDING")]
    fn default_limit_sigpending(&self) -> zbus::Result<u64>;

    /// DefaultLimitSIGPENDINGSoft property
    #[zbus(property, name = "DefaultLimitSIGPENDINGSoft")]
    fn default_limit_sigpendingsoft(&self) -> zbus::Result<u64>;

    /// DefaultLimitSTACK property
    #[zbus(property, name = "DefaultLimitSTACK")]
    fn default_limit_stack(&self) -> zbus::Result<u64>;

    /// DefaultLimitSTACKSoft property
    #[zbus(property, name = "DefaultLimitSTACKSoft")]
    fn default_limit_stacksoft(&self) -> zbus::Result<u64>;

    /// DefaultMemoryAccounting property
    #[zbus(property)]
    fn default_memory_accounting(&self) -> zbus::Result<bool>;

    /// DefaultMemoryPressureThresholdUSec property
    #[zbus(property, name = "DefaultMemoryPressureThresholdUSec")]
    fn default_memory_pressure_threshold_usec(&self) -> zbus::Result<u64>;

    /// DefaultMemoryPressureWatch property
    #[zbus(property)]
    fn default_memory_pressure_watch(&self) -> zbus::Result<String>;

    /// DefaultOOMPolicy property
    #[zbus(property, name = "DefaultOOMPolicy")]
    fn default_oompolicy(&self) -> zbus::Result<String>;

    /// DefaultOOMScoreAdjust property
    #[zbus(property, name = "DefaultOOMScoreAdjust")]
    fn default_oomscore_adjust(&self) -> zbus::Result<i32>;

    /// DefaultRestartUSec property
    #[zbus(property, name = "DefaultRestartUSec")]
    fn default_restart_usec(&self) -> zbus::Result<u64>;

    /// DefaultStandardError property
    #[zbus(property)]
    fn default_standard_error(&self) -> zbus::Result<String>;

    /// DefaultStandardOutput property
    #[zbus(property)]
    fn default_standard_output(&self) -> zbus::Result<String>;

    /// DefaultStartLimitBurst property
    #[zbus(property)]
    fn default_start_limit_burst(&self) -> zbus::Result<u32>;

    /// DefaultStartLimitIntervalUSec property
    #[zbus(property, name = "DefaultStartLimitIntervalUSec")]
    fn default_start_limit_interval_usec(&self) -> zbus::Result<u64>;

    /// DefaultTasksAccounting property
    #[zbus(property)]
    fn default_tasks_accounting(&self) -> zbus::Result<bool>;

    /// DefaultTasksMax property
    #[zbus(property)]
    fn default_tasks_max(&self) -> zbus::Result<u64>;

    /// DefaultTimeoutAbortUSec property
    #[zbus(property, name = "DefaultTimeoutAbortUSec")]
    fn default_timeout_abort_usec(&self) -> zbus::Result<u64>;

    /// DefaultTimeoutStartUSec property
    #[zbus(property, name = "DefaultTimeoutStartUSec")]
    fn default_timeout_start_usec(&self) -> zbus::Result<u64>;

    /// DefaultTimeoutStopUSec property
    #[zbus(property, name = "DefaultTimeoutStopUSec")]
    fn default_timeout_stop_usec(&self) -> zbus::Result<u64>;

    /// DefaultTimerAccuracyUSec property
    #[zbus(property, name = "DefaultTimerAccuracyUSec")]
    fn default_timer_accuracy_usec(&self) -> zbus::Result<u64>;

    /// Environment property
    #[zbus(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;

    /// ExitCode property
    #[zbus(property)]
    fn exit_code(&self) -> zbus::Result<u8>;

    /// Features property
    #[zbus(property)]
    fn features(&self) -> zbus::Result<String>;

    /// FinishTimestamp property
    #[zbus(property)]
    fn finish_timestamp(&self) -> zbus::Result<u64>;

    /// FinishTimestampMonotonic property
    #[zbus(property)]
    fn finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// FirmwareTimestamp property
    #[zbus(property)]
    fn firmware_timestamp(&self) -> zbus::Result<u64>;

    /// FirmwareTimestampMonotonic property
    #[zbus(property)]
    fn firmware_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// GeneratorsFinishTimestamp property
    #[zbus(property)]
    fn generators_finish_timestamp(&self) -> zbus::Result<u64>;

    /// GeneratorsFinishTimestampMonotonic property
    #[zbus(property)]
    fn generators_finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// GeneratorsStartTimestamp property
    #[zbus(property)]
    fn generators_start_timestamp(&self) -> zbus::Result<u64>;

    /// GeneratorsStartTimestampMonotonic property
    #[zbus(property)]
    fn generators_start_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDGeneratorsFinishTimestamp property
    #[zbus(property, name = "InitRDGeneratorsFinishTimestamp")]
    fn init_rdgenerators_finish_timestamp(&self) -> zbus::Result<u64>;

    /// InitRDGeneratorsFinishTimestampMonotonic property
    #[zbus(property, name = "InitRDGeneratorsFinishTimestampMonotonic")]
    fn init_rdgenerators_finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDGeneratorsStartTimestamp property
    #[zbus(property, name = "InitRDGeneratorsStartTimestamp")]
    fn init_rdgenerators_start_timestamp(&self) -> zbus::Result<u64>;

    /// InitRDGeneratorsStartTimestampMonotonic property
    #[zbus(property, name = "InitRDGeneratorsStartTimestampMonotonic")]
    fn init_rdgenerators_start_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDSecurityFinishTimestamp property
    #[zbus(property, name = "InitRDSecurityFinishTimestamp")]
    fn init_rdsecurity_finish_timestamp(&self) -> zbus::Result<u64>;

    /// InitRDSecurityFinishTimestampMonotonic property
    #[zbus(property, name = "InitRDSecurityFinishTimestampMonotonic")]
    fn init_rdsecurity_finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDSecurityStartTimestamp property
    #[zbus(property, name = "InitRDSecurityStartTimestamp")]
    fn init_rdsecurity_start_timestamp(&self) -> zbus::Result<u64>;

    /// InitRDSecurityStartTimestampMonotonic property
    #[zbus(property, name = "InitRDSecurityStartTimestampMonotonic")]
    fn init_rdsecurity_start_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDTimestamp property
    #[zbus(property, name = "InitRDTimestamp")]
    fn init_rdtimestamp(&self) -> zbus::Result<u64>;

    /// InitRDTimestampMonotonic property
    #[zbus(property, name = "InitRDTimestampMonotonic")]
    fn init_rdtimestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDUnitsLoadFinishTimestamp property
    #[zbus(property, name = "InitRDUnitsLoadFinishTimestamp")]
    fn init_rdunits_load_finish_timestamp(&self) -> zbus::Result<u64>;

    /// InitRDUnitsLoadFinishTimestampMonotonic property
    #[zbus(property, name = "InitRDUnitsLoadFinishTimestampMonotonic")]
    fn init_rdunits_load_finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// InitRDUnitsLoadStartTimestamp property
    #[zbus(property, name = "InitRDUnitsLoadStartTimestamp")]
    fn init_rdunits_load_start_timestamp(&self) -> zbus::Result<u64>;

    /// InitRDUnitsLoadStartTimestampMonotonic property
    #[zbus(property, name = "InitRDUnitsLoadStartTimestampMonotonic")]
    fn init_rdunits_load_start_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// KExecWatchdogUSec property
    #[zbus(property, name = "KExecWatchdogUSec")]
    fn kexec_watchdog_usec(&self) -> zbus::Result<u64>;
    #[zbus(property, name = "KExecWatchdogUSec")]
    fn set_kexec_watchdog_usec(&self, value: u64) -> zbus::Result<()>;

    /// KernelTimestamp property
    #[zbus(property)]
    fn kernel_timestamp(&self) -> zbus::Result<u64>;

    /// KernelTimestampMonotonic property
    #[zbus(property)]
    fn kernel_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// LoaderTimestamp property
    #[zbus(property)]
    fn loader_timestamp(&self) -> zbus::Result<u64>;

    /// LoaderTimestampMonotonic property
    #[zbus(property)]
    fn loader_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// LogLevel property
    #[zbus(property)]
    fn log_level(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_log_level(&self, value: &str) -> zbus::Result<()>;

    /// LogTarget property
    #[zbus(property)]
    fn log_target(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_log_target(&self, value: &str) -> zbus::Result<()>;

    /// NFailedJobs property
    #[zbus(property, name = "NFailedJobs")]
    fn nfailed_jobs(&self) -> zbus::Result<u32>;

    /// NFailedUnits property
    #[zbus(property, name = "NFailedUnits")]
    fn nfailed_units(&self) -> zbus::Result<u32>;

    /// NInstalledJobs property
    #[zbus(property, name = "NInstalledJobs")]
    fn ninstalled_jobs(&self) -> zbus::Result<u32>;

    /// NJobs property
    #[zbus(property, name = "NJobs")]
    fn njobs(&self) -> zbus::Result<u32>;

    /// NNames property
    #[zbus(property, name = "NNames")]
    fn nnames(&self) -> zbus::Result<u32>;

    /// Progress property
    #[zbus(property)]
    fn progress(&self) -> zbus::Result<f64>;

    /// RebootWatchdogUSec property
    #[zbus(property, name = "RebootWatchdogUSec")]
    fn reboot_watchdog_usec(&self) -> zbus::Result<u64>;
    #[zbus(property, name = "RebootWatchdogUSec")]
    fn set_reboot_watchdog_usec(&self, value: u64) -> zbus::Result<()>;

    /// RuntimeWatchdogPreGovernor property
    #[zbus(property)]
    fn runtime_watchdog_pre_governor(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_runtime_watchdog_pre_governor(&self, value: &str) -> zbus::Result<()>;

    /// RuntimeWatchdogPreUSec property
    #[zbus(property, name = "RuntimeWatchdogPreUSec")]
    fn runtime_watchdog_pre_usec(&self) -> zbus::Result<u64>;
    #[zbus(property, name = "RuntimeWatchdogPreUSec")]
    fn set_runtime_watchdog_pre_usec(&self, value: u64) -> zbus::Result<()>;

    /// RuntimeWatchdogUSec property
    #[zbus(property, name = "RuntimeWatchdogUSec")]
    fn runtime_watchdog_usec(&self) -> zbus::Result<u64>;
    #[zbus(property, name = "RuntimeWatchdogUSec")]
    fn set_runtime_watchdog_usec(&self, value: u64) -> zbus::Result<()>;

    /// SecurityFinishTimestamp property
    #[zbus(property)]
    fn security_finish_timestamp(&self) -> zbus::Result<u64>;

    /// SecurityFinishTimestampMonotonic property
    #[zbus(property)]
    fn security_finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// SecurityStartTimestamp property
    #[zbus(property)]
    fn security_start_timestamp(&self) -> zbus::Result<u64>;

    /// SecurityStartTimestampMonotonic property
    #[zbus(property)]
    fn security_start_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// ServiceWatchdogs property
    #[zbus(property)]
    fn service_watchdogs(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_service_watchdogs(&self, value: bool) -> zbus::Result<()>;

    /// ShowStatus property
    #[zbus(property)]
    fn show_status(&self) -> zbus::Result<bool>;

    /// SystemState property
    #[zbus(property)]
    fn system_state(&self) -> zbus::Result<String>;

    /// Tainted property
    #[zbus(property)]
    fn tainted(&self) -> zbus::Result<String>;

    /// TimerSlackNSec property
    #[zbus(property, name = "TimerSlackNSec")]
    fn timer_slack_nsec(&self) -> zbus::Result<u64>;

    /// UnitPath property
    #[zbus(property)]
    fn unit_path(&self) -> zbus::Result<Vec<String>>;

    /// UnitsLoadFinishTimestamp property
    #[zbus(property)]
    fn units_load_finish_timestamp(&self) -> zbus::Result<u64>;

    /// UnitsLoadFinishTimestampMonotonic property
    #[zbus(property)]
    fn units_load_finish_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// UnitsLoadStartTimestamp property
    #[zbus(property)]
    fn units_load_start_timestamp(&self) -> zbus::Result<u64>;

    /// UnitsLoadStartTimestampMonotonic property
    #[zbus(property)]
    fn units_load_start_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// UnitsLoadTimestamp property
    #[zbus(property)]
    fn units_load_timestamp(&self) -> zbus::Result<u64>;

    /// UnitsLoadTimestampMonotonic property
    #[zbus(property)]
    fn units_load_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// UserspaceTimestamp property
    #[zbus(property)]
    fn userspace_timestamp(&self) -> zbus::Result<u64>;

    /// UserspaceTimestampMonotonic property
    #[zbus(property)]
    fn userspace_timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// Version property
    #[zbus(property)]
    fn version(&self) -> zbus::Result<String>;

    /// Virtualization property
    #[zbus(property)]
    fn virtualization(&self) -> zbus::Result<String>;

    /// WatchdogDevice property
    #[zbus(property)]
    fn watchdog_device(&self) -> zbus::Result<String>;

    /// WatchdogLastPingTimestamp property
    #[zbus(property)]
    fn watchdog_last_ping_timestamp(&self) -> zbus::Result<u64>;

    /// WatchdogLastPingTimestampMonotonic property
    #[zbus(property)]
    fn watchdog_last_ping_timestamp_monotonic(&self) -> zbus::Result<u64>;
}
