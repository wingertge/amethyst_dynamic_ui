initSidebarItems({"enum":[["Value","Represents any valid JSON value."]],"mod":[["accessor","Contains `Accessor` and other related data structures."],["animation","Contains `Animation` and other related data structures."],["asset","Contains `Asset` metadata."],["buffer","Contains `Buffer`, `View`, and other related data structures."],["camera","Contains `Camera` and other related data structures."],["deserialize","Re-exports of `serde_json` deserialization functions."],["extensions","Contains extension specific data structures and the names of all 2.0 extensions supported by the library."],["extras","Contains `Extras`."],["image","Contains `Image` and other related data structures."],["material","Contains `Material` and other related data structures."],["mesh","Contains `Mesh` and other related data structures."],["path","Contains `Path`."],["root","Contains `Root`."],["scene","Contains `Scene`, `Node`, and other related data structures."],["serialize","Re-exports of `serde_json` serialization functions."],["skin","Contains `Skin` and other related data structures."],["texture","Contains `Texture`, `Sampler`, and other related data structures."],["validation","Contains functions that validate glTF JSON data against the specification."]],"struct":[["Accessor","A typed view into a buffer view."],["Animation","A keyframe animation."],["Asset","Metadata about the glTF asset."],["Buffer","A buffer points to binary data representing geometry, animations, or skins."],["Camera","A camera's projection."],["Error","This type represents all possible errors that can occur when serializing or deserializing JSON data."],["Image","Image data used to create a texture."],["Index","Represents an offset into an array of type `T` owned by the root glTF object."],["Material","The material appearance of a primitive."],["Mesh","A set of primitives to be rendered."],["Node","A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes. A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present."],["Path","An immutable JSON source path."],["Root","The root object of a glTF 2.0 asset."],["Scene","The root `Node`s of a scene."],["Skin","Joints and matrices defining a skin."],["Texture","A texture and its sampler."]],"type":[["Extras","Data type of the `extras` attribute on all glTF objects."]]});