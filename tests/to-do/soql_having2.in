class SoqlHaving2 {
  {
    [SELECT COUNT(Id)
      FROM Lead
      GROUP BY LeadSource
      HAVING (COUNT(Name) > 100 AND LeadSource LIKE 'San%') OR (LeadSource = 'San%' AND LeadSource IN ('Val1', 'Val2'))];
  }
}
