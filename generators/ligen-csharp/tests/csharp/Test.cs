using System;
using Rust;

namespace csharp {
    class Program {
        static void Main(string[] args) {
            Console.WriteLine(Test.Add(1, 2));
            Console.WriteLine(Test.GetName());
        }
    }
}
