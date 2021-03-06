initSidebarItems({"struct":[["ArcBallControlBundle","The bundle that creates an arc ball movement system. Note: Will not actually create a moving entity. It will only register the needed resources and systems. The generic parameters A and B are the ones used in InputHandler<A,B>. You might want to add “fly_movement” and “free_rotation” as dependencies of the TransformSystem. Adding this bundle will grab the mouse, hide it and keep it centered."],["ArcBallControlTag","To add an arc ball behaviour, add this to a camera which already has the FlyControlTag added."],["ArcBallRotationSystem","The system that manages the arc ball movement; In essence, the system will align the camera with its target while keeping the distance to it and while keeping the orientation of the camera."],["ControlTagPrefab","`PrefabData` for loading control tags on an `Entity`"],["CursorHideSystem","System which hides the cursor when the window is focused. Requires the usage MouseFocusUpdateSystem at the same time."],["CursorHideSystemDesc","Builds a `CursorHideSystem`."],["FlyControlBundle","The bundle that creates a flying movement system."],["FlyControlTag","Add this to a camera if you want it to be a fly camera. You need to add the FlyControlBundle or the required systems for it to work."],["FlyMovementSystem","The system that manages the fly movement."],["FlyMovementSystemDesc","Builds a `FlyMovementSystem`."],["FreeRotationSystem","The system that manages the view rotation."],["FreeRotationSystemDesc","Builds a `FreeRotationSystem`."],["HideCursor","Resource indicating if the mouse should be grabbed and hidden by the CursorHideSystem when the window is focused."],["MouseFocusUpdateSystem","A system which reads Events and saves if a window has lost focus in a WindowFocus resource"],["MouseFocusUpdateSystemDesc","Builds a `MouseFocusUpdateSystem`."],["WindowFocus","Struct which holds information about whether the window is focused. Written to by MouseFocusUpdateSystem"]]});