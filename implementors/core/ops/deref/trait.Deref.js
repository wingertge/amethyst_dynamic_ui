(function() {var implementors = {};
implementors["amethyst_dynamic_ui"] = [{"text":"impl Deref for Fonts","synthetic":false,"types":[]},{"text":"impl Deref for Textures","synthetic":false,"types":[]},{"text":"impl Deref for LAYOUT","synthetic":false,"types":[]},{"text":"impl Deref for UiCachedImage","synthetic":false,"types":[]},{"text":"impl Deref for LayoutIdentifier","synthetic":false,"types":[]}];
implementors["amethyst_fmod"] = [{"text":"impl Deref for AudioSystem","synthetic":false,"types":[]}];
implementors["arrayvec"] = [{"text":"impl&lt;A:&nbsp;Array&lt;Item = u8&gt;&gt; Deref for ArrayString&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Deref for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl Deref for Bytes","synthetic":false,"types":[]},{"text":"impl Deref for BytesMut","synthetic":false,"types":[]}];
implementors["colored"] = [{"text":"impl Deref for SHOULD_COLORIZE","synthetic":false,"types":[]},{"text":"impl Deref for ColoredString","synthetic":false,"types":[]}];
implementors["cpal"] = [{"text":"impl&lt;'a, T&gt; Deref for InputBuffer&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Deref for OutputBuffer&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sample,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; Deref for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;T&gt; Deref for CachePadded&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Deref for ShardedLockReadGuard&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Deref for ShardedLockWriteGuard&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Deref for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Deref,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Deref&lt;Target = L::Target&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["euclid"] = [{"text":"impl&lt;T&gt; Deref for NonEmpty&lt;T&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; Deref for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["gfx_hal"] = [{"text":"impl&lt;'a, B:&nbsp;Backend&gt; Deref for RenderPassInlineEncoder&lt;'a, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;B:&nbsp;Backend, S:&nbsp;Shot&gt; Deref for SubpassCommandBuffer&lt;B, S&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, B:&nbsp;Backend, T:&nbsp;'a&gt; Deref for Reader&lt;'a, B, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, B:&nbsp;Backend, T:&nbsp;'a&gt; Deref for Writer&lt;'a, B, T&gt;","synthetic":false,"types":[]}];
implementors["gimli"] = [{"text":"impl&lt;'input, Endian&gt; Deref for EndianSlice&lt;'input, Endian&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Endian: Endianity,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["gltf"] = [{"text":"impl Deref for Data","synthetic":false,"types":[]},{"text":"impl Deref for Gltf","synthetic":false,"types":[]}];
implementors["humantime"] = [{"text":"impl Deref for Duration","synthetic":false,"types":[]},{"text":"impl Deref for Timestamp","synthetic":false,"types":[]}];
implementors["image"] = [{"text":"impl&lt;P, Container&gt; Deref for ImageBuffer&lt;P, Container&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Pixel + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::Subpixel: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Container: Deref&lt;Target = [P::Subpixel]&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libloading"] = [{"text":"impl&lt;T&gt; Deref for Symbol&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'lib, T&gt; Deref for Symbol&lt;'lib, T&gt;","synthetic":false,"types":[]}];
implementors["lock_api"] = [{"text":"impl&lt;'a, R:&nbsp;RawMutex + 'a, T:&nbsp;?Sized + 'a&gt; Deref for MutexGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawMutex + 'a, T:&nbsp;?Sized + 'a&gt; Deref for MappedMutexGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawMutex + 'a, G:&nbsp;GetThreadId + 'a, T:&nbsp;?Sized + 'a&gt; Deref for ReentrantMutexGuard&lt;'a, R, G, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawMutex + 'a, G:&nbsp;GetThreadId + 'a, T:&nbsp;?Sized + 'a&gt; Deref for MappedReentrantMutexGuard&lt;'a, R, G, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawRwLock + 'a, T:&nbsp;?Sized + 'a&gt; Deref for RwLockReadGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawRwLock + 'a, T:&nbsp;?Sized + 'a&gt; Deref for RwLockWriteGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawRwLockUpgrade + 'a, T:&nbsp;?Sized + 'a&gt; Deref for RwLockUpgradableReadGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawRwLock + 'a, T:&nbsp;?Sized + 'a&gt; Deref for MappedRwLockReadGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, R:&nbsp;RawRwLock + 'a, T:&nbsp;?Sized + 'a&gt; Deref for MappedRwLockWriteGuard&lt;'a, R, T&gt;","synthetic":false,"types":[]}];
implementors["memmap"] = [{"text":"impl Deref for Mmap","synthetic":false,"types":[]},{"text":"impl Deref for MmapMut","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U1, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U2, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U3, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U4, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U5, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U6, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U1, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U1, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U1, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U1, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U1, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U1, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U2, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U2, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U2, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U2, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U2, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U2, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U3, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U3, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U3, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U3, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U3, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U3, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U4, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U4, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U4, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U4, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U4, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U4, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U5, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U5, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U5, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U5, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U5, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U5, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U6, U2, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U6, U3, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U6, U4, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U6, U5, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar, S&gt; Deref for Matrix&lt;N, U6, U6, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: ContiguousStorage&lt;N, U6, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, R, C&gt; Deref for ArrayStorage&lt;N, R, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: DimName,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: DimName,<br>&nbsp;&nbsp;&nbsp;&nbsp;R::Value: Mul&lt;C::Value&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Prod&lt;R::Value, C::Value&gt;: ArrayLength&lt;N&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Deref for Unit&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Point&lt;N, U1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Point&lt;N, U2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Point&lt;N, U3&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Point&lt;N, U4&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Point&lt;N, U5&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Point&lt;N, U6&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; Deref for Quaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Translation&lt;N, U1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Translation&lt;N, U2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Translation&lt;N, U3&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U3&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Translation&lt;N, U4&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U4&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Translation&lt;N, U5&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U5&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar&gt; Deref for Translation&lt;N, U6&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U6&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nodrop"] = [{"text":"impl&lt;T&gt; Deref for NoDrop&lt;T&gt;","synthetic":false,"types":[]}];
implementors["ordered_float"] = [{"text":"impl&lt;T:&nbsp;Float&gt; Deref for OrderedFloat&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Float&gt; Deref for NotNan&lt;T&gt;","synthetic":false,"types":[]}];
implementors["palette"] = [{"text":"impl&lt;C, T:&nbsp;Float&gt; Deref for PreAlpha&lt;C, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;C, T&gt; Deref for Alpha&lt;C, T&gt;","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl Deref for Literal","synthetic":false,"types":[]}];
implementors["rendy_command"] = [{"text":"impl&lt;'a, B&gt; Deref for RenderPassEncoder&lt;'a, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Backend,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, B&gt; Deref for RenderPassInlineEncoder&lt;'a, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Backend,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, B, C, L&gt; Deref for Encoder&lt;'a, B, C, L&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Backend,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["rendy_descriptor"] = [{"text":"impl&lt;B&gt; Deref for DescriptorSet&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Backend,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["rendy_factory"] = [{"text":"impl&lt;B&gt; Deref for Factory&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Backend,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["rendy_resource"] = [{"text":"impl&lt;T&gt; Deref for Escape&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Deref for Handle&lt;T&gt;","synthetic":false,"types":[]}];
implementors["rendy_util"] = [{"text":"impl&lt;B&gt; Deref for Device&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Backend,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["rental"] = [{"text":"impl Deref for SimpleRefDeref","synthetic":false,"types":[]},{"text":"impl Deref for SimpleMutDeref","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref, T:&nbsp;'static&gt; Deref for RentRef&lt;H, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref + DerefMut, T:&nbsp;'static&gt; Deref for RentMut&lt;H, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref, T:&nbsp;'static&gt; Deref for RentRefCell&lt;H, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref + DerefMut, T:&nbsp;'static&gt; Deref for RentRefCellMut&lt;H, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref + DerefMut, T:&nbsp;'static&gt; Deref for RentMutex&lt;H, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref, T:&nbsp;'static&gt; Deref for RentRwLock&lt;H, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;H:&nbsp;'static + StableDeref, T:&nbsp;'static&gt; Deref for RentRwLockMut&lt;H, T&gt;","synthetic":false,"types":[]}];
implementors["rusttype"] = [{"text":"impl&lt;'a&gt; Deref for SharedBytes&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["scopeguard"] = [{"text":"impl&lt;T, F, S&gt; Deref for ScopeGuard&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnOnce(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Strategy,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["serde_bytes"] = [{"text":"impl Deref for Bytes","synthetic":false,"types":[]},{"text":"impl Deref for ByteBuf","synthetic":false,"types":[]}];
implementors["shred"] = [{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Deref for Ref&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Deref for RefMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b, T:&nbsp;?Sized&gt; Deref for AccessorCow&lt;'a, 'b, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as System&lt;'a&gt;&gt;::SystemData as DynamicSystemData&lt;'a&gt;&gt;::Accessor: 'b,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: System&lt;'a&gt; + 'b,<br>&nbsp;&nbsp;&nbsp;&nbsp;'a: 'b,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, F&gt; Deref for Read&lt;'a, T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Resource,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, F&gt; Deref for Write&lt;'a, T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Resource,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Deref for Fetch&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Resource,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Deref for FetchMut&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Resource,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["slice_deque"] = [{"text":"impl&lt;T&gt; Deref for SliceDeque&lt;T&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Deref for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["smithay_client_toolkit"] = [{"text":"impl Deref for ThemedPointer","synthetic":false,"types":[]},{"text":"impl Deref for AutoPointer","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Deref for Underscore","synthetic":false,"types":[]},{"text":"impl Deref for Add","synthetic":false,"types":[]},{"text":"impl Deref for And","synthetic":false,"types":[]},{"text":"impl Deref for At","synthetic":false,"types":[]},{"text":"impl Deref for Bang","synthetic":false,"types":[]},{"text":"impl Deref for Caret","synthetic":false,"types":[]},{"text":"impl Deref for Colon","synthetic":false,"types":[]},{"text":"impl Deref for Comma","synthetic":false,"types":[]},{"text":"impl Deref for Div","synthetic":false,"types":[]},{"text":"impl Deref for Dollar","synthetic":false,"types":[]},{"text":"impl Deref for Dot","synthetic":false,"types":[]},{"text":"impl Deref for Eq","synthetic":false,"types":[]},{"text":"impl Deref for Gt","synthetic":false,"types":[]},{"text":"impl Deref for Lt","synthetic":false,"types":[]},{"text":"impl Deref for Or","synthetic":false,"types":[]},{"text":"impl Deref for Pound","synthetic":false,"types":[]},{"text":"impl Deref for Question","synthetic":false,"types":[]},{"text":"impl Deref for Rem","synthetic":false,"types":[]},{"text":"impl Deref for Semi","synthetic":false,"types":[]},{"text":"impl Deref for Star","synthetic":false,"types":[]},{"text":"impl Deref for Sub","synthetic":false,"types":[]},{"text":"impl Deref for Tilde","synthetic":false,"types":[]},{"text":"impl&lt;'c, 'a&gt; Deref for StepCursor&lt;'c, 'a&gt;","synthetic":false,"types":[]}];
implementors["tinystr"] = [{"text":"impl Deref for TinyStr16","synthetic":false,"types":[]},{"text":"impl Deref for TinyStr4","synthetic":false,"types":[]},{"text":"impl Deref for TinyStr8","synthetic":false,"types":[]},{"text":"impl Deref for TinyStrAuto","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Deref for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Deref for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["wayland_client"] = [{"text":"impl Deref for Display","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Deref for CursorImageBuffer&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["wayland_sys"] = [{"text":"impl Deref for WAYLAND_CLIENT_OPTION","synthetic":false,"types":[]},{"text":"impl Deref for WAYLAND_CLIENT_HANDLE","synthetic":false,"types":[]},{"text":"impl Deref for WAYLAND_EGL_OPTION","synthetic":false,"types":[]},{"text":"impl Deref for WAYLAND_EGL_HANDLE","synthetic":false,"types":[]},{"text":"impl Deref for WAYLAND_CURSOR_OPTION","synthetic":false,"types":[]},{"text":"impl Deref for WAYLAND_CURSOR_HANDLE","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()