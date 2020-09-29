(function() {var implementors = {};
implementors["amethyst_assets"] = [{"text":"impl&lt;'a, A&gt; SystemData&lt;'a&gt; for AssetLoaderSystemData&lt;'a, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Asset,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;A&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; SystemData&lt;'a&gt; for PrefabLoader&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;Prefab&lt;T&gt;&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["amethyst_dynamic_ui"] = [{"text":"impl&lt;'a, C:&nbsp;ToLayoutElement&gt; SystemData&lt;'a&gt; for DynamicUiLoader&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;Prefab&lt;(Option&lt;UiTransformData&lt;()&gt;&gt;, Option&lt;UiButtonData&lt;u32&gt;&gt;, Option&lt;DynamicLayout&gt;, Option&lt;LayoutIdentifier&gt;, Option&lt;ExtraButtonData&gt;, Option&lt;Tinted&gt;, Option&lt;UiCachedImage&gt;, Option&lt;UiTextPrefab&gt;, C::PrefabData)&gt;&gt;&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, ScreenDimensions&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["amethyst_rendy"] = [{"text":"impl&lt;'a&gt; SystemData&lt;'a&gt; for ShapeUpload&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;Mesh&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["amethyst_ui"] = [{"text":"impl&lt;'a, G:&nbsp;PartialEq + Send + Sync + 'static, I:&nbsp;WidgetId&gt; SystemData&lt;'a&gt; for UiButtonBuilderResources&lt;'a, G, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;FontAsset&gt;&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;Texture&gt;&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Entities&lt;'a&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiImage&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, Interactable&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, Parent&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiText&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiTransform&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteExpect&lt;'a, Widgets&lt;UiButton, I&gt;&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiSoundRetrigger&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiButtonActionRetrigger&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, Selectable&lt;G&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, I:&nbsp;WidgetId&gt; SystemData&lt;'a&gt; for UiLabelBuilderResources&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: WidgetId,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;FontAsset&gt;&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Entities&lt;'a&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiText&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, UiTransform&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteExpect&lt;'a, Widgets&lt;UiLabel, I&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, C, W&gt; SystemData&lt;'a&gt; for UiLoader&lt;'a, C, W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: ToNativeWidget&lt;W&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: WidgetId,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadExpect&lt;'a, Loader&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Read&lt;'a, AssetStorage&lt;UiPrefab&lt;C::PrefabData, W&gt;&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, C, W&gt; SystemData&lt;'a&gt; for UiCreator&lt;'a, C, W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: ToNativeWidget&lt;W&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: WidgetId,<br>&nbsp;&nbsp;&nbsp;&nbsp;UiLoader&lt;'a, C, W&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Entities&lt;'a&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;WriteStorage&lt;'a, Handle&lt;UiPrefab&lt;C::PrefabData, W&gt;&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; SystemData&lt;'a&gt; for UiFinder&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Entities&lt;'a&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadStorage&lt;'a, UiTransform&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["amethyst_utils"] = [{"text":"impl&lt;'a, T&gt; SystemData&lt;'a&gt; for TagFinder&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Entities&lt;'a&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadStorage&lt;'a, Tag&lt;T&gt;&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["specs_hierarchy"] = [{"text":"impl&lt;'a, P&gt; SystemData&lt;'a&gt; for ParentData&lt;'a, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Component + Parent,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::Storage: Tracked,<br>&nbsp;&nbsp;&nbsp;&nbsp;Entities&lt;'a&gt;: SystemData&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ReadStorage&lt;'a, P&gt;: SystemData&lt;'a&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()