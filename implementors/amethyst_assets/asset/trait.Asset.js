(function() {var implementors = {};
implementors["amethyst_animation"] = [{"text":"impl&lt;T&gt; Asset for Sampler&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: InterpolationPrimitive + Send + Sync + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Asset for Animation&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AnimationSampling,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["amethyst_audio"] = [{"text":"impl Asset for Source","synthetic":false,"types":[]}];
implementors["amethyst_locale"] = [{"text":"impl Asset for Locale","synthetic":false,"types":[]}];
implementors["amethyst_rendy"] = [{"text":"impl Asset for Material","synthetic":false,"types":[]},{"text":"impl Asset for SpriteSheet","synthetic":false,"types":[]},{"text":"impl Asset for Mesh","synthetic":false,"types":[]},{"text":"impl Asset for Texture","synthetic":false,"types":[]}];
implementors["amethyst_ui"] = [{"text":"impl Asset for FontAsset","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()