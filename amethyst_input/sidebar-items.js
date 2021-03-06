initSidebarItems({"enum":[["Axis","Represents any input represented by a float value from -1 to 1. Retrieve the value of this with axis_value."],["BindingError","An enum of possible errors that can occur when binding an action or axis."],["BindingsFileError","An error occurred while loading the bindings file."],["Button","A Button is any kind of digital input that the engine supports."],["ControllerAxis","Controller axes matching SDL controller model"],["ControllerButton","Controller buttons matching SDL controller model"],["ControllerEvent","Controller events generated by the SDL events system."],["ElementState","Describes the input state of a key."],["InputEvent","Events generated by the input system"],["MouseAxis","Mouse axis"],["ScrollDirection","Indicates in what direction a mouse wheel scroll event was."],["VirtualKeyCode","Symbolic name for a keyboard key."]],"fn":[["get_input_axis_simple","Gets the input axis value from the `InputHandler`. If the name is None, it will return the default value of the axis (0.0)."],["get_key","If this event was for manipulating a keyboard key then this will return the `VirtualKeyCode` and the new state."],["get_mouse_button","If this event was for manipulating a mouse button, this will return the `MouseButton` and the new state."],["is_close_requested","Returns true if the event passed in is a request to close the game window."],["is_key_down","Returns true if the event passed in is a key down event for the provided `VirtualKeyCode`."],["is_key_up","Returns true if the event passed in is a key up event for the provided `VirtualKeyCode`."],["is_mouse_button_down","Returns true if the event passed in is a mouse button down event for the provided `MouseButton`."]],"struct":[["Bindings","Used for saving and loading input settings."],["InputBundle","Bundle for adding the `InputHandler`."],["InputHandler","This struct holds state information about input devices."],["InputSystem","Input system"],["InputSystemDesc","Builds an `InputSystem`."],["StringBindings","The builtin `BindingTypes` implementation, set of types for binding configuration keys. Uses `String` for both axes and actions. Usage of this type is discouraged and it’s meant mainly for prototypes. Check `BindingTypes` for examples."]],"trait":[["BindingTypes","Define a set of types used for bindings configuration. Usually defaulted to `StringBindings`, which uses `String`s."]]});