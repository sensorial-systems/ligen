using NUnit.Framework;
using Rust;

namespace tests {
    public class Tests {
        [SetUp]
        public void Setup() {}

        [Test]
        public void Add() {
            Assert.AreEqual(3, Test.Add(1, 2));
        }

//        [Test]
//        public void GetName() {
//            Assert.AreEqual("Name", Test.GetName());
//        }
    }
}