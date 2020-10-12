initSidebarItems({"derive":[["SystemData",""]],"enum":[["AccessorCow","Either an `Accessor` of the system `T` or a reference to it."],["RunningTime",""]],"macro":[["par","The `par!` macro may be used to easily create a structure which runs things in parallel."],["seq","The `seq!` macro may be used to easily create a structure which runs things sequentially."]],"mod":[["cell","Helper module for some internals, most users don't need to interact with it."]],"struct":[["AsyncDispatcher","Like, `Dispatcher` but works asynchronously."],["BatchAccessor","The `BatchAccessor` is used to notify the main dispatcher of the read and write resources of the `System`s contained in the batch (\"sub systems\")."],["BatchUncheckedWorld","The `BatchUncheckedWorld` wraps an instance of the world. You have to specify this as `SystemData` for a `System` implementing `BatchController`."],["DefaultBatchControllerSystem","The `DefaultBatchControllerSystem` is a simple implementation that will dispatch the inner dispatcher one time."],["DefaultProvider","A `SetupHandler` that simply uses the default implementation."],["Dispatcher","The dispatcher struct, allowing systems to be executed in parallel."],["DispatcherBuilder","Builder for the `Dispatcher`."],["Entry","An entry to a resource of the `World` struct. This is similar to the Entry API found in the standard library."],["Fetch","Allows to fetch a resource in a system immutably."],["FetchMut","Allows to fetch a resource in a system mutably."],["MetaIter","An iterator for the `MetaTable`."],["MetaIterMut","A mutable iterator for the `MetaTable`."],["MetaTable","The `MetaTable` which allows to store object-safe trait implementations for resources."],["PanicHandler","A setup handler that simply does nothing and thus will cause a panic on fetching."],["Par","Runs two tasks in parallel. These two tasks are called `head` and `tail` in the following documentation."],["ParSeq","A dispatcher intended to be used with `Par` and `Seq`  structures."],["Read","Allows to fetch a resource in a system immutably."],["ResourceId","The id of a `Resource`, which simply wraps a type id and a \"dynamic ID\". The \"dynamic ID\" is usually just left `0`, and, unless such documentation says otherwise, other libraries will assume that it is always `0`; non-zero IDs are only used for special resource types that are specifically defined in a more dynamic way, such that resource types can essentially be created at run time, without having different static types."],["Seq","Runs two tasks sequentially. These two tasks are called `head` and `tail` in the following documentation."],["StaticAccessor","The static accessor that is used for `SystemData`."],["World","A [Resource] container, which provides methods to insert, access and manage the contained resources."],["Write","Allows to fetch a resource in a system mutably."]],"trait":[["Accessor","A trait for accessing read/write multiple resources from a system. This can be used to create dynamic systems that don't specify what they fetch at compile-time."],["BatchController","The `BatchController` is the additional trait that a normal System must implement in order to be used as a system controlling the execution of a batch."],["CastFrom","Helper trait for the `MetaTable`. This trait is required to be implemented for a trait to be compatible with the meta table."],["DynamicSystemData","A struct implementing system data indicates that it bundles some resources which are required for the execution."],["Resource","A resource is a data slot which lives in the `World` can only be accessed according to Rust's typical borrowing model (one writer xor multiple readers)."],["RunNow","Trait for fetching data and running systems. Automatically implemented for systems."],["RunWithPool","Similar to `RunNow` except additionally taking in a rayon::ThreadPool for parallelism."],["SetupHandler","A setup handler performing the fetching of `T`."],["System","A `System`, executed with a set of required `Resource`s."],["SystemData","A static system data that can specify its dependencies at statically (at compile-time). Most system data is a `SystemData`, the `DynamicSystemData` type is only needed for very special setups."]],"type":[["ReadExpect","Allows to fetch a resource in a system immutably. This will panic if the resource does not exist. Usage of `Read` or `Option<Read>` is therefore recommended."],["Resources","Alias for `World` for easier migration to the new version. Will be removed in the future."],["WriteExpect","Allows to fetch a resource in a system mutably. This will panic if the resource does not exist. Usage of `Write` or `Option<Write>` is therefore recommended."]]});