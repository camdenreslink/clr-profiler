using System;
using System.Threading;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            TMethod();
            FMethod();
        }
        static void TMethod()
        {
            Console.WriteLine("Hello, Tiny World!");
        }
        static void FMethod()
        {
            var s = "Hello, Fat World!";
            Console.WriteLine(s);
        }
    }
}
