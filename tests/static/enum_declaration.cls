public class EnumDeclaration {
  public enum Plain {
    ALPHA,
    BETA
  }

  public enum WithInterface implements FirstImplement {
    ALPHA,
    BETA
  }

  private enum WithInterfaces implements FirstImplement, SecondImplement {
    ALPHA
  }

  public enum Empty {
  }
}
