initSidebarItems({"enum":[["OutputColor","Definition of render target color output image."],["RenderOrder","Collection of predefined constants for action ordering in the builtin targets. Two actions with the same order will be applied in their insertion order. The list is provided mostly as a comparison point. If you can’t find the exact ordering you need, provide custom `i32` that fits into the right place."],["RenderableAction","An action that represents a single transformation to the render graph, e.g. addition of single render group."],["Target","An identifier for render target used in render plugins. Predefined targets are part of default rendering flow used by builtin amethyst render plugins, but the list can be arbitrarily extended for custom usage in user plugins using custom str identifiers."],["TargetImage","An identifier for output image of specific render target."]],"struct":[["ImageOptions","Set of options required to create an image node in render graph."],["RenderPlan","Builder of a rendering plan for specified target."],["RenderingBundle","A bundle of systems used for rendering using `Rendy` render graph."],["TargetMetadata","Metadata for a planned render target. Defines effective size and layer count that target’s renderpass will operate on."],["TargetPlanContext","A planning context focused on specific render target."],["TargetPlanOutputs","Definition for set of outputs for a given render target."]],"trait":[["IntoAction","Trait for easy conversion of various types into `RenderableAction` shell."],["RenderPlugin","Basic building block of rendering in [RenderingBundle]."]]});