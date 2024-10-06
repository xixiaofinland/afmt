class Conditional extends Object {
  void assignmentExpression() {
    Integer i = 0;
    Integer j = 1;
    if (i = j) {
    }
    if (i &= j) {
    }
    if (i |= j) {
    }
    if (i ^= j) {
    }
    if (i += j) {
    }
    if (i -= j) {
    }
    if (i *= j) {
    }
    if (i /= j) {
    }
    if (i <<= j) {
    }
    if (i >>= j) {
    }
    if (i >>>= j) {
    }
  }

  void ternaryExpression() {
    Integer a = 2;
    Integer b = 1;
    Integer c = a > b ? a : b;
    a < b ? System.debug('Hi') : System.debug('Hello');
  }

  void binaryExpression() {
    Integer i = 1;
    Integer j = 2;
    Integer a;

    a = i + j;
    a = i - j;
    a = i * j;
    a = i / j;
    a = i << j;
    a = i >> j;
    a = i >>> j;
    a = i ^ j;
    a = i & j;
    a = i | j;
  }

  void castExpression() {
    String a;

    Id b = (Id) a;
    Schema.SObjectType sobjectType = ((Id) a).getSObjectType();
  }

  void instanceOfExpression() {
    SObject o;
    Boolean b = o instanceof Account;
  }

  void booleanExpression() {
    Boolean a = true;
    Boolean b = false;
    Boolean c;

    c = a == b;
    c = a === b;
    c = a !== b;
    c = a != b;
    c = a <> b;
    c = a < b;
    c = a > b;
    c = a <= b;
    c = a >= b;
    c = a && b;
    c = a || b;

    // This is different than what you'd expect. jorje does not automatically
    // group this as left(anExtremelyVeryLongVariable > anotherVeryLongVariable)
    // and right (anExtremelyVeryLongVariable < anotherVeryLongVariable).
    // Instead it is
    // left (anExtremelyVeryLongVariable > anotherVeryLongVariable == anExtremelyVeryLongVariable)
    // and right (anotherVeryLongVariable).
    // Therefore the formatted code does not look as nice as Prettier's core,
    // but we can't change it because it will change the code's behavior.
  }

  void triggerVariableExpr_inOtherSuite() {
  }

  void packageVersionExpression() {
    Package.Version.Request;
  }

  void prefixExpression() {
    Integer a = 1;
    Integer b;

    b = +a;
    b = -a;
    b = !a;
    b = ~a;
    b = ++a;
    b = --a;
  }

  void postfixExpression() {
    Integer a = 1;
    Integer b;

    b = a++;
    b = a--;
  }

  void arrayExpression() {
    Integer b;

    b = a[0];
    b = a[a[0]];

    b = 'String Literal'[0];
    b = 'Very Long String Literal That Will Break The Line For Sure Without Help From Any Other Element'[0];

    String longResult = o[0].callMethod('first param');
  }

  void variableExpression() {
    Integer a = 1;
    Integer b = 2;
    Integer c = a + b;
  }

  void methodCallExpression() {
    SomeObject o = new SomeObject();
  }

  void javaMethodCallExpression_notNeeded() {
  }

  void javaVariableExpression_notNeeded() {
  }

  void superVariableExpression() {
    super.methodCall();
    super.methodCall(1, 2, 3);

    // Super cannot be followed by any whitespace, so we can't add newlines after it
    // even if the chain is very long
  }

  void thisVariableExpression() {
    this.a = 'Hello World';
    System.debug(this.a + this.b);
  }

  void longStaticMethodCallChainExpression() {
  }

  void classRefExpression() {
    System.Type t = Integer.class;
  }

  void literalExpression() {
    String a = 'Hello';
    Integer b = 1;
    Long c = 2147483648L;
    Double d = 0.1d;
    Decimal e = 2.14;
    Boolean f = true;
    Boolean g = false;
    Object h = null;
  }

  void dottedExprMethod() {
    a.Field__c = null;
    a.Field__c = null;
  }

  void soqlExpression_inOtherSuite() {
  }

  void soslExpression_inOtherSuite() {
  }

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
