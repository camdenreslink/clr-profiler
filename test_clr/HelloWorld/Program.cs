﻿using System;
using System.IO;
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
            try
            {
                using (var sr = new StreamReader("/home/creslink/git/camdenreslink/clr-profiler/README.md"))
                {
                    sr.ReadToEnd();
                }
            }
            catch (Exception e)
            {
                throw e;
            }
            try
            {
                var x0 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x1 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x2 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x3 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x4 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x5 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x6 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
                var x7 = String.IsNullOrWhiteSpace("s").GetHashCode().GetHashCode().GetHashCode().GetHashCode();
            }
            catch (Exception e)
            {
                throw e;
            }
        }
    }
}
