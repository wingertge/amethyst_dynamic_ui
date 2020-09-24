initSidebarItems({"fn":[["cast_cow","Cast `cow` of some arbitrary type into `cow` of bytes. Can lead to UB if allocator changes. Use with caution. TODO: Replace with something safer."],["cast_slice","Cast slice of some arbitrary type into slice of bytes."],["cast_vec","Cast vec of some arbitrary type into vec of bytes. Can lead to UB if allocator changes. Use with caution. TODO: Replace with something safer."],["identical_cast","Casts identical types. Useful in generic environment where caller knows that two types are the same but Rust is not convinced."]],"macro":[["device_owned","Implement ownership checking for value with `device: DeviceId` field."],["instance_owned","Implement ownership checking for value with `instance: InstanceId` field."],["rendy_backend_match","Execute arm with matching backend. If particular backend is disabled then its arm is stripped from compilation altogether."],["rendy_slow_assert","`assert!` that is exists only if `\"no-slow-safety-checks\"` feature is not enabled."],["rendy_slow_assert_eq","`assert_eq!` that is exists only if `\"no-slow-safety-checks\"` feature is not enabled."],["rendy_slow_assert_ne","`assert_ne!` that is exists only if `\"no-slow-safety-checks\"` feature is not enabled."],["rendy_with_dx12_backend","Resolve into input AST if dx12 backend is enabled."],["rendy_with_empty_backend","Resolve into input AST if empty backend is enabled."],["rendy_with_metal_backend","Resolve into input AST if metal backend is enabled."],["rendy_with_slow_safety_checks","Resolve into input AST if rendy is requested to perform slow safety checks."],["rendy_with_vulkan_backend","Resolve into input AST if vulkan backend is enabled."],["rendy_without_dx12_backend","Resolve into input AST if dx12 backend is disabled."],["rendy_without_empty_backend","Resolve into input AST if empty backend is disabled."],["rendy_without_metal_backend","Resolve into input AST if metal backend is disabled."],["rendy_without_slow_safety_checks","Resolve into input AST if rendy is requested to not perform slow safety checks."],["rendy_without_vulkan_backend","Resolve into input AST if vulkan backend is disabled."]],"mod":[["empty","Dummy backend implementation to test the code for compile errors outside of the graphics development environment."],["types","Types shared across rendy"]],"struct":[["Device","Raw device wrapper with id."],["DeviceId","Id of the instance."],["Instance","Raw instance wrapper with id."],["InstanceId","Id of the hal instance."]]});