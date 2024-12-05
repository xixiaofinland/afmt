public class VariableDeclaration {
  public void declareVariables() {
    String firstString, secondString;
    String a = 'aaa', b = 'bbb';
    String c = 'c';
    String d = 'd';
    String e = c, d = a;
    HttpRequest request = new HttpRequest();
    List<String> anotherArray = new String[1];
    String[] emptyStrings;
    Integer maxNums = 5;
    stringArrays = new List<String>();

    List<String> anotherArray = new List<String>(maxNums);
    List<List<String>> listOfList = new List<List<String>>();
    listOfList = new List<List<String>>(0);
    listOfList = new List<List<String>>(maxNums);
  }
}
