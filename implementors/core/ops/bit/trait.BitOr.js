(function() {var implementors = {};
implementors["andrew"] = [{"text":"impl BitOr&lt;Sides&gt; for Sides","synthetic":false,"types":[]}];
implementors["cassowary"] = [{"text":"impl BitOr&lt;WeightedRelation&gt; for f64","synthetic":false,"types":[]},{"text":"impl BitOr&lt;WeightedRelation&gt; for f32","synthetic":false,"types":[]},{"text":"impl BitOr&lt;WeightedRelation&gt; for Variable","synthetic":false,"types":[]},{"text":"impl BitOr&lt;WeightedRelation&gt; for Term","synthetic":false,"types":[]},{"text":"impl BitOr&lt;WeightedRelation&gt; for Expression","synthetic":false,"types":[]},{"text":"impl BitOr&lt;f64&gt; for PartialConstraint","synthetic":false,"types":[]},{"text":"impl BitOr&lt;f32&gt; for PartialConstraint","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Variable&gt; for PartialConstraint","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Term&gt; for PartialConstraint","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Expression&gt; for PartialConstraint","synthetic":false,"types":[]}];
implementors["gfx_hal"] = [{"text":"impl BitOr&lt;Usage&gt; for Usage","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Access&gt; for Access","synthetic":false,"types":[]},{"text":"impl BitOr&lt;CommandBufferFlags&gt; for CommandBufferFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Aspects&gt; for Aspects","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ImageFeature&gt; for ImageFeature","synthetic":false,"types":[]},{"text":"impl BitOr&lt;BufferFeature&gt; for BufferFeature","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ViewCapabilities&gt; for ViewCapabilities","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Usage&gt; for Usage","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Access&gt; for Access","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Properties&gt; for Properties","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Dependencies&gt; for Dependencies","synthetic":false,"types":[]},{"text":"impl BitOr&lt;CommandPoolCreateFlags&gt; for CommandPoolCreateFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;DescriptorPoolCreateFlags&gt; for DescriptorPoolCreateFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ColorMask&gt; for ColorMask","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Face&gt; for Face","synthetic":false,"types":[]},{"text":"impl BitOr&lt;PipelineStage&gt; for PipelineStage","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ShaderStageFlags&gt; for ShaderStageFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;PipelineCreationFlags&gt; for PipelineCreationFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ResultFlags&gt; for ResultFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;PipelineStatistic&gt; for PipelineStatistic","synthetic":false,"types":[]},{"text":"impl BitOr&lt;CompositeAlpha&gt; for CompositeAlpha","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Features&gt; for Features","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;T, S, '_, '_&gt; BitOr&lt;&amp;'_ HashSet&lt;T, S&gt;&gt; for &amp;'_ HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hibitset"] = [{"text":"impl&lt;T&gt; BitOr&lt;T&gt; for BitSet <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; BitOr&lt;T&gt; for &amp;'a BitSet <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; BitOr&lt;T&gt; for AtomicBitSet <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; BitOr&lt;T&gt; for &amp;'a AtomicBitSet <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, T&gt; BitOr&lt;T&gt; for BitSetNot&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, T&gt; BitOr&lt;T&gt; for &amp;'a BitSetNot&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, T&gt; BitOr&lt;T&gt; for BitSetAnd&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, B, T&gt; BitOr&lt;T&gt; for &amp;'a BitSetAnd&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, T&gt; BitOr&lt;T&gt; for BitSetOr&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, B, T&gt; BitOr&lt;T&gt; for &amp;'a BitSetOr&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A, B, T&gt; BitOr&lt;T&gt; for BitSetXor&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, B, T&gt; BitOr&lt;T&gt; for &amp;'a BitSetXor&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: BitSetLike,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; BitOr&lt;T&gt; for BitSetAll <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; BitOr&lt;T&gt; for &amp;'a BitSetAll <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitSetLike,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl BitOr&lt;AtFlags&gt; for AtFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;OFlag&gt; for OFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SealFlag&gt; for SealFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SpliceFFlags&gt; for SpliceFFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;FallocateFlags&gt; for FallocateFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ModuleInitFlags&gt; for ModuleInitFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;DeleteModuleFlags&gt; for DeleteModuleFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MntFlags&gt; for MntFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MQ_OFlag&gt; for MQ_OFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;InterfaceFlags&gt; for InterfaceFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;PollFlags&gt; for PollFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;CloneFlags&gt; for CloneFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;EpollFlags&gt; for EpollFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;EpollCreateFlags&gt; for EpollCreateFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;EfdFlags&gt; for EfdFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MemFdCreateFlag&gt; for MemFdCreateFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ProtFlags&gt; for ProtFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MapFlags&gt; for MapFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MlockAllFlags&gt; for MlockAllFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Options&gt; for Options","synthetic":false,"types":[]},{"text":"impl BitOr&lt;QuotaValidFlags&gt; for QuotaValidFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SaFlags&gt; for SaFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SfdFlags&gt; for SfdFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SockFlag&gt; for SockFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;MsgFlags&gt; for MsgFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SFlag&gt; for SFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Mode&gt; for Mode","synthetic":false,"types":[]},{"text":"impl BitOr&lt;FsFlags&gt; for FsFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;InputFlags&gt; for InputFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;OutputFlags&gt; for OutputFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;LocalFlags&gt; for LocalFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;WaitPidFlag&gt; for WaitPidFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;AddWatchFlags&gt; for AddWatchFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;InitFlags&gt; for InitFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;AccessFlags&gt; for AccessFlags","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl BitOr&lt;BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BitOr&lt;BigInt&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; BitOr&lt;&amp;'b BigInt&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BitOr&lt;&amp;'a BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl BitOr&lt;BigUint&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BitOr&lt;BigUint&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; BitOr&lt;&amp;'b BigUint&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; BitOr&lt;&amp;'a BigUint&gt; for BigUint","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl BitOr&lt;Transformations&gt; for Transformations","synthetic":false,"types":[]}];
implementors["ron"] = [{"text":"impl BitOr&lt;Extensions&gt; for Extensions","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl BitOr&lt;B0&gt; for B0","synthetic":false,"types":[]},{"text":"impl BitOr&lt;B1&gt; for B0","synthetic":false,"types":[]},{"text":"impl&lt;Rhs:&nbsp;Bit&gt; BitOr&lt;Rhs&gt; for B1","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned&gt; BitOr&lt;U&gt; for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;B:&nbsp;Bit, U:&nbsp;Unsigned&gt; BitOr&lt;UTerm&gt; for UInt&lt;U, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned, Ur:&nbsp;Unsigned&gt; BitOr&lt;UInt&lt;Ur, B0&gt;&gt; for UInt&lt;Ul, B0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: BitOr&lt;Ur&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned, Ur:&nbsp;Unsigned&gt; BitOr&lt;UInt&lt;Ur, B1&gt;&gt; for UInt&lt;Ul, B0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: BitOr&lt;Ur&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned, Ur:&nbsp;Unsigned&gt; BitOr&lt;UInt&lt;Ur, B0&gt;&gt; for UInt&lt;Ul, B1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: BitOr&lt;Ur&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned, Ur:&nbsp;Unsigned&gt; BitOr&lt;UInt&lt;Ur, B1&gt;&gt; for UInt&lt;Ul, B1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: BitOr&lt;Ur&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["wayland_client"] = [{"text":"impl BitOr&lt;DndAction&gt; for DndAction","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Resize&gt; for Resize","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Transient&gt; for Transient","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Capability&gt; for Capability","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Mode&gt; for Mode","synthetic":false,"types":[]}];
implementors["wayland_protocols"] = [{"text":"impl BitOr&lt;ContentHint&gt; for ContentHint","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Anchor&gt; for Anchor","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Gravity&gt; for Gravity","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ConstraintAdjustment&gt; for ConstraintAdjustment","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Anchor&gt; for Anchor","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Flags&gt; for Flags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ConstraintAdjustment&gt; for ConstraintAdjustment","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()