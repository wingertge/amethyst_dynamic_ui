initSidebarItems({"enum":[["AssetPrefab","Convenience `PrefabData` for loading assets of type `A` using `Format` `F`."],["Completion","Completion status, returned by `ProgressCounter::complete`."],["ProcessingState","Returned by processor systems, describes the loading state of the asset."]],"macro":[["register_format","Register a dynamically deserializable format for given asset data type. Note that provided asset data type must also be registered using `register_format_type` macro."],["register_format_type","Register specific asset data types that can be deserialized with dynamic formats. This is very useful for all assets that have any format types explicitly implemented. Registered assets are used during loading of nested assets to determine format type which will be used to deserialize that asset."]],"struct":[["AssetLoaderSystemData","Helper type for loading assets"],["AssetStorage","An asset storage, storing the actual assets and allocating handles to them."],["Cache","A simple cache for asset handles of type `A`. This stores `WeakHandle`, so it doesn’t keep the assets alive."],["Directory","Directory source."],["FormatValue","The `Ok` return value of `Format::import` for a given asset type `A`."],["Handle","A handle to an asset. This is usually what the user deals with, the actual asset (`A`) is stored in an `AssetStorage`."],["HotReloadBundle","This bundle activates hot reload for the `Loader`, adds a `HotReloadStrategy` and the `HotReloadSystem`."],["HotReloadStrategy","An ECS resource which allows to configure hot reloading."],["HotReloadSystem","System for updating `HotReloadStrategy`."],["Loader","The asset loader, holding the sources and a reference to the `ThreadPool`."],["Prefab","Main `Prefab` structure, containing all data loaded in a single prefab."],["PrefabLoader","Helper structure for loading prefabs."],["PrefabLoaderSystem","System that load `Prefab`s for `PrefabData` `T`."],["PrefabLoaderSystemDesc","Builds a `PrefabLoaderSystem`."],["Processor","A default implementation for an asset processing system which converts data to assets and maintains the asset storage for `A`."],["ProgressCounter","A progress tracker which is passed to the `Loader` in order to check how many assets are loaded."],["RonFormat","Format for loading from RON files. Mostly useful for prefabs. This type cannot be used for tagged deserialization. It is meant to be used at top-level loading, manually specified to the loader."],["SingleFile","An implementation of `Reload` which just stores the modification time and the path of the file."],["ThreadPool","Represents a user created thread-pool."],["WeakHandle","A weak handle, which is useful if you don’t directly need the asset like in caches. This way, the asset can still get dropped (if you want that)."]],"trait":[["Asset","One of the three core traits of this crate."],["Format","A format, providing a conversion from bytes to asset data, which is then in turn accepted by `Asset::from_data`. Examples for formats are `Png`, `Obj` and `Wave`."],["FormatRegisteredData","A trait for all asset types that have their format types. Use this as a bound for asset data types when used inside boxed format types intended for deserialization. registered with `register_format_type` macro."],["PrefabData","Trait for loading a prefabs data for a single entity"],["ProcessableAsset","Defines a way to process asset’s data into the asset. This allows using default `Processor` system to process assets that implement that type."],["Progress","The `Progress` trait, allowing to track which assets are imported already."],["Reload","The `Reload` trait provides a method which checks if an asset needs to be reloaded."],["SerializableFormat","SerializableFormat is a marker trait which is required for Format types that are supposed to be serialized. This trait implies both `Serialize` and `Deserialize` implementation."],["Source","A trait for asset sources, which provides methods for loading bytes."],["Tracker","The `Tracker` trait which will be used by the loader to report back to `Progress`."]]});