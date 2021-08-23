using System;
using System.Runtime.InteropServices;

namespace counter
{
    public struct Person {
        public int age;
        public float height;
    }

    class Program
    {
        [DllImport("counter.dll")]
        private static extern int test();

        static void Main(string[] args)
        {
            Person p;
            p.age = 33;
            p.height = 1.85f;

            Console.WriteLine("Hello World!");
            Console.WriteLine(Program.test());
            Console.WriteLine(p.age + ", " + p.height);
        }
    }
}
