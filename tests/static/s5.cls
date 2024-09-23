public class Me {
  {
    String name = [ SELECT Name FROM Account WHERE Id = :A.Id ].Name;
  }
}
