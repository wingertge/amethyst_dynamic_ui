initSidebarItems({"enum":[["Primitive","The various primitives supported by this library."]],"fn":[["parse","Parses a wavefront `.obj` file, returning either the successfully parsed file, or an error. Support in this parser for the full file format is best-effort and realistically I will only end up supporting the subset of the file format which falls under the “things I see exported from blender” category."]],"struct":[["Geometry","A set of shapes, all using the given material."],["ObjSet","A set of objects, as listed in an `.obj` file."],["Object","A mesh object."],["Shape","A shape gathers a primitive and groups."],["TVertex","A single 3-dimensional point on a texture. “Texure Vertex”."],["Vertex","A single 3-dimensional point on the corner of an object."]],"type":[["GroupName","Name of a group."],["Normal","A single 3-dimensional normal"],["NormalIndex","An index into the `normals` array of an object."],["TextureIndex","An index into the `texture vertex` array of an object."],["VTNIndex","An index into the vertex array, with an optional index into the texture array. This is used to define the corners of shapes which may or may not be textured."],["VertexIndex","An index into the `vertices` array of an object, representing a vertex in the mesh. After parsing, this is guaranteed to be a valid index into the array, so unchecked indexing may be used."]]});