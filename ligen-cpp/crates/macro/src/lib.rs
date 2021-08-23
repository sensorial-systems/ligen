//! Generator macro.

ligen::define_binding_generator!(name = "ligen_cpp", generator = "ligen_cpp_core::Generator");
// Or if you want to create a project generator:
// ligen::define_project_generator!(name = "ligen_cpp", generator = "ligen_cpp_core::Generator");