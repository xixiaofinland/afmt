class A {
  String propertyMemberWithGet {
    get {
    }
  }
  String propertyMemberWithSet {
    set {
    }
  }

  public String propertyMemberWithGetPrivateSet {
    get {
      return 1;
    }
    private set;
  }
  String propertyMemberWithOnlySet {
    set {
      memberThree = value;
    }
  }
  private String privatePropertyMemberWithOnlyGet {
    get {
      return 2;
    }
  }
}
