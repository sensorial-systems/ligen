        [DllImport("openlimits_csharp", EntryPoint = "{{ffi_name}}", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        unsafe private static extern {{{ffi_return_type}}} FFI{{name}}({{{ffi_parameters}}});

        public {{static}} {{{return_type}}} {{name}}({{{parameters}}}) {
            return FFI{{name}}({{arguments}});
        }
