        public static implicit operator string(FFIString from) {
            unsafe {
                return Marshal.PtrToStringUTF8(from.GetPointer());
            }
        }

        public static implicit operator FFIString(string from) {
            unsafe {
                return new FFIString(Marshal.StringToHGlobalAnsi(from));
                // FIXME: Memory leak https://docs.microsoft.com/pt-br/dotnet/api/system.runtime.interopservices.marshal.freehglobal?view=net-5.0
            }
        }

        public static implicit operator IntPtr(FFIString from) {
            unsafe {
                return from.GetPointer();
            }
        }
