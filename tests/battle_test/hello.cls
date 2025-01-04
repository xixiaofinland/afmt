public class TestClass {
  String name = 'Test Class';
  {
    String name2 = this // test comment 1
        .method1() // test comment 2
        .method2() // test comment 3
        .name // test comment 4
        ; // test comment 5
  }
}
