class SafeNavigationOperator {
  {
    a[0].b.c();
    a[0].super.c();
  }
}

//  seq(
//    choice(
//      field("name", $.identifier),
//      seq(
//        field("object", choice($.primary_expression, $.super)),
//        $._property_navigation,
//        optional(seq($.super, $._property_navigation)),
//        field("type_arguments", optional($.type_arguments)),
//        field("name", $.identifier)
//      )
//    ),
//    field("arguments", $.argument_list)
//  ),

