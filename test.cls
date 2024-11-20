class ExceptionClass {
  class InnerClass extends ExpressionClass {
    InnerClass() {
      this(1);
    }

    InnerClass(Integer i) {
      super(i);
    }
  }
}

