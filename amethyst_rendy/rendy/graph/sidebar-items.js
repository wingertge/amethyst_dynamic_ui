initSidebarItems({"fn":[["gfx_acquire_barriers","Convert graph barriers into gfx barriers."],["gfx_release_barriers","Convert graph barriers into gfx barriers."],["is_metal","Check if backend is metal."]],"mod":[["present","Defines present node."],["render","Advanced render pass node. Will replace render pass node when polished."]],"struct":[["BufferAccess","Buffer access node will perform. Node must not perform any access to the buffer not specified in `access`. All access must be between logically first and last `stages`."],["BufferBarrier","Buffer pipeline barrier."],["BufferId","Id of the buffer in graph."],["DescBuilder","Builder for the node."],["Graph","Graph that renders whole frame."],["GraphBuilder","Build graph from nodes and resource."],["GraphContext","Graphics context contains all transient resources managed by graph."],["ImageAccess","Image access node wants to perform."],["ImageBarrier","Image pipeline barrier. Node implementation must insert it before first command that uses the image. Barrier must be inserted even if this node doesn’t use the image."],["ImageId","Id of the image (or target) in graph."],["NodeBuffer","Buffer shared between nodes."],["NodeId","Id of the node in graph."],["NodeImage","Image shared between nodes."]],"trait":[["DynNode","Trait-object safe `Node`."],["Node","The node is building block of the framegraph. Node defines set of resources and operations to perform over them. Read-only data for operations comes from auxiliary data source `T`."],["NodeBuilder","Dynamic node builder that emits `DynNode`."],["NodeDesc","Description of the node. Implementation of the builder type provide framegraph with static information about node that is used for building the node."],["NodeSubmittable","NodeSubmittable"]]});