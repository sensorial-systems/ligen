//! Generator macro.

ligen::define_binding_generator!(name = "ligen_csharp", generator = "ligen_csharp_core::Generator");
// Or if you want to create a project generator:
// ligen::define_project_generator!(name = "ligen_csharp", generator = "ligen_csharp_core::Generator");