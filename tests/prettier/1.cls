public class Me {
  class InnerClass extends ExpressionClass {
    InnerClass() {
      // ThisMethodCallExpression
      this(1);
    }

    InnerClass(Integer i) {
      // SuperMethodCallExpression
      super(i);
    }
  }
}
