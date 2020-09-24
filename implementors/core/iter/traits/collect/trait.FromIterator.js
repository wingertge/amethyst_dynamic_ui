(function() {var implementors = {};
implementors["andrew"] = [{"text":"impl FromIterator&lt;Sides&gt; for Sides","synthetic":false,"types":[]}];
implementors["arrayvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl FromIterator&lt;u8&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;u8&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a u8&gt; for BytesMut","synthetic":false,"types":[]}];
implementors["crossbeam_deque"] = [{"text":"impl&lt;T&gt; FromIterator&lt;Steal&lt;T&gt;&gt; for Steal&lt;T&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; FromIterator&lt;T&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["gfx_hal"] = [{"text":"impl FromIterator&lt;Usage&gt; for Usage","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Access&gt; for Access","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;CommandBufferFlags&gt; for CommandBufferFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Aspects&gt; for Aspects","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ImageFeature&gt; for ImageFeature","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;BufferFeature&gt; for BufferFeature","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ViewCapabilities&gt; for ViewCapabilities","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Usage&gt; for Usage","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Access&gt; for Access","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Properties&gt; for Properties","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Dependencies&gt; for Dependencies","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;CommandPoolCreateFlags&gt; for CommandPoolCreateFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;DescriptorPoolCreateFlags&gt; for DescriptorPoolCreateFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ColorMask&gt; for ColorMask","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Face&gt; for Face","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;PipelineStage&gt; for PipelineStage","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ShaderStageFlags&gt; for ShaderStageFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;PipelineCreationFlags&gt; for PipelineCreationFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ResultFlags&gt; for ResultFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;PipelineStatistic&gt; for PipelineStatistic","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;CompositeAlpha&gt; for CompositeAlpha","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Features&gt; for Features","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;K, V, S&gt; FromIterator&lt;(K, V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FromIterator&lt;T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hibitset"] = [{"text":"impl FromIterator&lt;u32&gt; for BitSet","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a u32&gt; for BitSet","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;u32&gt; for AtomicBitSet","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a u32&gt; for AtomicBitSet","synthetic":false,"types":[]}];
implementors["linked_hash_map"] = [{"text":"impl&lt;K:&nbsp;Hash + Eq, V, S:&nbsp;BuildHasher + Default&gt; FromIterator&lt;(K, V)&gt; for LinkedHashMap&lt;K, V, S&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl FromIterator&lt;AtFlags&gt; for AtFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;OFlag&gt; for OFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SealFlag&gt; for SealFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SpliceFFlags&gt; for SpliceFFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FallocateFlags&gt; for FallocateFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ModuleInitFlags&gt; for ModuleInitFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;DeleteModuleFlags&gt; for DeleteModuleFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MntFlags&gt; for MntFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MQ_OFlag&gt; for MQ_OFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;InterfaceFlags&gt; for InterfaceFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;PollFlags&gt; for PollFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;CloneFlags&gt; for CloneFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;EpollFlags&gt; for EpollFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;EpollCreateFlags&gt; for EpollCreateFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;EfdFlags&gt; for EfdFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MemFdCreateFlag&gt; for MemFdCreateFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ProtFlags&gt; for ProtFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MapFlags&gt; for MapFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MlockAllFlags&gt; for MlockAllFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Options&gt; for Options","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;QuotaValidFlags&gt; for QuotaValidFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SaFlags&gt; for SaFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SfdFlags&gt; for SfdFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SockFlag&gt; for SockFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MsgFlags&gt; for MsgFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SFlag&gt; for SFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Mode&gt; for Mode","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FsFlags&gt; for FsFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;InputFlags&gt; for InputFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;OutputFlags&gt; for OutputFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;LocalFlags&gt; for LocalFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;WaitPidFlag&gt; for WaitPidFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;AddWatchFlags&gt; for AddWatchFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;InitFlags&gt; for InitFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;AccessFlags&gt; for AccessFlags","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl FromIterator&lt;Transformations&gt; for Transformations","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl FromIterator&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["ron"] = [{"text":"impl FromIterator&lt;(Value, Value)&gt; for Map","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Extensions&gt; for Extensions","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl FromIterator&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;Value&gt;&gt; FromIterator&lt;T&gt; for Value","synthetic":false,"types":[]}];
implementors["slice_deque"] = [{"text":"impl&lt;T&gt; FromIterator&lt;T&gt; for SliceDeque&lt;T&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["specs"] = [{"text":"impl&lt;T&gt; FromIterator&lt;(Entity, T)&gt; for ChangeSet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AddAssign,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; FromIterator&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; FromIterator&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array + Default&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array + Default&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["toml"] = [{"text":"impl FromIterator&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]}];
implementors["wayland_client"] = [{"text":"impl FromIterator&lt;DndAction&gt; for DndAction","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Resize&gt; for Resize","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Transient&gt; for Transient","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Capability&gt; for Capability","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Mode&gt; for Mode","synthetic":false,"types":[]}];
implementors["wayland_protocols"] = [{"text":"impl FromIterator&lt;ContentHint&gt; for ContentHint","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Anchor&gt; for Anchor","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Gravity&gt; for Gravity","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ConstraintAdjustment&gt; for ConstraintAdjustment","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Anchor&gt; for Anchor","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Flags&gt; for Flags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ConstraintAdjustment&gt; for ConstraintAdjustment","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()