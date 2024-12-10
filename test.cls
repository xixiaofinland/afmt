public class Me {
  {
    List<List<SObject>> res = [
      FIND :myString1
      IN ALL FIELDS
      RETURNING
        Account(Id, FirstName, LastName, ThisGoodOne, Name WHERE Name LIKE :myString2 LIMIT :myInt3),
        Contact,
        Opportunity,
        Lead
      WITH DIVISION = :myString4
      LIMIT :myInt5
    ];
  }
}
